mod levelscreen;
mod navigation;
mod titlescreen;

use crate::Input;
use ggez::graphics::{self, DrawParam, FilterMode};
use ggez::Context;
use ggez::GameResult;
pub use levelscreen::system::LevelScreenSystem;
pub use navigation::Navigation;
pub use titlescreen::system::TitleScreenSystem;

/// Represents a collection of screen systems, which can
/// be switched between.
///
/// The screen system is responsible for updating and drawing
/// the current screen.
pub struct ScreenSystems {
    title_screen: TitleScreenSystem,
    level_screen: LevelScreenSystem,
}

impl ScreenSystems {
    /// Creates a new collection of screen systems.
    pub fn new(game_title: &str) -> Self {
        let title_screen = TitleScreenSystem::new(game_title);
        let level_screen = LevelScreenSystem::new();
        Self {
            title_screen,
            level_screen,
        }
    }

    /// Updates the current screen.
    pub fn update(
        &mut self,
        _context: &mut Context,
        navigation: &mut Navigation,
        input: &Input,
    ) -> GameResult {
        match navigation {
            Navigation::TitleScreen => self.title_screen.update(navigation, input)?,
            Navigation::LevelScreen => self.level_screen.update(navigation, input)?,
            Navigation::Settings => {}
        };
        Ok(())
    }

    /// Sets up the initial state of all screens.
    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        self.title_screen.setup(context)?;
        self.level_screen.setup(context)?;
        Ok(())
    }

    /// Draws the current screen.
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
