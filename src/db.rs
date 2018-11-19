#[database("social_db")]
pub struct SocialDbConn(diesel::SqliteConnection);
