pub mod app;

pub use app::*;

pub use quick_core as core;
pub use quick_layout as layout;
pub use quick_markup as markup;
pub use quick_render as render;
pub use quick_style as style;
pub use quick_widgets as widgets;
pub use quick_window as window;

pub mod prelude {
    pub use crate::app::*;
    pub use quick_core::prelude::*;
    pub use quick_layout::prelude::*;
    pub use quick_markup::prelude::*;
    pub use quick_render::prelude::*;
    pub use quick_style::prelude::*;
    pub use quick_widgets::prelude::*;
    pub use quick_window::prelude::*;
}
