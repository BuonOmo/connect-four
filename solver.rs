use crate::position::Position;

pub fn solve(position: String) -> Result<u16, &'static str> {
	// let maybe_position: Result<Position, &'static str> = position;
	match Position::try_from(position) {
		Result::Ok(pos) => Ok(solve_(pos)),
		Result::Err(err) => Err(err)
	}
}

fn solve_(position: Position) -> u16 {
	1
}

#[test]
pub fn test_solve() {
	
}
