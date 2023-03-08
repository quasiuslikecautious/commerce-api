use uuid::Uuid;
use diesel::{ prelude::*, RunQueryDsl, QueryDsl, };
use serde::{ Serialize, Deserialize };

use super::schema;
use crate::{ establish_connection, net::Pagination };

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

impl Deal {
    pub fn get(id: Uuid) -> Option<Deal> {
        use schema::deals::dsl::*;

        let connection = &mut establish_connection();
        let response = connection.build_transaction()
        .read_only()
        .run(|conn| {
            deals
                .filter(uuid.eq(id))
                .first::<Deal>(conn)
        });

        response.ok()
    }

    pub fn get_all(pagination: Pagination) -> Option<Vec<Deal>> {
        use schema::deals::dsl::*;

        let connection = &mut establish_connection();
        let response = connection.build_transaction()
        .read_only()
        .run(|conn| {
            deals
                .limit(pagination.get_limit())
                .offset(pagination.get_offset())
                .load::<Deal>(conn)
        });

        response.ok()
    }

    pub fn insert(&self) -> Option<Deal> {
        use schema::deals::dsl::*;

        let connection = &mut establish_connection();
        let response = connection.build_transaction()
        .read_write()
        .run(|conn| {
            diesel::insert_into(deals)
                .values((
                    name.eq(&self.name),
                    image.eq(&self.image),
                    price.eq(&self.price),
                ))
                .get_result::<Deal>(conn)
            });

        response.ok()
    }
}
