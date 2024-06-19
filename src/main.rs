
mod game;
mod user;
mod computer;

use game::*;
use GameStatus::*;
use std::fs;

fn main() {
    //assert!(SIZE >= 5);

    let art = fs::read_to_string("assets/ascii_art.txt");
    if art.is_ok() {
        println!("{}\n\n", art.unwrap());
    }

    println!("Rusty battleships\n");

    let mut game = Game::new();
    let mut num_turns = 0;

    loop {
        let status = game.turn();
        num_turns += 1;

        if let Win(player_id) = status {
            println!("{} wins!", game.get_player_name(player_id));

            //print other winformation
            //see what I did there ;)
            println!("Number of turns: {}", num_turns);
            break;
        }
    }
}