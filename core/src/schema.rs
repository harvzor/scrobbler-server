table! {
    scrobbles (id) {
        id -> Int4,
        trackable_id -> Int4,
        timestamp -> Timestamp,
    }
}

table! {
    trackables (id) {
        id -> Int4,
        name -> Varchar,
        colour -> Varchar,
        deleted -> Bool,
    }
}

joinable!(scrobbles -> trackables (trackable_id));

allow_tables_to_appear_in_same_query!(
    scrobbles,
    trackables,
);
