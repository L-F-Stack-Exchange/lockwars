//! The game controller.

use crate::{Cell, Game, Object, Player, Players};
use anyhow::Result;
use piston::{Button, ButtonArgs, ButtonState};
use std::borrow::Borrow;

/// A game controller that handles input events.
#[derive(Clone, Debug)]
pub struct GameController {
    settings: GameControllerSettings,
}

impl GameController {
    /// Creates a new game controller.
    pub fn new(settings: GameControllerSettings) -> Self {
        Self { settings }
    }

    /// Handles a button event.
    pub fn button_event(&mut self, game: &mut Game, args: ButtonArgs) -> Result<()> {
        let settings = &self.settings;

        if args.state != ButtonState::Release {
            return Ok(());
        }

        for &player in &[Player::Left, Player::Right] {
            let key_binding = &settings.key_binding[player];
            let selected_position = game.players()[player].selected_position;

            if args.button == key_binding.remove {
                game.set_cell(player, selected_position, Cell::empty())?;
            } else if let Some(index) = find(&key_binding.place, &args.button) {
                let object = match settings.objects.get(index) {
                    None => continue,
                    Some(object) => object.clone(),
                };
                let cell = Cell {
                    object: Some(object),
                };
                game.set_cell(player, selected_position, cell)?;
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

            game.move_selection(player, delta)?;
        }

        Ok(())
    }
}

/// Game controller settings.
#[derive(Clone, Debug)]
pub struct GameControllerSettings {
    /// The key binding for players.
    pub key_binding: Players<KeyBinding>,

    /// The objects that can be placed in the game.
    ///
    /// Each object is assigned an index,
    /// which is equal to its position in `objects`.
    pub objects: Vec<Object>,
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
