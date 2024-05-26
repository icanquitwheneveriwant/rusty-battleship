

pub const SIZE: usize = 8;

struct Board {
    state: [[bool; SIZE]; SIZE],
}

pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl std::str::FromStr for Coord {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (prefix, s) = s.split_once('(').ok_or(())?;
        if !prefix.is_empty() { return Err(()) }
        let (first, s) = s.split_once(',').ok_or(())?;
        let (second, s) = s.split_once(')').ok_or(())?;
        if !s.is_empty() { return Err(()) }

        let first = first.parse().map_err(|_| ())?;
        let second = second.parse().map_err(|_| ())?;
        Ok(Coord{x: first, y: second})
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
