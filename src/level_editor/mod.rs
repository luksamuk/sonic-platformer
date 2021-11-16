use ggez::event::EventHandler;
use ggez::graphics::DrawParam;
use ggez::graphics::FilterMode;
use ggez::graphics::PxScale;
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::Context;
use ggez::GameError;
use ggez::GameResult;
use ggez::{
    self,
    graphics::{self, Color},
};
use glam::*;

mod editors;
mod navigation;

use editors::*;
use navigation::EditorNavigation;

pub struct EditorState {
    level_name: String,
    navigation: EditorNavigation,
    tileviewer: TileViewer,
}

impl EditorState {
    pub fn new(level_name: &str) -> Self {
        Self {
            level_name: String::from(level_name),
            navigation: EditorNavigation::default(),
            tileviewer: TileViewer::new(&format!("/levels/{}/tiles.png", level_name)),
        }
    }

    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        match self.navigation {
            EditorNavigation::TileViewer => self.tileviewer.reload(context),
            _ => Ok(()),
        }
    }
}

impl EventHandler<GameError> for EditorState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match self.navigation {
            EditorNavigation::TileViewer => self.tileviewer.update(ctx),
            _ => Ok(()),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        match self.navigation {
            EditorNavigation::TileViewer => self.tileviewer.draw(ctx)?,
            _ => {},
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
}
