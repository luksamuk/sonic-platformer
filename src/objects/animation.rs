use crate::objects::general::Position;
use ggez::graphics::{self, DrawParam, Image, Rect};
use ggez::{Context, GameResult};
use glam::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct AnimatorData {
    pub atlas: Image,
    pub data: HashMap<String, (Vec<u32>, bool)>,
}

impl AnimatorData {
    pub fn new(context: &mut Context, atlas_path: &str) -> GameResult<Self> {
        let atlas = Image::new(context, atlas_path)?;

        Ok(Self {
            atlas,
            data: HashMap::new(),
        })
    }
    pub fn add_animation(&mut self, name: &str, frames: &[u32], loops: bool) {
        self.data
            .insert(name.trim().to_owned(), (Vec::from(frames), loops));
    }

    pub fn with_data(&mut self, data_set: &[(&str, &[u32], bool)]) -> &Self {
        for data in data_set {
            self.add_animation(data.0, data.1, data.2);
        }
        self
    }
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct Animator {
    animation_name: String,
    frame_count: usize,
    current_frame: u32,
}

impl Animator {
    pub fn set(&mut self, animation: String) {
        // Set new animation, but leave current frame intact.
        self.animation_name = animation.trim().to_string();
        self.frame_count = 0;
    }

    pub fn update(&mut self, animdata: &AnimatorData) {
        // TODO: Animation speed and loopback frame
        if let Some(data) = animdata.data.get(&self.animation_name) {
            // Increment frame count and handle loop
            self.frame_count = if data.1 {
                (self.frame_count + 1) % data.0.len()
            } else if self.frame_count > (data.0.len() - 1) {
                data.0.len() - 1
            } else {
                self.frame_count + 1
            };
            // Fetch animation frame number
            self.current_frame = *data.0.get(self.frame_count).unwrap_or(&0);
        }
    }

    pub fn calculate_frame(img_size: Vec2, frame_size: Vec2, frame_number: u32) -> Rect {
        // TODO: Refactor using &self
        let frames_per_line = img_size.x / frame_size.x;
        let frame_line = (frame_number as f32 / frames_per_line).trunc();
        let frame_column = frame_number as f32 % frames_per_line;

        let frame_size_texels = frame_size / img_size;

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
        animdata: &AnimatorData,
        hotspot: &Position,
    ) -> GameResult {
        if let Some(_) = animdata.data.get(&self.animation_name) {
            // TODO: outsource these to animator
            let frame_size = Vec2::new(60.0, 60.0);
            let image_size = Vec2::new(
                animdata.atlas.width() as f32,
                animdata.atlas.height() as f32,
            );

            let frame = Animator::calculate_frame(image_size, frame_size, self.current_frame);

            let destination = hotspot.0 - (frame_size / 2.0);

            let params = DrawParam::default()
                .src(frame)
                .scale(Vec2::new(1.0, 1.0))
                .dest(destination);
            graphics::draw(context, &animdata.atlas, params)?;
        }
        Ok(())
    }
}
