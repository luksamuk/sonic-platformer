/// Represents the player's speed constants.
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
    fn default() -> Self {
        Self {
            acc:         0.046875,
            dec:         0.5,
            frc:         0.046875,
            top:         6.0,
            slp:         0.125,
            slprollup:   0.078125,
            slprolldown: 0.3125,
            fall:        2.5,
            air:         0.09375,
            jmp:         6.5,
            grv:         0.21875,
        }
    }
}

impl PlayerConstants {
    pub fn default_knuckles() -> Self {
        Self {
            jmp: 6.0,
            ..Self::default()
        }
    }
}


