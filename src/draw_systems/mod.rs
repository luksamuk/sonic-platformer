mod titlescreen_draw_system;

use crate::Navigation;
use ggez::graphics::{self, DrawParam, FilterMode};
use ggez::Context;
use ggez::GameResult;
pub use titlescreen_draw_system::TitleScreenDrawSystem;

pub struct DrawSystems {
    title_screen: TitleScreenDrawSystem,
}

impl DrawSystems {
    pub fn new(game_title: &str) -> Self {
        let title_screen = TitleScreenDrawSystem::new(game_title);
        Self { title_screen }
    }

    pub fn run(&self, context: &mut Context, navigation: &Navigation) -> GameResult {
        match navigation {
            Navigation::TitleScreen => self.title_screen.run(context)?,
            Navigation::Settings => {}
        };

        graphics::draw_queued_text(context, DrawParam::new(), None, FilterMode::Linear)?;
        Ok(())
    }
}
