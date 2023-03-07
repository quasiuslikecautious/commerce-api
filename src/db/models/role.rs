use uuid::Uuid;
use diesel::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::db::models::schema;

/// The struct to represent a role returned from the postgresql database
/// 
/// This struct is a representation of the schema from the roles table in the commerce database.
/// Currently this includes fields for the role's uuid and name. This data is primarily used for
/// enforcing access control, and as a field in the JWT we send to the user during auth.
/// 
/// role.uuid is the primary key of the table.
/// 
/// # Examples
/// 
/// ```
/// // this assumes you are using diesel
/// use commerce::db::models::schema::roles::dsl::*;
///
/// let connection = &mut establish_connection();
///
/// let response = roles
///     .filter(uuid.eq(role_id))
///     .first::<Role>(connection);
/// ```
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(primary_key(uuid), table_name = schema::roles)]
pub struct Role {
    pub uuid: Uuid,
    pub name: String,
}
