use crate::position::Position;

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
		Result::Ok(pos) => Ok(solve_(pos)),
		Result::Err(err) => Err(err)
	}
}

fn solve_(position: Position) -> Outcome {
	negamax(position).into()
}

// function negamax(node, depth, color) is
//     if depth = 0 or node is a terminal node then
//         return color × the heuristic value of node
//     value := −∞
//     for each child of node do
//         value := max(value, −negamax(child, depth − 1, −color))
//     return value

fn negamax(position: Position) -> i8 {
	println!("negamax({})", position.short_str());
	if position.will_win() { return 1; }
	if position.is_terminal() { return 0; }

	let mut max_score = i8::MIN;

	for mov in position.possible_moves() {
		let mut p2 = position.clone();
		p2.next(mov);

		let score = -negamax(p2);
		if score > max_score { max_score = score };
	}

	return max_score;
}

#[test]
pub fn test_solve() {
	assert_eq!(solve("23163416124767223154467471272416755633".to_string()), Ok(Outcome::Draw))
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
