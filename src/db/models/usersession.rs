use diesel::{ prelude::*, RunQueryDsl, };
use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use super::schema;
use crate::db::establish_connection;

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
        expires_at: Option<chrono::NaiveDateTime>,
        user_agent: Option<String>,
        ip: Option<String>,
        user_id: Option<Uuid>,
    ) -> Self {
        let now = chrono::Utc::now().naive_utc();
        let expires = expires_at.unwrap_or(
            now + chrono::Duration::hours(8)
        );

        Self {
            id,
            session_data,
            expires_at: expires,
            user_agent,
            last_activity: now,
            ip,
            user_id,
        }
    }

    /// Due to the way axum session layer currently works, on session load the layer is given a sort of
    /// initial session id, and then on session store a new session id is generated and used for the
    /// rest of the session. As such, when we receive a request on a route that requires a valid
    /// session id to already exist in the database (i.e. the fk requirement of nonces on the sessions
    /// table), we must insert the new session id if it does not already exist. Hopefully this is a
    /// temporary fix.
    pub fn redundant_guarantee(sid: &str) -> Result<usize, diesel::result::Error>{
        use schema::sessions::dsl::*;

        let connection = &mut establish_connection();
        let response = connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(sessions)
                        .values(
                            UserSession::new(
                                sid.to_string(),
                                None,
                                None,
                                None,
                                None,
                                None
                            )
                        )
                        .on_conflict_do_nothing()
                        .execute(conn)
            });

        response
    }
}
