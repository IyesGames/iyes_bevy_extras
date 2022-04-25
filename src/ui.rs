use bevy::prelude::*;

/// For disabling some UI elements
#[derive(Component)]
pub struct UiInactive;

pub fn init_camera(
    mut commands: Commands,
) {
    commands.spawn_bundle(UiCameraBundle::default());
}

/// Condition to help with handling multiple buttons
///
/// Returns true when a button identified by a given component is clicked.
pub fn on_butt_interact<B: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<B>, Without<UiInactive>)>,
) -> bool {
    for interaction in query.iter() {
        if *interaction == Interaction::Clicked {
            return true;
        }
    }

    false
}
