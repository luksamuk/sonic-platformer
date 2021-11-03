use crate::objects::general::Position;
use ggez::graphics::{self, DrawParam, Image, Rect};
use ggez::{Context, GameError, GameResult};
use glam::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, PartialEq)]
pub struct AnimatorData {
    pub atlas: Image,
    pub data: HashMap<String, (Vec<u32>, bool, usize, Duration)>,
    pub frame_size: Vec2,
}

impl AnimatorData {
    pub fn new(context: &mut Context, atlas_path: &str, frame_size: Vec2) -> GameResult<Self> {
        let atlas = Image::new(context, atlas_path)?;

        Ok(Self {
            atlas,
            data: HashMap::new(),
            frame_size,
        })
    }

    pub fn get_image_size(&self) -> Vec2 {
        Vec2::new(self.atlas.width() as f32, self.atlas.height() as f32)
    }

    pub fn add_animation(
        &mut self,
        name: &str,
        frames: &[u32],
        loops: bool,
        loopback: usize,
        millis_per_frame: u64,
    ) -> GameResult {
        if loops && loopback >= frames.len() {
            Err(GameError::ConfigError(String::from(
                "Animation loopback frame index must be valid",
            )))
        } else {
            self.data.insert(
                name.trim().to_owned(),
                (
                    Vec::from(frames),
                    loops,
                    loopback,
                    Duration::from_millis(millis_per_frame),
                ),
            );
            Ok(())
        }
    }

    pub fn with_data(
        &mut self,
        data_set: &[(&str, &[u32], bool, usize, u64)],
    ) -> GameResult<&Self> {
        for data in data_set {
            self.add_animation(data.0, data.1, data.2, data.3, data.4)?;
        }
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AnimationDirection {
    Left,
    Right,
}

impl Into<f32> for AnimationDirection {
    fn into(self) -> f32 {
        match self {
            AnimationDirection::Left => -1.0,
            AnimationDirection::Right => 1.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Animator {
    animation_name: String,
    frame_count: usize,
    current_frame: u32,
    last_update: Instant,
    pub direction: AnimationDirection,
    scale: f32,
}

impl Default for Animator {
    fn default() -> Self {
        Animator {
            animation_name: String::new(),
            frame_count: 0,
            current_frame: 0,
            last_update: Instant::now(),
            direction: AnimationDirection::Right,
            scale: 1.0,
        }
    }
}

impl Animator {
    pub fn get_name(&self) -> String {
        self.animation_name.clone()
    }

    pub fn set(&mut self, animation: String) {
        // Set new animation, but leave current frame intact.
        if self.animation_name != animation {
            self.animation_name = animation.trim().to_string();
            self.frame_count = 0;
            self.last_update = Instant::now();
        }
    }

    // TODO: Animation data should be refactored! Using tuples is too intricate now.
    // TODO: Animation speed should be mutable, so we should fetch animation speed on
    //       first frame maybe?
    pub fn update(&mut self, animdata: &AnimatorData) {
        if let Some(data) = animdata.data.get(&self.animation_name) {
            let now = Instant::now();
            let delta = (now - self.last_update) as Duration;
            let frame_duration = data.3;
            if delta > frame_duration {
                let elapsed_frames = (delta.as_millis() / frame_duration.as_millis()) as usize;
                self.last_update = now;
                // Increment frame count and handle loop
                self.frame_count = if data.1 {
                    // If loops
                    let new_frame = self.frame_count + elapsed_frames;
                    if new_frame > data.0.len() - 1 {
                        let extra_frames = new_frame - (data.0.len() - 1);
                        data.2 + extra_frames - 1 // Loopback frame
                    } else {
                        new_frame
                    }
                } else if self.frame_count > (data.0.len() - 1) {
                    data.0.len() - 1
                } else {
                    self.frame_count + elapsed_frames
                };
            }
            // Fetch animation frame number
            self.current_frame = *data.0.get(self.frame_count).unwrap_or(&0);
        }
    }

    pub fn calculate_frame(&self, img_size: Vec2, frame_size: Vec2) -> Rect {
        let frames_per_line = img_size.x / frame_size.x;
        let frame_line = (self.current_frame as f32 / frames_per_line).trunc();
        let frame_column = self.current_frame as f32 % frames_per_line;

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
            let frame = self.calculate_frame(animdata.get_image_size(), animdata.frame_size);
            let floatdir: f32 = self.direction.into();
            let xscale = floatdir * self.scale;
            let half_frame = Vec2::new(
                (animdata.frame_size.x / 2.0) * xscale,
                (animdata.frame_size.y / 2.0) * self.scale,
            );
            let destination = hotspot.0 - half_frame;

            let params = DrawParam::default()
                .src(frame)
                .scale(Vec2::new(xscale, self.scale))
                .dest(destination);
            graphics::draw(context, &animdata.atlas, params)?;
        }
        Ok(())
    }
}
