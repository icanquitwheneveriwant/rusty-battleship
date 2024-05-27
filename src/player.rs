
use DisplayState::*;
use crate::game::{SIZE, Player, Coord, Orientation};
use Orientation::*;
use std::io::stdin;
use std::str::FromStr;
use rand::Rng;

#[derive(Clone, Copy)]
enum DisplayState {
    Hit, Miss, Blank,
}

struct PlayerView {
    state: [[DisplayState; SIZE]; SIZE]
}

use std::fmt;
impl fmt::Display for PlayerView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        write!(f, "  |");

        for i in 1..=SIZE {
            write!(f, "{}|", i)?;
        }

        assert!(SIZE <= 26);
        for (i, col) in self.state.iter().enumerate() {
            //Just let it crash or something if it's more than 26 lol
            write!(f, "\n{} |", alphabet.as_bytes()[i] as char)?;

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

    //random for now 
    fn place_ships(&self) -> [(usize, Coord, Orientation); 5] {  
        const NUM_SHIPS: usize = 5;
        let mut board = [[false; SIZE]; SIZE];
        let mut placements = [(0, Coord { x: 0, y: 0 },  Up); NUM_SHIPS];

        let mut ship_size = 1;

        while ship_size < NUM_SHIPS {
            let mut rng = rand::thread_rng();

            let orient = match rng.gen_range(0..4) {
                0 => Up,
                1 => Down,
                2 => Left,
                3 => Right,
                _ => todo!(),
            };

            let start = Coord { x: rng.gen_range(0..SIZE), y: rng.gen_range(0..SIZE) };
            let mut curr_coord = start;

            let mut break_flag = false;

            let mut new_board = board.clone();

            for _ in 0..ship_size {
                if !curr_coord.in_board() || board[curr_coord.x][curr_coord.y] {
                    break_flag = true;
                    break;
                }

                new_board[curr_coord.x][curr_coord.y] = true;
                curr_coord = curr_coord.shift(orient);
            }

            if break_flag { continue; }

            board = new_board;
            ship_size += 1;

            placements[ship_size].0 = ship_size;
            placements[ship_size].1 = curr_coord;
            placements[ship_size].2 = orient;
        }

        placements
    }
}

