use bevy::prelude::*;

use std::fmt::Debug;
use std::hash::Hash;

/// Marker trait for all types that are valid for use as Bevy States
pub trait BevyState: Component + Debug + Clone + Eq + Hash {}
impl<T: Component + Debug + Clone + Eq + Hash> BevyState for T {}

/// Recursively despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_all<T: Component>(
    mut cmd: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        cmd.entity(e).despawn_recursive();
    }
}
