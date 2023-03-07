use diesel::prelude::*;
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use crate::db::models::schema;

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(primary_key(id), table_name = schema::sessions)]
pub struct UserSession {
    pub id: String,
    pub session_data: Option<String>,
    pub expires_at: chrono::NaiveDateTime,
    pub user_agent: Option<String>,
    pub last_activity: chrono::NaiveDateTime,
    pub ip: Option<String>,
    pub user_id: Option<Uuid>,
}

impl UserSession {
    pub fn new(
        id: String, 
        session_data: Option<String>,
        expires_at: chrono::NaiveDateTime,
        user_agent: Option<String>,
        last_activity: chrono::NaiveDateTime,
        ip: Option<String>,
        user_id: Option<Uuid>,
    ) -> Self {
        Self {
            id,
            session_data,
            expires_at,
            user_agent,
            last_activity,
            ip,
            user_id,
        }
    }
}
