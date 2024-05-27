

pub const SIZE: usize = 8;

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
    pub fn shift(&self, dir: Orientation) -> Coord {
        match dir {
            Up => Coord { x: self.x, y: self.y+1 },
            Down => Coord { x: self.x, y: self.y-1 },
            Left => Coord { x: self.x-1, y: self.y },
            Right => Coord { x: self.x+1, y: self.y },
        }
    }

    pub fn shift_dist(&self, dir: Orientation, dist: usize) -> Coord {
        match dir {
            Up => Coord { x: self.x, y: self.y+dist },
            Down => Coord { x: self.x, y: self.y-dist },
            Left => Coord { x: self.x-dist, y: self.y },
            Right => Coord { x: self.x+dist, y: self.y },
        }
    }

    pub fn in_board(&self) -> bool {
        self.x >= 0 && self.x < SIZE && self.y >= 0 && self.y < SIZE
    }
}

/*
impl opps::Add<Coord> for Coord {
    type Output = Coord;

    fn step(self, _rhs: Coord) {

    }
}*/

pub trait Player {
    fn place_ships(&self) -> [(usize, Coord, Orientation); 5];
    fn turn(&self) -> Coord;
}

pub struct Game<'a> {
    board: Board,
    p1: &'a mut dyn Player,
    p2: &'a mut dyn Player,
}
