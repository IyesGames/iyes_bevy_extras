use bevy::{ecs::query::QueryFilter, prelude::*};

/// Run Condition to check if any entities match a given query filter
pub fn any_filter<F: QueryFilter>(q: Query<(), F>) -> bool {
    !q.is_empty()
}

/// Run Condition to check if no entities match a given query filter
pub fn none_filter<F: QueryFilter>(q: Query<(), F>) -> bool {
    q.is_empty()
}

/// Run Condition to check for entities where the given component was added
///
/// Note: due to lack of archetypal change detection, this function may be
/// a performance footgun. It needs to iterate all entities.
pub fn any_added_component<T: Component>(q: Query<(), Added<T>>) -> bool {
    !q.is_empty()
}

/// Run Condition to check for entities where the given component was changed
///
/// Note: due to lack of archetypal change detection, this function may be
/// a performance footgun. It needs to iterate all entities.
pub fn any_changed_component<T: Component>(q: Query<(), Changed<T>>) -> bool {
    !q.is_empty()
}
