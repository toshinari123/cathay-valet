extern crate serde;
#[macro_use] extern crate rocket;

pub mod models;
pub mod routes;

use rocket::build;
use crate::routes::{json, edit};

#[rocket::main]
async fn main() {
    let _ = build()
        .mount("/", routes![get]).launch().await;
}
