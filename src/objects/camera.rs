use super::general::Position;
use ggez::graphics::{self, Color, DrawMode, MeshBuilder, Rect};
use ggez::Context;
use ggez::GameResult;
use glam::*;

/// Represents data related to camera.
/// 
/// Remember that this is not a component. Each screen should have
/// its camera instantiated, and it should be updated according to
/// whatever objects it follows.
/// 
/// The camera is to be mostly understood as this element which
/// recalculates elements' position for rendering, and is not an
/// actual object to be contained on the scene.
#[derive(Debug, PartialEq)]
pub struct Camera {
    pub position: Position,
    border: Rect,
    center: Vec2,
}


/// Describes the vertical behaviour of the camera.
/// 
/// This behaviour mostly relates to how the Player is behaving
/// on scene.
pub enum CameraVerticalBehaviour {
    /// When the player is on the ground, follow it so that
    /// the camera is always centered on the Y axis. The camera
    /// lags behind slowly, but finds the player at a pace of 6
    /// pixels per frame at most.
    CenterYSlow,
    /// When the player is on the ground, follow it so that
    /// the camera is always centered on the Y axis. The camera
    /// lags behind slowly, but finds the player at a pace of 16
    /// pixels per frame at most.
    CenterYFast,
    /// Default behaviour, and also the behaviour whenever the
    /// player is in the air. The camera respects the boundaries
    /// that were set, so that the player lags behind on Y axis.
    /// If it goes beyond the minimum or maximum Y boundaries, the
    /// camera finds the player at a pace of 16 pixels per frame
    /// at most, so that the player is contained inside the
    /// boundaries again.
    RespectBounds,
}

fn get_screen_center(context: &Context) -> Vec2 {
    let rect = graphics::screen_coordinates(context);
    glam::vec2(rect.w / 2.0, rect.h / 2.0)
}

impl Camera {
    /// Create a new camera object with its default boundaries.
    pub fn new(context: &Context) -> Self {
        Self {
            position: Position::default(),
            border: Rect::new(-16.0, -32.0, 16.0, 64.0),
            center: get_screen_center(context),
        }
    }

    /// Transform a vertex so that it is contained on screen.GameResult
    /// 
    /// This function should be used to recalculate the position of
    /// any elements being drawn on screen.
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

    /// Update camera position according to its behaviour, specially
    /// if the camera is supposed to follow any object on screen, whose
    /// position may be optionally passed as well.
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

    /// Draw a debug indicator on the center of screen so that one can
    /// debug the behaviour of camera.
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

        graphics::draw(context, &camera_mesh, (screen_center, 0.0, Color::new(1.0, 1.0, 1.0, 0.1)))
    }
}
