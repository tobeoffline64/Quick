pub mod event;
pub mod geometry;
pub mod signals;

pub use event::*;
pub use geometry::*;
pub use signals::*;

#[cfg(feature = "mimalloc")]
pub use mimalloc::MiMalloc;

/// Common prelude for Quick applications.
pub mod prelude {
    pub use crate::event::*;
    pub use crate::geometry::*;
    pub use crate::signals::*;
    pub use bumpalo::Bump;
}
