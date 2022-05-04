use std::borrow::Borrow;
use std::process::exit;
use diesel::RunQueryDsl;
use rand::Rng;
use tba::establish_connection;
use tba::models::{room::Room, item::Item};
use tba::schema::rooms::dsl::*;
use crate::get_input;
use diesel::prelude::*;
use tba::models::direction::Direction;
use tba::models::room_direction::{ RoomConnection};
use tba::schema::room_connections::dsl::room_connections;

pub struct Game {
    db_connection: MysqlConnection,
    is_running: bool,
    current_room: Room
}

impl Game {
    pub fn new() -> Self {
        let connection = establish_connection();
        let results = rooms.load::<Room>(&connection).expect("Failed to load rooms");
        let nb_rooms = results.len();
        if nb_rooms == 0 {
            println!("You must have at least one room in the database.");
            exit(1);
        }
        let index = match nb_rooms - 1 {
            0 => 0,
            max_range => rand::thread_rng().gen_range(0..max_range)
        };
        Game {
            db_connection: establish_connection(),
            is_running: false,
            current_room: results.get(index).expect("Error during defining initial room").to_owned()
        }
    }

    pub fn start(&mut self) -> () {
        self.is_running = true;
    }

    pub fn stop(&mut self) -> () {
        self.is_running = false;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn get_current_room(&self) -> &Room {
        &self.current_room
    }

    // Describes the game's execution cycle
    pub fn update(&mut self) -> () {
        let current_room  = self.get_current_room();
        // Describe the place
        println!("You are in the {}", current_room.name);

        // Show allowed directions
        for connection in current_room.get_connection_from() {
            println!("- {} is the {}", connection.get_direction().name ,connection.get_to_room().name)
        }

        // Display all items available to interact with
        let items: Vec<Item> = Item::belonging_to(self.get_current_room()).load(&self.db_connection).expect("Error during loading items");

        match items.is_empty() {
            true => println!("No visible items."),
            false => {
                println!("Visible items: ");
                for item in items {
                    println!("- {}", item.name)
                }
            }
        }

        // Waiting for user input
        let input = get_input("Please type a command...");
        if input == "exit" {
            self.stop();
        }

        match Direction::find_by_command(input) {
            None => {}
            Some(direction) => {
                let next_room = self.get_current_room().get_room_by_direction(direction);
                match next_room {
                    None => {
                        println!("You cannot go into that direction !");
                        return;
                    },
                    Some(next_room) => {
                        self.current_room = next_room;
                    }
                }
                return;
            }
        }
    }
}