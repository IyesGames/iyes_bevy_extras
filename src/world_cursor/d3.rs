use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{PrimaryWindow, WindowRef};

use crate::prelude::*;

pub struct WorldCursorPlugin3dGround;

impl Plugin for WorldCursorPlugin3dGround {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            world_cursor_3dground
                .run_if(on_event::<CursorMoved>)
                .run_if(any_filter::<(With<Camera3d>, With<WorldCursorCamera>)>)
                .run_if(any_with_component::<GroundPlane>)
                .in_set(SetStage::Provide(WorldCursorSS)),
        );
    }
}

#[derive(Component)]
pub struct GroundPlane;

fn world_cursor_3dground(
    mut crs: ResMut<WorldCursor>,
    q_windows: Query<&Window>,
    q_primary_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<WorldCursorCamera>>,
    xf_plane: Single<&GlobalTransform, With<GroundPlane>>,
) {
    let Ok((camera, xf_camera)) = q_camera.get_single() else {
        return;
    };
    let RenderTarget::Window(w_id) = camera.target else {
        error!("Cursor camera must render to a window!");
        return;
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
    let Ok(ray) = camera.viewport_to_world(xf_camera, vpos) else {
        return;
    };
    let plane_origin = xf_plane.translation();
    let plane = InfinitePlane3d::new(xf_plane.up());
    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        return;
    };
    let global_cursor = ray.get_point(distance);
    let inverse_transform_matrix = xf_plane.compute_matrix().inverse();
    let local_cursor = inverse_transform_matrix.transform_point3(global_cursor);
    let cursor = local_cursor.xz();
    if crs.pos == cursor && crs.pos_prev == cursor {
        return;
    }
    crs.pos_prev = crs.pos;
    crs.pos = cursor;
}
