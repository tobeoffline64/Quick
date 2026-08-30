use quick_core::event::Event;
use quick_core::geometry::{Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::Style;
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub trait Widget {
    fn widget_type(&self) -> &'static str;
    fn id(&self) -> Option<&str> {
        None
    }
    fn classes(&self) -> &[String] {
        &[]
    }
    fn style(&self) -> &Style;
    fn style_mut(&mut self) -> &mut Style;

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError>;
    fn update_layout(&mut self, engine: &LayoutEngine, parent_origin: Point) {
        let _ = (engine, parent_origin);
    }
    fn paint(&self, canvas: &mut Canvas, bounds: Rect);
    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        let _ = (event, bounds);
        false
    }
}
