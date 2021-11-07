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
    /// Final position of the camera.
    pub position: Position,
    /// Vertical behaviour of the camera boundaries.
    pub vertical_behaviour: CameraVerticalBehaviour,
    /// Displacement behaviour of the camera.
    pub displacement_behaviour: CameraDisplacementBehaviour,
    raw_position: Vec2,
    border: Rect,
    center: Vec2,
    displacement: Vec2,
}

/// Describes the vertical behaviour of the camera.
///
/// This behaviour mostly relates to how the Player is behaving
/// on scene.
#[derive(Debug, PartialEq)]
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

/// Describes the displacement behaviour of the camera.
/// 
/// The displacement behaviour changes the camera center so that
/// the camera slowly pans towards some direction. This is
/// particularly useful to make the character look up or down,
/// or to have an extended camera while running.
#[derive(Debug, PartialEq)]
pub enum CameraDisplacementBehaviour {
    /// No behaviour; centers the camera if not centralized.
    None,
    /// Slowly moves up the camera until it reaches
    /// 104 pixels up.
    LookUp,
    /// Slowly moves down the camera until it reaches
    /// 88 pixels down.
    LookDown,
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
            raw_position: Vec2::ZERO,
            border: Rect::new(-16.0, -32.0, 16.0, 64.0),
            center: get_screen_center(context),
            vertical_behaviour: CameraVerticalBehaviour::RespectBounds,
            displacement_behaviour: CameraDisplacementBehaviour::None,
            displacement: Vec2::ZERO,
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
        let pos = &self.raw_position;
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
    pub fn update(&mut self, followed: Option<&Position>) -> GameResult {
        if let Some(followed_pos) = followed {
            let (left, right, top, bottom) = self.boundaries();
            let target = followed_pos.0;

            // Horizontal borders
            self.raw_position.x += if target.x < left {
                -(left - target.x).min(16.0)
            } else if target.x > right {
                (target.x - right).min(16.0)
            } else {
                0.0
            };

            // Vertical borders
            self.raw_position.y += match self.vertical_behaviour {
                CameraVerticalBehaviour::CenterYSlow => {
                    if target.y < self.raw_position.y {
                        -(self.raw_position.y - target.y).min(6.0)
                    } else if target.y > self.raw_position.y {
                        (target.y - self.raw_position.y).min(6.0)
                    } else {
                        0.0
                    }
                }
                CameraVerticalBehaviour::CenterYFast => {
                    if target.y < self.raw_position.y {
                        -(self.raw_position.y - target.y).min(16.0)
                    } else if target.y > self.raw_position.y {
                        (target.y - self.raw_position.y).min(16.0)
                    } else {
                        0.0
                    }
                }
                CameraVerticalBehaviour::RespectBounds => {
                    if target.y < top {
                        -(top - target.y).min(16.0)
                    } else if target.y > bottom {
                        (target.y - bottom).min(16.0)
                    } else {
                        0.0
                    }
                }
            };
        }

        // Apply displacement behaviour
        match self.displacement_behaviour {
            CameraDisplacementBehaviour::None => {
                self.displacement.y = if self.displacement.y > 0.0 {
                    (self.displacement.y - 2.0).max(0.0)
                } else if self.displacement.y < 0.0 {
                    (self.displacement.y + 2.0).min(0.0)
                } else {
                    0.0
                };
            }
            CameraDisplacementBehaviour::LookDown => {
                self.displacement.y = (self.displacement.y + 2.0).min(88.0)
            }
            CameraDisplacementBehaviour::LookUp => {
                self.displacement.y = (self.displacement.y - 2.0).max(-104.0)
            }
        }

        // Define position considering displacement. Also prevent
        // going beyond minimum position
        self.position.0 = (self.raw_position + self.displacement).max(self.center);

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

        graphics::draw(
            context,
            &camera_mesh,
            (screen_center, 0.0, Color::new(1.0, 1.0, 1.0, 0.1)),
        )
    }
}
