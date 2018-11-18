use crate::schema::*;

table! {
    activities (id, activity_type) {
        id -> Integer,
        actor -> Integer,
        object -> Text,
        target -> Nullable<Integer>,
        activity_type -> Text,
        created_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    activities,
    users,
);

allow_tables_to_appear_in_same_query!(
    activities,
    follows,
);

