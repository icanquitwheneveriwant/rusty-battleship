
use crate::user::*;
use crate::computer::*;
use strum_macros::EnumIter;
use Orientation::*;
use GameStatus::*;


pub const SIZE: usize = 10;
pub const NUM_SHIPS: usize = 4;

#[derive(Debug)]
struct Board {
    state: [[bool; SIZE]; SIZE],
    //consider using hash table for convenience
    //ships: Vec<Ship>,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone, Copy)]
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
}

pub struct Game {
    status: GameStatus,
    p1: Box<dyn Player>,
    p2: Box<dyn Player>,
    p1_board: Board,
    p2_board: Board,
}

#[derive(Copy, Clone)]
pub enum GameStatus {
    Initialization,
    Turn(usize),
    Win(usize),
}

impl Game {
    pub fn new() -> Self {
        Game {
            status: Initialization,
            p1: Box::new(User::new("Player 1")),
            p2: Box::new(Computer::new("Computer")),
            p1_board: Board { state: [[false; SIZE]; SIZE], },//ships: Vec::new(), },
            p2_board: Board { state: [[false; SIZE]; SIZE], },//ships: Vec::new(), },

        }
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

                let (player, enemy_board) = if player_id == 0 { 
                    (&mut (*self.p1) ,  &mut self.p2_board) 
                } else {
                    (&mut (*self.p2), &mut self.p1_board) 
                };


                let shot_coord = player.turn();
                println!("{} plays ({}, {})", player.get_name(), shot_coord.x+1, shot_coord.y+1);
                let was_hit = enemy_board.state[shot_coord.x][shot_coord.y];

                println!("({}, {}) is a {}!", shot_coord.x+1, shot_coord.y+1, if was_hit { "hit" } else { "miss" });
                
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
}
