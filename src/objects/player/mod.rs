mod constants;
mod general;
mod sensors;
mod state;

pub mod animation;
pub mod physics;

pub use constants::PlayerConstants;
pub use general::Player;
pub use sensors::PlayerSensors;
pub use state::PlayerState;

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
