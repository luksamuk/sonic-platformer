use ggez::graphics::{self, Color};
use ggez::{
    conf::{FullscreenType, WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};
use sonic_platformer::*;
use std::env;

mod level_editor;

use level_editor::*;

const GAME_NAME: &str = "Level Editor";

fn main() -> GameResult {
    let window_setup = WindowSetup::default().title(GAME_NAME).vsync(true);

    let window_mode = WindowMode::default()
        .fullscreen_type(FullscreenType::Windowed)
        .resizable(false);
    let (ctx, event_loop) = ContextBuilder::new("lvledit", "Lucas S. Vieira")
        .window_mode(window_mode)
        .window_setup(window_setup)
        .add_resource_path(resources::get_resource_dir())
        .build()?;
    let editor_state = EditorState {};
    event::run(ctx, event_loop, editor_state)
}
