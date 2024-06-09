
use crate::user::{ViewState, PlayerView};
use crate::game::{SIZE, Player, Coord, Orientation, NUM_SHIPS};
use Orientation::*;
use ViewState::*;
use strum::IntoEnumIterator;
use rand::Rng;

pub struct Computer {
    view: PlayerView,
    name: String
}


impl Computer {
    //Heat map of all possible opponent ship placements
    fn gen_heat_map(&self) -> [[u8; SIZE]; SIZE] {
        let mut heat_map = [[0; SIZE]; SIZE];

        for ship_size in 2..=NUM_SHIPS {
            for dir in Orientation::iter() {
                for x in 0..SIZE {
                    for y in 0..SIZE {

                        let placement = self.view.place_ship(ship_size, Coord {x: x, y: y}, dir);
                        if placement.is_ok() {
                            let mut coord = Coord {x: x, y: y};

                            for _ in 0..ship_size {
                                heat_map[coord.x][coord.y] += 1;
                                //unwrap_or is to handle last iteration, which is unused
                                coord = coord.shift(dir).unwrap_or(coord);
                            }
                        }
                    }
                }
            }
        }

        heat_map
    }

    pub fn new(name: &str) -> Self {
        Computer { 
            name: name.to_string(),
            view: PlayerView { state: [[Blank; SIZE]; SIZE] }
        }
    }
}

impl Player for Computer {
    fn place_ships(&self) -> [(usize, Coord, Orientation); NUM_SHIPS] {  
        let mut placements = [(0, Coord { x: 0, y: 0 },  Up); NUM_SHIPS];

        let mut ship_size = 2;

        while ship_size <= NUM_SHIPS+1 {
            let mut rng = rand::thread_rng();

            let rand_orient = match rng.gen_range(0..4) {
                0 => Up,
                1 => Down,
                2 => Left,
                3 => Right,
                _ => unreachable!(),
            };

            let rand_coord = Coord { x: rng.gen_range(0..SIZE), y: rng.gen_range(0..SIZE) };

            let new_view = self.view.place_ship(ship_size, rand_coord, rand_orient);

            if new_view.is_err() { continue; }

            placements[ship_size-2].0 = ship_size;
            placements[ship_size-2].1 = rand_coord;
            placements[ship_size-2].2 = rand_orient;

            ship_size += 1;
        }

        placements
    }
    
    fn turn(&self) -> Coord {
        let mut hottest_coord = Coord { x: 0, y: 0 };
        let mut hottest_val = 0;

        let heat_map = self.gen_heat_map();

        for x in 0..SIZE {
            for y in 0..SIZE {
                if heat_map[x][y] > hottest_val {
                    hottest_coord = Coord { x: x, y: y };
                    hottest_val = heat_map[x][y];
                }
            }
        }

        //so fucking hot
        hottest_coord
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
}
