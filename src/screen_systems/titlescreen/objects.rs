use crate::objects::general::{Position, Tag};
use crate::screen_systems::Navigation;
use crate::{Input, InputButton};
use ggez::graphics::MeshBuilder;
use ggez::graphics::{self, Color};
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;
use legion::{IntoQuery, World};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Marker {
    pub num_options: u64,
    pub draw_step: f32,
}

impl Marker {
    pub fn update(world: &mut World, navigation: &mut Navigation, input: &Input) -> GameResult {
        let mut query = <(&Marker, &mut Tag, &mut Position)>::query();
        for (marker, tag, position) in query.iter_mut(world) {
            if input.pressed(InputButton::Down) {
                tag.0 += 1;
            } else if input.pressed(InputButton::Up) {
                tag.0 = if tag.0 == 0 {
                    marker.num_options - 1
                } else {
                    tag.0 - 1
                };
            }
            // Adjust option to a valid one and setup draw position
            tag.0 %= marker.num_options;
            position.0.y = tag.0 as f32 * marker.draw_step;

            if (tag.0 == 0) && input.pressed(InputButton::Start) {
                *navigation = Navigation::LevelScreen;
            }
        }

        Ok(())
    }

    pub fn draw(world: &World, context: &mut Context, hotspot: Vec2) -> GameResult {
        let mut query = <(&Marker, &Position)>::query();
        for (_, position) in query.iter(world) {
            let mesh = MeshBuilder::new()
                .triangles(
                    &[
                        glam::vec2(0.0, 0.0),
                        glam::vec2(0.0, 10.0),
                        glam::vec2(10.0, 5.0),
                    ],
                    Color::WHITE,
                )?
                .build(context)?;
            graphics::draw(context, &mesh, (hotspot + position.0, 0.0, Color::WHITE))?;
        }
        Ok(())
    }
}
