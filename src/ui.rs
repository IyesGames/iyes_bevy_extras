use bevy::prelude::*;
use crate::IntoChainResultSystem;

/// For disabling some UI elements
#[derive(Component)]
pub struct UiInactive;

/// Helper for adding a button handler system
pub fn butt_handler<B: Component + Clone, Params>(handler: impl IntoSystem<B, (), Params>) -> impl System<In = (), Out = ()> {
    on_butt_interact.chain_result(handler, move |_: In<()>| {})
}

/// Condition to help with handling multiple buttons
///
/// Returns true when a button identified by a given component is clicked.
fn on_butt_interact<B: Component + Clone>(
    query: Query<(&Interaction, &B), (Changed<Interaction>, With<Button>, Without<UiInactive>)>,
) -> Result<B, ()> {
    for (interaction, b) in query.iter() {
        if *interaction == Interaction::Clicked {
            return Ok(b.clone());
        }
    }
    Err(())
}
