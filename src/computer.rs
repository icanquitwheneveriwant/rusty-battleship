
use enum_as_inner::EnumAsInner;
use crate::user::{ViewState, PlayerView};
use crate::game::{SIZE, Player, Coord, Orientation, NUM_SHIPS};
use Orientation::*;
use ViewState::*;
use BrainState::*;
use strum::IntoEnumIterator;
use rand::Rng;


struct IteratingState {
    initial_hit: Coord,
    coord: Coord,
    dir: Orientation,
    must_be_vertical: bool,
}

#[derive(PartialEq)]
enum BrainState {
    Searching,
    Iterating,
}

pub struct Computer {
    view: PlayerView,
    name: String,
    brain_state: BrainState,
    iter_state: IteratingState
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
            view: PlayerView { state: [[Blank; SIZE]; SIZE] },
            brain_state: Searching,

            //iter_state initial value is meaningless
            iter_state: IteratingState { 
                initial_hit: Coord { x: 0, y: 0 },
                coord: Coord { x: 0, y: 0 },
                dir: Up,
                must_be_vertical: false,
             }
        }
    }

    //Attempts to find a ship vertically first,
    //and then tries horizontally

    //Sticks to either vertical or horizontal,
    //and goes back to probability guessing after
    //there are no more hits along that axis
    pub fn try_new_direction(&mut self) {

        match self.iter_state.dir {
            Up => {
                self.iter_state.coord = self.iter_state.initial_hit;
                self.iter_state.dir = Down;
            },
            Down => {
                if self.iter_state.must_be_vertical {
                    self.brain_state = Searching;
                } else {
                    self.iter_state.dir = Left;
                }
            },
            Left => {
                self.iter_state.coord = self.iter_state.initial_hit;
                self.iter_state.dir = Right;
            },
            Right => {
                self.brain_state = Searching;
            },
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
    
    fn turn(&mut self) -> Coord {

        match self.brain_state {
            Searching => {

                let heat_map = self.gen_heat_map();

                //finds coordinates of max value in heat map
                /*
               let (hottest_x, hottest_y) = (0..SIZE)
                    .flat_map(|x| (0..SIZE).map(move |y| (x, y)))
                    .max_by_key(|(x, y)| &heat_map[*x][*y]).unwrap();*/

                //----------
                //let mut hottest_coord = Coord { x: 0, y: 0 };
                let mut hottest_x = 0;
                let mut hottest_y = 0;
                let mut hottest_val = 0;

                for x in 0..SIZE {
                    for y in 0..SIZE {
                        if heat_map[x][y] > hottest_val {
                            //hottest_coord = Coord { x: x, y: y };
                            (hottest_x, hottest_y) = (x, y);
                            hottest_val = heat_map[x][y];
                        }
                    }
                }


                //----------

                //so fucking hot
                Coord { x: hottest_x, y: hottest_y }
            }

            Iterating => {
                let mut next_coord = self.iter_state.coord.shift(self.iter_state.dir);
                if next_coord.is_ok() && 
                    self.view.state[next_coord.unwrap().x][next_coord.unwrap().y] == Blank {

                    self.try_new_direction();
                    self.turn()
                } else {
                    next_coord.unwrap()
                }
            },
        }

    }

    fn hit_feedback(&mut self, coord: Coord, hit: bool) {
        self.view.state[coord.x][coord.y] = if hit { Hit } else { Miss };

        if hit && self.brain_state == Searching {
            self.brain_state = Iterating;
            //add sequence length variable later
            self.iter_state = IteratingState{ initial_hit: coord, coord: coord, dir: Up, must_be_vertical: false };
       
        } else if self.brain_state == Iterating && !hit {
            self.try_new_direction();
        }
    }

   fn count_hits(&self) -> usize {
        self.view.state.iter().flatten().filter(|&&state| state == Hit).count()
    }

    fn get_name(&self) -> &str {
        &self.name
    }
}
