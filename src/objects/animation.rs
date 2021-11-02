use ggez::graphics::Image;
use ggez::{Context, GameResult};
use std::collections::HashMap;
use crate::objects::general::Position;

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
    pub current_frame: u32,
}

impl Animator {
    pub fn set(&mut self, animation: String) {
        // Set new animation, but leave current frame intact.
        self.animation_name = animation.trim().to_string();
        self.frame_count = 0;
    }
    
    pub fn update(&mut self, animdata: &AnimatorData) {
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

    pub fn draw(&self, context: &mut Context, animdata: &AnimatorData, hotspot: &Position) -> GameResult {
        if let Some(data) = animdata.data.get(&self.animation_name) {
            todo!();
            
        }
        Ok(())
    }
}
