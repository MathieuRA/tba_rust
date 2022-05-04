use schema::room_connections;
use crate::{establish_connection, models, schema};

use models::room::Room;
use crate::schema::rooms::dsl::rooms;

use diesel::prelude::*;
use crate::models::direction::Direction;
use crate::schema::directions::dsl::directions;

#[derive(Identifiable, Associations, Queryable, Debug)]
#[table_name = "room_connections"]
#[belongs_to(Room, foreign_key = "from_room_id")]
pub struct RoomConnection  {
    pub id: i32,
    pub from_room_id: i32,
    pub to_room_id:i32,
    pub direction_id: i32
}

impl RoomConnection {
    pub fn get_to_room(&self) -> Room {
        rooms.find(self.to_room_id).first(&establish_connection())
            .expect(&format!("Unable to find room {}", self.to_room_id))
    }
    pub fn get_direction(&self) -> Direction {
        directions.find(self.direction_id).first(&establish_connection())
            .expect(&format!("Unable to find direction {}", self.direction_id))
    }
}