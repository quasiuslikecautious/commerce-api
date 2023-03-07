use uuid::uuid;
use serde::Serialize;
use std::convert::From;

use crate::db::User;
use crate::jwt::lib::*;
use crate::jwt::models::claims::Claims;


#[derive(Serialize)]
pub struct UserAuthPayload {
    pub token: String,
}

impl From<User> for UserAuthPayload {
    fn from(user: User) -> Self {
        let claims = Claims::new(user.uuid.unwrap(), uuid!("d582df1f-3642-4191-b822-0c9a73719259"), user.role);
        let token = encrypt_jwt(&get_secret(), claims).unwrap();

        Self {
            token: token,
        }
    }
}
