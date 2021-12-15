// use std::collections::LinkedList;
use ggez::event::{KeyCode, KeyMods};
use ggez::{event, graphics, Context, GameResult};

mod position;
mod solver;

use crate::position::{Position, GRID_SIZE};

const GRID_CELL_SIZE_PX: usize = 256;

const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.width as f32 * GRID_CELL_SIZE_PX as f32,
    GRID_SIZE.height as f32 * GRID_CELL_SIZE_PX as f32,
);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: u8,
    y: u8,
}

impl GridPosition {
    // pub fn new(x: i16, y: i16) -> Self {
    //     GridPosition { x, y }
    // }

    fn point(&self) -> [f32; 2] {
        let x = GRID_CELL_SIZE_PX as f32 / 2.0 + (self.x as usize * GRID_CELL_SIZE_PX) as f32;
        let y = GRID_CELL_SIZE_PX as f32 / 2.0 + ((GRID_SIZE.width - 2 - self.y) as usize * GRID_CELL_SIZE_PX) as f32;
        [x as f32, y as f32]
    }

    fn radius(&self) -> f32 {
        GRID_CELL_SIZE_PX as f32 / 2.2
    }
}

impl From<(u8, u8)> for GridPosition {
    fn from(pos: (u8, u8)) -> Self {
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
            KeyCode::Space => Some(Move::Drop),
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

#[derive(Clone, Copy)]
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

// trait Drawable {
//     fn draw(&mut self, ctx: &mut Context) -> GameResult<()>;
// }

// impl Drawable for Position {
//     fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
//         for char in self.moves.chars() {
//             draw_cell
//         }

//         Ok(());
//     }
// }

struct GameState {
    position: Position,
    cursor: u8,
    who: Who
}

impl GameState {
    pub fn new(_ctx: &mut Context, start_position: Option<String>) -> GameState {
        println!("Starting position: {:?}", start_position);
        GameState {
            position: match start_position {
                Some(str) => Position::try_from(str).unwrap_or(Position::new_empty()),
                None => Position::new_empty()
            },
            cursor: 3,
            who: Who::PlayerRed,
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
        if !self.position.can_play(self.cursor as u8) { return; }
        if self.position.wins(self.cursor as u8) {
            println!("win!");
        }
        self.position.next(self.cursor as u8); // TODO: handle false?
        println!("{}", self.position);
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
        let mut counters: [u8; GRID_SIZE.width as usize] = [0; GRID_SIZE.width as usize];

        let mut who = self.who;
        for column in self.position.moves.clone() {
            self.draw_cell(ctx, (column, counters[column as usize]).into(), who.color())?;
            counters[column as usize] += 1;
            who = who.next();
        }

        for (column, count) in counters.into_iter().enumerate() {
            for row in count..(GRID_SIZE.height as u8) {
                self.draw_cell(ctx, (column as u8, row).into(), Palette::White.into())?;
            }
        }

        self.draw_cursor(ctx, who.color())?;

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
            Some(Move::Left) => self.cursor = (GRID_SIZE.width + self.cursor - 1) % GRID_SIZE.width,
            Some(Move::Right) => self.cursor = (GRID_SIZE.width + self.cursor + 1) % GRID_SIZE.width ,
            Some(Move::Drop) => self.try_drop(),
            None => (),
        }
        //println!("cursor: {}", self.cursor);
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
    let state = GameState::new(&mut ctx, std::env::args().nth(1));

    // Run!
    event::run(ctx, event_loop, state);
}
