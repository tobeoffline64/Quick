pub mod event;
pub mod geometry;
pub mod signals;
pub mod telemetry;

pub use event::*;
pub use geometry::*;
pub use signals::*;
pub use telemetry::*;

#[cfg(feature = "mimalloc")]
pub use mimalloc::MiMalloc;

/// Common prelude for Quick applications.
pub mod prelude {
    pub use crate::event::*;
    pub use crate::geometry::*;
    pub use crate::signals::*;
    pub use crate::telemetry::*;
    pub use bumpalo::Bump;
}
