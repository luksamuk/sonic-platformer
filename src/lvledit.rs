use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event, ContextBuilder, GameError, GameResult,
};
use sonic_platformer::*;
use std::env;

mod level_editor;

use level_editor::*;

const GAME_NAME: &str = "Level Editor";

fn main() -> GameResult {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(GameError::CustomError(
            "Cannot grab level name!".to_string(),
        ));
    }

    let level_name = &args[1];

    let window_setup = WindowSetup::default().title(GAME_NAME).vsync(true);

    let window_mode = WindowMode::default()
        //.fullscreen_type(FullscreenType::Desktop)
        .dimensions(1280.0, 720.0);

    let (mut ctx, event_loop) = ContextBuilder::new("lvledit", "Lucas S. Vieira")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .add_resource_path(resources::get_resource_dir())
        .build()?;

    let mut editor_state = EditorState::new(level_name);
    editor_state.setup(&mut ctx)?;
    event::run(ctx, event_loop, editor_state)
}
