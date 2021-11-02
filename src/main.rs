#![allow(dead_code)]
#![allow(unused_imports)]

use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};
use std::path::PathBuf;
use std::env;
use sonic_platformer::*;

const GAME_NAME: &str = "Platformer";


fn get_resource_dir() -> PathBuf {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("./resources")
    }
}


fn main() -> GameResult {
    let window_setup = WindowSetup::default().title(GAME_NAME).vsync(false);

    let window_mode = WindowMode::default()
        //.dimensions(1280.0, 720.0)
        //.fullscreen_type(FullscreenType::Desktop)
        .dimensions(854.0, 480.0)
        .resizable(false);

    let (mut ctx, event_loop) = ContextBuilder::new("sonic_platformer", "Lucas S. Vieira")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .add_resource_path(get_resource_dir())
        .build()?;

    let mut main_state = MainState::new(GAME_NAME)?;
    main_state.setup(&mut ctx)?;
    event::run(ctx, event_loop, main_state)
}
