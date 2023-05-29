use async_graphql::{
    futures_util::future::join, Context, Error, InputObject, Object, Result, Schema, Subscription,
    ID,
};
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, Stream, StreamExt};
use ulid::Ulid;

use crate::storage::{
    models::{Message, Room, User},
    storage::Storage,
};

pub struct UserID(pub Ulid);

#[Object]
impl User {
    async fn id(&self) -> ID {
        ID(self.id.to_string())
    }
    async fn name(&self) -> &Option<String> {
        &self.name
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

    async fn messages(&self, ctx: &Context<'_>) -> Vec<Message> {
        let storage = ctx.data_unchecked::<Storage>();
        let messages = storage.messages.lock().await;
        self.messages
            .iter()
            .filter_map(|id| messages.get(id).map(Clone::clone))
            .collect()
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
}

pub type ChappSchema = Schema<Query, Mutation, Subscription>;

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
    async fn create_user(&self, ctx: &Context<'_>) -> User {
        let user = User::new();
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

    async fn send_message(&self, ctx: &Context<'_>, input: CreateMessageInput) -> Option<Message> {
        let storage = ctx.data_unchecked::<Storage>();
        let user_id = ctx.data_opt::<UserID>()?;
        let room_id = Ulid::from_string(input.room_id.as_str()).ok()?;
        let message = Message::new(
            &user_id.0,
            room_id.clone(),
            input.content,
            input
                .replied_to
                .map(|id| Ulid::from_string(id.as_str()).ok())
                .flatten(),
        );
        let (mut messages, mut rooms) = join(storage.messages.lock(), storage.rooms.lock()).await;
        let (room, msg_sender) = rooms.get_mut(&room_id)?;
        room.messages.push(message.id.clone());
        msg_sender.send(message.clone());
        messages.insert(message.id.clone(), message.clone());
        Some(message)
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
