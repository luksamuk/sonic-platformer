use super::sprite_sheet::*;
use ggez::{Context, GameResult};
use legion::{Entity, World};

/// Represents the player's speed constants.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerConstants {
    /// Ground acceleration
    pub acc: f32,
    /// Deceleration
    pub dec: f32,
    /// Friction, normally equals [`acc`]
    pub frc: f32,
    /// Top horizontal speed
    pub top: f32,
    /// Slope factor when walking or running
    pub slp: f32,
    /// Slope factor when rolling uphill
    pub slprollup: f32,
    /// Slope factor when rolling downhill
    pub slprolldown: f32,
    /// Tolerance ground speed for sticking to walls and ceilings
    pub fall: f32,
    /// Air acceleration, normally 2x [`acc`]
    pub air: f32,
    /// Jump force
    pub jmp: f32,
    /// Gravity
    pub grv: f32,
}

impl Default for PlayerConstants {
    /// Default constants for player. Relates to Sonic and Tails.
    fn default() -> Self {
        Self {
            acc: 0.046875,
            dec: 0.5,
            frc: 0.046875,
            top: 6.0,
            slp: 0.125,
            slprollup: 0.078125,
            slprolldown: 0.3125,
            fall: 2.5,
            air: 0.09375,
            jmp: 6.5,
            grv: 0.21875,
        }
    }
}

impl PlayerConstants {
    /// Default constants for player. Relates to Knuckles.
    pub fn default_knuckles() -> Self {
        Self {
            jmp: 6.0,
            ..Self::default()
        }
    }
}

/// Represents the speed variables for a player.
///
/// A player has specific variables to determine its transformation
/// on air and on ground.
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct PlayerSpeed {
    /// Horizontal speed
    pub xsp: f32,
    /// Vertical speed
    pub ysp: f32,
    /// Ground movement speed
    pub gsp: f32,
    /// Ground angle
    pub gangle: f32,
}

/// Unit struct representing the player.
///
/// Exists only to hold a few utility functions related to player
/// entities. A Player can actually be found by using a proper
/// query on the ECS.
pub struct Player;

impl Player {
    /// Create and push a player entity to the ECS world.
    ///
    /// This will also load player assets such as animation data
    /// and sprites. You can also determine whether you want it
    /// to use Knuckles-related constants or not.
    pub fn create(context: &mut Context, world: &mut World, knuckles: bool) -> GameResult<Entity> {
        use crate::objects::animation::*;
        use crate::objects::general::*;
        // Player constant values
        let constants = if knuckles {
            PlayerConstants::default_knuckles()
        } else {
            PlayerConstants::default()
        };

        let position = Position::default();
        let speed = PlayerSpeed::default();
        let mut animation_data = AnimatorData::new(context, "/sprites/sonic.png")?;
        animation_data.with_data(&[
            (
                "idle",
                &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 3, 4, 4],
                true,
            ),
            ("walk", &[5, 6, 7, 8, 9, 10], true),
            ("run", &[11, 12, 13, 14], true),
            ("roll", &[15, 16, 17, 16, 19, 16, 21, 16], true),
            ("skid", &[23], true),
            ("peel", &[24, 25, 26, 27], true),
            ("push", &[28, 29, 30, 31], true),
            ("crouch", &[32], true),
            ("lookup", &[33], true),
            ("dead", &[34], true),
        ]);
        let animator = Animator::default();

        // TODO: Collision, Physics!

        Ok(world.push((constants, position, speed, animation_data, animator)))
    }
}
