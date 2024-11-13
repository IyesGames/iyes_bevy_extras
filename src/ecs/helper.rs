use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

use std::fmt::Display;

/// Convenience system for despawning all entities that match a given query filter
///
/// This is useful as an "exit" system in your app states, to clean up large
/// swaths of entities on state transition. For example, you could create a marker
/// component for all of your gameplay entities, and use this system to easily
/// despawn all of them when going back to the main menu.
///
/// Consider using [`despawn_all_with_recursive`]
/// instead, to ensure you are not left with broken hierarchies. This could happen if
/// you have an entity with the component in a hierarchy where not all entities have the
/// component. This system will only despawn the entities with the component.
pub fn despawn_all<F: QueryFilter>(world: &mut World, query: &mut QueryState<Entity, F>) {
    let entities: Vec<Entity> = query.iter(world).collect();
    for entity in entities {
        world.despawn(entity);
    }
}

/// Recursive version of [`despawn_all`]
pub fn despawn_all_recursive<F: QueryFilter>(world: &mut World, query: &mut QueryState<Entity, F>) {
    let entities: Vec<Entity> = query.iter(world).collect();
    for entity in entities {
        if let Ok(entity_mut) = world.get_entity_mut(entity) {
            entity_mut.despawn_recursive();
        }
    }
}

/// Convenience system for removing a resource of the given type
///
/// This is useful as an "exit" system in your app states, to remove resouces
/// that should only be present in a specific state.
pub fn remove_resource<T: Resource>(world: &mut World) {
    world.remove_resource::<T>();
}

/// Convenience system for initting a resource of the given type
///
/// This is useful as an "enter" system in your app states, to create resouces
/// that should only be present in a specific state.
pub fn init_resource<T: Resource + FromWorld>(world: &mut World) {
    world.init_resource::<T>();
}

/// Convenience system for removing a component from all entities that match a given query filter
///
/// This may be useful as an "exit" system in your app states.
pub fn remove_from_all<T: Bundle, F: QueryFilter>(
    world: &mut World,
    query: &mut QueryState<Entity, F>,
) {
    let entities: Vec<Entity> = query.iter(world).collect();
    for entity in entities {
        if let Ok(mut entity_mut) = world.get_entity_mut(entity) {
            entity_mut.remove::<T>();
        }
    }
}

/// Helper to log an error (if any) from a Bevy system function that returns `Result`
///
/// Intended for system piping.
pub fn log_error<T, E: Display>(msg: &str) -> impl FnMut(In<Result<T, E>>) -> Result<T, E> {
    let msg = msg.to_owned();
    move |In(result): In<Result<T, E>>| {
        if let Err(e) = &result {
            error!("{}: {:#}", msg, e);
        }
        result
    }
}

/// Discard the return value of a Bevy system function.
pub fn fuse<T>(_: In<T>) {}
