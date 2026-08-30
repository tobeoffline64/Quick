use crate::widget::Widget;
use quick_core::event::Event;
use quick_core::geometry::{BorderRadius, Color, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::Style;
use taffy::prelude::{NodeId, TaffyError};

pub struct Container {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub children: Vec<Box<dyn Widget>>,
    child_nodes: Vec<NodeId>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            id: None,
            classes: Vec::new(),
            style: Style::default(),
            children: Vec::new(),
            child_nodes: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn add_child(&mut self, child: impl Widget + 'static) {
        self.children.push(Box::new(child));
    }
}

impl Widget for Container {
    fn widget_type(&self) -> &'static str {
        "Container"
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn classes(&self) -> &[String] {
        &self.classes
    }

    fn style(&self) -> &Style {
        &self.style
    }

    fn style_mut(&mut self) -> &mut Style {
        &mut self.style
    }

    fn build_layout(&mut self, engine: &mut LayoutEngine) -> Result<NodeId, TaffyError> {
        self.child_nodes.clear();
        for child in &mut self.children {
            let child_node = child.build_layout(engine)?;
            self.child_nodes.push(child_node);
        }
        engine.new_with_children(&self.style, &self.child_nodes)
    }

    fn paint(&self, canvas: &mut Canvas, bounds: Rect) {
        if let Some(bg) = self.style.background_color {
            if let Some(radius) = self.style.border_radius {
                canvas.fill_rounded_rect(bounds, radius, bg);
            } else {
                canvas.fill_rect(bounds, bg);
            }
        }

        if let (Some(border_color), Some(border_width)) =
            (self.style.border_color, self.style.border_width)
        {
            if let Some(radius) = self.style.border_radius {
                canvas.stroke_rounded_rect(bounds, radius, border_color, border_width);
            } else {
                canvas.stroke_rect(bounds, border_color, border_width);
            }
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        for child in &mut self.children {
            if child.handle_event(event, bounds) {
                return true;
            }
        }
        false
    }
}
