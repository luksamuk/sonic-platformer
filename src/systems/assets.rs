use anyhow::Result;
use ggez::graphics::{Font, Image};
use std::collections::HashMap;
use ggez::Context;

#[derive(Default)]
pub struct AssetSystem {
    images: HashMap<&'static str, Image>,
}

impl AssetSystem {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_image(&mut self, context: &mut Context, path: &'static str) -> Result<&Image> {
        {
            if let Some(img) = self.images.get(path) {
                return Ok(&img.clone());
            }
        }
        
        let img = Image::new(context, path)?;
        self.images.insert(path, img);
        Ok(self.images.get(path).unwrap())
    }
}
