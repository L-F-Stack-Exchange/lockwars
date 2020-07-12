//! The game state.
//!
//! # Division line
//!
//! The division line separates the two players' territories.

use anyhow::{anyhow, Result};
use std::ops::Range;

/// The game state.
#[derive(Debug)]
pub struct Game {
    settings: GameSettings,
}

impl Game {
    /// Creates a new game.
    pub fn new(settings: GameSettings) -> Result<Self> {
        if settings.n_columns == 0 {
            Err(anyhow!("game must contain at least one column"))
        } else if settings.n_rows == 0 {
            Err(anyhow!("game must contain at least one row"))
        } else if settings.base_span.start >= settings.base_span.end {
            Err(anyhow!("base must span at least one row"))
        } else if settings.base_span.end > settings.n_rows {
            Err(anyhow!("base must not exceed game area"))
        } else {
            Ok(Self { settings })
        }
    }

    /// Returns the game settings.
    pub fn settings(&self) -> &GameSettings {
        &self.settings
    }
}

/// The game settings.
#[derive(Debug)]
pub struct GameSettings {
    /// The number of columns on each player's side.
    ///
    /// The total number of columns is `2 * n_columns`.
    pub n_columns: usize,

    /// The number of rows in the game.
    pub n_rows: usize,

    /// The rows that the bases span.
    pub base_span: Range<usize>,
}
