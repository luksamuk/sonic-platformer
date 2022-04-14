use crate::input::{Input, InputButton};
use crate::level_editor::editor::Editor;
use crate::objects::level::{slurp_file, Tile128, Tile16};
use crate::objects::sprite_atlas::SpriteAtlas;
use ggez::graphics::{self, Color, PxScale, Text, TextFragment};
use ggez::{Context, GameError, GameResult};
use glam::*;

pub struct ChunkEditor {
    tiles_path: String,
    pieces_path: String,
    chunks_path: String,
    tiles: Option<SpriteAtlas>,
    pieces: Vec<Tile16>,
    data: Vec<Tile128>,
    current_chunk: usize,
}

impl ChunkEditor {
    pub fn new(level_name: &str) -> Self {
        let tiles_path = format!("/levels/{}/tiles.png", level_name);
        let pieces_path = format!("/levels/{}/16x16.json", level_name);
        let chunks_path = format!("/levels/{}/128x128.json", level_name);
        Self {
            tiles_path,
            pieces_path,
            chunks_path,
            tiles: None,
            pieces: Vec::new(),
            data: Vec::new(),
            current_chunk: 1,
        }
    }
}

impl Editor for ChunkEditor {
    fn reload(&mut self, context: &mut Context) -> GameResult {
        self.tiles = Some(SpriteAtlas::new(
            context,
            &self.tiles_path,
            glam::vec2(8.0, 8.0),
        )?);
        self.pieces = serde_json::from_str(&slurp_file(context, &self.pieces_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;
        self.data = serde_json::from_str(&slurp_file(context, &self.chunks_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;

        Ok(())
    }

    fn update(&mut self, context: &mut Context, input: &Input) -> GameResult {
        if input.pressed(InputButton::Right) {
            self.current_chunk = (self.current_chunk + 1) % self.data.len();
        } else if input.pressed(InputButton::Left) {
            self.current_chunk = (self.current_chunk + self.data.len() - 1) % self.data.len();
        }

        if let Some(chunk) = self.data.get(self.current_chunk) {
            let screen_center = {
                let rect = graphics::screen_coordinates(context);
                glam::vec2(rect.w / 2.0, rect.h / 2.0)
            };

            let tiles = self.tiles.as_mut().unwrap();
            let pieces = &self.pieces;
            tiles.clear();
            chunk.put(pieces, tiles, glam::vec2(-96.0, -96.0), -screen_center, 2.0)?;
        }

        let text = TextFragment::new(format!(
            "Tile: {:02}/{:02}",
            self.current_chunk,
            if self.data.len() > 0 {
                self.data.len() - 1
            } else {
                0
            }
        ))
        .color(Color::WHITE)
        .scale(PxScale::from(24.0));

        let text = Text::new(text);

        graphics::queue_text(context, &text, glam::vec2(20.0, 80.0), None);

        Ok(())
    }

    fn draw(&self, context: &mut Context) -> GameResult {
        let tiles = self.tiles.as_ref().unwrap();
        tiles.draw(context)?;
        Ok(())
    }
}
