use bevy::prelude::*;

use std::fmt::Debug;
use std::hash::Hash;

/// Marker trait for all types that are valid for use as Bevy States
pub trait BevyState: Debug + Clone + Eq + Hash {}
impl<T: Debug + Clone + Eq + Hash> BevyState for T {}

/// Marker trait for Bevy States with some extra bounds that are nice to have
pub trait NiceBevyState: BevyState + Component + Copy + Send + Sync + 'static {}
impl<T: BevyState + Component + Copy + Send + Sync + 'static> NiceBevyState for T {}

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with<T: Component>(
    mut cmd: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        cmd.entity(e).despawn();
    }
}

/// Despawn all entities with a specific marker component
///
/// Useful when exiting states
pub fn despawn_with_recursive<T: Component>(
    mut cmd: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        cmd.entity(e).despawn_recursive();
    }
}

/// Remove a resource using Commands
pub fn remove_resource<T: Send + Sync + 'static>(
    mut cmd: Commands,
) {
    cmd.remove_resource::<T>();
}
