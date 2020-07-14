//! The game state.
//!
//! # Division line
//!
//! The division line separates the two players' territories.

use crate::{Object, Player, PlayerData, Players};
use anyhow::{anyhow, Context, Result};
use ndarray::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::ops::{Add, Range};

/// The game state.
///
/// Use the [`GameBuilder`] API to build a game.
#[derive(Debug)]
pub struct Game {
    settings: GameSettings,
    cells: Array2<Cell>,
    players: Players<PlayerData>,
}

impl Game {
    /// Returns the game settings.
    pub fn settings(&self) -> &GameSettings {
        &self.settings
    }

    /// Returns the cells.
    pub fn cells(&self) -> ArrayView2<Cell> {
        self.cells.view()
    }

    /// Returns the players.
    pub fn players(&self) -> &Players<PlayerData> {
        &self.players
    }

    /// Moves the selection of the specified player.
    pub fn move_selection(&mut self, player: Player, delta: (isize, isize)) -> Result<()> {
        let n_rows = isize::try_from(self.settings.n_rows)?;
        let n_columns = isize::try_from(self.settings.n_columns)?;

        let offset = match player {
            Player::Left => 0,
            Player::Right => n_columns,
        };

        let (row, column) = self.players[player].selected_position;

        let row = isize::try_from(row)?;
        let column = isize::try_from(column)?;
        let relative_column = column - offset;

        self.players[player].selected_position = (
            (isize::try_from(row)?.add(delta.0))
                .rem_euclid(n_rows)
                .try_into()?,
            (isize::try_from(relative_column)?.add(delta.1))
                .rem_euclid(n_columns)
                .add(offset)
                .try_into()?,
        );

        Ok(())
    }
}

/// The game settings.
#[derive(Clone, Debug)]
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

/// Builds a game.
#[derive(Clone, Debug)]
pub struct GameBuilder {
    settings: GameSettings,
    cells: Array2<Cell>,
    players: Option<Players<PlayerData>>,
}

impl GameBuilder {
    /// Creates a new game builder.
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
                players: None,
            })
        }
    }

    /// Presets an object.
    pub fn object(mut self, index: (usize, usize), object: Object) -> Result<Self> {
        let mut cell = self
            .cells
            .get_mut(index)
            .context(anyhow!("cannot preset object"))?;
        cell.object = Some(object);
        Ok(self)
    }

    /// Sets player data.
    pub fn players(mut self, players: Players<PlayerData>) -> Self {
        self.players = Some(players);
        self
    }

    /// Builds a game.
    pub fn finish(self) -> Result<Game> {
        Ok(Game {
            settings: self.settings,
            cells: self.cells,
            players: self
                .players
                .ok_or_else(|| anyhow!("player data must be provided"))?,
        })
    }
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
