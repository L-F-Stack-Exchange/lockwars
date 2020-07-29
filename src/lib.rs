#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]

//! A simple battle game.

pub mod controller;
pub mod cooldown;
pub mod game;
pub mod game_view;
pub mod object;
pub mod player;

pub use controller::Controller;
pub use cooldown::Cooldown;
pub use game::Game;
pub use game_view::GameView;
pub use object::Object;
pub use player::{Player, Players};
