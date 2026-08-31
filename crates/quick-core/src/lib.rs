pub mod event;
pub mod geometry;
pub mod signals;
pub mod telemetry;

#[cfg(feature = "silver")]
pub mod silver_bridge;

pub use event::*;
pub use geometry::*;
pub use signals::*;
pub use telemetry::*;

#[cfg(feature = "silver")]
pub use silver_bridge::*;

#[cfg(feature = "mimalloc")]
pub use mimalloc::MiMalloc;

/// Common prelude for Quick applications.
pub mod prelude {
    pub use crate::event::*;
    pub use crate::geometry::*;
    pub use crate::signals::*;
    pub use crate::telemetry::*;
    #[cfg(feature = "silver")]
    pub use crate::silver_bridge::*;
    pub use bumpalo::Bump;
}
