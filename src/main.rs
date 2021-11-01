#![allow(dead_code)]
#![allow(unused_imports)]

use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};
use sonic_platformer::*;

const GAME_NAME: &str = "Platformer";

fn main() -> GameResult {
    let window_setup = WindowSetup::default().title(GAME_NAME).vsync(false);

    let window_mode = WindowMode::default()
        //.dimensions(1280.0, 720.0)
        //.fullscreen_type(FullscreenType::Desktop)
        .dimensions(854.0, 480.0)
        .resizable(false);

    let (ctx, event_loop) = ContextBuilder::new("sonic_platformer", "Lucas S. Vieira")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .build()?;

    let main_state = MainState::new(GAME_NAME)?;
    event::run(ctx, event_loop, main_state)
}
