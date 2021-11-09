use crate::objects::sprite_atlas::SpriteAtlas;
use ggez::{Context, GameResult};
use glam::*;

pub struct Tile16 {
    pub tiles: Vec<u64>,
    pub height_map: u16,
    pub angle: f32,
}

impl Tile16 {
    pub fn draw(&self, context: &mut Context, sheet: &SpriteAtlas, hotspot: Vec2, camera_pos: Vec2) -> GameResult {
        let scale = glam::vec2(1.0, 1.0);
        let mut i = 0;
        for tile in self.tiles {
            let position = glam::vec2(
                (i % 2) as f32 * 8.0,
                (i / 2.0).floor() * 8.0,
            ) + hotspot;
            let position = position - camera_pos;
            sheet.draw(context, tile, position, scale)?;
            i += 1;
        }
        Ok(())
    }
}

pub struct Tile128 {
    pub tiles: Vec<u64>,
}

impl Tile128 {
    pub fn draw(&self, context: &mut Context, sheet: &SpriteAtlas, hotspot: Vec2, camera_pos: Vec2) -> GameResult {
        let mut i = 0;
        for tile in self.tiles {
            let position = glam::vec2(
                (i % 2) as f32 * 16.0,
                (i / 2.0).floor() * 16.0,
            ) + hotspot;
            tile.draw(context, sheet, position, camera_pos)?;
        }
        Ok(())
    }
}

pub struct Map {
    pub width: u64,
    pub map: Vec<u64>,
}

pub struct Level {
    tilesheet: SpriteAtlas,
    tiles16: Vec<Tile16>,
    tiles128: Vec<Tile128>,
    map: Map,
}

impl Level {
    pub fn draw(context: &mut Context, hotspot: Vec2, viewport_size: Vec2) -> GameResult {
        unimplemented!();
    }
}