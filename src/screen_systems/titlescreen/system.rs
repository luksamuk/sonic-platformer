use super::objects;
use crate::objects::general::{Position, Tag};
use crate::screen_systems::Navigation;
use crate::Input;
use ggez::graphics::{self, Text};
use ggez::graphics::{Color, PxScale, TextFragment};
use ggez::Context;
use ggez::GameResult;
use legion::*;

/// Defines the state of the title screen system.
pub struct TitleScreenSystem {
    title: TextFragment,
    play: TextFragment,
    settings: TextFragment,
    world: World,
}

impl TitleScreenSystem {
    /// Creates a new title screen system.
    pub fn new(game_title: &str) -> Self {
        let title = TextFragment::new(game_title)
            .color(Color::WHITE)
            .scale(PxScale::from(72.0));
        let play = TextFragment::new("Play")
            .color(Color::WHITE)
            .scale(PxScale::from(24.0));
        let settings = TextFragment::new("Settings")
            .color(Color::WHITE)
            .scale(PxScale::from(24.0));

        let world = World::default();
        Self {
            title,
            play,
            settings,
            world,
        }
    }

    /// Sets up the title screen system's initial state.
    pub fn setup(&mut self, _context: &mut Context) -> GameResult {
        let _ = self.world.push((
            objects::Marker {
                num_options: 2,
                draw_step: 25.0,
            },
            Position::default(),
            Tag::default(),
        ));

        Ok(())
    }

    /// Updates the title screen system.
    pub fn update(&mut self, navigation: &mut Navigation, input: &Input) -> GameResult {
        objects::Marker::update(&mut self.world, navigation, input)
    }

    /// Draws the title screen system.
    pub fn draw(&self, context: &mut Context) -> GameResult {
        let title = Text::new(self.title.clone());
        let play = Text::new(self.play.clone());
        let settings = Text::new(self.settings.clone());

        // Widths and heights for screen and the three texts
        let (screen_width, screen_height) = {
            let rect = graphics::screen_coordinates(context);
            (rect.w, rect.h)
        };

        let (title_width, title_height) = {
            let rect = title.dimensions(context);
            (rect.w, rect.h)
        };

        let (play_width, play_height) = {
            let rect = play.dimensions(context);
            (rect.w, rect.h)
        };

        let (settings_width, settings_height) = {
            let rect = settings.dimensions(context);
            (rect.w, rect.h)
        };

        let title_destination = [
            (screen_width / 2.0) - (title_width / 2.0),
            (screen_height / 2.0) - title_height,
        ];

        let play_destination = [
            (screen_width / 2.0) - (play_width / 2.0),
            (screen_height / 2.0) - (play_height / 2.0) + title_height,
        ];

        let settings_destination = [
            (screen_width / 2.0) - (settings_width / 2.0),
            (screen_height / 2.0) - (settings_height / 2.0) + title_height + play_height,
        ];

        let selection_hotspot = glam::vec2(
            (screen_width / 2.0) - (settings_width / 2.0) - 30.0,
            (screen_height / 2.0) - (play_height / 2.0) + title_height + 5.0,
        );

        graphics::queue_text(context, &title, title_destination, None);
        graphics::queue_text(context, &play, play_destination, None);
        graphics::queue_text(context, &settings, settings_destination, None);

        objects::Marker::draw(&self.world, context, selection_hotspot)?;

        Ok(())
    }
}
