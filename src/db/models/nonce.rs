use base64::{Engine as _, engine::general_purpose};
use diesel::prelude::*;
use rand::Rng;
use serde::{ Serialize, Deserialize };

use crate::db::models::schema;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(primary_key(session_id), table_name = schema::nonces)]
pub struct Nonce {
    pub nonce: String,
    pub created_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub session_id: String,
}

impl Nonce {
    pub fn generate(length: usize) -> String {
        let random_bytes: Vec<u8> = (0..length).map(|_| rand::thread_rng().gen()).collect();
        general_purpose::URL_SAFE_NO_PAD.encode(random_bytes)
    }
}
