use std::borrow::Borrow;
use std::process::exit;
use diesel::RunQueryDsl;
use rand::Rng;
use tba::establish_connection;
use tba::models::{room::Room, item::Item};
use tba::schema::rooms::dsl::*;
use crate::get_input;
use diesel::prelude::*;

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
        // Describe the place
        println!("You are in the {}", self.get_current_room().name);
        // Show allowed directions
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
        let input = get_input("Please type something...");
        if input == "exit" {
            self.stop();
        }
    }
}