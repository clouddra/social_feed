#![feature(proc_macro_hygiene, decl_macro, custom_attribute, custom_derive, plugin)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;
extern crate chrono;

mod controller;
mod schema;
mod views;
mod models;
mod repository;
mod db;

fn main() {
    rocket::ignite()
        .attach(db::SocialDbConn::fairing())
        .mount("/", routes![
            controller::follow,
            controller::unfollow,
            controller::like,
            controller::share,
            controller::post,
            controller::my_feed,
            controller::friends_feed])
        .launch();
}
