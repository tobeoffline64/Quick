use quick_core::geometry::Rect;

/// Tracks damaged (dirty) areas of a window/surface to optimize Wayland frame repaints.
#[derive(Debug, Clone, Default)]
pub struct DamageTracker {
    damage_rect: Option<Rect>,
}

impl DamageTracker {
    pub fn new() -> Self {
        Self { damage_rect: None }
    }

    /// Mark an area of the canvas as dirty.
    pub fn add_damage(&mut self, rect: Rect) {
        if rect.size.is_empty() {
            return;
        }
        self.damage_rect = match self.damage_rect {
            Some(existing) => Some(existing.union(&rect)),
            None => Some(rect),
        };
    }

    /// Mark the entire surface as dirty (e.g. on resize).
    pub fn damage_all(&mut self, width: f32, height: f32) {
        self.damage_rect = Some(Rect::new(0.0, 0.0, width, height));
    }

    pub fn is_dirty(&self) -> bool {
        self.damage_rect.is_some()
    }

    pub fn take_damage(&mut self) -> Option<Rect> {
        self.damage_rect.take()
    }

    pub fn reset(&mut self) {
        self.damage_rect = None;
    }
}
