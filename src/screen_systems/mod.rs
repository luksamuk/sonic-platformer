mod titlescreen_system;

use crate::Navigation;
use ggez::graphics::{self, DrawParam, FilterMode};
use ggez::Context;
use ggez::GameResult;
pub use titlescreen_system::TitleScreenSystem;

pub struct ScreenSystems {
    title_screen: TitleScreenSystem,
}

impl ScreenSystems {
    pub fn new(game_title: &str) -> Self {
        let title_screen = TitleScreenSystem::new(game_title);
        Self { title_screen }
    }

    pub fn update(&mut self, context: &mut Context, navigation: &Navigation) -> GameResult {
        match navigation {
            Navigation::TitleScreen => self.title_screen.update(context)?,
            Navigation::Settings => {},
        };
        Ok(())
    }

    pub fn draw(&self, context: &mut Context, navigation: &Navigation) -> GameResult {
        match navigation {
            Navigation::TitleScreen => self.title_screen.draw(context)?,
            Navigation::Settings => {},
        };

        graphics::draw_queued_text(context, DrawParam::new(), None, FilterMode::Linear)?;
        Ok(())
    }
}
