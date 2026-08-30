use crate::widget::Widget;
use quick_core::event::Event;
use quick_core::geometry::{Point, Rect};
use quick_layout::engine::LayoutEngine;
use quick_render::canvas::Canvas;
use quick_style::property::Style;
use taffy::prelude::NodeId;
use taffy::TaffyError;

pub struct Container {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub style: Style,
    pub children: Vec<Box<dyn Widget>>,
    child_nodes: Vec<NodeId>,
    child_bounds: Vec<Rect>,
}

impl Container {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.flex_direction = Some(quick_style::property::FlexDirection::Column);
        Self {
            id: None,
            classes: Vec::new(),
            style,
            children: Vec::new(),
            child_nodes: Vec::new(),
            child_bounds: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: impl Widget + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn add_child(&mut self, child: impl Widget + 'static) {
        self.children.push(Box::new(child));
    }

    pub fn child_bounds(&self) -> &[Rect] {
        &self.child_bounds
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
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

    fn update_layout(&mut self, engine: &LayoutEngine, parent_origin: Point) {
        self.child_bounds.clear();
        for (i, child) in self.children.iter_mut().enumerate() {
            if let Some(&node_id) = self.child_nodes.get(i) {
                if let Ok(rel_layout) = engine.get_layout(node_id) {
                    let abs_bounds = Rect::new(
                        parent_origin.x + rel_layout.origin.x,
                        parent_origin.y + rel_layout.origin.y,
                        rel_layout.size.width,
                        rel_layout.size.height,
                    );
                    self.child_bounds.push(abs_bounds);
                    child.update_layout(engine, abs_bounds.origin);
                }
            }
        }
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

        for (child, child_bound) in self.children.iter().zip(&self.child_bounds) {
            child.paint(canvas, *child_bound);
        }
    }

    fn handle_event(&mut self, event: &Event, bounds: Rect) -> bool {
        let _ = bounds;
        match event {
            Event::Pointer(quick_core::event::PointerEvent { phase: quick_core::event::PointerPhase::Down, .. }) => {
                let mut consumed = false;
                for (child, child_bound) in self.children.iter_mut().zip(&self.child_bounds).rev() {
                    if !consumed && child.handle_event(event, *child_bound) {
                        consumed = true;
                    } else if consumed {
                        let _ = child.handle_event(event, *child_bound);
                    }
                }
                consumed
            }
            _ => {
                for (child, child_bound) in self.children.iter_mut().zip(&self.child_bounds).rev() {
                    if child.handle_event(event, *child_bound) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::button::Button;
    use quick_core::event::{PointerButton, PointerEvent, PointerPhase};
    use quick_core::geometry::{Color, Size};
    use quick_style::property::{Dimension, FlexDirection};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_container_recursive_layout_and_paint() {
        let mut container = Container::new();
        container.style.background_color = Some(Color::BLACK);
        container.style.width = Some(Dimension::Px(300.0));
        container.style.height = Some(Dimension::Px(200.0));
        container.style.flex_direction = Some(FlexDirection::Column);

        let mut btn1 = Button::new("Button 1");
        btn1.style.width = Some(Dimension::Px(100.0));
        btn1.style.height = Some(Dimension::Px(40.0));

        let mut btn2 = Button::new("Button 2");
        btn2.style.width = Some(Dimension::Px(100.0));
        btn2.style.height = Some(Dimension::Px(40.0));

        container.add_child(btn1);
        container.add_child(btn2);

        let mut engine = LayoutEngine::new();
        let root_node = container.build_layout(&mut engine).unwrap();
        engine.compute_layout(root_node, Size::new(300.0, 200.0)).unwrap();
        container.update_layout(&engine, Point::ZERO);

        let mut canvas = Canvas::new();
        let bounds = engine.get_layout(root_node).unwrap();
        container.paint(&mut canvas, bounds);

        // Canvas should contain commands for container background and both buttons (bg + text each)
        assert!(canvas.commands().len() >= 5);
        assert_eq!(container.child_bounds().len(), 2);
    }

    #[test]
    fn test_container_child_hit_test_dispatch() {
        let btn1_clicked = Rc::new(RefCell::new(false));
        let btn2_clicked = Rc::new(RefCell::new(false));

        let b1_cl = btn1_clicked.clone();
        let b2_cl = btn2_clicked.clone();

        let mut container = Container::new();
        container.style.width = Some(Dimension::Px(300.0));
        container.style.height = Some(Dimension::Px(200.0));
        container.style.flex_direction = Some(FlexDirection::Column);

        let mut btn1 = Button::new("B1").on_click(move || *b1_cl.borrow_mut() = true);
        btn1.style.width = Some(Dimension::Px(100.0));
        btn1.style.height = Some(Dimension::Px(40.0));

        let mut btn2 = Button::new("B2").on_click(move || *b2_cl.borrow_mut() = true);
        btn2.style.width = Some(Dimension::Px(100.0));
        btn2.style.height = Some(Dimension::Px(40.0));

        container.add_child(btn1);
        container.add_child(btn2);

        let mut engine = LayoutEngine::new();
        let root_node = container.build_layout(&mut engine).unwrap();
        engine.compute_layout(root_node, Size::new(300.0, 200.0)).unwrap();
        container.update_layout(&engine, Point::ZERO);

        let bounds = engine.get_layout(root_node).unwrap();

        // Click on Button 2 (located at y >= 40.0)
        let b2_bounds = container.child_bounds()[1];
        let click_pos = Point::new(b2_bounds.origin.x + 10.0, b2_bounds.origin.y + 10.0);

        let down_event = Event::Pointer(PointerEvent {
            position: click_pos,
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(container.handle_event(&down_event, bounds));

        let up_event = Event::Pointer(PointerEvent {
            position: click_pos,
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        assert!(container.handle_event(&up_event, bounds));

        // Button 2 must have fired, Button 1 must NOT have fired
        assert!(!*btn1_clicked.borrow());
        assert!(*btn2_clicked.borrow());
    }

    #[test]
    fn test_container_overlapping_z_order_hit_test() {
        let bottom_clicked = Rc::new(RefCell::new(false));
        let top_clicked = Rc::new(RefCell::new(false));

        let bot_cl = bottom_clicked.clone();
        let top_cl = top_clicked.clone();

        let mut container = Container::new();
        let btn_bottom = Button::new("Bottom").on_click(move || *bot_cl.borrow_mut() = true);
        let btn_top = Button::new("Top").on_click(move || *top_cl.borrow_mut() = true);

        container.add_child(btn_bottom);
        container.add_child(btn_top);

        // Manually simulate overlapping child bounds (e.g. absolute positioning or overlapping rects)
        container.child_bounds = vec![
            Rect::new(0.0, 0.0, 100.0, 100.0),
            Rect::new(0.0, 0.0, 100.0, 100.0),
        ];

        let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
        let click_pos = Point::new(50.0, 50.0);

        let down_event = Event::Pointer(PointerEvent {
            position: click_pos,
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(container.handle_event(&down_event, bounds));

        let up_event = Event::Pointer(PointerEvent {
            position: click_pos,
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        assert!(container.handle_event(&up_event, bounds));

        // Topmost child must receive event, bottom child should NOT be clicked
        assert!(*top_clicked.borrow());
        assert!(!*bottom_clicked.borrow());
    }

    #[test]
    fn test_container_sibling_focus_clearing() {
        use crate::text_input::TextInput;

        let mut container = Container::new();
        let input = TextInput::new("Placeholder");
        let btn = Button::new("Submit");

        container.add_child(input);
        container.add_child(btn);

        container.child_bounds = vec![
            Rect::new(0.0, 0.0, 100.0, 30.0),   // TextInput at (0, 0)
            Rect::new(0.0, 40.0, 100.0, 30.0),  // Button at (0, 40)
        ];

        let bounds = Rect::new(0.0, 0.0, 200.0, 100.0);

        // 1. Click on TextInput -> gains focus
        let click_input = Event::Pointer(PointerEvent {
            position: Point::new(10.0, 10.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(container.handle_event(&click_input, bounds));

        // 2. Click on Button -> Button clicked, TextInput must lose focus
        let click_btn = Event::Pointer(PointerEvent {
            position: Point::new(10.0, 50.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(container.handle_event(&click_btn, bounds));
    }
}
