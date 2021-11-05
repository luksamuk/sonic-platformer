use crate::input::Input;
use crate::objects::general::*;
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

/// Enumeration for describing the current player action.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlayerAction {
    /// Player is idling, walking or running.
    Default,
    /// Player is jumping.
    Jumping,
    /// Player is rolling on the ground.
    Rolling,
    /// Player is ducking while standing still.
    Crouching,
    /// Player is looking up while standing still.
    LookingUp,
    /// Player is skidding
    Skidding,
}

impl Default for PlayerAction {
    fn default() -> Self {
        PlayerAction::Default
    }
}

/// Represents the state variables for a player.
///
/// These variables refer mostly to state such as ground state and
/// other information that does not involve transformations directly.
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct PlayerState {
    /// Whether the player is on ground
    pub ground: bool,
    /// Action for player
    pub action: PlayerAction,
    /// Direction for the player
    pub direction: Direction,
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

            if self.ground && self.action == PlayerAction::Jumping {
                self.action = PlayerAction::Default;
            }
        }
    }
}

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
            let (up, down, left, right) = (
                input.pressing(InputButton::Up),
                input.pressing(InputButton::Down),
                input.pressing(InputButton::Left),
                input.pressing(InputButton::Right),
            );

            // Fake ground. Remove later!
            if !state.ground && (position.0.y >= 240.0) {
                position.0.y = 240.0;
                speed.angle = 0.0;
                state.set_ground(true, speed, true);
            }

            // Horizontal movement
            if state.ground {
                // Ground movement
                // FIXME: Comparing floats for equality is dumb. But it works for now
                let abs_gsp = speed.gsp.abs();
                state.action = if (abs_gsp == 0.0) && (up && !down) {
                    PlayerAction::LookingUp
                } else if (abs_gsp == 0.0) && (!up && down) {
                    PlayerAction::Crouching
                } else if (state.action == PlayerAction::Crouching)
                    || (state.action == PlayerAction::LookingUp)
                {
                    PlayerAction::Default
                } else {
                    state.action
                };

                if (state.action == PlayerAction::Default)
                    || (state.action == PlayerAction::Skidding)
                {
                    if (!left && right) {
                        state.direction = Direction::Right;
                        speed.gsp += if speed.gsp < 0.0 {
                            // Decelerate if moving left
                            state.action = PlayerAction::Skidding;
                            constants.dec
                        } else {
                            // Accelerate otherwise
                            constants.acc
                        };

                        if (state.action == PlayerAction::Skidding) && (speed.gsp >= 0.0) {
                            state.action = PlayerAction::Default;
                        }
                    } else if (left && !right) {
                        state.direction = Direction::Left;
                        speed.gsp -= if speed.gsp > 0.0 {
                            // Decelerate if moving right
                            state.action = PlayerAction::Skidding;
                            constants.dec
                        } else {
                            // Accelerate otherwise
                            constants.acc
                        };

                        if (state.action == PlayerAction::Skidding) && (speed.gsp <= 0.0) {
                            state.action = PlayerAction::Default;
                        }
                    }
                }

                // Apply friction
                if !left && !right {
                    speed.gsp -= speed.gsp.abs().min(constants.frc) * speed.gsp.signum();
                }

                // Stop skidding action if speed is zero
                if (speed.gsp.abs() == 0.0) && state.action == PlayerAction::Skidding {
                    state.action = PlayerAction::Default;
                }

                let angle_sin = speed.angle.sin();
                let angle_cos = speed.angle.cos();

                // Apply slope factor
                if speed.gsp.abs() >= constants.min_slp {
                    speed.gsp -= angle_sin
                        * if state.action == PlayerAction::Rolling {
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

                // Air direction
                if (!left && right) {
                    state.direction = Direction::Right;
                } else if (left && !right) {
                    state.direction = Direction::Left;
                }
            }

            // Vertical movement
            if !state.ground {
                // Skidding on air makes no sense at all
                if state.action == PlayerAction::Skidding {
                    state.action = PlayerAction::Default;
                }

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
                    state.action = PlayerAction::Jumping;
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

    pub fn animation_update(world: &mut World) -> GameResult {
        use crate::objects::animation::{Animator, AnimatorData};
        let mut query = <(&PlayerState, &PlayerSpeed, &mut Animator, &AnimatorData)>::query();
        for (state, speed, animator, animdata) in query.iter_mut(world) {
            let gsp = speed.gsp.abs();
            if state.ground {
                animator.set(
                    String::from(match state.action {
                        PlayerAction::LookingUp => "lookup",
                        PlayerAction::Crouching => "crouch",
                        PlayerAction::Skidding => "skid",
                        PlayerAction::Default => {
                            if gsp >= 9.95 {
                                "peel"
                            } else if gsp >= 5.0 {
                                "run"
                            } else if gsp > 0.0 {
                                "walk"
                            } else {
                                "idle"
                            }
                        }
                        _ => "walk", /* uhhhh wat */
                    }),
                    animdata,
                );
                // Animation duration
                if (gsp > 0.0) && (gsp < 9.95) {
                    animator.set_duration_ms((16.0 * (9.0 - gsp).max(1.0).floor()) as u64);
                }
            } else {
                if (state.action == PlayerAction::Jumping)
                    || (state.action == PlayerAction::Rolling)
                {
                    animator.set("roll".to_string(), animdata);
                    animator.set_duration_ms((16.0 * (4.0 - gsp).max(1.0).floor()) as u64);
                }
            }

            // Update direction
            animator.direction = state.direction;
        }
        Ok(())
    }
}
