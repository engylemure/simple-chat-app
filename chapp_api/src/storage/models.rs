use chrono::{DateTime, Utc};
use ulid::Ulid;

#[derive(Clone)]
pub struct User {
  pub id: Ulid,
  pub name: Option<String>,
  pub created_at: DateTime<Utc>
}

impl User {
  pub fn new() -> User {
    User {
      id: Ulid::new(),
      name: None,
      created_at: Utc::now()
    }
  }
}

#[derive(Clone)]
pub struct Room {
  pub id: Ulid,
  pub name: String,
  pub messages: Vec<Ulid>,
  pub created_at: DateTime<Utc>
}


impl Room {
  pub fn new(name: String) -> Room {
    Self {
      id: Ulid::new(),
      name,
      messages: vec![],
      created_at: Utc::now()
    }
  }
}

#[derive(Clone, Debug)]
pub struct Message {
  pub id: Ulid,
  pub room_id: Ulid,
  pub user_id: Ulid,
  pub content: String,
  pub created_at: DateTime<Utc>,
  pub replied_to: Option<Ulid>
}


impl Message {
  pub fn new(user_id: &Ulid, room_id: Ulid, content: String, replied_to: Option<Ulid>) -> Self {
    Self {
      id: Ulid::new(),
      user_id: user_id.clone(),
      content,
      created_at: Utc::now(),
      replied_to,
      room_id,
    }
  }
}
