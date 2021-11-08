#![allow(dead_code)]

/// Enumeration to determine the current
/// screen being drawn and updated.
pub enum Navigation {
    TitleScreen,
    LevelScreen,
    Settings,
}

impl Default for Navigation {
    fn default() -> Self {
        Self::TitleScreen
    }
}
