use crate::{Input, InputButton};
use ggez::graphics::MeshBuilder;
use ggez::graphics::{self, Color};
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;
use legion::{IntoQuery, World};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Marker {
    pub num_options: u8,
    pub draw_step: f32,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub pos: Vec2,
}

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Tag {
    pub tag: u8,
}

impl Marker {
    pub fn update(world: &mut World, input: &Input) -> GameResult {
        let mut query = <(&Marker, &mut Tag, &mut Position)>::query();
        for (marker, tag, position) in query.iter_mut(world) {
            if input.pressed(InputButton::Down) {
                tag.tag += 1;
            } else if input.pressed(InputButton::Up) {
                tag.tag = if tag.tag == 0 {
                    marker.num_options - 1
                } else {
                    tag.tag - 1
                };
            }
            // Adjust option to a valid one and setup draw position
            tag.tag %= marker.num_options;
            position.pos.y = tag.tag as f32 * marker.draw_step;
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
            graphics::draw(context, &mesh, (hotspot + position.pos, 0.0, Color::WHITE))?;
        }
        Ok(())
    }
}
