use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::riders;

#[derive(Queryable, Serialize)]
pub struct Rider {
    pub id: i32,
    pub name: String,
    pub phone: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = riders)]
pub struct NewRider {
    pub name: String,
    pub phone: String,
}
