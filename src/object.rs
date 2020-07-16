//! The objects in the game.

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
    Key,
    /// A fire object.
    Fire,
}
