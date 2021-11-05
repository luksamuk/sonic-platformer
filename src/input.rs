#![allow(dead_code)]

use ggez::input::keyboard::KeyCode;

pub enum InputButton {
    Debug,
    Up,
    Down,
    Left,
    Right,
    Start,
    Back,
    A,
}

#[derive(Default, Clone)]
pub struct InputState {
    pub debug: bool,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub start: bool,
    pub back: bool,
    pub a: bool,
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
