//! The players.

/// A player.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Player {
    /// The left player.
    Left,
    /// The right player.
    Right,
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
#[derive(Clone, Debug)]
pub struct PlayerData {
    /// The index of the selected cell.
    pub selected_position: (usize, usize),
    /// The amount of keys the player owns.
    pub keys: u32,
}
