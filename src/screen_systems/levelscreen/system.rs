use legion::*;
use crate::input::Input;
use ggez::{Context, GameResult};
use crate::objects::general::*;
use crate::objects::player::*;
use crate::objects::animation::*;

pub struct LevelScreenSystem {
    world: World,
}

impl LevelScreenSystem {
    pub fn new() -> Self {
        let mut world = World::default();

        Self { world }
    }

    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        Player::create(context, &mut self.world, false)?;
        Ok(())
    }

    pub fn update(&mut self, input: &Input) -> GameResult {
        // Update players
        Player::physics_update(&mut self.world, input)?;

        // Update all animated sprites
        let mut query = <(&AnimatorData, &mut Animator)>::query();
        for (data, animator) in query.iter_mut(&mut self.world) {
            animator.update(data);
        }
        
        Ok(())
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        // Draw all animated sprites
        let mut query = <(&AnimatorData, &Animator, &Position)>::query();
        for (data, animator, hotspot) in query.iter(&self.world) {
            animator.draw(context, data, hotspot)?;
        }
        Ok(())
    }
}
