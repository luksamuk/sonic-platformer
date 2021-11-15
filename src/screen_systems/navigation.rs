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

/// Enumeration to determine the current
/// editor scrren being drawn and updated.
pub enum EditorNavigation {
    TileViewer,
    PieceViewer,
    ChunkViewer,
    LevelViewer,
}

impl Default for EditorNavigation {
    fn default() -> Self {
        Self::TileViewer
    }
}