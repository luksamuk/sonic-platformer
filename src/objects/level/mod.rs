mod map;
mod tile128;
mod tile16;

use crate::objects::sprite_atlas::SpriteAtlas;
use ggez::{Context, GameResult};
use glam::*;

use map::Map;
use tile128::Tile128;
use tile16::Tile16;

pub struct Level {
    tilesheet: SpriteAtlas,
    tiles16: Vec<Tile16>,
    tiles128: Vec<Tile128>,
    map: Map,
}

impl Level {
    pub fn load(context: &mut Context, level_path: &str) -> GameResult<Self> {
        use ggez::GameError;
        let sprites_path = format!("{}/tiles.png", level_path);
        let tiles16_path = format!("{}/16x16.json", level_path);
        let tiles128_path = format!("{}/128x128.json", level_path);
        let map_path = format!("{}/map.json", level_path);

        let tilesheet = SpriteAtlas::new(context, &sprites_path, glam::vec2(8.0, 8.0))?;

        let tiles16 = serde_json::from_str(&slurp_file(context, &tiles16_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;
        let tiles128 = serde_json::from_str(&slurp_file(context, &tiles128_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;
        let map = serde_json::from_str(&slurp_file(context, &map_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;

        Ok(Self {
            tilesheet,
            tiles16,
            tiles128,
            map,
        })
    }

    pub fn clear(&mut self) {
        self.tilesheet.clear();
    }

    pub fn update(&mut self, hotspot: Vec2, viewport_size: Vec2) -> GameResult {
        self.map.queue_draw(
            &self.tiles128,
            &self.tiles16,
            &mut self.tilesheet,
            Vec2::ZERO,
            hotspot + glam::vec2(-8.0, -8.0),
            viewport_size,
        )
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        self.tilesheet.draw(context)
    }
}

fn slurp_file(context: &Context, path: &str) -> GameResult<String> {
    use ggez::filesystem;
    use std::io::Read;
    let mut buffer = String::new();
    filesystem::open(context, path)?.read_to_string(&mut buffer)?;
    Ok(buffer)
}
