use super::{PlayerAction, PlayerSpeed, PlayerState};
use crate::objects::animation::Animator;
use ggez::GameResult;
use legion::*;

/// Performs updates on the animation component of the player.
pub fn update(world: &mut World) -> GameResult {
    let mut query = <(&PlayerState, &PlayerSpeed, &mut Animator)>::query();
    for (state, speed, animator) in query.iter_mut(world) {
        let gsp = speed.gsp.abs();
        if state.ground {
            animator.set(String::from(match state.action {
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
            }));
            // Animation duration
            if (gsp > 0.0) && (gsp < 9.95) {
                animator.set_duration_ms((16.0 * (9.0 - gsp).max(1.0).floor()) as u64);
            }
        } else if (state.action == PlayerAction::Jumping) || (state.action == PlayerAction::Rolling)
        {
            animator.set("roll".to_string());
            animator.set_duration_ms((16.0 * (4.0 - gsp).max(1.0).floor()) as u64);
        }

        // Update direction
        animator.direction = state.direction;
    }
    Ok(())
}
