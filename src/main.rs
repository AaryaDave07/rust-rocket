#[macro_use]
extern crate rocket;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_sync_db_pools::{database, diesel};

mod schema;

use schema::riders;

// ----------------- DB CONFIG -----------------
#[database("sqlite_db")]
pub struct DbConn(SqliteConnection);

// ----------------- MODELS -----------------
#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Rider {
    pub id: i32,
    pub name: String,
    pub phone: String,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = riders)]
pub struct NewRider {
    pub name: String,
    pub phone: String,
}

#[derive(AsChangeset, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = riders)]
pub struct UpdateRider {
    pub name: Option<String>,
    pub phone: Option<String>,
}

// ----------------- ROUTES -----------------

// Get all riders
#[get("/riders")]
async fn get_riders(conn: DbConn) -> Json<Vec<Rider>> {
    conn.run(|c| riders::table.load::<Rider>(c).unwrap()).await.into()
}

// Get single rider by ID
#[get("/riders/<rider_id>")]
async fn get_rider(conn: DbConn, rider_id: i32) -> Option<Json<Rider>> {
    conn.run(move |c| riders::table.find(rider_id).first::<Rider>(c).ok())
        .await
        .map(Json)
}

// Create new rider
#[post("/riders", data = "<new_rider>")]
async fn create_rider(conn: DbConn, new_rider: Json<NewRider>) -> Json<Rider> {
    conn.run(move |c| {
        diesel::insert_into(riders::table)
            .values(&*new_rider)
            .execute(c)
            .unwrap();

        // Fetch last inserted rider
        riders::table.order(riders::id.desc()).first::<Rider>(c).unwrap()
    })
    .await
    .into()
}

// Update rider
#[put("/riders/<rider_id>", data = "<rider_data>")]
async fn update_rider(
    conn: DbConn,
    rider_id: i32,
    rider_data: Json<UpdateRider>,
) -> Option<Json<Rider>> {
    conn.run(move |c| {
        let count = diesel::update(riders::table.find(rider_id))
            .set(&*rider_data)
            .execute(c)
            .unwrap();

        if count == 0 {
            None
        } else {
            Some(riders::table.find(rider_id).first::<Rider>(c).unwrap())
        }
    })
    .await
    .map(Json)
}

// Delete rider
#[delete("/riders/<rider_id>")]
async fn delete_rider(conn: DbConn, rider_id: i32) -> Json<usize> {
    conn.run(move |c| {
        diesel::delete(riders::table.find(rider_id))
            .execute(c)
            .unwrap()
    })
    .await
    .into()
}

// ----------------- ROCKET -----------------
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(DbConn::fairing())
        .mount(
            "/",
            routes![
                get_riders,
                get_rider,
                create_rider,
                update_rider,
                delete_rider
            ],
        )
}
