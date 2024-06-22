
use crate::user::*;
use crate::computer::*;
use strum_macros::EnumIter;
use Orientation::*;
use GameStatus::*;
use std::mem;


pub const SIZE: usize = 10;
pub const NUM_SHIPS: usize = 4;

#[derive(Debug)]
struct Board {
    state: [[bool; SIZE]; SIZE],
    //consider using hash table for convenience
    //ships: Vec<Ship>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ship {
    pub len: usize,
    pub coord: Coord,
    pub orient: Orientation,
}

impl Ship {
    pub fn uninitialized() -> Self {
        Self { len: 0, coord: Coord{x: 0, y: 0}, orient: Up }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}


impl std::str::FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();

        let (prefix, s) = s.split_once('(').ok_or(())?;
        if !prefix.is_empty() { return Err(()) }
        let (first, s) = s.split_once(',').ok_or(())?;
        let (second, s) = s.split_once(')').ok_or(())?;
        if !s.is_empty() { return Err(()) }

        let first: i32 = first.parse().map_err(|_| ())?;
        let second: i32 = second.parse().map_err(|_| ())?;

        if first > SIZE as i32 || first < 1 || second > SIZE as i32 || second < 1 {
            return Err(())
        }

        Ok(Coord {x: (first-1) as usize, y: (second-1) as usize })
    }
}

#[derive(Clone, Copy, EnumIter, PartialEq, Debug)]
pub enum Orientation { Up, Down, Left, Right }

impl Coord {

    pub fn shift_dist(&self, dir: Orientation, dist: usize) -> Result<Coord, ()> {
        match dir {
            Up => {
                let diff: i64 = self.y as i64 - dist as i64;
                if diff < 0 { return Err(()) }
                Ok(Coord { x: self.x, y: diff as usize })
            },
            Down => {
                let diff: i64 = self.y as i64 + dist as i64;
                if diff >= SIZE as i64 { return Err(()) }
                Ok(Coord { x: self.x, y: diff as usize })
            },
            Left => {
                let diff: i64 = self.x as i64 - dist as i64;
                if diff < 0 { return Err(()) }
                Ok(Coord { x: diff as usize, y: self.y })
            },
            
            Right => {
                let diff: i64 = self.x as i64 + dist as i64;
                if diff >= SIZE as i64 { return Err(()) }
                Ok(Coord { x: diff as usize, y: self.y })
            },
        }
    }

    pub fn shift(&self, dir: Orientation) -> Result<Coord, ()> {
        self.shift_dist(dir, 1)
    }


    pub fn in_board(&self) -> bool {
        self.x < SIZE && self.y < SIZE
    }
}


pub trait Player {
    //throughout the code, ships are identified by thier size
    fn place_ships(&self) -> [Ship; NUM_SHIPS];
    //consider changing name
    fn turn(&mut self) -> Coord;
    fn hit_feedback(&mut self, coord: Coord, hit: bool);
    //really wish Rust had inheritance!
    fn count_hits(&self) -> usize;
    fn get_name(&self) -> &str;
    //this literally only exists so that IO is off while
    //testing for the Computer's turn
    fn alert_opponent_move(&self, coord: Coord, hit: bool, enemy_name: &str);
}

pub struct Game {
    status: GameStatus,
    p1: Box<dyn Player>,
    p2: Box<dyn Player>,
    p1_board: Board,
    p2_board: Board,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GameStatus {
    Initialization,
    Turn(usize),
    Win(usize),
}

impl Game {
    pub fn new(p1: Box<dyn Player>, p2: Box<dyn Player>) -> Self {
        Game {
            status: Initialization,
            p1:p1,
            p2: p2,
            p1_board: Board { state: [[false; SIZE]; SIZE], },//ships: Vec::new(), },
            p2_board: Board { state: [[false; SIZE]; SIZE], },//ships: Vec::new(), },

        }
    }

    pub fn new_user_computer() -> Self {
        Game::new(Box::new(User::new("Player 1")), Box::new(Computer::new("Computer")))
    }

    pub fn new_two_user() -> Self {
        Game::new(Box::new(User::new("Player 1")), Box::new(User::new("Player 2")))
    }

    fn initialize(&mut self) {

        let placements = self.p1.place_ships();

        for placement in placements.iter() {
            let mut coord = placement.coord;

            for _ in 0..(placement.len) {
                self.p1_board.state[coord.x][coord.y] = true;
                //unwrap_or is just for last iteration of the loop
                coord = coord.shift(placement.orient).unwrap_or(Coord{ x: 0, y: 0 });
            }
        }

        let placements = self.p2.place_ships();

        for placement in placements.iter() {
            let mut coord = placement.coord;

            for _ in 0..(placement.len) {
                self.p2_board.state[coord.x][coord.y] = true;
                coord = coord.shift(placement.orient).unwrap_or(Coord{ x: 0, y: 0 });
            }
        }

        self.status = Turn(0)
    }

    pub fn turn(&mut self) -> GameStatus {

        match self.status {
            Initialization => {
                self.initialize();
            },

            Turn(player_id) => {

                //try to eliminate this mess later
                let (player, enemy, enemy_board) = if player_id == 0 { 
                    (&mut (*self.p1), &mut (*self.p2), &mut self.p2_board) 
                } else {
                    (&mut (*self.p2), &mut (*self.p1), &mut self.p1_board) 
                };

                let shot_coord = player.turn();
                let was_hit = enemy_board.state[shot_coord.x][shot_coord.y];
                enemy.alert_opponent_move(shot_coord, was_hit, player.get_name());   
                player.hit_feedback(shot_coord, was_hit);

                //n*(n+1)/2 is sum from 1 to N formula
                //super cool story about how this formula was discovered btw
                if player.count_hits() == (NUM_SHIPS)*(NUM_SHIPS+1)/2-1 {
                    self.status = Win(player_id);
                } else {
                    self.status = Turn((player_id+1)%2);
                }
            },

            Win(_) => {},
        }

        self.status
    }

    pub fn get_player_name(&self, player_id: usize) -> &str {
        if player_id == 0 { self.p1.get_name() } else { self.p2.get_name() }
    }  

    pub fn check_horiz_adjacency(current: Ship, placements: Vec<Ship>) -> bool {

        for other in placements {

            if other.len==0 || other==current { continue; }

            let (ship_size, coord, orient) = (current.len, current.coord, current.orient);

            let both_horiz = (orient == Left || orient == Right) &&
                                    (other.orient == Left || other.orient == Right);

            let adjacent_y_axis = coord.y.abs_diff(other.coord.y) == 1;

            let mut start_coord = current.coord;
            let mut end_coord = start_coord.shift_dist(orient, ship_size-1).unwrap();
            if start_coord.x > end_coord.x { 
                mem::swap(&mut start_coord, &mut end_coord); 
            }

            let mut other_start_coord = other.coord;
            let mut other_end_coord = other.coord.shift_dist(other.orient, other.len-1).unwrap();
            if other_start_coord.x > other_end_coord.x { 
                mem::swap(&mut other_start_coord, &mut other_end_coord); 
            }


            let (larger_span, smaller_span) = if ship_size > other.len { 
                ((start_coord, end_coord), (other_start_coord, other_end_coord))
            } else {
                ((other_start_coord, other_end_coord), (start_coord, end_coord))
            };

            let x_overlapping = smaller_span.1.x >= larger_span.0.x && smaller_span.0.x <= larger_span.1.x;

            if both_horiz && adjacent_y_axis && x_overlapping {
                return true;
            }
        }

        false
    }
}
