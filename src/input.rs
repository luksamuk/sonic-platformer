#![allow(dead_code)]

use ggez::input::keyboard::KeyCode;

/// An enumeration describing possible buttons on
/// the game, regardless of the platform.
pub enum InputButton {
    /// The debug button.
    Debug,
    /// The up button.
    Up,
    /// The down button.
    Down,
    /// The left button.
    Left,
    /// The right button.
    Right,
    /// The start button.
    Start,
    /// The select/back button.
    Back,
    /// The A button.
    A,
}

/// A structure describing an input state.
#[derive(Default, Clone)]
pub struct InputState {
    /// The state of the debug button.
    pub debug: bool,
    /// The state of the up button.
    pub up: bool,
    /// The state of the down button.
    pub down: bool,
    /// The state of the left button.
    pub left: bool,
    /// The state of the right button.
    pub right: bool,
    /// The state of the start button.
    pub start: bool,
    /// The state of the select/back button.
    pub back: bool,
    /// The state of the A button.
    pub a: bool,
}

/// A structure describing the input system.
///
/// This structure is used to track the state of the
/// input system in general, and to be updated by using
/// the proper key down/up functions related to its input
/// events.
#[derive(Default)]
pub struct Input {
    current: InputState,
    previous: InputState,
}

impl Input {
    /// Updates the input system state. Should be called
    /// every frame.
    pub fn post_update(&mut self) {
        self.previous = self.current.clone();
    }

    /// Sets the state of the given button to the given
    /// value, with respect to the keyboard key which was
    /// associated with said button.
    pub fn set_keyboard(&mut self, keycode: KeyCode, state: bool) {
        match keycode {
            KeyCode::F1 => self.current.debug = state,
            KeyCode::Up => self.current.up = state,
            KeyCode::Down => self.current.down = state,
            KeyCode::Left => self.current.left = state,
            KeyCode::Right => self.current.right = state,
            KeyCode::Return => self.current.start = state,
            KeyCode::Escape => self.current.back = state,
            KeyCode::Z => self.current.a = state,
            _ => {}
        }
    }

    /// Checks if the given button is currently being pressed.
    pub fn pressing(&self, btn: InputButton) -> bool {
        match btn {
            InputButton::Debug => self.current.debug,
            InputButton::Up => self.current.up,
            InputButton::Down => self.current.down,
            InputButton::Left => self.current.left,
            InputButton::Right => self.current.right,
            InputButton::Start => self.current.start,
            InputButton::Back => self.current.back,
            InputButton::A => self.current.a,
        }
    }

    /// Checks if the given button was tapped this frame.
    /// A button is never tapped for more than one frame.
    pub fn pressed(&self, btn: InputButton) -> bool {
        match btn {
            InputButton::Debug => self.current.debug && !self.previous.debug,
            InputButton::Up => self.current.up && !self.previous.up,
            InputButton::Down => self.current.down && !self.previous.down,
            InputButton::Left => self.current.left && !self.previous.left,
            InputButton::Right => self.current.right && !self.previous.right,
            InputButton::Start => self.current.start && !self.previous.start,
            InputButton::Back => self.current.back && !self.previous.back,
            InputButton::A => self.current.a && !self.previous.a,
        }
    }
}
