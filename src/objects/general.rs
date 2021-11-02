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
}
