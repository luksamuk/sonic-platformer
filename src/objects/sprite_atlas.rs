use ggez::graphics::{self, DrawParam, Image, Rect};
use ggez::{Context, GameResult};
use glam::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteAtlas {
    pub texture: Image,
    pub frame_size: Vec2,
    half_frame: Vec2,
}

impl SpriteAtlas {
    pub fn new(context: &mut Context, path: &str, frame_size: Vec2) -> GameResult<Self> {
        let texture = Image::new(context, path)?;
        Ok(Self {
            texture,
            frame_size,
            half_frame: frame_size / 2.0,
        })
    }

    pub fn get_image_size(&self) -> Vec2 {
        glam::vec2(self.texture.width() as f32, self.texture.height() as f32)
    }

    fn calculate_frame(&self, frame: u32) -> Rect {
        let image_size = self.get_image_size();
        let frames_per_line = image_size.x / self.frame_size.x;
        let frame_line = (frame as f32 / frames_per_line).trunc();
        let frame_column = frame as f32 % frames_per_line;
        let frame_size_texels = self.frame_size / image_size;

        Rect::new(
            frame_column * frame_size_texels.x,
            frame_line * frame_size_texels.y,
            frame_size_texels.x,
            frame_size_texels.y,
        )
    }

    pub fn draw(
        &self,
        context: &mut Context,
        frame: u32,
        hotspot: Vec2,
        scale: Vec2,
    ) -> GameResult {
        let frame = self.calculate_frame(frame);
        let half_frame = self.half_frame * scale;
        let destination = hotspot - half_frame;
        let params = DrawParam::default()
            .src(frame)
            .scale(scale)
            .dest(destination);
        graphics::draw(context, &self.texture, params)
    }
}
