use crate::input::Input;
use ggez::graphics::{self, Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::{Context, GameResult};
use glam::*;

#[derive(Default)]
pub struct TileMarker {
    text: String,
    rect: Rect,
    pos: glam::Vec2,
}

impl TileMarker {
    pub fn new(size: Vec2) -> Self {
        let rect = Rect {
            x: -(size.x / 2.0),
            y: -(size.x / 2.0),
            w: size.x,
            h: size.y,
        };

        Self {
            text: String::new(),
            rect,
            pos: Vec2::ZERO,
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn update(&mut self, input: &Input) -> GameResult {
        let (mx, my) = input.mouse_position();
        self.pos = Vec2::new(mx, my);
        Ok(())
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        let mesh: Mesh = MeshBuilder::new()
            .rectangle(DrawMode::stroke(3.0), self.rect, Color::WHITE)?
            .build(context)?;

        graphics::draw(context, &mesh, (self.pos, 0.0, Color::WHITE))?;

        Ok(())
    }
}
