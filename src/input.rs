#![allow(dead_code)]

use ggez::event::Axis;
use ggez::event::Button;
use ggez::input::keyboard::KeyCode;

const DEADZONE: f32 = 0.3;

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
    /// The state of the left stick axis.
    pub lstick: (f32, f32),
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

    fn correct_axes(&mut self) {
        if !self.current.up && !self.current.down {
            self.current.lstick.1 = 0.0;
        }
        if !self.current.left && !self.current.right {
            self.current.lstick.0 = 0.0;
        }
    }

    /// Sets the state of the given button to the given
    /// value, with respect to the keyboard key which was
    /// associated with said button.
    pub fn set_keyboard(&mut self, keycode: KeyCode, state: bool) {
        match keycode {
            KeyCode::F1 => self.current.debug = state,
            KeyCode::Up => {
                self.current.up = state;
                self.current.lstick.1 = 1.0;
            },
            KeyCode::Down => {
                self.current.down = state;
                self.current.lstick.1 = -1.0;
            },
            KeyCode::Left => {
                self.current.left = state;
                self.current.lstick.0 = -1.0;
            },
            KeyCode::Right => {
                self.current.right = state;
                self.current.lstick.0 = 1.0;
            },
            KeyCode::Return => self.current.start = state,
            KeyCode::Escape => self.current.back = state,
            KeyCode::Z => self.current.a = state,
            _ => {}
        }
        self.correct_axes();
    }

    /// Sets the state of the given button to the given
    /// value, with respect to the gamepad button which was
    /// associated with said button.
    pub fn set_gamepad(&mut self, btn: Button, state: bool) {
        match btn {
            Button::RightThumb => self.current.debug = state,
            Button::DPadUp => {
                self.current.up = state;
                self.current.lstick.1 = 1.0;
            },
            Button::DPadDown => {
                self.current.down = state;
                self.current.lstick.1 = -1.0;
            },
            Button::DPadLeft => {
                self.current.left = state;
                self.current.lstick.0 = -1.0;
            },
            Button::DPadRight => {
                self.current.right = state;
                self.current.lstick.0 = 1.0;
            },
            Button::Start => self.current.start = state,
            Button::Select => self.current.back = state,
            Button::South => self.current.a = state,
            _ => {}
        }
        self.correct_axes();
    }

    pub fn set_axis(&mut self, axis: Axis, value: f32) {
        let state = value.abs() >= DEADZONE;
        let value = if !state {
            0.0
        } else {
            value
        };

        match axis {
            Axis::LeftStickX => {
                if !state {
                    self.current.left = false;
                    self.current.right = false;
                    
                } else if value > 0.0 {
                    self.current.left = false;
                    self.current.right = true;
                } else {
                    self.current.left = true;
                    self.current.right = false;
                }
                self.current.lstick.0 = value;
            },
            Axis::LeftStickY => {
                if !state {
                    self.current.up = false;
                    self.current.down = false;
                } else if value > 0.0 {
                    self.current.up = true;
                    self.current.down = false;
                } else {
                    self.current.up = false;
                    self.current.down = true;
                }
                self.current.lstick.1 = value;
            },
            _ => {},
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

    /// Returns the current state of the left thumbstick.
    pub fn left_stick(&self) -> (f32, f32) {
        self.current.lstick
    }
}
