use uuid::Uuid;
use diesel::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::db::models::schema;

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
