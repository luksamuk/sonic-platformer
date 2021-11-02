#![allow(dead_code)]

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
