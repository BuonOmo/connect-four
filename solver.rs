use crate::position::Position;
use crate::position;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
	Draw,
	Win(u8),
	Loose(u8),
}

impl From<i8> for Outcome {
	fn from(u: i8) -> Outcome {
		if u > 0 { return Outcome::Win(u as u8); }
		if u < 0 { return Outcome::Loose((-u) as u8); }

		return Outcome::Draw;
	}
}



pub fn solve(position: String) -> Result<Outcome, &'static str> {
	// let maybe_position: Result<Position, &'static str> = position;
	match Position::try_from(position) {
		Result::Ok(pos) => Ok(negamax(pos).into()),
		Result::Err(err) => Err(err)
	}
}

fn negamax(pos: Position) -> i8 {
	if pos.can_win() {
		return (
			position::GRID_SIZE.width as i8 * position::GRID_SIZE.height as i8
			+ 1
			- pos.move_count as i8
		) / 2;
	}
	if pos.is_terminal() { return 0; }

	let mut max_score = i8::MIN;

	for mov in pos.possible_moves() {
		let score = -negamax(pos.next(mov));
		if score > max_score { max_score = score };
	}

	return max_score;
}

#[test]
fn test_solve() {
	assert_eq!(solve("23163416124767223154467471272416755633".to_string()), Ok(Outcome::Draw))
}

#[cfg(test)]
mod global_tests {
	use super::*;
	use std::fs::File;
	use std::io::{self, BufRead};
	use std::path::Path;

	fn test_file(filename: String) {
		fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
		where P: AsRef<Path>, {
			let file = File::open(filename)?;
			Ok(io::BufReader::new(file).lines())
		}
		for line in read_lines(filename).unwrap() {
			let line_str = line.unwrap();
			let mut split = line_str.split(' ');
			let pos_str = split.next().unwrap();
			let outcome_int: i8 = split.next().unwrap().parse().unwrap();
			assert_eq!(solve(pos_str.to_string()), Ok(Outcome::from(outcome_int)));
		}
	}

	#[test]
	fn test_solve_end_easy() {
		test_file(format!("{}/data/Test_L3_R1", env!("CARGO_MANIFEST_DIR")))
	}

	#[test]
	#[ignore = "too slow yet"]
	fn test_solve_middle_easy() {
		test_file(format!("{}/data/Test_L2_R1", env!("CARGO_MANIFEST_DIR")))
	}
}

// #![feature(test)]

// extern crate test;

// #[cfg(test)]
// mod tests {

// 	use test::Bencher;

// 	#[bench]
// 	fn bench_xor_1000_ints(b: &mut Bencher) {
// 		b.iter(|| {
// 			// Use `test::black_box` to prevent compiler optimizations from disregarding
// 			// Unused values
// 			test::black_box((0..1000).fold(0, |old, new| old ^ new));
// 		});
// 	}
// }
