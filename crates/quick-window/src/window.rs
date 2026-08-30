use quick_core::geometry::Size;

#[derive(Debug, Clone)]
pub struct WindowOptions {
    pub title: String,
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            title: "Quick Application".to_string(),
            size: Size::new(800.0, 600.0),
            min_size: Some(Size::new(200.0, 150.0)),
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
        }
    }
}

impl WindowOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = Size::new(width, height);
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
}
