### Dependencies

1. Install [Rust](https://rustup.rs/) compiler (nightly)
2. Install [Rocket](https://rocket.rs/)
3. Install sqlite3
4. Install [Diesel](https://github.com/diesel-rs/diesel)

### Running

1.

### Building and running project in development

1. Refer to Diesel website for instructions on how to run migrations
2. Use [seed/seed.sql](seed/seed.sql) to seed `development.sqlite3` database

### Design

The data store is sqlite3 backed by the following tables with the appropriate indices and constraints
1. users
2. follows
3. posts
4. likes

the activity stream is backed by a view (see [2018-11-17-044704_create_activities/up.sql](2018-11-17-044704_create_activities/up.sql)) for more details

The app consist of 7 endpoints
#[put("/<user>/follow/<friend>")]

