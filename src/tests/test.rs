
use crate::game::*;
use GameStatus::*;
use crate::computer::*;

struct Dummy {
	//reuse random ship placement code from computer
	computer_interface: Computer,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn computer_avg_win_turns() {
		let mut game = Game::new(Box::new(Computer::new("Computer")), Box::new(Dummy::new())); 

		let mut computer_turns = 0;

		const iterations: usize = 200;

		for _ in 0..iterations {
			while !matches!(game.turn(), Win(_)) {
				computer_turns += 1;
				//this turn is dummy turn, loop condition is computer turn
				game.turn();
			}

			game = Game::new(Box::new(Computer::new("Computer")), Box::new(Dummy::new()));
		}

		println!("\n\nNumber of turns to win: {}", computer_turns as f64 / iterations as f64);

		assert_ne!(computer_turns, 0);
	}
}

/*
pub trait Player {
    //throughout the code, ships are identified by thier size
    fn place_ships(&self) -> [Ship; NUM_SHIPS];
    //consider changing name
    fn turn(&mut self) -> Coord;
    fn hit_feedback(&mut self, coord: Coord, hit: bool);
    //really wish Rust had inheritance!
    fn count_hits(&self) -> usize;
    fn get_name(&self) -> &str;
}*/

//Placeholder player two that just does nothing
impl Dummy {
	pub fn new() -> Self {
		Dummy { computer_interface: Computer::new("") }
	}
}

impl Player for Dummy {
	fn place_ships(&self) -> [Ship; NUM_SHIPS] { self.computer_interface.place_ships() }
	fn turn(&mut self) -> Coord { Coord{ x: 0, y: 0 } }
	fn count_hits(&self) -> usize { 0 }
	fn get_name(&self) -> &str { "" }
	fn hit_feedback(&mut self, _coord: Coord, _hit: bool) {}
	fn alert_opponent_move(&self, _shot_coord: Coord, _hit: bool, _enemy_name: &str) {}
}
