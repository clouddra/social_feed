use crate::models::User;
use crate::repository;
use crate::SocialDbConn;
use rocket_contrib::json::Json;
use rocket::http::{RawStr, Status};
use chrono::Utc;
use chrono::prelude::*;
use chrono::NaiveDateTime;
use diesel::result::Error;

/// ### Perfomance
///
/// Although unlikely to cause performance issues, this endpoint can be pushed to a task queue to
/// be executed asychronously or event sourced without any side effects since this endpoint is
/// idempotent.
#[put("/<user>/follow/<friend>")]
pub fn follow(conn: SocialDbConn, user: &RawStr, friend: &RawStr) -> Status {
    match repository::follow(&conn.0, user.as_str(), friend.as_str()) {
        Ok(_) => Status::Ok,
        Err(diesel::result::Error::DatabaseError(UniqueViolation, _)) => Status::Ok,
        Err(diesel::result::Error::NotFound) => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[put("/<user>/unfollow/<friend>")]
pub fn unfollow(conn: SocialDbConn, user: &RawStr, friend: &RawStr) -> Status {
    match repository::unfollow(&conn, user.as_str(), friend.as_str()) {
        Ok(_) => Status::Ok,
        _ => Status::InternalServerError,
    }
}

#[post("/<user>/like", format = "json", data = "<like>")]
pub fn like(conn: SocialDbConn, user: &RawStr, like: Json<repository::NewLike>) -> Status {
    match repository::add_like(&conn, user.as_str(), &like) {
        Ok(_) => Status::Ok,
        Err(diesel::result::Error::DatabaseError(UniqueViolation, _)) => Status::Conflict,
        Err(diesel::result::Error::NotFound) => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[post("/<user>/share", format = "json", data = "<share>")]
pub fn share(conn: SocialDbConn, user: &RawStr, share: Json<repository::NewShare>) -> Status {
    match repository::add_share(&conn, user.as_str(), &share) {
        Ok(_) => Status::Ok,
        Err(diesel::result::Error::DatabaseError(UniqueViolation, _)) => Status::Conflict,
        Err(diesel::result::Error::NotFound) => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[post("/<user>/post", format = "json", data = "<post>")]
pub fn post(conn: SocialDbConn, user: &RawStr, post: Json<repository::NewPost>) -> Status {
    match repository::add_post(&conn, user.as_str(), &post) {
        Ok(_) => Status::Ok,
        Err(diesel::result::Error::DatabaseError(UniqueViolation, _)) => Status::Conflict,
        Err(diesel::result::Error::NotFound) => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/<user>/my_feed?<created_before>")]
pub fn my_feed(conn: SocialDbConn, user: &RawStr, created_before: Option<i64>) -> Json<Vec<repository::Activity>> {
//    round to nearest 10 sec for more stable caching
    let nearest_10_sec = match created_before {
        Some(timestamp) => timestamp,
        _ => {
            let now = Utc::now().naive_utc().timestamp();
            now - now % 10
        }
    };
    Json(repository::user_feed(&conn, user.as_str(), NaiveDateTime::from_timestamp(nearest_10_sec, 0)))
}

#[get("/<user>/feed/friends?<created_before>")]
pub fn friends_feed(conn: SocialDbConn, user: &RawStr, created_before: Option<i64>) -> Json<Vec<repository::Activity>> {
//    round to nearest 10 sec for more stable caching
    let nearest_10_sec = match created_before {
        Some(timestamp) => timestamp,
        _ => {
            let now = Utc::now().naive_utc().timestamp();
            now - now % 10
        }
    };
    Json(repository::follows_feed(&conn, user.as_str(), NaiveDateTime::from_timestamp(nearest_10_sec, 0)))
}
