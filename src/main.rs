#![allow(dead_code)]
#![allow(unused_imports)]

use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};
use sonic_platformer::*;

const GAME_NAME: &str = "sonic-platformer";

fn main() -> GameResult {
    let window_setup = WindowSetup::default().title(GAME_NAME).vsync(false);

    let window_mode = WindowMode::default()
        //.fullscreen_type(FullscreenType::Desktop)
        .dimensions(960.0, 540.0)
        .resizable(false);

    let (mut ctx, event_loop) = ContextBuilder::new("sonic_platformer", "Lucas S. Vieira")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .add_resource_path(resources::get_resource_dir())
        .build()?;

    let mut main_state = MainState::new(GAME_NAME)?;
    main_state.setup(&mut ctx)?;
    event::run(ctx, event_loop, main_state)
}
