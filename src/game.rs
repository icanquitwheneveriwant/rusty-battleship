

pub const SIZE: usize = 8;

struct Board {
    state: [[bool; SIZE]; SIZE],
}

#[derive(Debug)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
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

        Ok(Coord {x: (first-1) as u32, y: (second-1) as u32 })
    }
}

pub trait Player {
    fn turn(&self) -> Coord;
}

pub struct Game<'a> {
    board: Board,
    p1: &'a mut dyn Player,
    p2: &'a mut dyn Player,
}
