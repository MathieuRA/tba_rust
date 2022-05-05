use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use schema::items;
use crate::{establish_connection, models, schema};

use models::room::Room;

#[derive(Identifiable, Queryable, Associations, Debug)]
#[table_name = "items"]
#[belongs_to(Room, foreign_key = "room_id")]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub room_id: i32
}

impl Item {
    pub fn get_by_room_and_item_name(room: &Room, item_name: String) -> Option<Item> {
    schema::items::dsl::items
        .filter(items::room_id.eq(room.id))
        .filter(items::name.eq(&item_name))
        .first(&establish_connection())
        .optional()
        .expect(&format!("Unable to find item: {}", item_name))
    }
}