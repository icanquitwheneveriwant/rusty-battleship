
use crate::player::*;

pub const SIZE: usize = 8;

#[derive(Debug)]
struct Board {
    state: [[bool; SIZE]; SIZE],
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

        if first > 8 || first < 1 || second > 8 || second < 1 {
            return Err(())
        }

        Ok(Coord {x: (first-1) as usize, y: (second-1) as usize })
    }
}

#[derive(Clone, Copy)]
pub enum Orientation { Up, Down, Left, Right }

use Orientation::*;

impl Coord {

    pub fn shift_dist(&self, dir: Orientation, dist: usize) -> Result<Coord, ()> {
        match dir {
            Up => {
                let diff: i64 = self.y as i64 + dist as i64;
                if diff >= SIZE as i64 { return Err(()) }
                Ok(Coord { x: self.x, y: diff as usize })
            },
            Down => {
                let diff: i64 = self.y as i64 - dist as i64;
                if diff < 0 { return Err(()) }
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
    fn place_ships(&self) -> [(usize, Coord, Orientation); 5];
    fn turn(&self) -> Coord;
}

pub struct Game {
    status: GameStatus,
    p1: Box<dyn Player>,
    p2: Box<dyn Player>,
    p1_board: Board,
    p2_board: Board,
}

#[derive(PartialEq)]
pub enum GameStatus {
    Initialization,
    P1Turn,
    P2Turn,
    P1Win,
    P2Win,
}

use GameStatus::*;

impl Game {
    pub fn new() -> Game {
        Game {
            status: Initialization,
            p1: Box::new(User::new()),
            p2: Box::new(User::new()),
            p1_board: Board { state: [[false; SIZE]; SIZE] },
            p2_board: Board { state: [[false; SIZE]; SIZE] },

        }
    }

    fn initialize(&mut self) {

        let placements = self.p1.place_ships();

        for placement in placements.iter() {
            let mut coord = placement.1;

            for _ in 0..(placement.0) {
                self.p1_board.state[coord.x][coord.y] = true;
                
            }
        }

        let placements = self.p2.place_ships();

        for placement in placements.iter() {
            let mut coord = placement.1;

            for _ in 0..(placement.0) {
                self.p2_board.state[coord.x][coord.y] = true;
                
            }
        }
    }

    pub fn turn(&mut self) -> GameStatus {
        if self.status == Initialization {
            self.initialize();
            return P1Turn
        }

        let player = if self.status == P1Turn { &mut self.p1 } else { &mut self.p2 };

        //self.p2.turn();

        GameStatus::P1Turn
    }
}
