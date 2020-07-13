//! The game state.
//!
//! # Division line
//!
//! The division line separates the two players' territories.

use crate::Object;
use anyhow::{anyhow, Result};
use ndarray::prelude::*;
use std::ops::Range;

/// The game state.
#[derive(Debug)]
pub struct Game {
    settings: GameSettings,
    cells: Array2<Cell>,
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
            let n_rows = settings.n_rows;
            let n_total_columns = settings.n_columns * 2;

            Ok(Self {
                settings,
                cells: Array2::from_shape_simple_fn((n_rows, n_total_columns), Cell::empty),
            })
        }
    }

    /// Returns the game settings.
    pub fn settings(&self) -> &GameSettings {
        &self.settings
    }

    /// Returns the cells.
    pub fn cells(&self) -> ArrayView2<Cell> {
        self.cells.view()
    }

    /// Returns a mutable reference to the cells.
    ///
    /// Intended for presetting objects.
    ///
    /// TODO: replace with `GameBuilder` API
    pub fn cells_mut(&mut self) -> ArrayViewMut2<Cell> {
        self.cells.view_mut()
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

/// A cell.
#[derive(Clone, Debug)]
pub struct Cell {
    /// The optional object placed in the cell.
    pub object: Option<Object>,
}

impl Cell {
    /// Returns an empty cell.
    pub fn empty() -> Self {
        Self { object: None }
    }
}
