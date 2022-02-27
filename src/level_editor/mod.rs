use editors::editor::Editor;
use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use ggez::graphics::FilterMode;
use ggez::graphics::PxScale;
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::timer;
use ggez::Context;
use ggez::GameError;
use ggez::GameResult;
use ggez::{
    self,
    graphics::{self, Color},
};
use glam::*;
use sonic_platformer::input::*;

mod editors;
mod navigation;

use editors::*;
use navigation::EditorNavigation;

const DESIRED_FPS: u32 = 60;

pub struct EditorState {
    level_name: String,
    navigation: EditorNavigation,
    tileviewer: TileViewer,
    pieceeditor: PieceEditor,
    input: Input,
}

impl EditorState {
    pub fn new(level_name: &str) -> Self {
        Self {
            level_name: String::from(level_name),
            navigation: EditorNavigation::default(),
            tileviewer: TileViewer::new(&format!("/levels/{}/tiles.png", level_name)),
            pieceeditor: PieceEditor::new(level_name),
            input: Input::default(),
        }
    }

    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        self.tileviewer.reload(context)?;
        self.pieceeditor.reload(context)?;
        Ok(())
    }
}

impl EventHandler<GameError> for EditorState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while !timer::check_update_time(ctx, DESIRED_FPS) {} // Spinlock

        if self.input.pressed(InputButton::DbgNext)
            && (self.navigation == EditorNavigation::TileViewer)
        {
            self.navigation = EditorNavigation::PieceEditor;
            self.pieceeditor.reload(ctx)?;
        } else if self.input.pressed(InputButton::DbgPrev)
            && (self.navigation == EditorNavigation::PieceEditor)
        {
            self.navigation = EditorNavigation::TileViewer;
            self.tileviewer.reload(ctx)?;
        }

        let ret = match self.navigation {
            EditorNavigation::TileViewer => self.tileviewer.update(ctx, &self.input),
            EditorNavigation::PieceEditor => self.pieceeditor.update(ctx, &self.input),
            _ => Ok(()),
        };

        self.input.post_update();
        ret
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        match self.navigation {
            EditorNavigation::TileViewer => self.tileviewer.draw(ctx)?,
            EditorNavigation::PieceEditor => self.pieceeditor.draw(ctx)?,
            _ => {}
        }

        let header_letter_size = 20.0;

        let header = TextFragment::new(format!(
            "Editing {}\n\
             {:?}",
            self.level_name, self.navigation,
        ))
        .color(Color::WHITE)
        .scale(PxScale::from(header_letter_size));

        let header = Text::new(header);
        let header_position = glam::vec2(header_letter_size, header_letter_size);

        graphics::queue_text(ctx, &header, header_position, None);

        graphics::draw_queued_text(ctx, DrawParam::new(), None, FilterMode::Linear)?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _mod: KeyMods,
        _repeat: bool,
    ) {
        self.input.set_keyboard(keycode, true);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _mod: KeyMods) {
        self.input.set_keyboard(keycode, false);
    }
}
