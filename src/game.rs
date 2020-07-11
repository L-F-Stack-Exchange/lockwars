//! The game state.

/// The game state.
#[derive(Debug)]
pub struct Game(());

impl Game {
    /// Creates a new game.
    pub fn new() -> Self {
        Self(())
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
