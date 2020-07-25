#![warn(missing_docs)]

#![warn(clippy::cargo)]
#![warn(clippy::pedantic)]

//! A simple battle game.

pub mod cooldown;
pub mod game;
pub mod game_controller;
pub mod game_view;
pub mod object;
pub mod player;

pub use cooldown::Cooldown;
pub use game::{Cell, Game, GameBuilder, GameSettings};
pub use game_controller::{GameController, GameControllerSettings, KeyBinding};
pub use game_view::{GameView, GameViewSettings};
pub use object::{Object, ObjectKind};
pub use player::{Player, PlayerData, Players};
