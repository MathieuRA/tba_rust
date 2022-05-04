use schema::{rooms, room_connections};
use diesel::associations::BelongsTo;
use crate::models::direction::Direction;
use crate::models::room_direction::RoomConnection;
use crate::{establish_connection, schema};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::types::IsNull::No;

#[derive(Identifiable, Queryable, QueryableByName, Clone, Debug)]
#[table_name = "rooms"]
pub struct Room {
    pub id: i32,
    pub name: String
}

impl Room {
    pub fn get_connection_from(&self) -> Vec<RoomConnection> {
        let room_connections = RoomConnection::belonging_to(self).load(&establish_connection()).expect("Failed to load room_connections");
        room_connections
    }

    pub fn get_room_by_direction(&self, direction: Direction) -> Option<Self> {
        let query = format!("SELECT rooms.id, rooms.name FROM room_connections INNER JOIN rooms ON rooms.id = room_connections.to_room_id WHERE room_connections.from_room_id = {}  AND room_connections.direction_id = {}", self.id, direction.id);
        let room_result: Option<Vec<Room>> = sql_query(&query)
            .load(&establish_connection()).optional().expect("Error when getting room by direction.");

        let room = match room_result {
            None => None,
            Some(room) => {
                if room.len() > 1 {
                    panic!("An error as occur. Founded more that one room. SQL QUERY: {}", query);
                }
                Some(room.first().unwrap().to_owned())
            }
        };
        room
    }

}
