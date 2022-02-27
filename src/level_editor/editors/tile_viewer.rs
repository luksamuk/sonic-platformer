use crate::level_editor::Input;
use crate::level_editor::Editor;
use ggez::graphics::Text;
use ggez::graphics::{self, Color, PxScale, TextFragment};
use ggez::{Context, GameResult};
use glam::*;
use sonic_platformer::objects::sprite_atlas::SpriteAtlas;

pub struct TileViewer {
    tile_path: String,
    tiles: Option<SpriteAtlas>,
}

impl TileViewer {
    pub fn new(tile_path: &str) -> Self {
        Self {
            tile_path: String::from(tile_path),
            tiles: None,
        }
    }
}

impl Editor for TileViewer {
    fn reload(&mut self, context: &mut Context) -> GameResult {
        self.tiles = Some(SpriteAtlas::new(
            context,
            &self.tile_path,
            glam::vec2(8.0, 8.0),
        )?);
        Ok(())
    }

    fn update(&mut self, context: &mut Context, _input: &Input) -> GameResult {
        if self.tiles.is_some() {
            let max_columns = 8;
            let tiles_per_column = 14;

            let atlas = self.tiles.as_mut().unwrap();
            let scale_factor = 4.0;
            let scale = glam::vec2(scale_factor, scale_factor);

            atlas.clear();
            for j in 0..max_columns {
                // columns
                for i in 0..tiles_per_column {
                    // lines
                    let current_frame = i + (j * tiles_per_column);
                    let hotspot = glam::vec2(
                        5.0 + (8.0 * scale_factor)
                            + (j as f32 * 100.0)
                            + (j as f32 * (8.0 * scale_factor)),
                        70.0 + (8.0 * scale_factor)
                            + (i as f32 * (8.0 * scale_factor))
                            + (i as f32 * 10.0),
                    );
                    let text_position =
                        glam::vec2(hotspot.x + (8.0 * scale_factor), hotspot.y - 8.0);

                    let text = TextFragment::new(format!("{:02}", current_frame))
                        .color(Color::WHITE)
                        .scale(PxScale::from(24.0));
                    let text = Text::new(text);

                    graphics::queue_text(context, &text, text_position, None);
                    atlas.queue_draw(current_frame, hotspot, scale)?;
                }
            }
        }
        Ok(())
    }

    fn draw(&self, context: &mut Context) -> GameResult {
        if self.tiles.is_some() {
            let atlas = self.tiles.as_ref().unwrap();
            atlas.draw(context)?;
        }
        Ok(())
    }
}
