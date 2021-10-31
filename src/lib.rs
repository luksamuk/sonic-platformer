mod draw_states;
mod screen_systems;

use draw_states::navigation::Navigation;
use screen_systems::ScreenSystems;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::{Context, GameError, GameResult};
//use glam::*;

pub struct MainState {
    navigation: Navigation,
    screen_systems: ScreenSystems,
}

impl MainState {
    pub fn new(game_name: &'static str) -> GameResult<Self> {
        let navigation = Navigation::default();
        let screen_systems = ScreenSystems::new(game_name);
        Ok(Self {
            navigation,
            screen_systems,
        })
    }

    pub fn setup(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.screen_systems.update(ctx, &self.navigation)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.screen_systems.draw(ctx, &self.navigation)?;
        graphics::present(ctx)
    }
}
