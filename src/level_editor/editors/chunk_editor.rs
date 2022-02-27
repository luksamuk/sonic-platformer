use crate::objects::level::Tile128;
use crate::objects::level::Tile16;
use crate::objects::sprite_atlas::SpriteAtlas;

pub struct ChunkEditor {
    tiles_path: String,
    pieces_path: String,
    tiles: Option<SpriteAtlas>,
    pieces: Vec<Tile16>,
    data: Vec<Tile128>,
    current_tile: usize,
}

