#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod models;
mod lib;
use lib::{mongo};
mod routes;

#[tokio::main]
async fn main() {
    rocket::ignite()
        .mount("/recipes", routes![
            routes::recipes::index,
            routes::recipes::create
        ]).launch();
}
