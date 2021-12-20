use crate::position::Position;
use crate::position;

use std::collections::HashMap;

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
	positions_checked: u128,
	transposition_table: HashMap<u64, (u8, i8)>
}

type SolverResult = (u8, u128, Outcome);

struct MoveScore(u8, (u64, u8));

impl std::cmp::PartialEq for MoveScore {
    fn eq(&self, other: &Self) -> bool { self.1.eq(&other.1) }
}

impl std::cmp::PartialOrd for MoveScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { self.1.partial_cmp(&other.1) }
}

impl std::cmp::Eq for MoveScore {}

impl std::cmp::Ord for MoveScore {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.1.cmp(&other.1) }
}

impl Solver {
	pub fn explain_outcome(position: Position, outcome: Outcome) -> String {
		let end_in_x_moves = |x: u8|
			(Position::max_moves() as i8 - position.move_count as i8) / 2 - x as i8 + 1;
		match outcome {
			Outcome::Draw => "draw".to_string(),
			Outcome::Win(x) => format!("win in {} moves", end_in_x_moves(x)),
			Outcome::Loose(x) => format!("loose in {} moves", end_in_x_moves(x))
		}
	}

	pub fn solve(position: Position) -> SolverResult {
		let mut solver = Solver::new();
		match solver.strongly_solve(position) {
			(mov, outcome) => {
				assert_ne!(mov, u8::MAX);
				(mov, solver.positions_checked, outcome.into())
			}
		}
	}

	pub fn weakly_solve(position: Position) -> SolverResult {
		let mut solver = Solver::new();
		match solver.weakly_solve_(position) {
			(mov, outcome) => {
				// assert_ne!(mov, u8::MAX, "impossible best move");
				(mov, solver.positions_checked, outcome.into())
			}
		}
	}

	pub fn solve_str(position: String) -> Result<SolverResult, &'static str> {
		match Position::try_from(position) {
			Result::Ok(position) => Ok(Solver::solve(position)),
			Result::Err(err) => Err(err)
		}
	}

	pub fn weakly_solve_str(position: String) -> Result<SolverResult, &'static str> {
		match Position::try_from(position) {
			Result::Ok(position) => Ok(Solver::weakly_solve(position)),
			Result::Err(err) => Err(err)
		}
	}

	fn new() -> Solver { Solver { positions_checked: 0, transposition_table: HashMap::new() } }

	fn weakly_solve_(&mut self, position: Position) -> (u8, i8) {
		self.negamax(position, -1,  1, 14 + 2 * position.move_count as i8)
	}

	fn strongly_solve(&mut self, position: Position) -> (u8, i8) {
		self.negamax(position, i8::MIN + 1, i8::MAX - 1, 14 + 2 * position.move_count as i8)
	}

	fn negamax(&mut self, pos: Position, mut alpha: i8, mut beta: i8, depth: i8) -> (u8, i8) {
		self.positions_checked += 1;
		// Check for draw, this is ok to do it here, but if given an
		// already winning position with a full grid, negamax would
		// still consider it a draw.
		if pos.is_terminal() { return (0, 0) }

		// upper bound of the score (if winning, then this is the actual score).
		let mut position_evaluation = (
			position::GRID_SIZE.width as i8 * position::GRID_SIZE.height as i8
			+ 1
			- pos.move_count as i8
		) / 2;

		let mut estimate_scores: std::collections::BinaryHeap<MoveScore> = std::collections::BinaryHeap::new();

		for mov in pos.possible_moves() {
			if pos.wins(mov) {
				return (mov, position_evaluation)
			}

			estimate_scores.push(MoveScore(mov, (pos.move_score(mov), [0,1,2,3,2,1,0][mov as usize])))
		}

		// Initialized to make sure we compile, however, this will
		// effectively never be returned as it is.
		let mut best_mov = u8::MAX;
		// TODO: it still gives u8::MAX for position
		// 661444666637315414455515


		// Arbitrary chosen move, it will be changed if
		// a better move exits.
		// let mut best_mov = pos.possible_moves().nth(0);

		if let Some((cached_best_mov, cached_upper_bound)) = self.transposition_table.get(&pos.key()) {
			position_evaluation = *cached_upper_bound;
			best_mov = *cached_best_mov;
		}

		if beta > position_evaluation {
			beta = position_evaluation;                  // max possible score anyway.
			if alpha >= beta { return (best_mov, beta) } // we can prune early, the window is empty.
		}

		// A realy dirty way to go faster in early game. We return a position
		// estimation that is just: _I think this is a draw_.
		if depth == 0 {
			if let Some(MoveScore(mov, _)) = estimate_scores.peek() {
				return (*mov, 0); // just assume it is a draw.
			}
		}

		// Moves in the center are more likely to provide an efficient result, this
		// heuristic should massively improve our alpha-beta pruning.
		while let Some(MoveScore(mov, _)) = estimate_scores.pop() {
			// Since opponent win condition is the opposite of ours, their
			// window is [-beta;-alpha].
			let score = match self.negamax(pos.next(mov), -beta, -alpha, depth - 1) {
				(_, sc) => -sc
			};

			// Prune if we find better than our window.
			if score >= beta { return (mov, score) }

			// Reduce the alpha-beta window is possible.
			// TODO: use a move queue there and pick one at random.
			if score > alpha {
				alpha = score;
				best_mov = mov;
			}
		}

		self.transposition_table.insert(pos.key(), (best_mov, alpha)); // save the upper bound of the position
		return (best_mov, alpha)
	}
}

#[test]
fn test_solve() {
	// assert!(matches!(
	// 	Solver::weakly_solve_str("661444666637315414455515".to_string()),
	// 	Ok(_)
	// ));
	assert!(matches!(
		Solver::solve_str("661444666637315414455515".to_string()),
		Ok(_)
	));
	assert!(
		matches!(
			Solver::solve_str("4444233333246".to_string()),
			Ok((.., Outcome::Win(x))) if x > 5
		),
		"Got {:?}", Solver::solve_str("4444233333246".to_string())
	);
	assert!(matches!(Solver::solve_str("23163416124767223154467471272416755633".to_string()), Ok((.., Outcome::Draw))));
}

#[test]
fn test_from_beginning() {
	assert!(matches!(Solver::weakly_solve_str("".to_string()), Ok((3, _, Outcome::Win(_)))));
}
