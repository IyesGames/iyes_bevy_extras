use bevy::prelude::*;

use crate::prelude::*;

#[cfg(feature = "2d")]
pub mod d2;

#[cfg(feature = "3d")]
pub mod d3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub struct WorldCursorSS;

pub struct WorldCursorPlugin;

impl Plugin for WorldCursorPlugin {
    fn build(&self, app: &mut App) {
        app.configure_stage_set(Update, WorldCursorSS, resource_changed::<WorldCursor>);
        app.init_resource::<WorldCursor>();
    }
}

#[derive(Component)]
pub struct WorldCursorCamera;

#[derive(Resource, Default)]
pub struct WorldCursor {
    pub pos: Vec2,
    pub pos_prev: Vec2,
}
