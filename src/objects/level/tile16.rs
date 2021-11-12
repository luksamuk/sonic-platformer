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
    pub fn put(&self, sheet: &mut SpriteAtlas, hotspot: Vec2, camera_pos: Vec2) -> GameResult {
        let scale = glam::vec2(1.0, 1.0);
        let mut i = 0;
        for tile in &self.tiles {
            if *tile != 0 {
                let position =
                    glam::vec2((i % 2) as f32 * 8.0, (i as f32 / 2.0).floor() * 8.0) + hotspot;
                let position = position - camera_pos;
                sheet.queue_draw(*tile, position, scale)?;
            }
            i += 1;
        }
        Ok(())
    }
}
