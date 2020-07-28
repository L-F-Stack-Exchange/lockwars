#![warn(missing_docs)]
#![warn(clippy::pedantic)]

//! A simple battle game.

pub mod cooldown;
pub mod game;
pub mod game_controller;
pub mod game_view;
pub mod object;
pub mod player;

pub use cooldown::Cooldown;
pub use game::Game;
pub use game_controller::GameController;
pub use game_view::GameView;
pub use object::Object;
pub use player::{Player, Players};
