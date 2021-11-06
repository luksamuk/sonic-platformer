use super::PlayerAction;
use super::PlayerSpeed;
use crate::objects::general::Direction;

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
        if !self.ground && state {
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

            if state && self.action == PlayerAction::Jumping {
                self.action = PlayerAction::Default;
            }
        }
        self.ground = state;
    }
}
