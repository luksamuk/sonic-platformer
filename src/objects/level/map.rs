use super::tile128::Tile128;
use super::tile16::Tile16;
use crate::objects::sprite_atlas::SpriteAtlas;
use ggez::graphics::Rect;
use ggez::GameResult;
use glam::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Map {
    pub width: u64,
    pub map: Vec<usize>,
}

impl Map {
    pub fn queue_draw(
        &self,
        tiles128: &[Tile128],
        tiles16: &[Tile16],
        sheet: &mut SpriteAtlas,
        hotspot: Vec2,
        camera_pos: Vec2,
        viewport_size: Vec2,
    ) -> GameResult {
        let viewport = Rect::new(camera_pos.x, camera_pos.y, viewport_size.x, viewport_size.y);
        for (i, chunk) in self.map.iter().enumerate() {
            if *chunk != 0 {
                let position = glam::vec2(
                    (i as f32 % self.width as f32) * 128.0,
                    (i as f32 / self.width as f32).floor() * 128.0,
                ) + hotspot;

                let chunk_rect = Rect::new(position.x, position.y, 128.0, 128.0);

                if chunk_rect.overlaps(&viewport) {
                    tiles128[*chunk].put(tiles16, sheet, position, camera_pos)?;
                }
            }
        }
        Ok(())
    }
}
