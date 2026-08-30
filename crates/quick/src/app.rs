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
                    root.paint(&mut self.canvas, bounds);
                }
            }
        }

        &self.canvas
    }

    pub fn handle_event(&mut self, event: &quick_core::event::Event, window_size: Size) -> bool {
        if let Some(ref mut root) = self.root {
            let bounds = Rect::from_origin_size(quick_core::geometry::Point::ZERO, window_size);
            root.handle_event(event, bounds)
        } else {
            false
        }
    }
}
