use rocket_contrib::databases::diesel;
use diesel::QueryDsl;
use diesel::RunQueryDsl;

#[derive(Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String
}

impl User {
    pub fn by_id(conn: &diesel::SqliteConnection, user_id: i32) -> User {
        use crate::schema::users::dsl::*;

        users.find(user_id)
            .get_result(conn)
            .expect(&format!("Unable to find post {}", user_id))
    }
}