
mod game;
mod player;

use game::*;
use player::*;

fn main() {
    assert!(SIZE >= 5);
    
    let mut game: Game;
    println!("Rusty battleships\n");

    let user = User::new();
    let coord = user.turn();

    println!("{:?}", coord);
}

