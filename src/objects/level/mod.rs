use crate::objects::sprite_atlas::SpriteAtlas;
use ggez::{Context, GameResult};
use glam::*;

pub struct Tile16 {
    pub tiles: Vec<u64>,
    pub height_map: u16,
    pub angle: f32,
}

pub struct Tile128 {
    pub tiles: Vec<u64>,
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