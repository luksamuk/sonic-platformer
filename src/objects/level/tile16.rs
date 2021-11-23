use crate::objects::sprite_atlas::SpriteAtlas;
use ggez::GameResult;
use glam::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Tile16 {
    pub tiles: Vec<u32>,
    pub heightmask: u16,
    pub angle: f32,
}

impl Tile16 {
    /// Queues the tile16 for rendering at the given position.
    /// The position is automatically adjusted relative to the camera.
    pub fn put(
        &self,
        sheet: &mut SpriteAtlas,
        hotspot: Vec2,
        camera_pos: Vec2,
        scale: f32,
    ) -> GameResult {
        let scale_v = glam::vec2(scale, scale);
        for (i, tile) in self.tiles.iter().enumerate() {
            if *tile != 0 {
                let position = glam::vec2(
                    (i % 2) as f32 * 8.0 * scale,
                    (i as f32 / 2.0).floor() * 8.0 * scale,
                ) + hotspot;
                let position = position - camera_pos;
                sheet.queue_draw(*tile, position, scale_v)?;
            }
        }
        Ok(())
    }
}
