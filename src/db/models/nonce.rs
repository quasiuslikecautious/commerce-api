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
    pub fn new(session_id: &str) -> Self {
        let now = chrono::Utc::now().naive_utc();
        let expiry = now + chrono::Duration::minutes(5);

        Self {
            nonce: Self::generate_hash(64),
            created_at: now,
            expires_at: expiry,
            session_id: session_id.to_string(),
        }
    }

    pub fn generate_hash(length: usize) -> String {
        let random_bytes: Vec<u8> = (0..length).map(|_| rand::thread_rng().gen()).collect();
        general_purpose::URL_SAFE_NO_PAD.encode(random_bytes)
    }
}
