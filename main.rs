// use std::collections::LinkedList;
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

mod solver;

const GRID_SIZE: (i16, i16) = (7, 6);
const GRID_CELL_SIZE_PX: i16 = 256;

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE_PX as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE_PX as f32,
);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i16,
    y: i16,
}

impl GridPosition {
    // pub fn new(x: i16, y: i16) -> Self {
    //     GridPosition { x, y }
    // }

    fn point(&self) -> [f32; 2] {
        let x = GRID_CELL_SIZE_PX as f32 / 2.0 + (self.x * GRID_CELL_SIZE_PX) as f32;
        let y = GRID_CELL_SIZE_PX as f32 / 2.0 + ((GRID_SIZE.0 - 2 - self.y) * GRID_CELL_SIZE_PX) as f32;
        [x as f32, y as f32]
    }

    fn radius(&self) -> f32 {
        GRID_CELL_SIZE_PX as f32 / 2.2
    }
}

impl From<(i16, i16)> for GridPosition {
    fn from(pos: (i16, i16)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

enum Move {
    Left,
    Right,
    Drop
}

impl Move {
    pub fn from_keycode(key: KeyCode) -> Option<Move> {
        match key {
            KeyCode::Left => Some(Move::Left),
            KeyCode::Right => Some(Move::Right),
            KeyCode::Down => Some(Move::Drop),
            KeyCode::NumpadEnter => Some(Move::Drop),
            KeyCode::Return => Some(Move::Drop),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
enum Palette {
    Red,
    Yellow,
    White
}

impl From<Palette> for graphics::Color {
    fn from(palette: Palette) -> Self {
        match palette {
            Palette::Red => graphics::Color::from_rgb(231, 61, 36),
            Palette::Yellow => graphics::Color::from_rgb(228, 167, 74),
            Palette::White => graphics::Color::from_rgb(220, 220, 221),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Shr)]
struct Mask {
    value: u64
}

impl Mask {
    pub fn position(x: u64, y: u64) -> Self {
        Self { value: 1 << y << (x * GRID_SIZE.1 as u64) }
    }

    pub fn check_alignment(&self) -> bool {
        let factors = [
            1,               // vertical
            GRID_SIZE.1,     // horizontal
            GRID_SIZE.1 - 1, // diag 1
            GRID_SIZE.1 + 1, // diag 2
        ];

        for factor in factors {
            let m = self.value & (self.value >> factor);
            if m & (m >> 2 * factor) != 0 {
                return true
            }
        }

        return false
    }
}

impl From<u64> for Mask {
    fn from(value: u64) -> Mask { Mask { value } }
}

impl From<Mask> for bool {
    fn from(mask: Mask) -> bool { mask.value != 0 }
}

impl std::ops::BitAnd for Mask {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self { value: self.value & rhs.value }
    }
}

impl std::ops::BitOr for Mask {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self { value: self.value | rhs.value }
    }
}

impl std::ops::BitXor for Mask {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { value: self.value ^ rhs.value }
    }
}

impl std::ops::Add for Mask {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { value: self.value + rhs.value }
    }
}

enum Who {
    PlayerRed,
    PlayerYellow
}

impl Who {
    fn next(&self) -> Who {
        match *self {
            Who::PlayerRed => Who::PlayerYellow,
            Who::PlayerYellow => Who::PlayerRed,
        }
    }

    pub fn color(&self) -> graphics::Color {
        match *self {
            Who::PlayerRed => Palette::Red.into(),
            Who::PlayerYellow => Palette::Yellow.into(),
        }
    }
}

struct GameState {
    red_mask: Mask,
    yellow_mask: Mask,
    cursor: i16,
    who: Who
    // whosColor: Color,
    // cells: LinkedList<Cell> //[[Cell; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize]
}

// struct GameState {
//     player_mask: Mask,
//     pieces_mask: Mask,
//     moves: u8,
// }

impl GameState {
    pub fn new(_ctx: &mut Context) -> GameState {
        GameState {
            red_mask: 0.into(),
            yellow_mask: 0.into(),
            cursor: 3,
            who: Who::PlayerRed,
        }
    }

    fn color (&self, x: u64, y: u64) -> graphics::Color {
        let position_mask = Mask::position(x, y);
        if (position_mask & self.yellow_mask) == position_mask {
            Palette::Yellow.into()
        } else if (position_mask & self.red_mask) == position_mask {
            Palette::Red.into()
        } else {
            Palette::White.into()
        }
    }

    fn draw_cell(&mut self, ctx: &mut Context, pos: GridPosition, color: graphics::Color) -> GameResult<()> {
        let circle =
            graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                pos.point(),
                pos.radius(),
                0.1,
                color
            )?;

        graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }

    fn draw_cursor(&self, ctx: &mut Context, color: graphics::Color) -> GameResult<()> {
        let circle =
            graphics::Mesh::new_circle(
                ctx, graphics::DrawMode::fill(),
                [GRID_CELL_SIZE_PX as f32 * (self.cursor as f32+ 0.5), 10.0],
                8.0,
                0.1,
                color
            )?;

        graphics::draw(ctx, &circle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
        Ok(())
    }

    pub fn try_drop(&mut self) {
        let mut common_mask = self.red_mask | self.yellow_mask;
        let upper_position = Mask::position(self.cursor as u64, (GRID_SIZE.1 - 1) as u64);

        // Row already occupied.
        if (common_mask & upper_position) == upper_position {
            return
        }

        common_mask = common_mask | (common_mask + Mask::position(self.cursor as u64, 0u64));
        let opponent_mask = match self.who {
            Who::PlayerRed => self.yellow_mask,
            Who::PlayerYellow => self.red_mask,
        };

        let player_mask = common_mask ^ opponent_mask;

        if player_mask.check_alignment() {
            println!("Win!")
        }

        match self.who {
            Who::PlayerRed => self.red_mask = player_mask,
            Who::PlayerYellow => self.yellow_mask = player_mask,
        };

        self.who = self.who.next();
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        // TODO: if IA to move, then compute and update the move here
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::Color::from_rgb(57, 105, 239));
        for x in 0..GRID_SIZE.0  {
            for y in 0..GRID_SIZE.1 {
                self.draw_cell(ctx, (x, y).into(), self.color(x as u64, y as u64))?;
            }
        }

        self.draw_cursor(ctx, self.who.color())?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match Move::from_keycode(keycode) {
            Some(Move::Left) => self.cursor = (self.cursor - 1) % GRID_SIZE.0,
            Some(Move::Right) => self.cursor = (self.cursor + 1) % GRID_SIZE.0 ,
            Some(Move::Drop) => self.try_drop(),
            None => (),
        }
    }
}

fn main() {
    // solver::solve();
    // Make a Context.
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Connect 4", "Ulysse Buonomo")
        .window_setup(ggez::conf::WindowSetup::default().title("Connect 4!"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let state = GameState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, state);
}
