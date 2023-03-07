use uuid::Uuid;
use diesel::prelude::*;
use serde::{ Serialize, Deserialize };

use crate::db::models::schema;

/// The struct to represent a deal returned from the postgresql database
/// 
/// This struct is a representation of the schema from the deals table in the commerce database.
/// Currently this includes fields for the deal's uuid, name, image url, price in minor units, and
/// the deal's description. It is mainly used for parsing database responses.
/// 
/// deal.uuid is the primary key of the table
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
#[diesel(primary_key(uuid), table_name = schema::deals)]
pub struct Deal {
    #[diesel(deserialize_as = Uuid)]
    pub uuid: Option<Uuid>,
    pub name: String,
    pub image: String,
    pub price: i32,
    pub description: String,
}
