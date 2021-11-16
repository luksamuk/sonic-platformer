use sonic_platformer::objects::level::Tile16;
use ggez::GameResult;
use sonic_platformer::objects::level::slurp_file;
use ggez::Context;
use ggez::GameError;

/// This struct is used to store the state of the piece editor.
pub struct PieceEditor {
    pieces_path: String,
    data: Vec<Tile16>,
}

impl PieceEditor {
    /// Creates a new piece editor.
    pub fn new(pieces_path: &str) -> Self {
        let pieces_path = pieces_path.to_string();
        Self {
            pieces_path,
            data: Vec::new(),
        }
    }

    /// Reloads the piece data.
    pub fn reload(&mut self, context: &mut Context) -> GameResult {
        self.data = serde_json::from_str(&slurp_file(context, &self.pieces_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;
        println!("Number of tiles: {}", self.data.len());
        Ok(())
    }

    /// Updates the piece editor.
    pub fn update(&mut self, context: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draws the piece editor.
    pub fn draw(&self, context: &mut Context) -> GameResult {
        Ok(())
    }
}