#![warn(missing_docs)]

//! A simple battle game.

pub mod game;
pub mod game_view;

pub use game::{Game, GameSettings};
pub use game_view::{GameView, GameViewSettings};
