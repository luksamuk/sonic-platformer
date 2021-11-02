mod titlescreen;
mod levelscreen;

use crate::Input;
use crate::Navigation;
use ggez::graphics::{self, DrawParam, FilterMode};
use ggez::Context;
use ggez::GameResult;
pub use titlescreen::system::TitleScreenSystem;
pub use levelscreen::system::LevelScreenSystem;

pub struct ScreenSystems {
    title_screen: TitleScreenSystem,
    level_screen: LevelScreenSystem,
}

impl ScreenSystems {
    pub fn new(game_title: &str) -> Self {
        let title_screen = TitleScreenSystem::new(game_title);
        let level_screen = LevelScreenSystem::new();
        Self { title_screen, level_screen }
    }

    pub fn update(
        &mut self,
        _context: &mut Context,
        navigation: &Navigation,
        input: &Input,
    ) -> GameResult {
        match navigation {
            Navigation::TitleScreen => self.title_screen.update(input)?,
            Navigation::LevelScreen => self.level_screen.update(input)?,
            Navigation::Settings => {}
        };
        Ok(())
    }

    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        self.title_screen.setup(context)?;
        self.level_screen.setup(context)?;
        Ok(())
    }

    pub fn draw(&self, context: &mut Context, navigation: &Navigation) -> GameResult {
        match navigation {
            Navigation::TitleScreen => self.title_screen.draw(context)?,
            Navigation::LevelScreen => self.level_screen.draw(context)?,
            Navigation::Settings => {}
        };

        graphics::draw_queued_text(context, DrawParam::new(), None, FilterMode::Linear)?;
        Ok(())
    }
}
