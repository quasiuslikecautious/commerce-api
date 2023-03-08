use base64::{Engine as _, engine::general_purpose};
use diesel::{ prelude::*, RunQueryDsl, QueryDsl, };
use dotenvy::dotenv;
use rand::Rng;
use ring::{ digest, hmac, rand::SystemRandom };
use serde::{ Serialize, Deserialize };
use std::{ env, time::{ SystemTime, UNIX_EPOCH }};

use super::schema;
use crate::establish_connection;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(primary_key(session_id), table_name = schema::nonces)]
pub struct Nonce {
    pub nonce: String,
    pub session_id: String,
    pub key: String,
    pub created_at: i64,
}

impl Nonce {
    pub fn new(session_id: &str) -> Self {
        let nonce_rand_bytes: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();
        let nonce = general_purpose::URL_SAFE_NO_PAD.encode(nonce_rand_bytes);

        let rng = SystemRandom::new();
        let key_value: [u8; digest::SHA384_OUTPUT_LEN] = ring::rand::generate(&rng).expect("Failed to sign nonce").expose();
        let key = general_purpose::URL_SAFE_NO_PAD.encode(key_value);
        
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        Self {
            nonce,
            key,
            created_at: timestamp as i64,
            session_id: session_id.to_string(),
        }
    }

    pub fn take(sid: &str) -> Option<Nonce> {
        use super::schema::nonces::dsl::*;

        let connection = &mut crate::establish_connection();
        let result = connection.build_transaction()
            .read_only()
            .run(|conn| {
                nonces
                    .filter(session_id.eq(&sid))
                    .first::<Self>(conn)
            });

        if let Some(_) = result.as_ref().ok() {
            connection.build_transaction()
                .read_write()
                .run(|conn| {
                    diesel::delete(
                        nonces
                            .filter(session_id.eq(&sid))
                    ).execute(conn)
                })
                .unwrap();
        };

        result.ok()
    }

    pub fn insert(&self) -> Option<()> {
        use schema::nonces::dsl::*;

        let connection = &mut establish_connection();
        let response = connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(nonces)
                    .values(
                        self,
                    )
                    .on_conflict(session_id)
                    .do_update()
                    .set((
                        nonce.eq(&self.nonce),
                        key.eq(&self.key),
                    ))
                    .execute(conn)
            });

        match response.ok() {
            Some(_) => Some(()),
            None => None,
        }
    }

    pub fn validate(&self, tag: &str) -> bool {

        if self.is_expired() {
            return false;
        }

        let secret = Self::get_secret();
        let msg = format!("{}:{}:{}", self.created_at, self.nonce, secret);

        let key_value = match general_purpose::URL_SAFE_NO_PAD.decode(&self.key) {
            Ok(data) => data,
            Err(_) => return false,
        };

        let key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());

        hmac::verify(&key, &msg.as_ref(), tag.as_ref()).ok().is_some()
    }

    pub fn get_hmac(&self) -> String {
        dotenv().ok();
        let secret = Self::get_secret();

        let message = format!("{}:{}:{}", self.created_at, self.nonce, secret);
        
        let key_value = general_purpose::URL_SAFE_NO_PAD.decode(self.key.clone()).unwrap();
        let key = hmac::Key::new(hmac::HMAC_SHA384, key_value.as_ref());

        let tag = hmac::sign(&key, message.as_bytes());

        let key_string = general_purpose::URL_SAFE_NO_PAD.encode(key_value);
        println!("{}", &key_string);

        hex::encode(tag.as_ref())
    }

    pub fn is_expired(&self) -> bool {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        timestamp - (self.created_at as u64) > (60 * 5)
    }

    fn get_secret() -> String {
        dotenv().ok();
        let secret = env::var("NONCE_SECRET")
            .expect("NONCE_SECRET must be set")
            .as_bytes()
            .to_vec();

        hex::encode(secret)
    }
}
