use async_trait::async_trait;
use async_session::{ Result, Session, serde_json, SessionStore };
use diesel::{ pg::PgConnection, prelude::* };
use dotenvy::dotenv;
use log::{ trace };
use std::env;
use uuid::Uuid;

use crate::db::UserSession;

#[derive(Default, Debug, Clone)]
pub struct PostgresSessionStore {
    pub database_url: String,
}

impl PostgresSessionStore {
    pub fn new(database_url: String) -> Self {
        Self {
            database_url,
        }
    }

    pub fn connection(&self) -> PgConnection {
        trace!("Starting connection to PostgreSQL...");
        dotenv().ok();
    
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}

#[async_trait]
impl SessionStore for PostgresSessionStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        use crate::schema::sessions::dsl::*;

        let sid = Session::id_from_cookie_value(&cookie_value).unwrap().to_string();
        println!("load session ({})", &sid);

        let user_session = UserSession::new(
            sid.clone(),
            None,
            None,
            None,
            None,
            None
        );

        let mut connection = self.connection();
        let result = connection.build_transaction()
            .read_only()
            .run(|conn| {
                sessions
                    .filter(id.eq(&sid))
                    .filter(expires_at.ge(&user_session.last_activity))
                    .first::<UserSession>(conn)
            });
        
        match result {
            Ok(data) => {
                println!("{:?}", data);
                let session = data.session_data
                    .map(|session| serde_json::from_str::<Session>(&session))
                    .transpose();
                if session.is_ok() {
                    println!("parsed session: {:?}", session);
                }
                Ok(session?)

            },
            Err(_) => {
                Err(async_session::Error::msg("The jank continues"))
            },
        }
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        use crate::schema::sessions::dsl::*;

        let sid = session.id().to_string();
        println!("store session ({})", &sid);
        let s_data = Some(serde_json::to_string(&session)?);
        let s_user_id = session.get::<Uuid>("user_id");

        let expiry = session.expiry().map(|s| s.naive_utc());

        let user_session = UserSession::new(
            sid,
            s_data.clone(),
            expiry,
            None,
            None,
            s_user_id
        );

        let mut connection = self.connection();
        let result = connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::insert_into(sessions)
                    .values(&user_session)
                    .on_conflict(id)
                    .do_update()
                    .set((
                        session_data.eq(&user_session.session_data),
                        last_activity.eq(&user_session.last_activity),
                    ))
                    .execute(conn)
            });

        match result {
            Ok(_) => Ok(session.into_cookie_value()),
            Err(e) => Err(async_session::Error::new(e)),
        }

    }

    async fn destroy_session(&self, session: Session) -> Result {
        use crate::schema::sessions::dsl::*;

        let sid = session.id();

        let mut connection = self.connection();
        let result = connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::delete(
                    sessions.filter(id.eq(sid))
                )
                .execute(conn)
            });

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(async_session::Error::new(e)),
        }
    }

    async fn clear_store(&self) -> Result {
        use crate::schema::sessions::dsl::*;

        let mut connection = self.connection();

        let result = connection.build_transaction()
            .read_write()
            .run(|conn| {
                diesel::delete(
                    sessions
                )
                .execute(conn)
            });

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(async_session::Error::new(e)),
        }
    }
}
