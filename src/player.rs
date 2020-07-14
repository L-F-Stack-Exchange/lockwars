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

/// The player data.
#[derive(Clone, Debug)]
pub struct PlayerData {
    /// The index of the selected cell.
    pub selected_position: (usize, usize),
}
