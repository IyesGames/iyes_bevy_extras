use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowRef};
use bevy::render::camera::RenderTarget;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct WorldCursorSet;

pub struct WorldCursorPlugin;

impl Plugin for WorldCursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldCursor>();
        app.add_system(
            world_cursor.in_set(WorldCursorSet)
        );
    }
}

#[derive(Component)]
pub struct WorldCursorCamera;

#[derive(Resource, Default)]
pub struct WorldCursor {
    pub pos: Vec2,
}

fn world_cursor(
    mut crs: ResMut<WorldCursor>,
    q_windows: Query<&Window>,
    q_primary_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<WorldCursorCamera>>,
) {
    let (camera, xf_camera) = q_camera.single();
    let RenderTarget::Window(w_id) = camera.target
    else {
        panic!("Cursor camera must render to a window!");
    };
    let window = match w_id {
        WindowRef::Primary => q_primary_window.single(),
        WindowRef::Entity(e) => q_windows.get(e).unwrap(),
    };
    let Some(cursor) = window.cursor_position()
        .and_then(|pos| camera.viewport_to_world(xf_camera, pos))
        .map(|ray| ray.origin.truncate())
    else {
        return;
    };
    crs.pos = cursor;
}

