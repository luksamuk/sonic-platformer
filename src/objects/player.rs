use crate::input::Input;
use ggez::{Context, GameResult};
use glam::*;
use legion::{Entity, IntoQuery, World};

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
    /// Minimum absolute speed for applying slope factor (Sonic 3)
    pub min_slp: f32,
    /// Tolerance ground speed for sticking to walls and ceilings
    pub fall: f32,
    /// Air acceleration, normally 2x [`acc`]
    pub air: f32,
    /// Jump force
    pub jmp: f32,
    /// Gravity
    pub grv: f32,
    /// Minimum jump speed for when the jump button is released
    pub minjmp: f32,
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
            min_slp: 0.05078125,
            fall: 2.5,
            air: 0.09375,
            jmp: 6.5,
            grv: 0.21875,
            minjmp: -4.0,
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
    pub angle: f32,
}

/// Represents the state variables for a player.
///
/// These variables refer mostly to state such as ground state and
/// other information that does not involve transformations directly.
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct PlayerState {
    /// Whether the player is on ground
    pub ground: bool,
    /// Whether the player has jumped
    pub jumping: bool,
    /// Whether the player is rolling
    pub rolling: bool,
}

impl PlayerState {
    pub fn get_ground(&self) -> bool {
        self.ground
    }

    /// Define the ground state. This will also update the player speed.
    /// Remember to set the player speed's angle to the ground angle
    /// BEFORE calling this function.
    pub fn set_ground(&mut self, mut state: bool, speed: &mut PlayerSpeed, downward: bool) {
        if self.ground {
            self.ground = state;
        } else {
            if state {
                if downward {
                    // Shallow angle
                    if ((speed.angle >= 0.0) && (speed.angle <= 23.0))
                        || ((speed.angle >= 339.0) && (speed.angle <= 360.0))
                    {
                        speed.gsp = speed.xsp
                    }
                    // Half steep
                    else if ((speed.angle > 23.0) && (speed.angle <= 45.0))
                        || ((speed.angle >= 315.0) && (speed.angle < 339.0))
                    {
                        speed.gsp = if speed.xsp.abs() > speed.ysp.abs() {
                            speed.xsp
                        } else {
                            speed.ysp * 0.5 * -speed.angle.sin().signum()
                        };
                    }
                    // Full steep
                    else if ((speed.angle > 45.0) && (speed.angle <= 90.0))
                        || ((speed.angle >= 270.0) && (speed.angle < 315.0))
                    {
                        speed.gsp = if speed.xsp.abs() > speed.ysp.abs() {
                            speed.xsp
                        } else {
                            speed.ysp * -speed.angle.sin().signum()
                        };
                    }
                } else {
                    // Going upward
                    // Slope
                    if ((speed.angle > 90.0) && (speed.angle <= 135.0))
                        || ((speed.angle > 225.0) && (speed.angle <= 270.0))
                    {
                        // TODO: Attach to ceiling.
                        speed.gsp = speed.ysp * -speed.angle.sin().signum();
                    }
                    // Ceiling
                    else if (speed.angle > 135.0) && (speed.angle <= 225.0) {
                        speed.ysp = 0.0;
                        state = false;
                    }
                }
            }
            self.ground = state;
        }
    }
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

        let constants = if knuckles {
            PlayerConstants::default_knuckles()
        } else {
            PlayerConstants::default()
        };

        //let state = PlayerState::default();
        let state = PlayerState {
            ground: true,
            ..PlayerState::default()
        };
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

        // TODO: Hitboxes?

        Ok(world.push((state, constants, position, speed, animation_data, animator)))
    }

    pub fn respawn_all(world: &mut World) {
        use crate::objects::general::Position;
        let mut query = <(&mut PlayerState, &mut Position, &mut PlayerSpeed)>::query();
        for (state, position, speed) in query.iter_mut(world) {
            *position = Position::new(30.0, 240.0);
            *speed = PlayerSpeed::default();
            //state = PlayerState::default();
            *state = PlayerState {
                ground: true,
                ..PlayerState::default()
            };
        }
    }

    pub fn physics_update(world: &mut World, input: &Input) -> GameResult {
        use crate::input::InputButton;
        use crate::objects::general::*;
        let mut query = <(
            &mut PlayerState,
            &PlayerConstants,
            &mut Position,
            &mut PlayerSpeed,
        )>::query();
        for (state, constants, position, speed) in query.iter_mut(world) {
            let right = input.pressing(InputButton::Right);
            let left = input.pressing(InputButton::Left);

            // Fake ground. Remove later!
            if !state.ground && (position.0.y >= 240.0) {
                position.0.y = 240.0;
                speed.angle = 0.0;
                state.set_ground(true, speed, true);
            }

            // Horizontal movement
            if state.ground {
                // Ground movement
                if !left && right {
                    speed.gsp += if speed.gsp < 0.0 {
                        // Decelerate if moving left
                        constants.dec
                    } else {
                        // Accelerate otherwise
                        constants.acc
                    };
                } else if left && !right {
                    speed.gsp -= if speed.gsp > 0.0 {
                        // Decelerate if moving right
                        constants.dec
                    } else {
                        // Accelerate otherwise
                        constants.acc
                    }
                } else {
                    // If not pressing any directionals, friction kicks in
                    speed.gsp -= speed.gsp.abs().min(constants.frc) * speed.gsp.signum();
                }

                let angle_sin = speed.angle.sin();
                let angle_cos = speed.angle.cos();

                // Apply slope factor
                if speed.gsp.abs() >= constants.min_slp {
                    speed.gsp -= angle_sin
                        * if state.rolling {
                            if speed.gsp.signum() as i32 == angle_sin.signum() as i32 {
                                constants.slprollup
                            } else {
                                constants.slprolldown
                            }
                        } else {
                            constants.slp
                        };
                }

                // Apply top speed
                if speed.gsp.abs() >= constants.top {
                    speed.gsp = constants.top * speed.gsp.signum();
                }

                // Transform x and Y speed accordingly
                speed.xsp = speed.gsp * angle_cos;
                speed.ysp = speed.gsp * -angle_sin;
            } else {
                // Air movement
                let dir = if (right && !left) { 1.0 } else { -1.0 };
                speed.xsp += if (right && !left) || (!right && left) {
                    constants.air * dir
                } else {
                    0.0
                };
            }

            // Vertical movement
            if !state.ground {
                // Apply air drag
                if (speed.ysp < 0.0) && (speed.ysp > constants.minjmp) {
                    speed.xsp -= (speed.xsp % 0.125) / 256.0;
                }

                // Apply top speed
                if speed.xsp.abs() >= constants.top {
                    speed.xsp = constants.top * speed.xsp.signum();
                }

                // Apply gravity
                speed.ysp += constants.grv;

                // Apply jump cap
                if !input.pressing(InputButton::A) && (speed.ysp < constants.minjmp) {
                    speed.ysp = constants.minjmp;
                }
            } else {
                // Perform jump.
                if input.pressed(InputButton::A) {
                    state.set_ground(false, speed, true);
                    state.jumping = true;
                    speed.xsp -= constants.jmp * speed.angle.sin();
                    speed.ysp -= constants.jmp * speed.angle.cos();
                }
            }

            // Transform position
            position.0.x += speed.xsp;
            position.0.y += speed.ysp;
        }
        Ok(())
    }

    pub fn animation_update(world: &mut World, /*temporary*/ input: &Input) -> GameResult {
        use crate::input::InputButton;
        use crate::objects::animation::{AnimationDirection, Animator, AnimatorData};
        let mut query = <(&PlayerState, &PlayerSpeed, &mut Animator, &AnimatorData)>::query();
        for (state, speed, animator, animdata) in query.iter_mut(world) {
            let (up, down, left, right) = (
                input.pressing(InputButton::Up),
                input.pressing(InputButton::Down),
                input.pressing(InputButton::Left),
                input.pressing(InputButton::Right),
            );

            let gsp = speed.gsp.abs();
            if state.ground {
                // The assignment on physics_update kinda allows me to do that.
                // Is this a good idea, then? No, it's stupid. But it'll suffice for now
                animator.set(
                    String::from(if gsp == 0.0 {
                        if up && !down {
                            "lookup"
                        } else if !up && down {
                            "crouch"
                        } else {
                            "idle"
                        }
                    } else if gsp >= 9.95 {
                        "peel"
                    } else if gsp >= 5.0 {
                        "run"
                    } else {
                        "walk"
                    }),
                    animdata,
                );

                // Animation duration
                if (gsp > 0.0) && (gsp < 9.95) {
                    animator.set_duration_ms((16.0 * (9.0 - gsp).max(1.0).floor()) as u64);
                }
            } else {
                if state.jumping || state.rolling {
                    animator.set("roll".to_string(), animdata);
                    animator.set_duration_ms((16.0 * (4.0 - gsp).max(1.0).floor()) as u64);
                }
            }

            if left && !right {
                animator.direction = AnimationDirection::Left;
            } else if !left && right {
                animator.direction = AnimationDirection::Right;
            }
        }
        Ok(())
    }
}
