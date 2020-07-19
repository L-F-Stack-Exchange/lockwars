//! The objects in the game.

use crate::{Cooldown, Player};

/// An object.
#[derive(Clone, Debug)]
pub struct Object {
    /// The kind of the object.
    ///
    /// Contains kind-specific object information.
    pub kind: ObjectKind,
}

/// The kind of an object.
///
/// Contains kind-specific object information.
#[derive(Clone, Debug)]
pub enum ObjectKind {
    /// A key object.
    Key {
        /// The cooldown for key generation.
        cooldown: Cooldown,
    },
    /// A fire object.
    Fire,
}

/// An object owned by a player.
#[derive(Clone, Debug)]
pub struct OwnedObject {
    /// The object.
    pub object: Object,

    /// The owner of the object.
    pub owner: Player,
}
