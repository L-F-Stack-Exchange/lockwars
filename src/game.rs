//! The game state.
//!
//! # Division line
//!
//! The division line separates the two players' territories.

use crate::{Object, ObjectKind, OwnedObject, Player, PlayerData, Players};
use anyhow::{anyhow, Context, Result};
use ndarray::prelude::*;
use std::cell::RefCell;
use std::fmt;
use std::ops::Range;

/// The game state.
///
/// Use the [`GameBuilder`] API to build a game.
#[derive(Debug)]
pub struct Game {
    settings: GameSettings,
    cells: Array2<RefCell<Cell>>,
    players: Players<PlayerData>,
}

impl Game {
    /// Returns the game settings.
    pub fn settings(&self) -> &GameSettings {
        &self.settings
    }

    /// Returns the cells.
    pub fn cells(&self) -> ArrayView2<RefCell<Cell>> {
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
            .get(position)
            .ok_or_else(|| anyhow!("invalid position"))?;
        cell.borrow_mut().object = None;
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
        let (object, cost) = settings.get_object(index).context("invalid index")?;

        *keys = match keys.checked_sub(cost) {
            None => return Ok(false),
            Some(remaining_keys) => remaining_keys,
        };

        cell.borrow_mut().object = Some(OwnedObject {
            object: object,
            owner: player,
        });
        Ok(true)
    }

    /// Updates the state of the game.
    pub fn update(&mut self) -> Result<()> {
        let settings = &self.settings;

        for ((row, _column), cell) in self.cells.indexed_iter() {
            let mut cell = cell.borrow_mut();
            let object = match &mut cell.object {
                None => continue,
                Some(object) => object,
            };
            let owner = object.owner;

            match object.object.kind {
                ObjectKind::Key { ref mut cooldown } => {
                    let players = &mut self.players;
                    cooldown.execute(|| {
                        let keys = &mut players[owner].keys;
                        *keys = keys
                            .saturating_add(settings.key_generation)
                            .min(settings.max_keys);
                    });
                }
                ObjectKind::Fire {
                    damage,
                    ref mut cooldown,
                } => {
                    cooldown.execute(|| {
                        if let Some(target) = self.find_target(row, owner.toggle()) {
                            target.borrow_mut().receive_damage(damage);
                        }
                    });
                }
            }
        }

        Ok(())
    }

    /// Find a target on the specified row.
    ///
    /// `player` specifies the targeted player.
    fn find_target(&self, row: usize, player: Player) -> Option<&RefCell<Cell>> {
        fn find_in<I>(game: &Game, row: usize, column_range: I) -> Option<&RefCell<Cell>>
        where
            I: Iterator<Item = usize>,
        {
            for column in column_range {
                let cell = &game.cells[(row, column)];
                if cell.borrow().object.is_some() {
                    return Some(cell);
                }
            }
            None
        }

        let n_columns = self.settings.n_columns;
        let n_total_columns = 2 * n_columns;

        match player {
            Player::Left => find_in(self, row, (0..n_columns).rev()),
            Player::Right => find_in(self, row, n_columns..n_total_columns),
        }
    }
}

/// The game settings.
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

    /// The number of objects that can be placed in the game.
    pub object_count: usize,

    /// The objects that can be placed in the game,
    /// and their costs.
    ///
    /// Each object is assigned an index in `0..object_count`.
    /// Calling `objects` with an index
    /// returns the object with the corresponding index,
    /// along with its cost.
    pub objects: Box<dyn Fn(usize) -> Option<(Object, u32)>>,

    /// The amount of keys generated by key objects.
    pub key_generation: u32,
}

impl GameSettings {
    /// Returns the object with the corresponding index and its cost.
    pub fn get_object(&self, index: usize) -> Option<(Object, u32)> {
        (self.objects)(index)
    }
}

impl fmt::Debug for GameSettings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // skip individual objects
        f.debug_struct("GameSettings")
            .field("n_columns", &self.n_columns)
            .field("n_rows", &self.n_rows)
            .field("base_span", &self.base_span)
            .field("max_keys", &self.max_keys)
            .field("object_count", &self.n_columns)
            .field("key_generation", &self.key_generation)
            .finish()
    }
}

/// Builds a game.
#[derive(Debug)]
pub struct GameBuilder {
    settings: GameSettings,
    cells: Array2<RefCell<Cell>>,
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
                cells: Array2::from_shape_simple_fn((n_rows, n_total_columns), || {
                    RefCell::new(Cell::empty())
                }),
                players: None,
            })
        }
    }

    /// Presets an object.
    pub fn object(mut self, index: (usize, usize), owned_object: OwnedObject) -> Result<Self> {
        let cell = self.cells.get_mut(index).context("cannot preset object")?;
        cell.borrow_mut().object = Some(owned_object);
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
    pub object: Option<OwnedObject>,
}

impl Cell {
    /// Returns an empty cell.
    pub fn empty() -> Self {
        Self { object: None }
    }

    /// Receives the specified amount of damage,
    /// if an object is present.
    ///
    /// The object is removed if its health runs out.
    pub fn receive_damage(&mut self, damage: u32) {
        let object = match &mut self.object {
            None => return,
            Some(object) => &mut object.object,
        };

        if object.health > damage {
            object.health -= damage;
        } else {
            self.object = None;
        }
    }
}
