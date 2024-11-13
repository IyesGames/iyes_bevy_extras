use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{PrimaryWindow, WindowRef};

use crate::prelude::*;

pub struct WorldCursorPlugin2d;

impl Plugin for WorldCursorPlugin2d {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            world_cursor_2d.in_set(SetStage::Provide(WorldCursorSS)),
        );
    }
}

fn world_cursor_2d(
    mut crs: ResMut<WorldCursor>,
    q_windows: Query<&Window>,
    q_primary_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<WorldCursorCamera>>,
) {
    let Ok((camera, xf_camera)) = q_camera.get_single() else {
        return;
    };
    let RenderTarget::Window(w_id) = camera.target else {
        panic!("Cursor camera must render to a window!");
    };
    let window = match w_id {
        WindowRef::Primary => q_primary_window.single(),
        WindowRef::Entity(e) => q_windows.get(e).unwrap(),
    };
    let Some(wpos) = window.cursor_position() else {
        return;
    };
    let Some(vrect) = camera.logical_viewport_rect() else {
        return;
    };
    let Some(vpos) = vrect.contains(wpos).then(|| wpos - vrect.min) else {
        return;
    };
    let Ok(cursor) = camera.viewport_to_world_2d(xf_camera, vpos) else {
        return;
    };
    if crs.pos == cursor && crs.pos_prev == cursor {
        return;
    }
    crs.pos_prev = crs.pos;
    crs.pos = cursor;
}
