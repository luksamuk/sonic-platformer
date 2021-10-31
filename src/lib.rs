mod draw_states;
mod draw_systems;

use draw_states::navigation::Navigation;
use draw_systems::DrawSystems;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::{Context, GameError, GameResult};
//use glam::*;

pub struct MainState {
    navigation: Navigation,
    draw_systems: DrawSystems,
}

impl MainState {
    pub fn new(game_name: &'static str) -> GameResult<Self> {
        let navigation = Navigation::default();
        let draw_systems = DrawSystems::new(game_name);
        Ok(Self {
            navigation,
            draw_systems,
        })
    }

    pub fn setup(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.draw_systems.run(ctx, &self.navigation)?;
        graphics::present(ctx)
    }
}
