//! Iyes's crate of misc random bevy helpers and stuff
//!
//! Throw everything here until you figure out a better place to put it. :)

pub mod prelude {
    pub use crate::cleanup::*;
    pub use crate::system::{IntoChainResultSystem, IntoChainOptionalSystem};
}

#[cfg(feature = "bevy_ui")]
pub mod ui;
#[cfg(feature = "2d")]
pub mod d2;

pub mod cleanup;
pub mod system;
