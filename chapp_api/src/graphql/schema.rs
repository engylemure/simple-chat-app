use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    futures_util::future::join,
    Context, Error, InputObject, InputValueError, InputValueResult, Object, Result, Scalar,
    ScalarType, Subscription, Value, ID,
};
use chrono::Utc;
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};
use ulid::Ulid;

use crate::storage::{
    models::{Message, Room, User},
    storage::Storage,
};

pub struct UserID(pub Ulid);

struct DateTime(chrono::DateTime<Utc>);

#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: Value) -> InputValueResult<Self> {
        chrono::DateTime::parse_from_rfc3339(&value.to_string())
            .map_err(|_| InputValueError::custom("Invalid date, expected rfc3339"))
            .map(|dt| DateTime(chrono::DateTime::from(dt)))
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}

#[Object]
impl User {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }
    async fn name(&self) -> &Option<String> {
        &self.name
    }
    async fn created_at(&self) -> DateTime {
        DateTime(self.created_at)
    }
}

#[Object]
impl Room {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn messages(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, Message, EmptyFields, EmptyFields>> {
        let storage = ctx.data_unchecked::<Storage>();
        let messages: Vec<Message> = {
            let messages = storage.messages.lock().await;
            self.messages
                .iter()
                .filter_map(|id| messages.get(id).map(Clone::clone))
                .collect()
        };
        query(
            after,
            before,
            first,
            last,
            |after, before, first, last| async move {
                let mut start = after
                    .map(|after: String| {
                        let id = Ulid::from_string(&after).ok();
                        match id {
                            Some(id) => messages
                                .iter()
                                .enumerate()
                                .find_map(|(idx, msg)| (msg.id == id).then_some(idx)),
                            _ => None,
                        }
                    })
                    .flatten()
                    .unwrap_or(0);
                let mut end = before
                    .map(|after: String| {
                        let id = Ulid::from_string(&after).ok();
                        match id {
                            Some(id) => messages
                                .iter()
                                .enumerate()
                                .find_map(|(idx, msg)| (msg.id == id).then_some(idx)),
                            _ => None,
                        }
                    })
                    .flatten()
                    .unwrap_or(messages.len());
                if let Some(first) = first {
                    if end - start > first {
                        end = start + first;
                    }
                }
                if let Some(last) = last {
                    if end - start > last {
                        start = end - last;
                    }
                }
                let mut connection = Connection::new(start > 0, end < messages.len());
                connection.edges.extend(
                    messages
                        .into_iter()
                        .skip(start)
                        .take(end - start)
                        .map(|m| Edge::with_additional_fields(m.id.to_string(), m, EmptyFields)),
                );
                Ok::<_, async_graphql::Error>(connection)
            },
        )
        .await
    }

    async fn created_at(&self) -> DateTime {
        DateTime(self.created_at)
    }
}

#[derive(InputObject, Debug)]
struct CreateMessageInput {
    content: String,
    room_id: ID,
    replied_to: Option<ID>,
}

#[Object]
impl Message {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }

    async fn user(&self, ctx: &Context<'_>) -> Option<User> {
        let storage = ctx.data_unchecked::<Storage>();
        storage
            .users
            .lock()
            .await
            .get(&self.user_id)
            .map(Clone::clone)
    }

    async fn content(&self) -> &str {
        &self.content
    }

    async fn replied_to(&self, ctx: &Context<'_>) -> Option<Message> {
        let replied_to = self.replied_to?;
        let storage = ctx.data_unchecked::<Storage>();
        storage
            .messages
            .lock()
            .await
            .get(&replied_to)
            .map(Clone::clone)
    }

    async fn created_at(&self) -> DateTime {
        DateTime(self.created_at)
    }
}

pub struct Query;

#[Object]
impl Query {
    async fn me(&self, ctx: &Context<'_>) -> Option<User> {
        let user_id = ctx.data_opt::<UserID>()?;
        ctx.data_unchecked::<Storage>()
            .users
            .lock()
            .await
            .get(&user_id.0)
            .map(Clone::clone)
    }

    async fn room(&self, ctx: &Context<'_>, room_id: ID) -> Option<Room> {
        let room_id = Ulid::from_string(room_id.as_str()).ok()?;
        ctx.data_unchecked::<Storage>()
            .rooms
            .lock()
            .await
            .get(&room_id)
            .map(|v| v.0.clone())
    }

    async fn rooms(&self, ctx: &Context<'_>) -> Vec<Room> {
        ctx.data_unchecked::<Storage>()
            .rooms
            .lock()
            .await
            .values()
            .map(|v| v.0.clone())
            .collect()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn update_user(&self, ctx: &Context<'_>, name: String) -> Option<User> {
        if name.len() < 4 {
            return None;
        }
        let user_id = ctx.data_opt::<UserID>()?;
        ctx.data_unchecked::<Storage>()
            .users
            .lock()
            .await
            .get_mut(&user_id.0)
            .map(|user| {
                user.name = Some(name);
                user.clone()
            })
    }
    async fn create_user(&self, ctx: &Context<'_>, name: Option<String>) -> User {
        let mut user = User::new();
        user.name = name;
        ctx.data_unchecked::<Storage>()
            .users
            .lock()
            .await
            .insert(user.id.clone(), user.clone());
        user
    }

    async fn create_room(&self, ctx: &Context<'_>, room_name: String) -> Room {
        let room = Room::new(room_name);
        ctx.data_unchecked::<Storage>()
            .rooms
            .lock()
            .await
            .insert(room.id.clone(), (room.clone(), broadcast::channel(100).0));
        room
    }

    async fn send_message(&self, ctx: &Context<'_>, input: CreateMessageInput) -> Result<Message> {
        if input.content.len() == 0 {
            return Err(Error::new("content shouln't be empty"));
        }
        let storage = ctx.data_unchecked::<Storage>();
        let user = storage
            .users
            .lock()
            .await
            .get(
                &ctx.data_opt::<UserID>()
                    .ok_or(Error::new("user_id not provided"))?
                    .0,
            )
            .map(Clone::clone)
            .ok_or(Error::new("User not found"))?;
        let room_id =
            Ulid::from_string(input.room_id.as_str()).map_err(|_| Error::new("Invalid roomId"))?;
        let message = Message::new(
            &user.id,
            room_id.clone(),
            input.content,
            input
                .replied_to
                .map(|id| Ulid::from_string(id.as_str()).ok())
                .flatten(),
        );
        let (mut messages, mut rooms) = join(storage.messages.lock(), storage.rooms.lock()).await;
        let (room, msg_sender) = rooms
            .get_mut(&room_id)
            .ok_or(Error::new("Room not found"))?;
        room.messages.push(message.id.clone());
        if msg_sender.receiver_count() > 0 {
            let _ = msg_sender.send(message.clone());
        }
        messages.insert(message.id.clone(), message.clone());
        Ok(message)
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn room_messages(
        &self,
        ctx: &Context<'_>,
        room_id: ID,
    ) -> Result<impl Stream<Item = Message>> {
        let storage = ctx.data_unchecked::<Storage>();
        let room_id =
            Ulid::from_string(room_id.as_str()).map_err(|_| Error::new("Invalid room_id"))?;
        let receiver = storage
            .rooms
            .lock()
            .await
            .get(&room_id)
            .ok_or(Error::new("Room not found"))?
            .1
            .subscribe();
        let stream = BroadcastStream::new(receiver).filter_map(|i| i.ok());
        Ok(stream)
    }
}
