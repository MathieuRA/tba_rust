use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use schema::commands;
use crate::{establish_connection, schema};

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "commands"]
pub struct Command {
    pub id: i32,
    pub command: String,
    pub default_message: String
}

impl Command {
    fn extract_name(command: &String, position: i32) -> Option<&str>{
        let splited_command: Vec<&str> = command.split_whitespace().collect();
        if splited_command.len() != 2 {
            return None;
        }
        match position {
            1 => Some(splited_command.first().unwrap().to_owned()),
            2 => Some(splited_command.last().unwrap().to_owned()),
            _ => None
        }
    }

    pub fn extract_item_name(command: &String) -> Option<&str> {
        Command::extract_name(command, 2)
    }

    pub fn extract_command_name(command: &String) -> Option<&str>{
        Command::extract_name(command, 1)
    }

    pub fn get_by_command_name(command_name: String) -> Option<Self> {
        schema::commands::dsl::commands
            .filter(schema::commands::command.eq(command_name))
            .first::<Command>(&establish_connection())
            .optional().expect("")
    }
}