//! The game controller.

use crate::{Game, Player, Players};
use anyhow::{anyhow, Result};
use piston::{Button, ButtonArgs, ButtonState, UpdateArgs};
use std::borrow::Borrow;

/// A game controller that handles input events.
#[derive(Debug)]
pub struct GameController {
    settings: GameControllerSettings,
    game: Game,
    selected_cells: Players<(usize, usize)>,
}

impl GameController {
    /// Creates a new game controller.
    pub fn new(settings: GameControllerSettings, game: Game) -> Result<Self> {
        let selected_cells = settings.selected_cells;
        let n_rows = game.settings().n_rows;
        let n_columns = game.settings().n_columns;

        for (player, offset) in [(Player::Left, 0), (Player::Right, n_columns)]
            .iter()
            .copied()
        {
            let (row, column) = selected_cells[player];
            if row >= n_rows || column - offset >= n_columns {
                return Err(anyhow!("invalid selected cell"));
            }
        }

        Ok(Self {
            settings,
            game,
            selected_cells,
        })
    }

    /// Returns a reference to the game being controlled.
    pub fn game(&self) -> &Game {
        &self.game
    }

    /// Returns a reference the selected cells.
    pub fn selected_cells(&self) -> &Players<(usize, usize)> {
        &self.selected_cells
    }

    /// Handles a button event.
    pub fn button_event(&mut self, args: ButtonArgs) -> Result<()> {
        if args.state != ButtonState::Release {
            return Ok(());
        }

        for &player in &[Player::Left, Player::Right] {
            let settings = &self.settings;
            let game = &mut self.game;

            let key_binding = &settings.key_binding[player];
            let selected_cell = self.selected_cells[player];

            if args.button == key_binding.remove {
                game.clear_cell(player, selected_cell)?;
            }

            if let Some(index) = find(&key_binding.place, &args.button) {
                game.place_object(player, selected_cell, index)?;
            }

            let delta = if args.button == key_binding.up {
                (-1, 0)
            } else if args.button == key_binding.down {
                (1, 0)
            } else if args.button == key_binding.left {
                (0, -1)
            } else if args.button == key_binding.right {
                (0, 1)
            } else {
                continue;
            };

            self.move_selection(player, delta)?;
        }

        Ok(())
    }

    /// Moves the selection of the specified player.
    fn move_selection(&mut self, player: Player, delta: (isize, isize)) -> Result<()> {
        use std::convert::{TryFrom, TryInto};
        use std::ops::Add;

        let settings = self.game.settings();

        let n_rows = isize::try_from(settings.n_rows)?;
        let n_columns = isize::try_from(settings.n_columns)?;

        let offset = match player {
            Player::Left => 0,
            Player::Right => n_columns,
        };

        let (row, column) = self.selected_cells[player];

        let row = isize::try_from(row)?;
        let column = isize::try_from(column)?;
        let relative_column = column - offset;

        self.selected_cells[player] = (
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

    /// Handles an update event.
    pub fn update_event(&mut self, _args: UpdateArgs) -> Result<()> {
        self.game.update()
    }
}

/// Game controller settings.
#[derive(Clone, Debug)]
pub struct GameControllerSettings {
    /// The key binding for players.
    pub key_binding: Players<KeyBinding>,

    /// Initial selected cells.
    pub selected_cells: Players<(usize, usize)>,
}

/// Key binding for each player.
#[derive(Clone, Debug)]
pub struct KeyBinding {
    /// The key for moving the selection up.
    pub up: Button,
    /// The key for moving the selection down.
    pub down: Button,
    /// The key for moving the selection left.
    pub left: Button,
    /// The key for moving the selection right.
    pub right: Button,
    /// The key for removing an object.
    pub remove: Button,
    /// The keys for placing an object.
    ///
    /// Each key is assigned an index,
    /// which is equal to its position in `place`.
    /// The object with the corresponding index is placed.
    pub place: Vec<Button>,
}

/// Returns the index of the first element in the slice
/// that equals the given value.
fn find<T, U: ?Sized>(slice: &[T], value: &U) -> Option<usize>
where
    T: Borrow<U>,
    U: Eq,
{
    for (i, v) in slice.iter().enumerate() {
        if v.borrow() == value {
            return Some(i);
        }
    }
    None
}
