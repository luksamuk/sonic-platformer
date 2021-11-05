use crate::input::{Input, InputButton};
use crate::objects::animation::*;
use crate::objects::general::*;
use crate::objects::player::*;
use crate::screen_systems::Navigation;
use ggez::{Context, GameResult};
use legion::*;

pub struct LevelScreenSystem {
    world: World,
    first_update: bool,
    debug: bool,
}

impl LevelScreenSystem {
    pub fn new() -> Self {
        let world = World::default();
        let first_update = true;
        let debug = false;
        Self {
            world,
            first_update,
            debug,
        }
    }

    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        Player::create(context, &mut self.world, false)?;
        Ok(())
    }

    pub fn update(&mut self, navigation: &mut Navigation, input: &Input) -> GameResult {
        if self.first_update {
            self.first_update = false;
            Player::respawn_all(&mut self.world);
        }
        // Update players
        Player::animation_update(&mut self.world)?;
        Player::physics_update(&mut self.world, input)?;

        // Update all animated sprites
        let mut query = <(&AnimatorData, &mut Animator)>::query();
        for (data, animator) in query.iter_mut(&mut self.world) {
            animator.update(data);
        }

        if input.pressed(InputButton::Back) {
            self.first_update = true;
            *navigation = Navigation::TitleScreen;
        }

        if input.pressed(InputButton::Debug) {
            self.debug = !self.debug;
        }

        Ok(())
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        // Draw all animated sprites
        let mut query = <(&AnimatorData, &Animator, &Position)>::query();
        for (data, animator, hotspot) in query.iter(&self.world) {
            animator.draw(context, data, hotspot)?;
        }

        // Draw sensors
        if self.debug {
            let mut query = <(&PlayerState, &Position, &PlayerSpeed)>::query();
            for (state, position, speed) in query.iter(&self.world) {
                PlayerSensors::draw(context, state, position, speed)?;
            }
        }

        Ok(())
    }
}
