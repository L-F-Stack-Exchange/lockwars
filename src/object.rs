//! The objects in the game.

/// An object.
#[derive(Clone, Debug)]
pub struct Object {
    /// The kind of the object.
    ///
    /// Contains kind-specific object information.
    pub kind: ObjectKind,
}

impl Object {
    /// Returns the cost of the object.
    pub fn cost(&self) -> u32 {
        match self.kind {
            ObjectKind::Key => 20,
            ObjectKind::Fire => 40,
        }
    }
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
