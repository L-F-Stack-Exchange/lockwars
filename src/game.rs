//! The game state.
//!
//! # Division line
//!
//! The division line separates the two players' territories.

use crate::{Object, Player, PlayerData, Players};
use anyhow::{anyhow, Context, Result};
use ndarray::prelude::*;
use std::ops::Range;

/// The game state.
///
/// Use the [`GameBuilder`] API to build a game.
#[derive(Clone, Debug)]
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

    /// Clears the cell at the specified position.
    pub fn clear_cell(&mut self, _player: Player, position: (usize, usize)) -> Result<()> {
        let cell = self
            .cells
            .get_mut(position)
            .ok_or_else(|| anyhow!("invalid position"))?;
        cell.object = None;
        Ok(())
    }

    /// Places an object at the specified position
    /// according to the specified object index.
    ///
    /// The player's keys are deducted accordingly.
    /// Returns `true` if the placement is successful,
    /// or `false` if the player does not have enough keys.
    pub fn place_object(
        &mut self,
        player: Player,
        position: (usize, usize),
        index: usize,
    ) -> Result<bool> {
        let keys = &mut self.players[player].keys;
        let cell = self
            .cells
            .get_mut(position)
            .ok_or_else(|| anyhow!("invalid position"))?;

        let settings = &self.settings;
        let cost = *settings.costs.get(index).context("invalid index")?;

        *keys = match keys.checked_sub(cost) {
            None => return Ok(false),
            Some(remaining_keys) => remaining_keys,
        };

        cell.object = Some(
            settings
                .objects
                .get(index)
                .context("invalid index")?
                .clone(),
        );
        Ok(true)
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

    /// The maximum amount of keys each player can have.
    pub max_keys: u32,

    /// The objects that can be placed in the game.
    ///
    /// Each object is assigned an index,
    /// which is equal to its position in `objects`.
    pub objects: Vec<Object>,

    /// The costs of placing objects,
    /// following the index of objects.
    pub costs: Vec<u32>,
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
        let mut cell = self.cells.get_mut(index).context("cannot preset object")?;
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
