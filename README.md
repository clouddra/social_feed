## Dependencies

1. Install [Rust](https://rustup.rs/) compiler (nightly)
2. Install [Rocket](https://rocket.rs/)
3. Install sqlite3
4. Install [Diesel](https://github.com/diesel-rs/diesel)

## Running

1. Run `./social_feed` executable. The database is pre-seeded.
### Building and running from source

1. Refer to Diesel website for instructions on how to run migrations
2. Use [sql_scripts/seed.sql](sql_scripts/seed.sql) to seed `development.sqlite3` database by running `sqlite3 development.sqlite3 < sql_scripts/seed.sql`

## Design

### Data
The data store is sqlite3 backed by the following tables with the appropriate indices and constraints
1. users
2. follows
3. posts
4. likes

the activity stream is backed by a view (see [2018-11-17-044704_create_activities/up.sql](2018-11-17-044704_create_activities/up.sql)).

### Routing

The endpoints 
```
#[put("/<user>/follow/<friend>")]`
#[put("/<user>/unfollow/<friend>")]`
```

are both idempotent and hence are modeled as `Put` methods.
The other post endpoints
```
#[post("/<user>/like", format = "json", data = "<like>")]
#[post("/<user>/share", format = "json", data = "<share>")]
#[post("/<user>/post", format = "json", data = "<post>")] 
```
checks for double writes are returns the appropriate status codes.

Pagination for the feed is done limit of 10 per page. The unix timestamp `create_before` parameter is used to control pagination. `create_before` is defaulted to the current time if no value is passed in. This is how clients retrieve the first page of feeds. The next page retrieved by passing it the timestamp of the last feed of the first page.
```
#[get("/<user>/feed?<created_before>")]
#[get("/<user>/feed/friends?<created_before>")]
```

## Performance

### Writes
In an actual production system read/writes will be backed by a sequence of events messages queue (it was modeled as a DB view in this exercise).  The events can be projected (event sourcing) into the respective tables by having a queue processor reading off the queue.

However this means that we can longer return accurate HTTP status codes to the client. The server no longer knows the correct state of the entire database

### Reads

The feeds can be cached and retrieved when there is high load. To do that we need a stable cache key. The endpoints
```
#[get("/<user>/feed?<created_before>")]
#[get("/<user>/feed/friends?<created_before>")]
```
are designed to cache for the most recent feed for 10s (see `controller.rs` for more details). The url path together with `created_before` timestamp as the cache key.
