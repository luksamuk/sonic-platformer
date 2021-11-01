mod draw_states;
mod input;
mod screen_systems;

use draw_states::navigation::Navigation;
use ggez::event::EventHandler;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::{Context, GameError, GameResult};
use input::*;
use screen_systems::ScreenSystems;

pub struct MainState {
    navigation: Navigation,
    screen_systems: ScreenSystems,
    input: Input,
}

impl MainState {
    pub fn new(game_name: &'static str) -> GameResult<Self> {
        let navigation = Navigation::default();
        let screen_systems = ScreenSystems::new(game_name);
        let input = Input::default();
        Ok(Self {
            navigation,
            screen_systems,
            input,
        })
    }

    pub fn setup(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.screen_systems
            .update(ctx, &self.navigation, &self.input)?;
        self.input.post_update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        self.screen_systems.draw(ctx, &self.navigation)?;
        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _mod: KeyMods,
        _repeat: bool,
    ) {
        self.input.set_keyboard(keycode, true);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _mod: KeyMods) {
        self.input.set_keyboard(keycode, false);
    }
}