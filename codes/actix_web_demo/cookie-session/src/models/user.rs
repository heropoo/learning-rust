use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
}
