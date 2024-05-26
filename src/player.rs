
#[derive(Clone, Copy)]
enum DisplayState {
    Hit, Miss, Blank,
}
use DisplayState::*;
use crate::game::{SIZE, Player, Coord};
use std::io::stdin;
use std::str::FromStr;

struct PlayerView {
    state: [[DisplayState; SIZE]; SIZE]
}

use std::fmt;
impl fmt::Display for PlayerView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        write!(f, "  |");

        //let it crash if it's more than 26 lol
        for i in 0..SIZE {
            write!(f, "{}|", alphabet.as_bytes()[i] as char)?;
        }

        for (idx, col) in self.state.iter().enumerate() {
            write!(f, "\n{} |", idx+1)?;

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

impl User {
    pub fn new() -> User {
        User { 
            view: PlayerView { state: [[Blank; SIZE]; SIZE] }
        }
    }

}

impl Player for User {
    fn turn(&self) -> Coord {

        println!("{}", self.view);
        println!("\nEnter coordinates:\n");

        let mut fail_count: u32 = 0;

        loop {

            let mut coord_str = String::new();
            stdin().read_line(&mut coord_str).unwrap();
            let coord = Coord::from_str(&coord_str);

            if coord.is_ok() {
                return coord.unwrap();
            }

            fail_count += 1;

            match fail_count {
                1 => println!("Input error: format is (x, y), where x and y are integers from 1-{}", SIZE),
                //2 => println!("Is it really that hard to understand?"),
                //3 => println!("Jesus fucking christ how hard is it to enter a fucking coordinate pair"),
                _ => panic!("\nError: user is too stupid to follow simple instructions\n"),
            }
        }
    }
}

