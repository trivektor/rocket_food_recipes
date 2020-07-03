#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod routes;

fn main() {
    rocket::ignite()
        .mount("/recipes", routes![
            routes::recipes::index
        ]).launch();
}
