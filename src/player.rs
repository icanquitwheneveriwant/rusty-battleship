

enum DisplayState {
    Hit, Miss, Blank,
}
use DisplayState::*;
use crate::game::{SIZE, Player, Coord};
use std::io::stdin;

struct PlayerView {
    state: [[DisplayState; SIZE]; SIZE]
}

use std::fmt;
impl fmt::Display for PlayerView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        //let it crash if it's more than 26 lol
        for i in 0..SIZE {
            write!(f, "{}|", alphabet.as_bytes()[i])?;
        }

        for (idx, col) in self.state.iter().enumerate() {
            writeln!(f, "{} |", idx+1)?;

            for tile in col.iter() {
                let indicator = match tile {
                    Hit => "X",
                    Miss => "O",
                    Blank => " ",
                };

                write!(f, "{indicator}|")?;
            }
        }

        write!(f, " ")
    }
}

pub struct User {
    view: PlayerView,
}

impl Player for User {
    fn turn(&self) -> Coord {
        println!("{}", self.view);
        println!("Enter coordinates:\n");

        let mut coord_str = String::new();
        stdin().read_line(&mut coord_str).expect("What the fuck happened here");
        

        Coord {
            x: 0,
            y: 0,
        }
    }
}

