#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]
#![feature(custom_derive)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

mod controller;
mod schema;
mod models;
mod follows;

#[database("social_db")]
pub struct SocialDbConn(diesel::SqliteConnection);

fn main() {
    rocket::ignite()
        .attach(SocialDbConn::fairing())
        .mount("/", routes![controller::index])
        .mount("/", routes![controller::follows]).launch();
}