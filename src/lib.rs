//! Iyes's crate of misc random bevy helpers and stuff
//!
//! Throw everything here until you figure out a better place to put it. :)

pub mod prelude {
    pub use crate::ecs::*;
    #[cfg(any(feature = "2d", feature = "3d"))]
    pub use crate::world_cursor::*;
}

pub mod ecs;
#[cfg(any(feature = "2d", feature = "3d"))]
pub mod world_cursor;
