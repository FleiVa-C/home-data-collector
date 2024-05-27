use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uuid: Option<String>,
    pub name: String,
    pub firstname: String,
    pub email: String,
    pub password: String,
    pub ts_register: i64,
    pub is_admin: bool,
}

impl User {
    pub fn add_uuid(&mut self) {
        self.uuid = Some(Uuid::new_v4().to_string());
    }
}
