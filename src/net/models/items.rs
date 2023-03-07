use serde::Serialize;

use crate::db::models::deal::Deal;

#[derive(Serialize)]
pub struct Items {
    pub items: Vec<Deal>,
}
