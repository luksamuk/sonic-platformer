pub mod input;
pub mod objects;
pub mod resources;
pub mod screen_systems;

use ggez::event::Axis;
use ggez::event::Button;
use ggez::event::EventHandler;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics::{self, Color};
use ggez::input::gamepad::GamepadId;
use ggez::timer;
use ggez::{Context, GameError, GameResult};
use input::*;
use screen_systems::Navigation;
use screen_systems::ScreenSystems;

const DESIRED_FPS: u32 = 60;

/// Represents the main game state.
pub struct MainState {
    navigation: Navigation,
    screen_systems: ScreenSystems,
    input: Input,
}

impl MainState {
    /// Creates a new MainState.
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

    /// Sets up the MainState by loading assets and setting up the initial
    /// state.
    pub fn setup(&mut self, context: &mut Context) -> GameResult {
        self.screen_systems.setup(context)?;
        Ok(())
    }
}

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.screen_systems
                .update(ctx, &mut self.navigation, &self.input)?;
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

    fn gamepad_button_down_event(&mut self, _ctx: &mut Context, btn: Button, _id: GamepadId) {
        self.input.set_gamepad(btn, true);
    }

    fn gamepad_button_up_event(&mut self, _ctx: &mut Context, btn: Button, _id: GamepadId) {
        self.input.set_gamepad(btn, false);
    }

    fn gamepad_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: f32, _id: GamepadId) {
        if (axis == Axis::LeftStickX) || (axis == Axis::LeftStickY) {
            self.input.set_axis(axis, value);
        }
    }
}
