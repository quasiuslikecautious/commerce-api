use uuid::Uuid;
use serde::Serialize;
use std::convert::From;

use crate::db::models::user::User;

#[derive(Serialize)]
pub struct UserData {
    pub uuid: Uuid,
    pub email: String,
}

impl From<User> for UserData {
    fn from(user: User) -> Self {
        UserData {
            uuid: user.uuid.unwrap(),
            email: user.email,
        }
    }
}
