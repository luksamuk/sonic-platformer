use super::general::Position;
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Rect};
use ggez::Context;
use ggez::GameResult;
use glam::*;

/// Represents data related to camera.
#[derive(Debug, PartialEq)]
pub struct Camera {
    pub position: Position,
    border: Rect,
    center: Vec2,
}

pub enum CameraVerticalBehaviour {
    CenterYSlow,
    CenterYFast,
    RespectBounds,
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
            center: get_screen_center(context),
        }
    }

    pub fn transform(&self, vertex: Vec2) -> Vec2 {
        (vertex.clone() - self.position.0) + self.center
    }

    fn boundaries(&self) -> (f32, f32, f32, f32) {
        let pos = self.position.0;
        (
            pos.x + self.border.x,                 // Left
            pos.x + self.border.x + self.border.w, // Right
            pos.y + self.border.y,                 // Top
            pos.y + self.border.y + self.border.h, // Bottom
        )
    }

    pub fn update(
        &mut self,
        followed: Option<&Position>,
        vbehaviour: CameraVerticalBehaviour,
    ) -> GameResult {
        if let Some(followed_pos) = followed {
            let (left, right, top, bottom) = self.boundaries();
            let target = followed_pos.0;

            // Horizontal borders
            self.position.0.x += if target.x < left {
                -(left - target.x).min(16.0)
            } else if target.x > right {
                (target.x - right).min(16.0)
            } else {
                0.0
            };

            // Vertical borders
            self.position.0.y += match vbehaviour {
                CameraVerticalBehaviour::CenterYSlow => {
                    if target.y < self.position.0.y {
                        -(self.position.0.y - target.y).min(6.0)
                    } else if target.y > self.position.0.y {
                        (target.y - self.position.0.y).min(6.0)
                    } else {
                        0.0
                    }
                },
                CameraVerticalBehaviour::CenterYFast => {
                    if target.y < self.position.0.y {
                        -(self.position.0.y - target.y).min(16.0)
                    } else if target.y > self.position.0.y {
                        (target.y - self.position.0.y).min(16.0)
                    } else {
                        0.0
                    }
                },
                CameraVerticalBehaviour::RespectBounds => {
                    if target.y < top {
                        -(top - target.y).min(16.0)
                    } else if target.y > bottom {
                        (target.y - bottom).min(16.0)
                    } else {
                        0.0
                    }
                },
            };
        }

        // Prevent going beyond minimum position
        self.position.0 = self.position.0.max(self.center);

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
