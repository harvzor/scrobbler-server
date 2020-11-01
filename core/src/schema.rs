table! {
    drink_dranks (id) {
        id -> Int4,
        drink_id -> Int4,
        drank_timestamp -> Timestamp,
    }
}

table! {
    drinks (id) {
        id -> Int4,
        name -> Varchar,
        colour -> Varchar,
        deleted -> Bool,
    }
}

joinable!(drink_dranks -> drinks (drink_id));

allow_tables_to_appear_in_same_query!(
    drink_dranks,
    drinks,
);
