use ggez::graphics::MeshBuilder;
use ggez::graphics::{self, Text};
use ggez::graphics::{Color, PxScale, TextFragment};
use ggez::Context;
use ggez::GameResult;
use glam::*;

pub struct TitleScreenSystem {
    title: TextFragment,
    play: TextFragment,
    settings: TextFragment,
    selection: u8, // TODO: offload to a proper entity in ECS
}

impl TitleScreenSystem {
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
        let selection = 0;
        Self {
            title,
            play,
            settings,
            selection,
        }
    }

    pub fn update(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }

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

        let selection_mesh = MeshBuilder::new()
            .triangles(
                &[
                    glam::vec2(0.0, 0.0),
                    glam::vec2(0.0, 10.0),
                    glam::vec2(10.0, 5.0),
                ],
                Color::WHITE,
            )?
            .build(context)?;

        let selection_position = glam::vec2(
            (screen_width / 2.0) - (settings_width / 2.0) - 30.0,
            (screen_height / 2.0) - (play_height / 2.0)
                + title_height
                + 5.0
                + (self.selection as f32 * 25.0),
        );

        graphics::queue_text(context, &title, title_destination, None);
        graphics::queue_text(context, &play, play_destination, None);
        graphics::queue_text(context, &settings, settings_destination, None);
        graphics::draw(
            context,
            &selection_mesh,
            (selection_position, 0.0, Color::WHITE),
        )?;

        Ok(())
    }
}
