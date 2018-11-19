use crate::repository;
use crate::models::*;
use crate::db::SocialDbConn;
use rocket_contrib::json::Json;
use rocket::http::{RawStr, Status};
use chrono::{Utc, NaiveDateTime};
use diesel::result::Error;

#[put("/<user>/follow/<friend>")]
pub fn follow(conn: SocialDbConn, user: &RawStr, friend: &RawStr) -> Status {
    match repository::follow(&conn.0, user.as_str(), friend.as_str()) {
        Ok(_) => Status::Ok,
        Err(Error::DatabaseError(UniqueViolation, _)) => Status::Ok,
        Err(Error::NotFound) => Status::NotFound,
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
pub fn like(conn: SocialDbConn, user: &RawStr, like: Json<NewLike>) -> Status {
    match repository::add_like(&conn, user.as_str(), &like) {
        Ok(_) => Status::Ok,
        Err(Error::DatabaseError(UniqueViolation, _)) => Status::Conflict,
        Err(Error::NotFound) => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[post("/<user>/share", format = "json", data = "<share>")]
pub fn share(conn: SocialDbConn, user: &RawStr, share: Json<NewShare>) -> Status {
    match repository::add_share(&conn, user.as_str(), &share) {
        Ok(_) => Status::Ok,
        Err(Error::DatabaseError(UniqueViolation, _)) => Status::Conflict,
        Err(Error::NotFound) => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[post("/<user>/post", format = "json", data = "<post>")]
pub fn post(conn: SocialDbConn, user: &RawStr, post: Json<NewPost>) -> Status {
    match repository::add_post(&conn, user.as_str(), &post) {
        Ok(_) => Status::Ok,
        Err(Error::DatabaseError(UniqueViolation, _)) => Status::Conflict,
        Err(Error::NotFound) => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

/// ### Caching
///
/// Round down to the nearest 10 sec for a stable cache key
#[get("/<user>/feed?<created_before>")]
pub fn my_feed(conn: SocialDbConn, user: &RawStr, created_before: Option<i64>) -> Result<Json<Vec<Activity>>, Status> {
    let nearest_10_sec = match created_before {
        Some(timestamp) => NaiveDateTime::from_timestamp(timestamp, 0),
        _ => round_down_curr_time(10)
    };

    let feed_result = repository::user_feed(&conn, user.as_str(), nearest_10_sec);
    match feed_result {
        Ok(feed) => Ok(Json(feed)),
        Err(Error::NotFound) => Err(Status::NotFound),
        _ => Err(Status::NotFound),
    }
}

#[get("/<user>/feed/friends?<created_before>")]
pub fn friends_feed(conn: SocialDbConn, user: &RawStr, created_before: Option<i64>) -> Result<Json<Vec<Activity>>, Status> {
    let nearest_10_sec = match created_before {
        Some(timestamp) => NaiveDateTime::from_timestamp(timestamp, 0),
        _ => round_down_curr_time(10)
    };

    let feed_result = repository::friends_feed(&conn, user.as_str(), nearest_10_sec);
    match feed_result {
        Ok(feed) => Ok(Json(feed)),
        Err(Error::NotFound) => Err(Status::NotFound),
        _ => Err(Status::NotFound),
    }
}

fn round_down_curr_time(nearest_seconds: i32) -> NaiveDateTime {
    let now = Utc::now().naive_utc().timestamp();
    let rounded_now = now - now % nearest_seconds as i64;
    NaiveDateTime::from_timestamp(rounded_now, 0)
}
