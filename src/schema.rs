table! {
    commands (id) {
        id -> Integer,
        command -> Varchar,
        default_message -> Varchar,
    }
}

table! {
    directions (id) {
        id -> Integer,
        command -> Varchar,
        name -> Varchar,
    }
}

table! {
    effects (id) {
        id -> Integer,
        effect_type -> Varchar,
        item_id -> Integer,
        order -> Integer,
        command_id -> Integer,
        message -> Varchar,
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

joinable!(effects -> commands (command_id));
joinable!(effects -> items (item_id));
joinable!(items -> rooms (room_id));
joinable!(room_connections -> directions (direction_id));

allow_tables_to_appear_in_same_query!(
    commands,
    directions,
    effects,
    items,
    rooms,
    room_connections,
);
