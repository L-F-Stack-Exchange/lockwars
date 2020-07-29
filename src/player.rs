//! The players.

use crate::{Cooldown, Object};
use std::fmt;

/// A player.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Player {
    /// The left player.
    Left,
    /// The right player.
    Right,
}

impl Player {
    /// Returns the opposite player.
    pub fn toggle(self) -> Player {
        match self {
            Player::Left => Player::Right,
            Player::Right => Player::Left,
        }
    }
}

/// A container that holds the same data for both players.
#[derive(Clone, Copy, Debug, Default)]
pub struct Players<T> {
    /// The data associated with the left player.
    pub left: T,
    /// The data associated with the right player.
    pub right: T,
}

impl<T> std::ops::Index<Player> for Players<T> {
    type Output = T;

    fn index(&self, index: Player) -> &T {
        match index {
            Player::Left => &self.left,
            Player::Right => &self.right,
        }
    }
}

impl<T> std::ops::IndexMut<Player> for Players<T> {
    fn index_mut(&mut self, index: Player) -> &mut T {
        match index {
            Player::Left => &mut self.left,
            Player::Right => &mut self.right,
        }
    }
}

/// The player data.
#[derive(Debug)]
pub struct Data {
    /// The amount of keys the player owns.
    pub keys: u32,
    /// Placements.
    pub placements: Vec<Placement>,
}

/// A placement.
pub struct Placement {
    /// The cooldown of the placement.
    pub cooldown: Cooldown,
    /// The cost of the placement.
    pub cost: u32,
    /// Generates the object to be placed.
    pub generate_object: Box<dyn Fn() -> Object>,
}

impl Placement {
    /// Generates the object to be placed.
    pub fn generate_object(&self) -> Object {
        (self.generate_object)()
    }
}

impl fmt::Debug for Placement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Placement")
            .field("cooldown", &self.cooldown)
            .field("cost", &self.cost)
            .field("object", &self.generate_object())
            .finish()
    }
}
