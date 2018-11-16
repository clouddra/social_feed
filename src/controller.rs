use crate::models::User;
use crate::follows::Follow;
use crate::SocialDbConn;
use rocket_contrib::json::Json;
use rocket::http::RawStr;

#[get("/")]
pub fn index(conn: SocialDbConn) -> Json<User> {
    Json(User::by_id(&conn.0, 1))
}

#[post("/follow/<follower>")]
pub fn follows(conn: SocialDbConn, follower: &RawStr) {
    Follow::follow(&conn.0, "test".to_owned(), "tete".to_owned())
}
