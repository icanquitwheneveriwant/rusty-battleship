
mod game;
mod user;
mod computer;

use game::*;
use std::fs;

fn main() {
    //assert!(SIZE >= 5);

    let art = fs::read_to_string("assets/ascii_art.txt");
    if art.is_ok() {
        println!("{}\n\n", art.unwrap());
    }

    println!("Rusty battleships\n");

    let mut game = Game::new();
    loop {
        let status = game.turn();

        if status == GameStatus::P1Win {
            println!("Player 1 wins!");
            break;
        } else if status == GameStatus::P2Win {
            println!("Player 2 wins!");
            break;
        }
    }

}

