use std::collections::BTreeMap;
use tokio::sync::broadcast::{Sender};
use std::sync::Arc;
use tokio::sync::Mutex;
use ulid::Ulid;
use super::models::{Message, Room, User};

#[derive(Default)]
pub struct Storage {
  pub rooms: Arc<Mutex<BTreeMap<Ulid, (Room, Sender<Message>)>>>,
  pub users: Arc<Mutex<BTreeMap<Ulid, User>>>,
  pub messages: Arc<Mutex<BTreeMap<Ulid, Message>>>,
}




