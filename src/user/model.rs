use serde::{Serialize, Deserialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String
}

impl User {
    pub fn new(email: String) -> User {
        User {
            id: Ulid::new().to_string().to_lowercase(),
            email
        }
    }
}