use crate::objects::general::Direction;
use crate::objects::general::Position;
use ggez::graphics::{self, DrawParam, Image, Rect};
use ggez::{Context, GameError, GameResult};
use glam::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use super::sprite_atlas::SpriteAtlas;

#[derive(Clone, Debug, PartialEq)]
pub struct AnimatorData {
    pub atlas: SpriteAtlas,
    pub data: HashMap<String, (Vec<u32>, bool, usize, Duration)>,
}

impl AnimatorData {
    pub fn new(context: &mut Context, path: &str, frame_size: Vec2) -> GameResult<Self> {
        let atlas = SpriteAtlas::new(context, path, frame_size)?;

        Ok(Self {
            atlas,
            data: HashMap::new(),
        })
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

#[derive(Clone, Debug, PartialEq)]
pub struct Animator {
    animation_name: String,
    frame_count: usize,
    current_frame: u32,
    last_update: Instant,
    pub direction: Direction,
    scale: f32,
    frame_duration: Duration,
}

impl Default for Animator {
    fn default() -> Self {
        Animator {
            animation_name: String::new(),
            frame_count: 0,
            current_frame: 0,
            last_update: Instant::now(),
            direction: Direction::Right,
            scale: 1.0,
            frame_duration: Duration::from_millis(16),
        }
    }
}

impl Animator {
    pub fn get_name(&self) -> String {
        self.animation_name.clone()
    }

    pub fn set(&mut self, animation: String, animdata: &AnimatorData) {
        // Set new animation, but leave current frame intact.
        if self.animation_name != animation {
            self.animation_name = animation.trim().to_string();
            self.frame_count = 0;
            self.last_update = Instant::now();
            if let Some(data) = animdata.data.get(&animation) {
                self.frame_duration = data.3;
            }
        }
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.frame_duration = duration;
    }

    pub fn set_duration_ms(&mut self, duration: u64) {
        self.frame_duration = Duration::from_millis(duration);
    }

    // TODO: Animation data should be refactored! Using tuples is too intricate now.
    pub fn update(&mut self, animdata: &AnimatorData) {
        if let Some(data) = animdata.data.get(&self.animation_name) {
            let now = Instant::now();
            let delta = (now - self.last_update) as Duration;
            if delta > self.frame_duration {
                let frame_duration_mi = self.frame_duration.as_micros();
                let delta_mi = delta.as_micros();
                if frame_duration_mi > 0 {
                    let elapsed_frames = (delta_mi / frame_duration_mi) as usize;
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
        if animdata.data.get(&self.animation_name).is_some() {
            let direction: f32 = self.direction.into();
            let xscale = direction * self.scale;
            animdata.atlas.draw(context, self.current_frame, hotspot.0, glam::vec2(xscale, self.scale))?;
        }
        Ok(())
    }
}
