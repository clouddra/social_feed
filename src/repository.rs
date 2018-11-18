#![allow(proc_macro_derive_resolution_fallback)]

use rocket_contrib::databases::diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use crate::schema::*;
use crate::views::*;
use chrono::NaiveDateTime;

#[derive(Deserialize)]
pub struct NewLike {
    object: String,
    target: String,
}

#[derive(Deserialize)]
pub struct NewShare {
    object: String,
    target: String,
}

#[derive(Deserialize)]
pub struct NewPost {
    object: String
}

#[derive(Identifiable, Queryable, Associations, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Serialize)]
#[belongs_to(User, foreign_key = "actor")]
#[table_name = "activities"]
pub struct Activity {
    #[serde(skip)]
    pub id: i32,
    pub actor: i32,
    pub object: String,
    pub target: Option<i32>,
    pub activity_type: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct ActivityWithUserName {
    actor: String,
    object: String,
    target: String,
    activity_type: String,
    created_at: Option<NaiveDateTime>,
}

impl ActivityWithUserName {
    fn new(actor: String,
           object: String,
           target: String,
           activity_type: String,
           created_at: Option<NaiveDateTime>) {
        ActivityWithUserName { actor, object, target, activity_type, created_at };
    }
}

pub fn follow(conn: &diesel::SqliteConnection, follower: &str, followed: &str) -> QueryResult<usize> {
    let follower_id = user_id_by_name(conn, follower)?;
    let followed_id = user_id_by_name(conn, followed)?;
    diesel::insert_into(follows::table)
        .values((
            follows::follower.eq(follower_id),
            follows::followed.eq(followed_id)))
        .execute(conn)
}

pub fn unfollow(conn: &diesel::SqliteConnection, follower: &str, followed: &str) -> QueryResult<usize> {
    let follower_id = user_id_by_name(conn, follower)?;
    let followed_id = user_id_by_name(conn, followed)?;

    diesel::delete(
        follows::table
            .filter(follows::follower.eq(follower_id))
            .filter(follows::followed.eq(followed_id))
    ).execute(conn)
}

pub fn add_like(conn: &diesel::SqliteConnection, actor: &str, like: &NewLike) -> QueryResult<usize> {
    let target_id: i32 = user_id_by_name(conn, &like.target)?;
    let actor_id: i32 = user_id_by_name(conn, actor)?;

    diesel::insert_into(likes::table)
        .values((
            likes::object.eq(&like.object),
            likes::target.eq(target_id),
            likes::actor.eq(actor_id)))
        .execute(conn)
}

pub fn add_share(conn: &diesel::SqliteConnection, actor: &str, share: &NewShare) -> QueryResult<usize> {
    let target_id: i32 = user_id_by_name(conn, &share.target)?;
    let actor_id: i32 = user_id_by_name(conn, actor)?;

    diesel::insert_into(shares::table)
        .values((
            shares::object.eq(&share.object),
            shares::target.eq(target_id),
            shares::actor.eq(actor_id)))
        .execute(conn)
}

pub fn add_post(conn: &diesel::SqliteConnection, actor: &str, post: &NewPost) -> QueryResult<usize> {
    let actor_id: i32 = user_id_by_name(conn, actor)?;

    diesel::insert_into(posts::table)
        .values((
            posts::object.eq(&post.object),
            posts::actor.eq(actor_id)))
        .execute(conn)
}

pub fn user_feed(conn: &diesel::SqliteConnection, user: &str, before: NaiveDateTime) -> Vec<Activity> {
//    let user_id = user_id_by_name(conn, user);
//    activities::table.inner_join(user::table.on(users::name.eq(user)))
    let a = activities::table
        .filter(activities::created_at.lt(before))
        .order(activities::created_at.desc())
        .limit(10)
        .inner_join(
            users::table.on(users::id.eq(activities::actor)))
        .filter(users::name.eq(user))
        .load::<(Activity, User)>(conn)
        .expect("error loading feed");

    a.into_iter().map(|t| t.0).collect()
}

pub fn follows_feed(conn: &diesel::SqliteConnection, user: &str, before: NaiveDateTime) -> Vec<Activity> {
    let user_id: i32 = user_id_by_name(conn, user).expect("gg");

//    let activities = activities_before(&before)
//        .inner_join(follows::table.inner_join(
//            users::table.on(users::id.eq(follows::followed)))
//            .on(activities::actor.eq(follows::followed)))
//        .filter(follows::follower.eq(user_id))
//        .select((activities::all_columns, users::all_columns))
//        .load::<(Activity, User)>(conn)
//        .expect("error loading feed");
    let activities = follows::table
        .filter(follows::follower.eq(user_id))
        .inner_join(activities::table.on(activities::actor.eq(follows::followed)))
        .inner_join(users::table.on(users::id.eq(follows::followed)))
        .select((activities::all_columns, users::all_columns))
        .load::<(Activity, User)>(conn)
        .expect("err");

    activities
        .into_iter()
        .map(|t: (Activity, User)| {
//            let act = t.0;
//            ActivityWithUserName::new(act.actor, act.object, act.target, act.activity_type, act.created_at)
            t.0.created_at.and_then(|x| {
                println!("dddd {}", x.timestamp());
                Some(5)
            });
            println!("{}, {}", t.0.actor, t.1.name);
            t.0
        })
        .collect()
//    activities
}

fn activities_before(timestamp: &NaiveDateTime) -> activities::BoxedQuery<diesel::sqlite::Sqlite> {
    activities::table.filter(activities::created_at.lt(timestamp)).into_boxed()
}

fn user_id_by_name(conn: &diesel::SqliteConnection, name: &str) -> QueryResult<i32> {
    users::table
        .select(users::id)
        .filter(users::name.eq(name))
        .get_result(conn)
}
