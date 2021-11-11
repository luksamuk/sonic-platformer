use crate::objects::sprite_atlas::SpriteAtlas;
use ggez::{Context, GameResult};
use glam::*;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Tile16 {
    pub tiles: Vec<u32>,
    pub heightmask: u16,
    pub angle: f32,
}

impl Tile16 {
    pub fn put(
        &self,
        sheet: &mut SpriteAtlas,
        hotspot: Vec2,
        camera_pos: Vec2,
    ) -> GameResult {
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

#[derive(Deserialize)]
pub struct Tile128 {
    pub tiles: Vec<usize>,
}

impl Tile128 {
    pub fn put(
        &self,
        tiles16: &Vec<Tile16>,
        sheet: &mut SpriteAtlas,
        hotspot: Vec2,
        camera_pos: Vec2,
    ) -> GameResult {
        let mut i = 0;
        for tile in &self.tiles {
            if *tile != 0 {
                let position =
                    glam::vec2((i % 8) as f32 * 16.0, (i as f32 / 8.0).floor() * 16.0) + hotspot;
                tiles16[*tile].put(sheet, position, camera_pos)?;
            }
            i += 1;
        }
        Ok(())
    }
}

#[derive(Deserialize)]
pub struct Map {
    pub width: u64,
    pub map: Vec<usize>,
}

impl Map {
    pub fn queue_draw(
        &self,
        tiles128: &Vec<Tile128>,
        tiles16: &Vec<Tile16>,
        sheet: &mut SpriteAtlas,
        hotspot: Vec2,
        camera_pos: Vec2,
    ) -> GameResult {
        let mut i = 0;
        for chunk in &self.map {
            if *chunk != 0 {
                let position = glam::vec2(
                    (i as f32 % self.width as f32) * 128.0,
                    (i as f32 / self.width as f32).floor() * 128.0,
                ) + hotspot;
                tiles128[*chunk].put(tiles16, sheet, hotspot, camera_pos)?;
            }
            i += 1;
        }
        Ok(())
    }
}

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
        let map_path = format!("{}/level.json", level_path);

        let tilesheet = SpriteAtlas::new(context, &sprites_path, glam::vec2(8.0, 8.0))?;

        println!("Loading 16x16");
        let tiles16 = serde_json::from_str(&Level::slurp_file(context, &tiles16_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;
        println!("Loading 128x128");
        let tiles128 = serde_json::from_str(&Level::slurp_file(context, &tiles128_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;
        println!("Loading level map");
        let map = serde_json::from_str(&Level::slurp_file(context, &map_path)?)
            .map_err(|e| GameError::ConfigError(e.to_string()))?;
        println!("All ok");

        Ok(Self {
            tilesheet,
            tiles16,
            tiles128,
            map,
        })
    }

    fn slurp_file(context: &Context, path: &str) -> GameResult<String> {
        println!("Slurping {}", path);
        use ggez::filesystem::{self, File};
        use std::io::Read;
        let mut buffer = String::new();
        filesystem::open(context, path)?.read_to_string(&mut buffer)?;
        Ok(buffer)
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
        )
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        self.tilesheet.draw(context)
    }
}
