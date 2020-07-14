//! The game controller.

use crate::{Game, Player, Players};
use anyhow::Result;
use piston::{Button, ButtonArgs, ButtonState};

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
            let delta = if args.button == settings.key_binding[player].up {
                (-1, 0)
            } else if args.button == settings.key_binding[player].down {
                (1, 0)
            } else if args.button == settings.key_binding[player].left {
                (0, -1)
            } else if args.button == settings.key_binding[player].right {
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
}
