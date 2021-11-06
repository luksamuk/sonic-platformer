use super::{PlayerAction, PlayerConstants, PlayerSpeed, PlayerState};
use crate::input::Input;
use crate::objects::general::Position;
use ggez::GameResult;
use legion::*;

pub fn update(world: &mut World, input: &Input) -> GameResult {
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

            if (state.action == PlayerAction::Default) || (state.action == PlayerAction::Skidding) {
                if !left && right {
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
                } else if left && !right {
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
            if (!left && !right) || (left && right) {
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
            let dir = if right && !left {
                1.0
            } else if !right && left {
                -1.0
            } else {
                0.0
            };
            speed.xsp += if (right && !left) || (!right && left) {
                constants.air * dir
            } else {
                0.0
            };

            // Air direction
            if !left && right {
                state.direction = Direction::Right;
            } else if left && !right {
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
