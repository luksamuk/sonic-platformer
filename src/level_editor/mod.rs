use ggez::event::EventHandler;
use ggez::Context;
use ggez::GameError;
use ggez::GameResult;
use ggez::{
    self,
    graphics::{self, Color},
};

pub struct EditorState;

impl EventHandler<GameError> for EditorState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        graphics::present(ctx)
    }
}
