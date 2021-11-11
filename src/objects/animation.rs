use super::sprite_atlas::SpriteAtlas;
use crate::objects::general::Direction;
use crate::objects::general::Position;
use ggez::{Context, GameError, GameResult};
use glam::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationData {
    pub frames: Vec<u32>,
    pub loops: bool,
    pub loopback_index: usize,
    pub speed: Duration,
}
/// A helper structure to build an animator.
///
/// Allows you to build an animator by adding animations to it.
pub struct AnimatorBuilder {
    pub data: HashMap<String, AnimationData>,
}

impl AnimatorBuilder {
    /// Creates a new animator builder.
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Adds an animation to the builder.
    pub fn add_animation(
        &mut self,
        name: &str,
        frames: &[u32],
        loops: bool,
        loopback_index: usize,
        ms_per_frame: u64,
    ) -> GameResult<&mut Self> {
        if loops && loopback_index >= frames.len() {
            Err(GameError::ConfigError(String::from(
                "Animation loopback frame index must be valid",
            )))
        } else {
            self.data.insert(
                name.to_string(),
                AnimationData {
                    frames: Vec::from(frames),
                    loops,
                    loopback_index,
                    speed: Duration::from_millis(ms_per_frame),
                },
            );
            Ok(self)
        }
    }

    /// Builds the animator.
    pub fn build(&self) -> Animator {
        Animator {
            data: self.data.clone(),
            ..Animator::default()
        }
    }
}

/// An animator structure, responsible for managing and
/// storing information related to animations.
/// Animators are responsible for handling logic, and
/// queueing SpriteAtlas for drawing frames in specific
/// locations. To perform actual rendering, draw the
/// sprite atlas in non-immediate mode.
#[derive(Clone, Debug, PartialEq)]
pub struct Animator {
    animation_name: String,
    frame_count: usize,
    current_frame: u32,
    last_update: Instant,
    pub direction: Direction,
    scale: f32,
    frame_duration: Duration,
    data: HashMap<String, AnimationData>,
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
            data: HashMap::new(),
        }
    }
}

impl Animator {
    /// Gets the current animation name.
    pub fn get_name(&self) -> String {
        self.animation_name.clone()
    }

    /// Sets the current animation by its name, if it is registered.
    pub fn set(&mut self, animation: String) {
        // Set new animation, but leave current frame intact.
        if self.animation_name != animation {
            self.animation_name = animation.trim().to_string();
            self.frame_count = 0;
            self.last_update = Instant::now();
            if let Some(data) = self.data.get(&animation) {
                self.frame_duration = data.speed;
            }
        }
    }

    /// Sets the duration of a single animation frame.
    pub fn set_duration(&mut self, duration: Duration) {
        self.frame_duration = duration;
    }

    /// Sets the duration of a single animation frame, in milliseconds.
    pub fn set_duration_ms(&mut self, duration: u64) {
        self.frame_duration = Duration::from_millis(duration);
    }

    /// Updates the animation.
    pub fn update(&mut self, atlas: &mut SpriteAtlas, hotspot: &Position) -> GameResult {
        if let Some(data) = self.data.get(&self.animation_name) {
            let now = Instant::now();
            let delta = (now - self.last_update) as Duration;
            if delta > self.frame_duration {
                let frame_duration_mi = self.frame_duration.as_micros();
                let delta_mi = delta.as_micros();
                if frame_duration_mi > 0 {
                    let elapsed_frames = (delta_mi / frame_duration_mi) as usize;
                    self.last_update = now;

                    // Increment frame count and handle loop
                    self.frame_count = if data.loops {
                        let new_frame = self.frame_count + elapsed_frames;
                        if new_frame > data.frames.len() - 1 {
                            let extra_frames = new_frame - (data.frames.len() - 1);
                            data.loopback_index + extra_frames - 1 // Loopback frame
                        } else {
                            new_frame
                        }
                    } else if self.frame_count > (data.frames.len() - 1) {
                        data.frames.len() - 1
                    } else {
                        self.frame_count + elapsed_frames
                    };
                }
            }
            // Fetch animation frame number
            self.current_frame = *data.frames.get(self.frame_count).unwrap_or(&0);

            // Queue drawing
            let direction: f32 = self.direction.into();
            let xscale = direction * self.scale;
            atlas.queue_draw(
                self.current_frame,
                hotspot.0,
                glam::vec2(xscale, self.scale))
        } else {
            Ok(())
        }
    }
}
