use crate::input::{Input, InputButton};
use crate::objects::animation::*;
use crate::objects::camera::Camera;
use crate::objects::general::*;
use crate::objects::level::*;
use crate::objects::player::{self, *};
use crate::objects::sprite_atlas::SpriteAtlas;
use crate::screen_systems::Navigation;
use ggez::{Context, GameResult};
use glam::*;
use legion::*;

/// Defines the state for a level screen system.
pub struct LevelScreenSystem {
    world: World,
    first_update: bool,
    debug: bool,
    camera: Option<Camera>,
    camera_timer: i32,
    level: Option<Level>,
    viewport_size: Vec2,
}

impl Default for LevelScreenSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl LevelScreenSystem {
    /// Creates a new level screen system.
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
            level: None,
            viewport_size: Vec2::ZERO,
        }
    }

    /// Sets up the initial state of the level screen system.
    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        self.camera = Some(Camera::new(context));
        self.level = Some(Level::load(context, "/levels/R0")?);
        self.viewport_size = {
            let rect = ggez::graphics::screen_coordinates(context);
            glam::vec2(rect.w, rect.h)
        };
        Player::create(context, &mut self.world, false)?;
        Ok(())
    }

    /// Updates the level screen system.
    pub fn update(&mut self, navigation: &mut Navigation, input: &Input) -> GameResult {
        if self.first_update {
            self.first_update = false;
            Player::respawn_all(&mut self.world);
        }

        // Update all animated sprites
        let mut query = <(&mut Animator, &mut SpriteAtlas, &Position)>::query();
        for (animator, atlas, position) in query.iter_mut(&mut self.world) {
            let hotspot = Position::wrap(if let Some(camera) = &self.camera {
                camera.transform(position.0)
            } else {
                position.0
            });
            atlas.clear();
            animator.update(atlas, &hotspot)?;
        }

        // Update players
        player::animation::update(&mut self.world)?;
        player::physics::update(&mut self.world, input)?;

        // Update camera panning
        self.camera_timer = {
            let (up, down) = (
                input.pressing(InputButton::Up),
                input.pressing(InputButton::Down),
            );
            if (up && !down) || (!up && down) {
                (self.camera_timer + 1).min(120)
            } else {
                0
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
                } else if speed.xsp >= 6.0 {
                    CameraDisplacementBehaviour::ExtendRight
                } else if speed.xsp <= -6.0 {
                    CameraDisplacementBehaviour::ExtendLeft
                } else {
                    CameraDisplacementBehaviour::None
                };

                camera.vertical_behaviour = vbehaviour;
                camera.displacement_behaviour = dbehaviour;
                camera.update(Some(position))?;
            }
        }

        // Update level
        if self.level.is_some() {
            use glam::*;
            let level = self.level.as_mut().unwrap();
            let camera_pos = if let Some(camera) = &self.camera {
                camera.transform(Vec2::ZERO)
            } else {
                Vec2::ZERO
            };
            level.clear();
            level.update(-camera_pos, self.viewport_size)?;
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
        let mut bg_builder = MeshBuilder::new();

        // Crosses on background
        for i in 0..30 {
            for j in 0..30 {
                let center = glam::vec2(100.0 * i as f32, 100.0 * j as f32);
                let _ = bg_builder
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
        let mut floor_builder = MeshBuilder::new();
        let _ = floor_builder.line(
            &[
                glam::vec2(0.0, FAKE_GROUND_Y + 16.0 + 5.0),
                glam::vec2(3000.0, FAKE_GROUND_Y + 16.0 + 5.0),
            ],
            5.0,
            Color::new(0.3, 0.0, 0.2, 0.5),
        )?;

        let bg_mesh = bg_builder.build(context)?;
        let floor_mesh = floor_builder.build(context)?;
        let position = if let Some(camera) = &self.camera {
            camera.transform(Vec2::ZERO)
        } else {
            Vec2::ZERO
        };

        // Parallax effect
        let bg_position = position * 0.65;

        graphics::draw(context, &bg_mesh, (bg_position, 0.0, Color::WHITE))?;
        graphics::draw(context, &floor_mesh, (position, 0.0, Color::WHITE))?;
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
            "FPS    {:>5.2}\n\
             ACTION {:?}\n\
             POSX   {:>13.6}\n\
             POSY   {:>13.6}\n\
             GSP    {:>13.6}\n\
             XSP    {:>13.6}\n\
             YSP    {:>13.6}\n\
             THETA  {:>13.6}",
            ggez::timer::fps(context),
            state.action,
            pos.0.x,
            pos.0.y,
            speed.gsp,
            speed.xsp,
            speed.ysp,
            speed.angle,
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

    /// Draws the level screen.
    pub fn draw(&self, context: &mut Context) -> GameResult {
        // Draw test graphics
        self.draw_test_graphics(context)?;

        // Draw level
        if self.level.is_some() {
            let level = self.level.as_ref().unwrap();
            level.draw(context)?;
        }

        // Draw all animated sprites
        let mut query = <&SpriteAtlas>::query();
        for atlas in query.iter(&self.world) {
            atlas.draw(context)?;
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
