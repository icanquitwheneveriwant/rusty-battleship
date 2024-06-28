
use ViewState::*;
use crate::game::{Game, SIZE, Player, Coord, Orientation, Ship, NUM_SHIPS};
use Orientation::*;
use std::io::stdin;
use std::str::FromStr;
use std::mem;


#[derive(Clone, Copy, PartialEq)]
pub enum ViewState {
    Hit, Miss, Blank,
}

#[derive(Clone)]
pub struct PlayerView {
    pub state: [[ViewState; SIZE]; SIZE]
}

use std::fmt;
impl fmt::Display for PlayerView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

        //Rust throws a temper tantrum if I don't store the return value
        _ = write!(f, "\n  |");

        for i in 1..=SIZE {
            write!(f, "{}|", i)?;
        }

        assert!(SIZE <= 26);
        for y in 0..SIZE {
            //Just let it crash or something if it's more than 26 lol
            write!(f, "\n{} |", alphabet.as_bytes()[y] as char)?;

            for x in 0..SIZE {
                let indicator = match self.state[x][y] {
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


impl PlayerView {
    pub fn place_ship(&self, ship_size: usize, coord: Coord, orient: Orientation) -> Result<Self, ()> {
        
        let Ship = Hit;

        let mut curr_coord = Ok(coord);
        let mut new_view = self.clone();

        for _ in 0..ship_size {
            if let Ok(new_coord) = curr_coord {
                if new_coord.in_board() && new_view.state[new_coord.x][new_coord.y] == Blank {
                    new_view.state[new_coord.x][new_coord.y] = Ship;
                    curr_coord = new_coord.shift(orient);
                    continue;
                } else {
                    return Err(());
                }
            } else {
                return Err(());
            }
        }

        Ok(new_view)
    }

}

pub struct User {
    pub name: String,
    pub view: PlayerView,
}

impl User {
    pub fn new(name: &str) -> User {
        User { 
            name: name.to_string(),
            view: PlayerView { state: [[Blank; SIZE]; SIZE] }
        }
    }
}

impl Player for User {

     fn place_ships(&self) -> [Ship; NUM_SHIPS] {  
        let mut placement_view = PlayerView { state: [[Blank; SIZE]; SIZE] };

        let mut placements = [Ship::uninitialized(); NUM_SHIPS];

        //ship sizes start at 2 according to the rules
        let mut ship_size = 2;
        let mut told_user_ai_error = false;

        let mut told_user_ai_error = false;

        while ship_size <= NUM_SHIPS+1 {

            let ship_type_str = match ship_size {
                5 => String::from("Carrier"),
                4 => String::from("Battleship"),
                3 => String::from("Cruiser"),
                2 => String::from("Destroyer"),
                _ => format!("Size {ship_size}"),
            };

            println!("{}", placement_view);
            println!("Place your {ship_type_str} ship:\n");

            let mut input_str = String::new();

            stdin().read_line(&mut input_str).unwrap();
            let coord = Coord::from_str(&input_str);

            input_str.clear();
            stdin().read_line(&mut input_str).unwrap();
            //let trimmed_slice = input_str.as_str().trim();

            let orient: Result<Orientation, ()> = match input_str.as_str().trim() {
                "Up" => Ok(Up),
                "Down" => Ok(Down),
                "Left" => Ok(Left),
                "Right" => Ok(Right),
                _ => Err(())
            };

            if coord.is_err() || orient.is_err() {
                println!("Usage: \n([x], [y])\n[Up, Down, Left, or Right]");
                continue;
            }

            let orient = orient.unwrap();
        

            let mut curr_coord = coord;
            let mut valid_flag = true;

            let mut new_view = placement_view.clone();

            for _ in 0..ship_size {
                if let Ok(new_coord) = curr_coord {
                    if new_coord.in_board() && new_view.state[new_coord.x][new_coord.y] != Hit {
                        new_view.state[new_coord.x][new_coord.y] = Hit;
                        curr_coord = new_coord.shift(orient);
                        continue;
                    }
                }

                valid_flag = false;
                break;
            }

            if !valid_flag {
                println!("Invalid ship placement");
                continue;  
            }

            //Best way to compensate for an algorithm edge case
            let coord = coord.unwrap();

            let current = Ship{ len: ship_size, coord: coord, orient: orient };

            if Game::check_horiz_adjacency(current, placements.to_vec()) {
                println!("\nInvalid placement: AI algorithm performs poorly \
                    when two horizontal ships are placed next to each other");

                if !told_user_ai_error {
                    println!("\nIt's a perfectly legal move btw,\n \
                    I just don't feel like adapting the algorithm lmao");
                    told_user_ai_error = true;
                }

                continue;
            }

            placement_view = new_view;

            //-2 since ship placements start at size 2
            placements[ship_size-2] = current;

            ship_size += 1;
        }

        placements
    }
    

    fn turn(&mut self) -> Coord {

        println!("{}", self.view);
        println!("\n{}'s turn\nEnter coordinates:\n", self.name);

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
                2 => println!("Is it really that fucking hard to understand?"),
                3 => println!("Jesus fucking christ how hard is it to enter a fucking coordinate pair"),
                _ => panic!("\nError: user is too stupid to follow simple instructions\n"),
            }
        }
    }

    fn hit_feedback(&mut self, coord: Coord, hit: bool) {
        self.view.state[coord.x][coord.y] = if hit { Hit } else { Miss };
    }

    fn count_hits(&self) -> usize {
        self.view.state.iter().flatten().filter(|&&state| state == Hit).count()
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn alert_opponent_move(&self, shot_coord: Coord, hit: bool, enemy_name: &str) {
        println!("{} plays ({}, {})", enemy_name, shot_coord.x+1, shot_coord.y+1);
        println!("({}, {}) is a {}!", shot_coord.x+1, shot_coord.y+1, if hit { "hit" } else { "miss" });
    }
}

