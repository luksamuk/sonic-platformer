use super::PlayerAction;
use super::PlayerConstants;
use super::PlayerSpeed;
use super::PlayerState;
use crate::input::Input;
use crate::objects::general::Position;
use ggez::Context;
use ggez::GameResult;
use glam::*;
use legion::*;

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

        let constants = if knuckles {
            PlayerConstants::default_knuckles()
        } else {
            PlayerConstants::default()
        };

        let state = PlayerState::default();
        let position = Position::new(30.0, 240.0);
        let speed = PlayerSpeed::default();
        let mut animation_data =
            AnimatorData::new(context, "/sprites/sonic.png", Vec2::new(60.0, 60.0))?;
        animation_data.with_data(&[
            (
                "idle",
                &[
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2,
                    3, 3, 4, 4,
                ],
                true,
                26,
                125,
            ),
            ("walk", &[5, 6, 7, 8, 9, 10], true, 0, 100),
            ("run", &[11, 12, 13, 14], true, 0, 63),
            ("roll", &[15, 16, 17, 16, 19, 16, 21, 16], true, 0, 125),
            ("skid", &[23], true, 0, 1000),
            ("peel", &[24, 25, 26, 27], true, 0, 60),
            ("push", &[28, 29, 30, 31], true, 0, 500),
            ("crouch", &[32], true, 0, 1000),
            ("lookup", &[33], true, 0, 1000),
            ("dead", &[34], true, 0, 1000),
        ])?;
        let mut animator = Animator::default();
        animator.set("idle".to_string(), &animation_data);

        Ok(world.push((state, constants, position, speed, animation_data, animator)))
    }

    /// Respawns all players in the world.
    pub fn respawn_all(world: &mut World) {
        let mut query = <(&mut PlayerState, &mut Position, &mut PlayerSpeed)>::query();
        for (state, position, speed) in query.iter_mut(world) {
            *position = Position::new(30.0, 240.0);
            *speed = PlayerSpeed::default();
            *state = PlayerState::default();
        }
    }
}
