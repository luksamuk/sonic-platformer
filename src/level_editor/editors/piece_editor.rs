use sonic_platformer::objects::level::Tile16;
use ggez::GameResult;
use sonic_platformer::objects::level::slurp_file;
use ggez::Context;
use ggez::GameError;
use sonic_platformer::objects::sprite_atlas::SpriteAtlas;
use ggez::graphics::{self};
use glam::*;

/// This struct is used to store the state of the piece editor.
pub struct PieceEditor {
    tiles_path: String,
    pieces_path: String,
    tiles: Option<SpriteAtlas>,
    data: Vec<Tile16>,
    current_tile: usize,
}

impl PieceEditor {
    /// Creates a new piece editor.
    pub fn new(level_name: &str) -> Self {
        let tiles_path = format!("/levels/{}/tiles.png", level_name);
        let pieces_path = format!("/levels/{}/16x16.json", level_name);
        Self {
            tiles_path,
            pieces_path,
            tiles: None,
            data: Vec::new(),
            current_tile: 0,
        }
    }

    /// Reloads the piece data.
    pub fn reload(&mut self, context: &mut Context) -> GameResult {
        self.tiles = Some(SpriteAtlas::new(context, &self.tiles_path, glam::vec2(8.0, 8.0))?);
        self.data = serde_json::from_str(&slurp_file(context, &self.pieces_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;
        Ok(())
    }

    /// Updates the piece editor.
    pub fn update(&mut self, context: &mut Context) -> GameResult {
        if let Some(tile) = self.data.get(self.current_tile) {
            let screen_center = {
                let rect = graphics::screen_coordinates(context);
                glam::vec2(rect.w, rect.h)
            };

            let tiles = self.tiles.as_mut().unwrap();
            tiles.clear();
            tile.put(tiles, Vec2::ZERO, -screen_center)?;
        }
        Ok(())
    }

    /// Draws the piece editor.
    pub fn draw(&self, context: &mut Context) -> GameResult {
        let tiles = self.tiles.as_ref().unwrap();
        tiles.draw(context)?;
        Ok(())
    }
}