use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use schema::effects;
use crate::models::command::Command;
use crate::models::item::Item;
use crate::{establish_connection, schema};
use crate::schema::effects::message;

const MESSAGE_TYPE_EFFECT: &str = "Message";
const END_TYPE_EFFECT: &str = "End";

pub trait AbstractEffect {
    fn trigger(&self);
}

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "effects"]
pub struct Effect {
    pub id: i32,
    pub effect_type: String,
    pub item_id: i32,
    pub order: i32,
    pub command_id: i32,
    pub message: String,
}

impl Effect {
    pub fn get_by_item_and_command(item: &Item, command: &Command) -> Vec<Effect> {
       schema::effects::dsl::effects
            .filter(effects::command_id.eq(command.id))
            .filter(effects::item_id.eq(item.id))
            .order(effects::order)
            .load(&establish_connection())
            .expect(&format!("Error when loading effects from item: {}", item.id))
    }

    pub fn to_abstract_effect(self) -> Box<dyn AbstractEffect> {
        match self.effect_type.as_str() {
            MESSAGE_TYPE_EFFECT => {
                Box::new(MessageEffect::new(self))
            },
            END_TYPE_EFFECT => {
                // End effects
                Box::new(MessageEffect::new(self))
            }
            _ => {
                panic!("Invalid effect type: {}", self.effect_type)
            }
        }
    }
}

pub struct MessageEffect {
    pub message: String
}
impl AbstractEffect for MessageEffect {
    fn trigger(&self) -> () {
        println!("{}", self.message);
    }
}
impl MessageEffect {
    pub fn new(effect: Effect) -> Self {
        Self {
            message: effect.message
        }
    }
}

pub struct EndEffect {
}

impl AbstractEffect for EndEffect {
    fn trigger(&self) -> () {

    }
}