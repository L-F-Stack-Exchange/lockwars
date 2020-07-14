#![warn(missing_docs)]

//! A simple battle game.

pub mod game;
pub mod game_view;
pub mod object;
pub mod player;

pub use game::{Cell, Game, GameSettings};
pub use game_view::{GameView, GameViewSettings};
pub use object::{Object, ObjectKind};
pub use player::{Player, PlayerData, Players};
