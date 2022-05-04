table! {
    directions (id) {
        id -> Integer,
        command -> Varchar,
        name -> Varchar,
    }
}

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

table! {
    room_connections (id) {
        id -> Integer,
        from_room_id -> Integer,
        to_room_id -> Integer,
        direction_id -> Integer,
    }
}

joinable!(items -> rooms (room_id));
joinable!(room_connections -> directions (direction_id));

allow_tables_to_appear_in_same_query!(
    directions,
    items,
    rooms,
    room_connections,
);
