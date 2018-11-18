table! {
    follows (follower, followed) {
        follower -> Integer,
        followed -> Integer,
    }
}

table! {
    likes (id) {
        id -> Integer,
        actor -> Integer,
        object -> Text,
        target -> Integer,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    posts (id) {
        id -> Integer,
        actor -> Integer,
        object -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    shares (id) {
        id -> Integer,
        actor -> Integer,
        object -> Text,
        target -> Integer,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

joinable!(posts -> users (actor));

allow_tables_to_appear_in_same_query!(
    follows,
    likes,
    posts,
    shares,
    users,
);
