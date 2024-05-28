
mod game;
mod player;

use game::*;
use player::*;
use std::fs;

fn main() {
    assert!(SIZE >= 5);

    let art = fs::read_to_string("assets/ascii_art.txt");
    if art.is_ok() {
        println!("{}\n\n", art.unwrap());
    }

    println!("Rusty battleships\n");

    let mut game = Game::new();
    _ = game.turn();

}

