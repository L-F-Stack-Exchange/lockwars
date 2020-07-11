#![warn(missing_docs)]

//! A simple battle game.

pub mod game;
pub mod game_view;

pub use game::Game;
pub use game_view::{GameView, GameViewSettings};
