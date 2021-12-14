use std::collections::LinkedList;

pub struct GridSize { pub height: u8, pub width: u8 }

pub const GRID_SIZE: GridSize = GridSize { height: 6, width: 7 };

#[derive(Debug, PartialEq, Eq, Clone)]
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

	pub fn short_str(&self) -> String {
		self.moves.iter().fold(String::new(), |a, b| a + &(b+1).to_string())
	}

	pub fn is_win(&self, col: u8) -> bool {
		// let mut mask = self.player_mask;
		// println!("mov={}\nold_pos\n{}\n", col, Mask::from(mask));
		// mask ^= self.pieces_mask;
		// // println!("opponent\n{}\n", Mask::from(mask));
		// mask ^= self.pieces_mask | (self.pieces_mask + (1 << (col * GRID_SIZE.height)));

		// println!("new_pos\n{}", Mask::from(mask));

		// println!("new_pos aligned? {}\n", Position::check_alignment(mask));
		// return Position::check_alignment(mask);
		let mut mask =  self.player_mask;
        mask |= (self.pieces_mask + Position::bottom_mask(col)) & Position::column_mask(col);
		println!("old\n{}\nnew\n{}\n", Mask::from(self.player_mask), Mask::from(mask));
		println!("new_pos aligned? {}", Position::check_alignment(mask));
		return Position::check_alignment(mask);
	}

    pub fn will_win(&self) -> bool {
		for mov in self.possible_moves() {
			if self.is_win(mov) { return true; }
		}


		return false
    }

	fn check_alignment(mask: u64) -> bool {
		let factors = [
            1,                    // vertical
            GRID_SIZE.height + 1, // horizontal
            GRID_SIZE.height + 2, // diag 1
            GRID_SIZE.height,     // diag 2
        ];

		let dbg = std::collections::HashMap::from([
            (1,                    "vertical"),
            (GRID_SIZE.height + 1, "horizontal"),
            (GRID_SIZE.height - 1, "diag 1"),
            (GRID_SIZE.height,     "diag 2"),
        ]);

        for factor in factors {
            let m = mask & (mask >> factor);
            if (m & (m >> (2 * factor))) != 0 {
                return true;
            }
        }

        return false;
	}

	pub fn is_terminal(&self) -> bool {
		return self.move_count == GRID_SIZE.width * GRID_SIZE.height;
	}

	pub fn can_play(&self, column: u8) -> bool {
		let column_mask = 1 << (GRID_SIZE.height - 1) << (column * (GRID_SIZE.height + 1));

		// println!("col\n{}", Mask::from(column_mask));
		// println!("pieces\n{}", Mask::from(self.pieces_mask));

		return (self.pieces_mask & column_mask) == 0;
	}

	pub fn possible_moves(&self) -> impl Iterator<Item=u8> + '_ {
		return (0..GRID_SIZE.width).filter(|x| self.can_play(*x));
	}

	pub fn next(&mut self, column: u8) -> bool {
		if !self.can_play(column) { return false; }

		self.player_mask ^= self.pieces_mask;
		self.pieces_mask |= self.pieces_mask + (1 << (column * (GRID_SIZE.height + 1)));
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

	// return a bitmask containg a single 1 corresponding to the top cel of a given column
	pub fn top_mask(col: u8) -> u64 {
		return (1 << (GRID_SIZE.height - 1)) << (col*(GRID_SIZE.height+1));
	}

	// return a bitmask containg a single 1 corresponding to the bottom cell of a given column
	pub fn bottom_mask(col: u8) -> u64 {
		return 1 << (col*(GRID_SIZE.height+1));
	}

	// return a bitmask 1 on all the cells of a given column
	pub fn column_mask(col: u8) -> u64 {
		return ((1 << GRID_SIZE.height)-1) << col*(GRID_SIZE.height+1);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_play_1() {
		let pos = Position::try_from("111111".to_string()).unwrap();



		assert!(!pos.can_play(0));
		for i in 1..GRID_SIZE.width { assert!(pos.can_play(i)) };
	}

	#[test]
	fn can_play_2() {
		let pos = Position::try_from("222222".to_string()).unwrap();

		assert!(pos.can_play(0));
		assert!(!pos.can_play(1));
		for i in 2..GRID_SIZE.width { assert!(pos.can_play(i)) };
	}

	#[test]
	fn is_win() {
		assert!(
			Position::try_from("343434").unwrap().is_win(2),
			"Vertical"
		);
		assert!(
			Position::try_from("112233").unwrap().is_win(3),
			"Horizontal"
		);
		assert!(
			Position::try_from("1224333447").unwrap().is_win(3),
			"Diagonal"
		);
		assert!(
			Position::try_from("444466575").unwrap().is_win(4),
			"Rev diagonal"
		);
	}

	#[test]
	fn check_alignment() {
		println!("Check alignment {:?}", std::env::args().nth(2));
		assert!(
			!Position::check_alignment(Mask::from("
				0000000
				1100001
				0100000
				0011001
				1110111
				1011000
				1100100
			").into())
		)
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

impl TryFrom<&str> for Position {
	type Error = &'static str;
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		return Position::try_from(s.to_string());
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

#[derive(Debug, PartialEq, Eq)]
struct Mask(u64);

impl std::fmt::Display for Mask {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s: Vec<char> = format!("{:#051b}", self.0).chars().rev().collect();

		for row in (0..(GRID_SIZE.height + 1)).rev() {
			writeln!(f, "{}", (0..GRID_SIZE.width).map(
				|i| s[(row + i * (GRID_SIZE.height + 1)) as usize]
			).fold(String::new(), |a, b| a + &b.to_string()))?;
		}
		return Ok(());
	}
}

impl Into<u64> for Mask {
	fn into(self) -> u64 { self.0 }
}

impl From<u64> for Mask {
	fn from(x: u64) -> Self { Mask(x) }
}

impl From<&str> for Mask {
	fn from(s: &str) -> Self {
		let bidym_chars = s.trim()
			.split("\n")
			.map(|line|line.trim().chars().map(|c|c.to_digit(10).unwrap() as u64).collect::<Vec<u64>>())
			.collect::<Vec<_>>();

		assert_eq!(bidym_chars.len(), GRID_SIZE.width as usize);
		assert_eq!(bidym_chars[0].len(), (GRID_SIZE.height + 1) as usize);

		// let mut list = std::collections::LinkedList::new();
		let mut rv = 0u64;
		let mut pow = 0;

		for i in 0..GRID_SIZE.width {
			for j in 0..GRID_SIZE.height {
				rv |= bidym_chars[(GRID_SIZE.height - j - 1) as usize][i as usize] << pow;
				pow += 1;
				// list.push_back(bidym_chars[(GRID_SIZE.1 - j - 1) as usize][i as usize]);
			}
		}

		// let mut pow = -1;

		Mask(
			rv
			// list.iter().fold(0, |a, b| {
			// 	pow += 1;
			// 	a | (b << pow)
			// })
		)
	}
}

#[test]
fn test_mask_from_str() {
	assert_eq!(
		Mask((1 | 2 | 4) << 6),
		Mask::from("
			0000000
			0000000
			0000000
			0000000
			0100000
			0100000
			0100000
		")
	);
	assert_eq!(
		Mask::from("
			0000000
			1100001
			0100000
			0011001
			1110111
			1011000
			1100100
		").to_string(),
		"\
			1100001\n\
		 	0100000\n\
			0011001\n\
			1110111\n\
			1011000\n\
			1100100\n\
		"
	)
}


impl std::fmt::Display for Position {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "moves({:0>2}): {}\n", self.move_count, self.short_str())?;

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
