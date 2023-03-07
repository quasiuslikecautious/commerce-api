use uuid::Uuid;
use diesel::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::db::models::schema;

/// The struct to represent a jwt issuer returned from the postgresql database
/// 
/// This struct is a representation of the schema from the jwt_issuer table in the commerce 
/// database. Currently this includes fields for the issuer's uuid and name. This data is primarily
/// used to verify the issuer of a JWT is within the trusted issuers that are known to the database
/// and for monitoring of tokens.
/// 
/// jwt_issuer.uuid is the primary key of the table.
/// 
/// # Examples
/// 
/// ```
/// // this assumes you are using diesel
/// use commerce::db::models::schema::deals::dsl::*;
///
/// let connection = &mut establish_connection();
///
/// let response = deals
///     .filter(uuid.eq(item_id))
///     .first::<Deal>(connection);
/// ```
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(primary_key(uuid), table_name = schema::jwt_issuers)]
pub struct JwtIssuer {
    pub uuid: Uuid,
    pub name: String,
}
