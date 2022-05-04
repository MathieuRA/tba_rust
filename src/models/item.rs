use schema::items;
use crate::{models, schema};

use models::room::Room;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "items"]
#[belongs_to(Room, foreign_key = "room_id")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub room_id: i32
}