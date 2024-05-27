
mod game;
mod player;

use game::*;
use player::*;
use std::fs;

fn main() {
    assert!(SIZE >= 5);

    println!("{}\n\n", fs::read_to_string("assets/ascii_art.txt").unwrap());
    println!("Rusty battleships\n");

    let mut game = Game::new();
    _ = game.turn();

}

