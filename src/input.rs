#![allow(dead_code)]

use ggez::input::keyboard::KeyCode;

pub enum InputButton {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Clone)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Default)]
pub struct Input {
    pub current: InputState,
    pub previous: InputState,
}

impl Input {
    pub fn post_update(&mut self) {
        self.previous = self.current.clone();
    }

    pub fn set_keyboard(&mut self, keycode: KeyCode, state: bool) {
        match keycode {
            KeyCode::Up => self.current.up = state,
            KeyCode::Down => self.current.down = state,
            KeyCode::Left => self.current.left = state,
            KeyCode::Right => self.current.right = state,
            _ => {}
        }
    }

    pub fn pressing(&self, btn: InputButton) -> bool {
        match btn {
            InputButton::Up => self.current.up,
            InputButton::Down => self.current.down,
            InputButton::Left => self.current.left,
            InputButton::Right => self.current.right,
        }
    }

    pub fn pressed(&self, btn: InputButton) -> bool {
        match btn {
            InputButton::Up => self.current.up && !self.previous.up,
            InputButton::Down => self.current.down && !self.previous.down,
            InputButton::Left => self.current.left && !self.previous.left,
            InputButton::Right => self.current.right && !self.previous.right,
        }    
    }
}