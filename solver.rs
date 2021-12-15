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

pub struct Solver {
	positions_checked: u128
}

impl Solver {
	pub fn solve(position: String) -> Result<(u128, Outcome), &'static str> {
		let mut solver = Solver::new();
		match solver.strongly_solve(position) {
			Result::Ok(outcome) => Ok((solver.positions_checked, outcome)),
			Result::Err(err) => Err(err)
		}
	}

	fn new() -> Solver { Solver { positions_checked: 0 } }

	fn strongly_solve(&mut self, position: String) -> Result<Outcome, &'static str> {
		match Position::try_from(position) {
			Result::Ok(pos) => Ok(self.negamax(pos).into()),
			Result::Err(err) => Err(err)
		}
	}

	fn negamax(&mut self, pos: Position) -> i8 {
		self.positions_checked += 1;
		// Check for draw, this is ok to do it here, but if given an
		// already winning position with a full grid, negamax would
		// still consider it a draw.
		if pos.is_terminal() { return 0; }

		// upper bound of the score (if winning, then this is the actual score).
		let position_evaluation = (
			position::GRID_SIZE.width as i8 * position::GRID_SIZE.height as i8
			+ 1
			- pos.move_count as i8
		) / 2;

		if pos.can_win() { return position_evaluation; }

		let	mut max_score = i8::MIN;

		for mov in pos.possible_moves() {
			max_score = std::cmp::max(max_score, -self.negamax(pos.next(mov)));
		}

		return max_score;
	}
}

#[test]
fn test_solve() {
	assert!(matches!(Solver::solve("23163416124767223154467471272416755633".to_string()), Ok((_, Outcome::Draw))))
}
