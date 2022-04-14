/// Enumeration to determine the current
/// editor screen being drawn and updated.
#[derive(Debug, PartialEq, Clone)]
pub enum EditorNavigation {
    /// Viewing mode for 8x8 tiles. Cannot edit.
    TileViewer,
    /// Editor mode for 16x16 tiles.
    PieceEditor,
    /// Editor mode for 128x128 tiles.
    ChunkEditor,
    /// Editor mode for entire level map.
    MapEditor,
}

impl Default for EditorNavigation {
    fn default() -> Self {
        Self::TileViewer
    }
}
