use quick_core::geometry::{Color, Rect, Size};
use quick_layout::engine::LayoutEngine;
use quick_markup::builder::{build_ui_tree, DataContext};
use quick_markup::quick_parser::{parse_quick, parse_quick_file};
use quick_markup::toml_parser::parse_toml;
use quick_markup::xml_parser::parse_xml;
use quick_render::canvas::Canvas;
use quick_render::damage::DamageTracker;
use quick_style::rule::StyleSheet;
use quick_widgets::widget::Widget;
use quick_window::runner::{AppController, WindowRunner};
use quick_window::window::WindowOptions;
use std::path::Path;

pub struct App {
    window_options: WindowOptions,
    root: Option<Box<dyn Widget>>,
    stylesheet: StyleSheet,
    layout_engine: LayoutEngine,
    canvas: Canvas,
    damage_tracker: DamageTracker,
}

impl App {
    pub fn new(options: WindowOptions) -> Self {
        Self {
            window_options: options,
            root: None,
            stylesheet: StyleSheet::new(),
            layout_engine: LayoutEngine::new(),
            canvas: Canvas::new(),
            damage_tracker: DamageTracker::new(),
        }
    }

    pub fn window_options(&self) -> &WindowOptions {
        &self.window_options
    }

    pub fn damage_tracker(&self) -> &DamageTracker {
        &self.damage_tracker
    }

    pub fn damage_tracker_mut(&mut self) -> &mut DamageTracker {
        &mut self.damage_tracker
    }

    pub fn with_root(mut self, root: impl Widget + 'static) -> Self {
        self.root = Some(Box::new(root));
        self
    }

    /// Load and instantiate UI from a `.quick` file content string.
    pub fn from_quick(mut self, quick_content: &str, data_ctx: &mut DataContext) -> Result<Self, String> {
        let doc = parse_quick(quick_content)?;
        let (root_widget, stylesheet) = build_ui_tree(&doc, data_ctx);
        self.root = Some(root_widget);
        self.stylesheet = stylesheet;
        Ok(self)
    }

    /// Load and instantiate UI directly from a `.quick` file path on disk.
    pub fn from_quick_file(mut self, path: impl AsRef<Path>, data_ctx: &mut DataContext) -> Result<Self, String> {
        let doc = parse_quick_file(path)?;
        let (root_widget, stylesheet) = build_ui_tree(&doc, data_ctx);
        self.root = Some(root_widget);
        self.stylesheet = stylesheet;
        Ok(self)
    }

    pub fn from_xml(mut self, xml: &str, data_ctx: &mut DataContext) -> Result<Self, String> {
        let doc = parse_xml(xml)?;
        let (root_widget, stylesheet) = build_ui_tree(&doc, data_ctx);
        self.root = Some(root_widget);
        self.stylesheet = stylesheet;
        Ok(self)
    }

    pub fn from_toml(mut self, toml_str: &str, data_ctx: &mut DataContext) -> Result<Self, String> {
        let doc = parse_toml(toml_str)?;
        let (root_widget, stylesheet) = build_ui_tree(&doc, data_ctx);
        self.root = Some(root_widget);
        self.stylesheet = stylesheet;
        Ok(self)
    }

    /// Run a layout & paint cycle on the widget tree for the given window size.
    pub fn render_frame(&mut self, window_size: Size) -> &Canvas {
        self.canvas.reset();
        self.canvas.clear(Color::from_hex("#11111b").unwrap_or(Color::BLACK));

        if let Some(ref mut root) = self.root {
            self.layout_engine.reset();
            if let Ok(root_node) = root.build_layout(&mut self.layout_engine) {
                let _ = self.layout_engine.compute_layout(root_node, window_size);
                if let Ok(bounds) = self.layout_engine.get_layout(root_node) {
                    root.update_layout(&self.layout_engine, bounds.origin);
                    root.paint(&mut self.canvas, bounds);
                }
            }
        }

        &self.canvas
    }

    pub fn handle_event(&mut self, event: &quick_core::event::Event, window_size: Size) -> bool {
        if let Some(ref mut root) = self.root {
            self.layout_engine.reset();
            if let Ok(root_node) = root.build_layout(&mut self.layout_engine) {
                let _ = self.layout_engine.compute_layout(root_node, window_size);
                if let Ok(bounds) = self.layout_engine.get_layout(root_node) {
                    root.update_layout(&self.layout_engine, bounds.origin);
                    return root.handle_event(event, bounds);
                }
            }
            let bounds = Rect::from_origin_size(quick_core::geometry::Point::ZERO, window_size);
            root.handle_event(event, bounds)
        } else {
            false
        }
    }

    /// Launch the interactive Wayland/X11 desktop window and run the UI event loop.
    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let options = self.window_options.clone();
        let runner = WindowRunner::new(options, self);
        runner.run()
    }
}

impl AppController for App {
    fn render_frame(&mut self, size: Size) -> &Canvas {
        self.render_frame(size)
    }

    fn handle_event(&mut self, event: &quick_core::event::Event, size: Size) -> bool {
        self.handle_event(event, size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_from_quick_and_render() {
        let quick_doc = r#"
<VStack style="background: #1e1e2e; padding: 20px;">
    <Text id="label" text="Hello Quick!" />
    <Button id="btn" text="Click me" />
</VStack>
"#;
        let mut ctx = DataContext::new();
        let mut app = App::new(WindowOptions::new().title("Test App"))
            .from_quick(quick_doc, &mut ctx)
            .unwrap();

        let canvas = app.render_frame(Size::new(400.0, 300.0));
        assert!(canvas.commands().len() >= 4);
    }

    #[test]
    fn test_app_interactive_click_and_rerender() {
        use quick_core::event::{PointerButton, PointerEvent, PointerPhase};
        use quick_core::geometry::Point;
        use quick_core::signals::{create_computed, Signal};

        let counter = Signal::new(0);
        let counter_sig = counter.clone();
        let greeting = create_computed(move || format!("Clicks: {}", counter_sig.get()));

        let mut ctx = DataContext::new();
        ctx.bind_signal("greeting", greeting.clone());

        let inc = counter.clone();
        ctx.bind_action("increment", move || {
            inc.update(|v| *v += 1);
        });

        let quick_doc = r#"
<VStack style="background: #0d1117; width: 400px; height: 300px; padding: 20px; align-items: center;">
    <Text id="label" text="$greeting" />
    <Button id="btn-inc" text="Click" onclick="increment" />
</VStack>
"#;
        let mut app = App::new(WindowOptions::new().title("Interactive App"))
            .from_quick(quick_doc, &mut ctx)
            .unwrap();

        let window_size = Size::new(400.0, 300.0);
        let canvas_1 = app.render_frame(window_size);
        assert!(canvas_1.commands().len() >= 4);
        assert_eq!(greeting.get(), "Clicks: 0");

        // Click on the button
        let down = quick_core::event::Event::Pointer(PointerEvent {
            position: Point::new(200.0, 55.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Down,
            modifiers: Default::default(),
        });
        assert!(app.handle_event(&down, window_size));

        let up = quick_core::event::Event::Pointer(PointerEvent {
            position: Point::new(200.0, 55.0),
            button: Some(PointerButton::Primary),
            phase: PointerPhase::Up,
            modifiers: Default::default(),
        });
        assert!(app.handle_event(&up, window_size));

        assert_eq!(greeting.get(), "Clicks: 1");

        let canvas_2 = app.render_frame(Size::new(400.0, 300.0));
        assert!(canvas_2.commands().len() >= 4);
    }

    #[test]
    fn test_app_from_xml_and_toml() {
        let xml_doc = r#"
<VStack style="background: #11111b; padding: 16px;">
    <Text text="XML Hello" />
    <Button text="Click" />
</VStack>
"#;
        let mut ctx = DataContext::new();
        let mut app_xml = App::new(WindowOptions::new().title("XML App"))
            .from_xml(xml_doc, &mut ctx)
            .unwrap();
        let c1 = app_xml.render_frame(Size::new(300.0, 200.0));
        assert!(!c1.commands().is_empty());

        let toml_doc = r#"
[root]
type = "VStack"
style = "padding: 16px;"

[[root.children]]
type = "Text"
text = "TOML Hello"
"#;
        let mut app_toml = App::new(WindowOptions::new().title("TOML App"))
            .from_toml(toml_doc, &mut ctx)
            .unwrap();
        let c2 = app_toml.render_frame(Size::new(300.0, 200.0));
        assert!(!c2.commands().is_empty());

        app_toml.damage_tracker_mut().add_damage(Rect::new(0.0, 0.0, 100.0, 100.0));
        assert!(app_toml.damage_tracker().is_dirty());
    }
}

