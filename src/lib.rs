mod input;
mod objects;
mod screen_systems;

use screen_systems::Navigation;
use ggez::event::EventHandler;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::timer;
use ggez::{Context, GameError, GameResult};
use input::*;
use screen_systems::ScreenSystems;

const DESIRED_FPS: u32 = 60;

pub struct MainState {
    navigation: Navigation,
    screen_systems: ScreenSystems,
    input: Input,
}

impl MainState {
    pub fn new(game_name: &'static str) -> GameResult<Self> {
        //let navigation = Navigation::default();
        let navigation = Navigation::LevelScreen;
        let screen_systems = ScreenSystems::new(game_name);
        let input = Input::default();
        Ok(Self {
            navigation,
            screen_systems,
            input,
        })
    }

    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        self.screen_systems.setup(context)?;
        Ok(())
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.screen_systems
                .update(ctx, &self.navigation, &self.input)?;
            self.input.post_update();
        }
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
