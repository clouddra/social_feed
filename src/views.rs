use crate::schema::*;

table! {
    activities (actor, object, verb) {
        actor -> Text,
        object -> Text,
        target -> Nullable<Text>,
        verb -> Text,
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

