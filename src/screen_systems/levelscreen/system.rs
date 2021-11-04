use crate::input::Input;
use crate::objects::animation::*;
use crate::objects::general::*;
use crate::objects::player::*;
use crate::screen_systems::Navigation;
use ggez::{Context, GameResult};
use legion::*;

pub struct LevelScreenSystem {
    world: World,
}

impl LevelScreenSystem {
    pub fn new() -> Self {
        let world = World::default();
        Self { world }
    }

    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        Player::create(context, &mut self.world, false)?;
        Ok(())
    }

    pub fn update(&mut self, _navigation: &mut Navigation, input: &Input) -> GameResult {
        // Update players
        Player::animation_update(&mut self.world, input)?;
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
