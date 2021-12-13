use std::collections::LinkedList;

pub const GRID_SIZE: (u8, u8) = (7, 6);

#[derive(Debug, PartialEq, Eq)]
pub struct Position {
	player_mask: u64,
	pieces_mask: u64,
	move_count: u8,
	pub moves: LinkedList<u8>
}

impl Position {
	pub fn new_empty() -> Position {
		Position { player_mask: 0, pieces_mask: 0, move_count: 0, moves: LinkedList::new() }
	}
    pub fn is_winning(&self) -> bool {
        let factors = [
            1,               // vertical
            GRID_SIZE.1,     // horizontal
            GRID_SIZE.1 - 1, // diag 1
            GRID_SIZE.1 + 1, // diag 2
        ];

        for factor in factors {
            let m = self.player_mask & (self.player_mask >> factor);
            if m & (m >> (2 * factor)) != 0 {
                return true
            }
        }

        return false
    }

	pub fn can_play(&self, column: u8) -> bool {
		let column_mask = 1 << (GRID_SIZE.1 - 1) << (column * GRID_SIZE.1);

		// println!("col\n{}", Mask::from(column_mask));
		// println!("pieces\n{}", Mask::from(self.pieces_mask));

		return (self.pieces_mask & column_mask) == 0;
	}

	pub fn next(&mut self, column: u8) -> bool {
		if !self.can_play(column) { return false; }

		self.player_mask ^= self.pieces_mask;
		self.pieces_mask |= self.pieces_mask + (1 << (column * GRID_SIZE.1));
		self.moves.push_back(column);
		self.move_count += 1;
		return true;
		// Immutable?
		// let pieces_mask = self.pieces_mask | (self.pieces_mask + 1 << column * GRID_SIZE.1);
		// let mut moves = self.moves.clone();
		// moves.push_back(column);
		// Some(
		// 	Position {
		// 		player_mask: self.pieces_mask ^ self.player_mask,
		// 		pieces_mask: pieces_mask,
		// 		move_count: self.move_count + 1,
		// 		moves: moves,
		// 	}
		// )
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_play_1() {
		let pos = Position::try_from("111111".to_string()).unwrap();



		assert!(!pos.can_play(0));
		for i in 1..GRID_SIZE.0 { assert!(pos.can_play(i)) };
	}

	#[test]
	fn can_play_2() {
		let pos = Position::try_from("222222".to_string()).unwrap();

		assert!(pos.can_play(0));
		assert!(!pos.can_play(1));
		for i in 2..GRID_SIZE.0 { assert!(pos.can_play(i)) };
	}
}

impl TryFrom<String> for Position {
	type Error = &'static str;
	fn try_from(s: String) -> Result<Self, Self::Error>  {
		if s.chars().any(|c|!c.is_digit(10)) {
			return Err("Not a position.");
		}

		let mut pos = Position::new_empty();
		for mov in s.chars().map(|c|c.to_digit(10).unwrap()-1) {
			if !pos.next(mov as u8) { return Err("Position contains an invalid move.") }
		}
		return Ok(pos);
	}
}

#[cfg(test)]
mod tests_try_from_string {
    use super::*;


	#[test]
	fn str_1() {
		assert_eq!(Position::try_from("1".to_string()), Ok(Position {
			player_mask: 0,
			pieces_mask: 1,
			move_count: 1,
			moves: LinkedList::from([0])
		}));
	}

	#[test]
	fn str_111() {
		assert_eq!(Position::try_from("111".to_string()), Ok(Position {
			player_mask: 2,
			pieces_mask: 7,
			move_count: 3,
			moves: LinkedList::from([0, 0, 0])
		}));
	}

	#[test]
	fn str_22() {
		assert_eq!(Position::try_from("22".to_string()), Ok(Position {
			player_mask: 1 << 6,
			pieces_mask: (1|2) << 6,
			move_count: 2,
			moves: LinkedList::from([1, 1])
		}));
	}

	#[test]
	fn column_full() {
		assert_eq!(
			Position::try_from("6666666".to_string()),
			Err("Position contains an invalid move.")
		)
	}

	#[test]
	#[ignore = "not sure how this will be implemented"]
	fn already_won() {
		assert_eq!(
			Position::try_from("44335522".to_string()),
			Err("Position contains an invalid move.")
		)
	}

	#[test]
	fn invalid() {
		assert_eq!(Position::try_from("hey".to_string()), Err("Not a position."));
	}
}

struct Mask(u64);

impl std::fmt::Display for Mask {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s: Vec<char> = format!("{:#044b}", self.0).chars().rev().collect();

		for row in (0..GRID_SIZE.1).rev() {
			writeln!(f, "{}", (0..GRID_SIZE.0).map(
				|i| s[(row + i * (GRID_SIZE.1)) as usize]
			).fold(String::new(), |a, b| a + &b.to_string()))?;
		}
		return Ok(());
	}
}

impl From<u64> for Mask {
	fn from(x: u64) -> Self { Mask(x) }
}


impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "moves({:0>2}): {}\n", self.move_count, self.moves.iter().fold(String::new(), |a, b| a + &(b+1).to_string()))?;

		writeln!(f, "pieces_mask")?;
		writeln!(f, "{}", Mask::from(self.pieces_mask))?;
		writeln!(f, "player_mask")?;
		writeln!(f, "{}", Mask::from(self.player_mask))?;

		Ok(())
	}
}

#[test]
fn test_display_position() {
	assert_eq!(format!("{}", Position {
		player_mask: 2 << 6,
		pieces_mask: 7 << 6,
		move_count: 3,
		moves: LinkedList::from([1, 1, 1])
	}),
"moves(03): 222

pieces_mask
0000000
0000000
0000000
0100000
0100000
0100000

player_mask
0000000
0000000
0000000
0000000
0100000
0000000

");
}
