mod game;
mod board;
mod bitboard;
mod player;
mod ui;
mod config;
mod bot;
mod mcts;

use ggez::{ContextBuilder, GameResult};
use game::GameState;
use config::{SCREEN_WIDTH, SCREEN_HEIGHT};

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("connect_four", "you")
        .window_setup(ggez::conf::WindowSetup::default().title("Connect Four"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;

    let state = GameState::new();
    ggez::event::run(ctx, event_loop, state)
}