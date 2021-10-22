use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use glam::*;

mod objects;

struct MainState {
    circulo_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let state = MainState { circulo_x: 0.0 };
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.circulo_x = self.circulo_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        let circulo = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(0.0, 0.0),
            100.0,
            0.001,
            graphics::Color::WHITE,
        )?;
        graphics::draw(ctx, &circulo, (Vec2::new(self.circulo_x, 380.0),))?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let context_builder = ggez::ContextBuilder::new("platformer", "Lucas S. Vieira");
    let (ctx, event_loop) = context_builder.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
