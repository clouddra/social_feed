#![allow(proc_macro_derive_resolution_fallback)]
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String
}

#[derive(Deserialize)]
pub struct NewLike {
    pub object: String,
    pub target: String,
}

#[derive(Deserialize)]
pub struct NewShare {
    pub object: String,
    pub target: String,
}

#[derive(Deserialize)]
pub struct NewPost {
    pub object: String
}

#[derive(Queryable, Serialize)]
#[table_name = "activities"]
pub struct Activity {
    pub actor: String,
    pub object: String,
    pub target: Option<String>,
    pub verb: String,
    pub created_at: Option<NaiveDateTime>,
}
