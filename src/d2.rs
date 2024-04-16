use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowRef};
use bevy::render::camera::RenderTarget;

use crate::state::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct WorldCursorSS;

pub struct WorldCursorPlugin;

impl Plugin for WorldCursorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_stage_set(
            Update,
            WorldCursorSS,
            resource_changed::<WorldCursor>
        );
        app.init_resource::<WorldCursor>();
        app.add_systems(Update,
            world_cursor.in_set(SetStage::Provide(WorldCursorSS))
        );
    }
}

#[derive(Component)]
pub struct WorldCursorCamera;

#[derive(Resource, Default)]
pub struct WorldCursor {
    pub pos: Vec2,
    pub pos_prev: Vec2,
}

fn world_cursor(
    mut crs: ResMut<WorldCursor>,
    q_windows: Query<&Window>,
    q_primary_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<WorldCursorCamera>>,
) {
    let Ok((camera, xf_camera)) = q_camera.get_single() else {
        return;
    };
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
    if crs.pos == cursor && crs.pos_prev == cursor {
        return;
    }
    crs.pos_prev = crs.pos;
    crs.pos = cursor;
}

