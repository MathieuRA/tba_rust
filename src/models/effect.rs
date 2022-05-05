use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use schema::effects;
use crate::models::command::Command;
use crate::models::item::Item;
use crate::{establish_connection, schema};

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "effects"]
pub struct Effect {
    pub id: i32,
    pub effect_type: String,
    pub item_id: i32,
    pub order: i32,
    pub command_id: i32,
    pub message: String
}

impl Effect {
    pub fn get_by_item_and_command(item: &Item, command: &Command) -> Vec<Effect> {
       schema::effects::dsl::effects
            .filter(effects::command_id.eq(command.id))
            .filter(effects::item_id.eq(item.id))
            .order(effects::order)
            .load::<Effect>(&establish_connection())
           .expect("")
    }
}