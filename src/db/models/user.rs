use uuid::Uuid;
use diesel::{ prelude::*, RunQueryDsl, QueryDsl, };
use serde::{ Serialize, Deserialize };

use super::schema;
use crate::db::{
    crypt,
    establish_connection,
    gen_salt,
};

/// The struct to represent a user returned from the postgresql database
/// 
/// This struct is a representation of the schema from the users table in the commerce database.
/// Currently this includes fields for the user's uuid, email, password, and the user's role uuid
/// It is mainly used for parsing database responses.
/// 
/// user.uuid is the primary key of the table, but as email is constrained to unique, you can also
/// query by that field. For authentication we query by user.email and the encrypted user.password
/// 
/// # Examples
/// 
/// ```
/// // this assumes you are using diesel
/// use commerce::db::models::schema::users::dsl::*;
///
/// let connection = &mut establish_connection();
///
/// let response = users
///     .filter(uuid.eq(user_id))
///     .first::<User>(connection);
/// ```
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(primary_key(uuid), table_name = schema::users)]
pub struct User {
    #[diesel(deserialize_as = Uuid)]
    pub uuid: Option<Uuid>,
    pub email: String,
    pub password: String,
    pub role: Uuid,
}

impl User {
    pub fn get(user_id: Uuid) -> Option<User> {
        use schema::users::dsl::*;

        let connection = &mut establish_connection();
        let response = connection.build_transaction()
        .read_only()
        .run(|conn| {
            users
                .filter(uuid.eq(user_id))
                .first::<User>(conn)
        });

        response.ok()
    }

    pub fn get_from_auth(user_email: &str, user_password: &str) -> Option<User> {
        use schema::users::dsl::*;

        let connection = &mut establish_connection();
        let response = connection.build_transaction()
        .read_only()
        .run(|conn| {
            users
                .filter(email.eq(user_email))
                .filter(password.eq(crypt(user_password, password)))
                .first::<User>(conn)
            });

        response.ok()
    }

    pub fn insert(user_email: &str, user_password: &str) -> Option<User> {
        use schema::users::dsl::*;

        let connection = &mut establish_connection();
        let response = connection.build_transaction()
        .read_write()
        .run(|conn| {
            diesel::insert_into(users)
                .values((
                    email.eq(user_email),
                    password.eq(crypt(user_password, gen_salt("bf"))),
                ))
                .get_result::<User>(conn)
        });

        response.ok()
    }
}


