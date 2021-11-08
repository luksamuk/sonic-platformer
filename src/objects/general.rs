#![allow(clippy::from_over_into)]

use glam::Vec2;

/// Refers to a tag component which can be attached to any entity.
///
/// This is a multi-purpose tag which can be used as a flag or as
/// an extra generic value.
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Tag(pub u64);

/// Refers to a position component.
///
/// Should be used with any entity that has a position on the
/// 2D plane.
#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Position(pub Vec2);

impl Position {
    /// Create new position from specific point
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    /// Create position from another
    pub fn from(pos: &Self) -> Self {
        Self(Vec2::new(pos.0.x, pos.0.y))
    }

    /// Wrap a vector into a Position struct
    pub fn wrap(pos: Vec2) -> Self {
        Self(pos)
    }
}

/// Represents a direction for anything. May be converted to
/// a unitary float value for rendering purposes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl Into<f32> for Direction {
    fn into(self) -> f32 {
        match self {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Right
    }
}
