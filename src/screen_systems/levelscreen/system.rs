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
    camera_timer: i32,
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
            camera_timer: 0,
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

        // Update camera panning
        self.camera_timer = {
            let (up, down) = (
                input.pressing(InputButton::Up),
                input.pressing(InputButton::Down),
            );
            if (up && !down) || (!up && down) {
                (self.camera_timer + 1).min(120)
            } else {
                (self.camera_timer - 1).max(0)
            }
        };

        // Update camera
        let mut query = <(&PlayerState, &Position, &PlayerSpeed)>::query();
        for (state, position, speed) in query.iter(&self.world) {
            if self.camera.is_some() {
                use crate::objects::camera::{
                    CameraDisplacementBehaviour, CameraVerticalBehaviour,
                };
                let camera = self.camera.as_mut().unwrap();

                let vbehaviour = if !state.get_ground() {
                    CameraVerticalBehaviour::RespectBounds
                } else {
                    let ysp = speed.ysp.abs();
                    if ysp <= 6.0 {
                        CameraVerticalBehaviour::CenterYSlow
                    } else {
                        CameraVerticalBehaviour::CenterYFast
                    }
                };

                let dbehaviour = if self.camera_timer >= 120 {
                    match state.action {
                        PlayerAction::LookingUp => CameraDisplacementBehaviour::LookUp,
                        PlayerAction::Crouching => CameraDisplacementBehaviour::LookDown,
                        _ => CameraDisplacementBehaviour::None,
                    }
                } else {
                    CameraDisplacementBehaviour::None
                };

                camera.vertical_behaviour = vbehaviour;
                camera.displacement_behaviour = dbehaviour;
                camera.update(Some(position))?;
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

    fn draw_test_graphics(&self, context: &mut Context) -> GameResult {
        // Draw a grid for camera testing
        use crate::objects::player::physics::FAKE_GROUND_Y;
        use ggez::graphics::{self, Color, MeshBuilder};
        use glam::*;
        let mut builder = MeshBuilder::new();

        // Crosses on background
        for i in 0..30 {
            for j in 0..30 {
                let center = glam::vec2(100.0 * i as f32, 100.0 * j as f32);
                let _ = builder
                    .line(
                        &[
                            glam::vec2(center.x, center.y - 10.0),
                            glam::vec2(center.x, center.y + 10.0),
                        ],
                        1.0,
                        Color::new(0.0, 1.0, 1.0, 0.3),
                    )?
                    .line(
                        &[
                            glam::vec2(center.x - 10.0, center.y),
                            glam::vec2(center.x + 10.0, center.y),
                        ],
                        1.0,
                        Color::new(0.0, 1.0, 1.0, 0.3),
                    )?;
            }
        }

        // Representation for floor
        let _ = builder.line(
            &[
                glam::vec2(0.0, FAKE_GROUND_Y + 16.0 + 5.0),
                glam::vec2(3000.0, FAKE_GROUND_Y + 16.0 + 5.0),
            ],
            5.0,
            Color::new(0.3, 0.0, 0.2, 0.5),
        )?;

        let mesh = builder.build(context)?;
        let position = if let Some(camera) = &self.camera {
            camera.transform(Vec2::ZERO)
        } else {
            Vec2::ZERO
        };
        graphics::draw(context, &mesh, (position, 0.0, Color::WHITE))?;
        Ok(())
    }

    fn draw_debug_text(
        &self,
        context: &mut Context,
        state: &PlayerState,
        speed: &PlayerSpeed,
        pos: &Position,
    ) -> GameResult {
        use ggez::graphics::{self, Color, PxScale, Text, TextFragment};

        let mut hud_text = format!(
            "ACTION {:?}\n\
             POSX   {:>13.6}\n\
             POSY   {:>13.6}\n\
             GSP    {:>13.6}\n\
             XSP    {:>13.6}\n\
             YSP    {:>13.6}\n\
             THETA  {:>13.6}",
            state.action, pos.0.x, pos.0.y, speed.gsp, speed.xsp, speed.ysp, speed.angle,
        );

        if let Some(camera) = &self.camera {
            hud_text = format!(
                "{}\n\
            CAMX   {:>13.6}\n\
            CAMY   {:>13.6}",
                hud_text, camera.position.0.x, camera.position.0.y,
            );
        }

        let text = TextFragment::new(hud_text)
            .color(Color::WHITE)
            .scale(PxScale::from(12.0));
        let text = Text::new(text);
        graphics::queue_text(context, &text, glam::vec2(10.0, 10.0), None);
        Ok(())
    }

    pub fn draw(&self, context: &mut Context) -> GameResult {
        // Draw test graphics
        self.draw_test_graphics(context)?;

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
                PlayerSensors::debug_draw(context, state, &hotspot, speed)?;
                self.draw_debug_text(context, state, speed, position)?;
            }

            if let Some(camera) = &self.camera {
                camera.debug_draw(context)?;
            }
        }

        Ok(())
    }
}
