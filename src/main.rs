use crate::modules::simple_user_input::get_input;
use crate::structs::game::Game;

mod modules;
mod structs;

fn main(){
    let mut game = Game::new();
    game.start();
    while game.is_running() {
        game.update();
    }
}

