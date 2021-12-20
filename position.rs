pub struct GridSize { pub height: u8, pub width: u8 }

pub const GRID_SIZE: GridSize = GridSize { height: 6, width: 7 };

const fn bottom_row_mask(width: u64, height: u64) -> u64 {
	match width {
		0 => 0,
		_ => bottom_row_mask(width - 1, height) | (1u64 << (width-1)*(height+1))
	}
}

const BOTTOM_ROW_MASK: u64 = bottom_row_mask(GRID_SIZE.width as u64, GRID_SIZE.height as u64);
const BOARD_MASK: u64 = BOTTOM_ROW_MASK * ((1 << GRID_SIZE.height)-1);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Position {
	player_mask: u64,
	pieces_mask: u64,
	pub move_count: u8,
}

impl Position {
	pub fn max_moves() -> u8 {
		GRID_SIZE.height * GRID_SIZE.width
	}
	pub fn new_empty() -> Position {
		Position { player_mask: 0, pieces_mask: 0, move_count: 0 }
	}

	pub fn wins(&self, col: u8) -> bool {
		let mut mask =  self.player_mask;
        mask |= (self.pieces_mask + Position::bottom_mask(col)) & Position::column_mask(col);
		return Position::check_alignment(mask);
	}

    pub fn can_win(&self) -> bool {
		self.possible_moves().any(|mov|self.wins(mov))
    }

	pub fn is_terminal(&self) -> bool {
		return self.move_count == GRID_SIZE.width * GRID_SIZE.height;
	}

	pub fn can_play(&self, column: u8) -> bool {
		return (self.pieces_mask & Position::top_mask(column)) == 0;
	}

	pub fn possible_moves(&self) -> impl Iterator<Item=u8> + '_ {
		return (0..GRID_SIZE.width).filter(|x| self.can_play(*x));
	}

	// Must be called on a playable move, see `can_play`.
	pub fn next(&self, column: u8) -> Position {
		Position {
			player_mask: self.player_mask ^ self.pieces_mask,
			pieces_mask: self.pieces_mask | (self.pieces_mask + Position::bottom_mask(column)),
			move_count: self.move_count + 1
		}
	}

	pub fn key(&self) -> u64 {
		self.player_mask + self.pieces_mask
	}

	fn check_alignment(mask: u64) -> bool {
		let factors = [
            1,                    // vertical
            GRID_SIZE.height + 1, // horizontal
            GRID_SIZE.height + 2, // diag 1
            GRID_SIZE.height,     // diag 2
        ];

        for factor in factors {
            let m = mask & (mask >> factor);
            if (m & (m >> (2 * factor))) != 0 {
                return true;
            }
        }

        return false;
	}

	// return a bitmask containg a single 1 corresponding to the top cel of a given column
	fn top_mask(col: u8) -> u64 {
		return 1 << (GRID_SIZE.height - 1) << (col * (GRID_SIZE.height+1));
	}

	// return a bitmask containg a single 1 corresponding to the bottom cell of a given column
	fn bottom_mask(col: u8) -> u64 {
		return 1 << (col*(GRID_SIZE.height+1));
	}

	// return a bitmask 1 on all the cells of a given column
	fn column_mask(col: u8) -> u64 {
		return ((1 << GRID_SIZE.height)-1) << col*(GRID_SIZE.height+1);
	}

	fn winning_position_mask(&self) -> u64 {
		Position::compute_winning_position(self.player_mask, self.pieces_mask)
	}

	pub fn move_score(&self, mov: u8) -> u64 {
		let next_pieces_mask = self.pieces_mask | (self.pieces_mask + Position::bottom_mask(mov));
		Position::pop_count(
			Position::compute_winning_position(
				(self.player_mask ^ self.pieces_mask) ^ next_pieces_mask,
				next_pieces_mask
			)
		)
	}

	fn compute_winning_position(player_mask: u64, pieces_mask: u64) -> u64 {
		// vertical;
		let mut r: u64 = (player_mask << 1) & (player_mask << 2) & (player_mask << 3);

		//horizontal
		let mut p: u64 = (player_mask << (GRID_SIZE.height+1)) & (player_mask << (2*(GRID_SIZE.height+1)));
		r |= p & (player_mask << (3*(GRID_SIZE.height+1)));
		r |= p & (player_mask >> (GRID_SIZE.height+1));
		p = (player_mask >> (GRID_SIZE.height+1)) & (player_mask >> (2*(GRID_SIZE.height+1)));
		r |= p & (player_mask << (GRID_SIZE.height+1));
		r |= p & (player_mask >> (3*(GRID_SIZE.height+1)));

		//diagonal 1
		p = (player_mask << GRID_SIZE.height) & (player_mask << (2*GRID_SIZE.height));
		r |= p & (player_mask << (3*GRID_SIZE.height));
		r |= p & (player_mask >> GRID_SIZE.height);
		p = (player_mask >> GRID_SIZE.height) & (player_mask >> (2*GRID_SIZE.height));
		r |= p & (player_mask << GRID_SIZE.height);
		r |= p & (player_mask >> (3*GRID_SIZE.height));

		//diagonal 2
		p = (player_mask << (GRID_SIZE.height+2)) & (player_mask << (2*(GRID_SIZE.height+2)));
		r |= p & (player_mask << (3*(GRID_SIZE.height+2)));
		r |= p & (player_mask >> (GRID_SIZE.height+2));
		p = (player_mask >> (GRID_SIZE.height+2)) & (player_mask >> (2*(GRID_SIZE.height+2)));
		r |= p & (player_mask << (GRID_SIZE.height+2));
		r |= p & (player_mask >> (3*(GRID_SIZE.height+2)));

		return r & (BOARD_MASK ^ pieces_mask);
	}

	const fn pop_count(mut mask: u64) -> u64 {
		let mut c = 0;
		while mask != 0 {
			c+=1;
			mask &= mask - 1;
		}
		c
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
	fn wins() {
		assert!(
			Position::try_from("343434").unwrap().wins(2),
			"Vertical"
		);
		assert!(
			Position::try_from("112233").unwrap().wins(3),
			"Horizontal"
		);
		assert!(
			Position::try_from("1224333447").unwrap().wins(3),
			"Diagonal"
		);
		assert!(
			Position::try_from("444466575").unwrap().wins(4),
			"Rev diagonal"
		);
	}

	#[test]
	fn check_alignment() {
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
			if !pos.can_play(mov as u8) { return Err("Position contains an invalid move.") }

			pos = pos.next(mov as u8);
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
		}));
	}

	#[test]
	fn str_111() {
		assert_eq!(Position::try_from("111".to_string()), Ok(Position {
			player_mask: 2,
			pieces_mask: 7,
			move_count: 3,
		}));
	}

	#[test]
	fn str_22() {
		assert_eq!(Position::try_from("22".to_string()), Ok(Position {
			player_mask: 1 << (GRID_SIZE.height + 1),
			pieces_mask: (1|2) << (GRID_SIZE.height + 1),
			move_count: 2
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

		let mut rv = 0u64;
		let mut pow = 0;

		for i in 0..GRID_SIZE.width {
			for j in 0..(GRID_SIZE.height + 1) {
				rv |= bidym_chars[(GRID_SIZE.height - j) as usize][i as usize] << pow;
				pow += 1;
			}
		}

		Mask(rv)
	}
}

#[test]
fn test_mask_from_str() {
	assert_eq!(
		Mask((1 | 2 | 4) << (GRID_SIZE.height + 1)),
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
			0000000\n\
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
		writeln!(f, "move_count: {:0>2}\n", self.move_count)?;

		writeln!(f, "pieces_mask")?;
		writeln!(f, "{}", Mask(self.pieces_mask))?;
		writeln!(f, "player_mask")?;
		writeln!(f, "{}", Mask(self.player_mask))?;

		Ok(())
	}
}

#[test]
fn test_display_position() {
	assert_eq!(format!("{}", Position {
		player_mask: 2 << (GRID_SIZE.height + 1),
		pieces_mask: 7 << (GRID_SIZE.height + 1),
		move_count: 3
	}),
"move_count: 03

pieces_mask
0000000
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
0000000
0100000
0000000

");
}
