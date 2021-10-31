pub enum Navigation {
    TitleScreen,
    Settings,
}

impl Default for Navigation {
    fn default() -> Self {
        Self::TitleScreen
    }
}