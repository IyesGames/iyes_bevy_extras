//! Iyes's crate of misc random bevy helpers and stuff
//!
//! Throw everything here until you figure out a better place to put it. :)

pub mod prelude {
    pub use crate::cleanup::*;
    pub use crate::system::*;
    #[cfg(feature = "2d")]
    pub use crate::d2::*;
}

pub mod cleanup;
pub mod system;
#[cfg(feature = "2d")]
pub mod d2;
