use crate::Mask;

struct Position {
	player_mask: Mask,
	pieces_mask: Mask,
	moves: u8
}

impl Position {
    pub fn check_alignment(&self) -> bool {
        let factors = [
            1,               // vertical
            GRID_SIZE.1,     // horizontal
            GRID_SIZE.1 - 1, // diag 1
            GRID_SIZE.1 + 1, // diag 2
        ];

        for factor in factors {
            let m = self.player_mask & (self.player_mask >> factor);
            if m & (m >> 2 * factor) != 0 {
                return true
            }
        }

        return false
    }
}

impl TryFrom<String> for Position {
	type Error = &'static str;
	fn try_from(s: String) -> Result<Self, Self::Error>  {
		if s.chars().any(|c|!c.is_digit(10)) {
			return Err("Not a position");
		}

		return Ok(Position {
			player_mask: 0.into(),
			pieces_mask: 0.into(),
			moves: 0
		})
	}
}

pub fn solve(position: String) -> Result<u16, &'static str> {
	// let maybe_position: Result<Position, &'static str> = position;
	return match Position::try_from(position) {
		Result::Ok(pos) => Ok(solve_(pos)),
		Result::Err(err) => Err(err)
	};
}

fn solve_(position: Position) -> u16 {
	1
}

#[test]
pub fn test_solve() {
	assert_eq!(solve("hey".to_string()), Err("Not a position"));
}
