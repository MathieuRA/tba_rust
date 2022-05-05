use std::fmt::Error;
use diesel::{ExpressionMethods, NotFound, OptionalExtension, QueryDsl, QueryResult, RunQueryDsl};
use schema::directions;
use crate::{establish_connection, schema};

#[derive(Identifiable, Queryable, Debug)]
#[table_name = "directions"]
pub struct Direction {
    pub id: i32,
    pub command: String,
    pub name: String
}

impl Direction {
    pub fn find_by_command(command: &String) -> Option<Direction>{
        schema::directions::dsl::directions
            .filter(schema::directions::command.eq(command))
            .first::<Direction>(&establish_connection())
            .optional().expect("Error when searching for command")
    }
}