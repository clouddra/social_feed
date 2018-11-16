table! {
    follows (id) {
        id -> Integer,
        follower -> Integer,
        followed -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    follows,
    users,
);
