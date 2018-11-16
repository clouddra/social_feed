use rocket_contrib::databases::diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::prelude::*;
use crate::schema::follows;

#[derive(Insertable)]
#[table_name="follows"]
pub struct Follow {
    pub followed: i32,
    pub follower: i32,
}
impl Follow {

    pub fn follow(conn: &diesel::SqliteConnection, follower_id: String, following_id: String) {
        use crate::schema::follows::dsl::*;

        let new_follow = Follow {
            follower: 1,
            followed: 1
        };
        diesel::insert_into(follows)
            .values(&new_follow)
            .execute(conn);

    }
}

//users.find(user_id).get_result::<User>(conn)