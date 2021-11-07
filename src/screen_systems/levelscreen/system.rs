use crate::input::{Input, InputButton};
use crate::objects::animation::*;
use crate::objects::camera::Camera;
use crate::objects::general::*;
use crate::objects::player::{self, *};
use crate::screen_systems::Navigation;
use ggez::{Context, GameResult};
use legion::*;

pub struct LevelScreenSystem {
    world: World,
    first_update: bool,
    debug: bool,
    camera: Option<Camera>,
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
            camera: None,
        }
    }

    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        self.camera = Some(Camera::new(context));
        Player::create(context, &mut self.world, false)?;
        Ok(())
    }

    pub fn update(&mut self, navigation: &mut Navigation, input: &Input) -> GameResult {
        if self.first_update {
            self.first_update = false;
            Player::respawn_all(&mut self.world);
        }
        // Update players
        player::animation::update(&mut self.world)?;
        player::physics::update(&mut self.world, input)?;

        // Update all animated sprites
        let mut query = <(&AnimatorData, &mut Animator)>::query();
        for (data, animator) in query.iter_mut(&mut self.world) {
            animator.update(data);
        }

        // Update camera
        let mut query = <(&PlayerState, &Position, &PlayerSpeed)>::query();
        for (state, position, speed) in query.iter(&self.world) {
            if self.camera.is_some() {
                use crate::objects::camera::CameraVerticalBehaviour;
                let camera = self.camera.as_mut().unwrap();

                let behaviour = if !state.get_ground() {
                    CameraVerticalBehaviour::RespectBounds
                } else {
                    let ysp = speed.ysp.abs();
                    if ysp <= 6.0 {
                        CameraVerticalBehaviour::CenterYSlow
                    } else {
                        CameraVerticalBehaviour::CenterYFast
                    }
                };

                camera.update(Some(position), behaviour)?;
            }
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
        for (data, animator, position) in query.iter(&self.world) {
            let hotspot = Position::wrap(if let Some(camera) = &self.camera {
                camera.transform(position.0)
            } else {
                position.0
            });
            animator.draw(context, data, &hotspot)?;
        }

        // Draw sensors and camera
        if self.debug {
            let mut query = <(&PlayerState, &Position, &PlayerSpeed)>::query();
            for (state, position, speed) in query.iter(&self.world) {
                let hotspot = Position::wrap(if let Some(camera) = &self.camera {
                    camera.transform(position.0)
                } else {
                    position.0
                });
                PlayerSensors::draw(context, state, &hotspot, speed)?;
            }

            if let Some(camera) = &self.camera {
                camera.debug_draw(context)?;
            }
        }

        Ok(())
    }
}
