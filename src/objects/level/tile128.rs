use super::tile16::Tile16;
use crate::objects::sprite_atlas::SpriteAtlas;
use ggez::GameResult;
use glam::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Tile128 {
    pub tiles: Vec<usize>,
}

impl Tile128 {
    pub fn put(
        &self,
        tiles16: &[Tile16],
        sheet: &mut SpriteAtlas,
        hotspot: Vec2,
        camera_pos: Vec2,
        scale: f32,
    ) -> GameResult {
        for (i, tile) in self.tiles.iter().enumerate() {
            if *tile != 0 {
                let position = glam::vec2(
                    (i % 8) as f32 * 16.0 * scale,
                    (i as f32 / 8.0).floor() * 16.0 * scale,
                ) + hotspot;
                tiles16[*tile].put(sheet, position, camera_pos, scale)?;
            }
        }
        Ok(())
    }
}
