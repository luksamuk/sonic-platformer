use super::general::Position;
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Rect};
use ggez::Context;
use ggez::GameResult;
use glam::*;

/// Represents data related to camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub position: Position,
    border: Rect,
    minimum: Vec2,
}

fn get_screen_center(context: &Context) -> Vec2 {
    let rect = graphics::screen_coordinates(context);
    glam::vec2(rect.w / 2.0, rect.h / 2.0)
}

impl Camera {
    pub fn new(context: &Context) -> Self {
         Self {
            position: Position::default(),
            border: Rect::new(-16.0, -32.0, 16.0, 64.0),
            minimum: get_screen_center(context),
        }
    }

    pub fn update(&mut self, followed: Option<&Position>) -> GameResult {
        // TODO!
        if let Some(position) = followed {
            self.position.0 = position.0;    
        }
        Ok(())
    }

    pub fn debug_draw(&self, context: &mut Context) -> GameResult {
        let camera_mesh = MeshBuilder::new()
            .rectangle(DrawMode::stroke(1.0), self.border, Color::WHITE)?
            .line(
                &[
                    glam::vec2(self.border.x, 0.0),
                    glam::vec2(self.border.x + self.border.w, 0.0),
                ],
                1.0,
                Color::GREEN,
            )?
            .build(context)?;

        let screen_center = get_screen_center(context);

        graphics::draw(context, &camera_mesh, (screen_center, 0.0, Color::WHITE))
    }
}
