use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::*;
use crate::views::*;
use crate::models::*;

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

pub fn user_feed(conn: &diesel::SqliteConnection, user: &str, before: NaiveDateTime) -> QueryResult<Vec<Activity>> {
    user_id_by_name(conn, user)?;
    activities_before(&before)
        .filter(activities::actor.eq(user))
        .order(activities::created_at.desc())
        .load::<Activity>(conn)
}

pub fn friends_feed(conn: &diesel::SqliteConnection, user: &str, before: NaiveDateTime) -> QueryResult<Vec<Activity>> {
    let user_id: i32 = user_id_by_name(conn, user)?;
    let friends = follows::table
        .filter(follows::follower.eq(user_id))
        .inner_join(users::table.on(users::id.eq(follows::followed)))
        .select(users::name)
        .load::<String>(conn)?;

    activities_before(&before)
        .filter(activities::actor.eq_any(friends))
        .load::<Activity>(conn)
}

fn activities_before(timestamp: &NaiveDateTime) -> activities::BoxedQuery<diesel::sqlite::Sqlite> {
    activities::table
        .filter(activities::created_at.lt(timestamp))
        .limit(10)
        .order_by(activities::created_at.desc())
        .into_boxed()
}

fn user_id_by_name(conn: &diesel::SqliteConnection, name: &str) -> QueryResult<i32> {
    users::table
        .select(users::id)
        .filter(users::name.eq(name))
        .get_result(conn)
}
