use std::collections::HashMap;
use ggez::{Context, GameResult};
use ggez::graphics::Image;

pub struct SpriteSheet {
    sheet: Image,
}

impl SpriteSheet {
    fn new(context: &mut Context, path: &'static str) -> GameResult<Self> {
        Ok(Self {
            sheet: Image::new(context, path)?,
        })
    }
}

#[derive(Default)]
pub struct Animator {
    animation_data: HashMap<&'static str, Vec<u32>>,
}

impl Animator {
    pub fn add_animation(&mut self, name: &'static str, frames: &[u32]) {
        self.animation_data.insert(name, Vec::from(frames));
    }
}

#[macro_export]
macro_rules! build_animator {
    ($(($name:tt, $data:expr),)+) => {
        {
            let mut animator: Animator = Default::default();
            $(animator.add_animation($name, &$data);)*
            animator
        }
    }
}
