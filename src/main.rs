
mod game;
mod user;
mod computer;
mod tests;

use game::*;
use GameStatus::*;
use std::fs;
use std::io::stdin;
use rand::Rng;

fn main() {
    //assert!(SIZE >= 5);

    let art = fs::read_to_string("assets/ascii_art.txt");
    if art.is_ok() {
        println!("{}\n\n", art.unwrap());
    }

    println!("Rusty battleships\n");

    let mut num_turns = 0;

    //Laziest game menu I've ever written honestly
    println!("Welcome to Battleship, or something");

    let mut game = loop {
        println!("Select game mode:");
        println!("Two person game - 0\nPlay computer - 1\n");

        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();

        match input.as_str() {
            "0" => {
                break Game::new_two_user();
            },

            "1" => {

                let mut rng = rand::thread_rng();
                //I'm having a bad night, and so must you
                if rng.gen_range(0..5) == 0 {
                    println!("\nNo friends to play with huh?");
                }

                break Game::new_user_computer();
            },

            _ => {
                println!("Input error");
            },
        }
    };

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

