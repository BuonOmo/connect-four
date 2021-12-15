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

	pub fn weakly_solve(position: String) -> Result<(u128, Outcome), &'static str> {
		let mut solver = Solver::new();
		match solver._weakly_solve(position) {
			Result::Ok(outcome) => Ok((solver.positions_checked, outcome)),
			Result::Err(err) => Err(err)
		}
	}

	fn new() -> Solver { Solver { positions_checked: 0 } }

	fn _weakly_solve(&mut self, position: String) -> Result<Outcome, &'static str> {
		match Position::try_from(position) {
			Result::Ok(pos) => Ok(self.negamax(pos, -1,  1).into()),
			Result::Err(err) => Err(err)
		}
	}


	fn strongly_solve(&mut self, position: String) -> Result<Outcome, &'static str> {
		match Position::try_from(position) {
			Result::Ok(pos) => Ok(self.negamax(pos, i8::MIN + 1, i8::MAX - 1).into()),
			Result::Err(err) => Err(err)
		}
	}

	fn negamax(&mut self, pos: Position, mut alpha: i8, mut beta: i8) -> i8 {
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

		if beta > position_evaluation {
			beta = position_evaluation;       // max possible score anyway.
			if alpha >= beta { return beta; } // we can prun early, the window is empty.
		}

		// Moves in the center are more likely to provide an efficient result, this
		// heuristic should massively improve our alpha-beta pruning.
		for mov in [3, 4, 2, 5, 1, 6, 0] {
			if !pos.can_play(mov) { continue }

			// Prune the window by checking the score of the opponent.
			// Since opponent win condition is the opposite of ours, their
			// window is [-beta;-alpha].
			alpha = std::cmp::max(alpha, -self.negamax(pos.next(mov), -beta, -alpha));
			if alpha >= beta { break; }
		}

		return alpha;
	}
}

#[test]
fn test_solve() {
	assert!(matches!(Solver::solve("23163416124767223154467471272416755633".to_string()), Ok((_, Outcome::Draw))))
}
