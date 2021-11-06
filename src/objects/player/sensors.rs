use super::PlayerAction;
use super::PlayerSpeed;
use super::PlayerState;
use crate::objects::general::Position;
use ggez::Context;
use ggez::GameResult;

/// Unit struct representing the Player's sensors.
///
/// This is not supposed to be used as a component. Instead, it only
/// relates to logic and drawing of player sensors.
pub struct PlayerSensors;

impl PlayerSensors {
    /// Draws a representation for player sensors. Requires player data
    /// such as its state, position and rotation.
    pub fn draw(
        context: &mut Context,
        state: &PlayerState,
        position: &Position,
        speed: &PlayerSpeed,
    ) -> GameResult {
        use ggez::graphics::{self, Color, DrawMode, MeshBuilder};
        let smaller = (state.action == PlayerAction::Rolling)
            || (state.action == PlayerAction::Jumping)
            || (state.action == PlayerAction::Crouching);

        let mut crouch_offset = 0.0;
        if smaller {
            crouch_offset = 5.0;
        }

        let hotspot = if smaller {
            glam::vec2(position.0.x, position.0.y + 5.0)
        } else {
            position.0
        };

        let alpha = 1.0;

        let sensors = MeshBuilder::new()
            // Left ground sensor (A)
            .line(
                &[
                    glam::vec2(-8.0, 0.0),
                    glam::vec2(-8.0, 18.0 - crouch_offset),
                ],
                1.0,
                Color::new(0.0, 0.94, 0.0, alpha),
            )?
            // Left ceiling sensor (C)
            .line(
                &[
                    glam::vec2(-8.0, 0.0),
                    glam::vec2(-8.0, -20.0 + crouch_offset),
                ],
                1.0,
                Color::new(0.0, 0.68, 0.93, alpha),
            )?
            // Right ground sensor (B)
            .line(
                &[glam::vec2(8.0, 0.0), glam::vec2(8.0, 18.0 - crouch_offset)],
                1.0,
                Color::new(0.22, 1.0, 0.63, alpha),
            )?
            // Right ceiling sensor (D)
            .line(
                &[glam::vec2(8.0, 0.0), glam::vec2(8.0, -20.0 + crouch_offset)],
                1.0,
                Color::new(1.0, 0.94, 0.22, alpha),
            )?
            // Left wall sensor (E)
            .line(
                &[glam::vec2(0.0, 0.0), glam::vec2(-11.0, 0.0)],
                1.0,
                Color::new(1.0, 0.22, 1.0, alpha),
            )?
            // Right wall sensor (F)
            .line(
                &[glam::vec2(0.0, 0.0), glam::vec2(11.0, 0.0)],
                1.0,
                Color::new(1.0, 0.32, 0.32, alpha),
            )?
            // Points
            // Central
            .circle(
                DrawMode::fill(),
                glam::vec2(0.0, 0.0),
                3.0,
                0.1,
                Color::new(0.54, 0.54, 0.54, 1.0),
            )?
            .circle(
                DrawMode::fill(),
                glam::vec2(0.0, 0.0),
                1.0,
                0.1,
                Color::BLACK,
            )?
            // Wall sensors
            .line(
                &[glam::vec2(-11.0, 0.0), glam::vec2(-10.0, 0.0)],
                1.0,
                Color::WHITE,
            )?
            .line(
                &[glam::vec2(10.0, 0.0), glam::vec2(11.0, 0.0)],
                1.0,
                Color::WHITE,
            )?
            // Ceiling sensors
            .line(
                &[
                    glam::vec2(-8.0, -19.0 + crouch_offset),
                    glam::vec2(-8.0, -20.0 + crouch_offset),
                ],
                1.0,
                Color::WHITE,
            )?
            .line(
                &[
                    glam::vec2(8.0, -19.0 + crouch_offset),
                    glam::vec2(8.0, -20.0 + crouch_offset),
                ],
                1.0,
                Color::WHITE,
            )?
            // Bottom sensors
            .line(
                &[
                    glam::vec2(-8.0, 18.0 - crouch_offset),
                    glam::vec2(-8.0, 17.0 - crouch_offset),
                ],
                1.0,
                Color::WHITE,
            )?
            .line(
                &[
                    glam::vec2(8.0, 18.0 - crouch_offset),
                    glam::vec2(8.0, 17.0 - crouch_offset),
                ],
                1.0,
                Color::WHITE,
            )?
            // Build mesh
            .build(context)?;

        let hitbox_rect =
            if (state.action == PlayerAction::Rolling) || (state.action == PlayerAction::Jumping) {
                graphics::Rect::new(-8.0, -10.0, 17.0, 21.0)
            } else if state.action == PlayerAction::Crouching {
                graphics::Rect::new(-8.0, -4.0, 17.0, 17.0)
            } else {
                graphics::Rect::new(-8.0, -16.0, 17.0, 33.0)
            };

        let hitbox = MeshBuilder::new()
            .rectangle(
                DrawMode::fill(),
                hitbox_rect,
                Color::new(1.0, 0.0, 1.0, 0.5),
            )?
            .build(context)?;

        graphics::draw(context, &hitbox, (hotspot, 0.0, Color::WHITE))?;
        graphics::draw(context, &sensors, (hotspot, speed.angle, Color::WHITE))?;
        Ok(())
    }
}
