use crate::level_editor::Input;
use ggez::Context;
use ggez::GameResult;

/// Trait representing a contract for an editor.
pub trait Editor {
    /// Reloads the editor.
    fn reload(&mut self, context: &mut Context) -> GameResult;

    /// Runs a single update step for the editor.
    fn update(&mut self, context: &mut Context, input: &Input) -> GameResult;

    /// Draws the editor on screen.
    fn draw(&self, context: &mut Context) -> GameResult;
}
