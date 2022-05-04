table! {
    items (id) {
        id -> Integer,
        name -> Varchar,
        room_id -> Integer,
    }
}

table! {
    rooms (id) {
        id -> Integer,
        name -> Varchar,
    }
}

joinable!(items -> rooms (room_id));

allow_tables_to_appear_in_same_query!(
    items,
    rooms,
);
