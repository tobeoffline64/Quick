use bumpalo::Bump;
use quick_core::geometry::{BorderRadius, Color, Point, Rect};
use quick_style::theme::tokens::{ElevationTokens, Shadow};

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Clear(Color),
    FillRect(Rect, Color),
    StrokeRect(Rect, Color, f32),
    FillRoundedRect(Rect, BorderRadius, Color),
    StrokeRoundedRect(Rect, BorderRadius, Color, f32),
    DrawShadow {
        rect: Rect,
        radius: BorderRadius,
        shadow: Shadow,
    },
    DrawText {
        text: String,
        origin: Point,
        color: Color,
        font_size: f32,
        font_family: Option<String>,
    },
    DrawLine {
        start: Point,
        end: Point,
        color: Color,
        width: f32,
    },
    PushClip(Rect),
    PopClip,
    Save,
    Restore,
    Translate(f32, f32),
    Scale(f32, f32),
}

/// 2D Canvas recording display commands for rendering via Skia or software rasterizer.
/// Supports both vector command lists and ephemeral frame bump arena allocation.
#[derive(Debug, Default)]
pub struct Canvas {
    commands: Vec<DrawCommand>,
    frame_arena: Bump,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            commands: Vec::with_capacity(128),
            frame_arena: Bump::with_capacity(16 * 1024),
        }
    }

    /// Access the per-frame bump allocator for temporary allocations
    pub fn arena(&self) -> &Bump {
        &self.frame_arena
    }

    pub fn clear(&mut self, color: Color) {
        self.commands.push(DrawCommand::Clear(color));
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        self.commands.push(DrawCommand::FillRect(rect, color));
    }

    pub fn stroke_rect(&mut self, rect: Rect, color: Color, width: f32) {
        self.commands.push(DrawCommand::StrokeRect(rect, color, width));
    }

    pub fn fill_rounded_rect(&mut self, rect: Rect, radius: BorderRadius, color: Color) {
        self.commands.push(DrawCommand::FillRoundedRect(rect, radius, color));
    }

    pub fn stroke_rounded_rect(&mut self, rect: Rect, radius: BorderRadius, color: Color, width: f32) {
        self.commands.push(DrawCommand::StrokeRoundedRect(rect, radius, color, width));
    }

    /// Records a single box shadow layer
    pub fn draw_shadow(&mut self, rect: Rect, radius: BorderRadius, shadow: Shadow) {
        self.commands.push(DrawCommand::DrawShadow { rect, radius, shadow });
    }

    /// Records dual-pass elevation shadow (key + ambient)
    pub fn draw_elevation_shadow(
        &mut self,
        rect: Rect,
        radius: BorderRadius,
        level: u8,
        elevation_tokens: &ElevationTokens,
    ) {
        if level == 0 {
            return;
        }
        let elev = elevation_tokens.get(level);
        if let Some(ambient) = elev.ambient_shadow {
            self.draw_shadow(rect, radius, ambient);
        }
        if let Some(key) = elev.key_shadow {
            self.draw_shadow(rect, radius, key);
        }
    }

    /// Helper to fill rounded rect with dynamic surface tint overlay
    pub fn fill_surface_tint(
        &mut self,
        rect: Rect,
        radius: BorderRadius,
        base_color: Color,
        tint_color: Color,
        opacity: f32,
    ) {
        let opacity_clamped = if opacity.is_nan() { 0.0 } else { opacity.clamp(0.0, 1.0) };
        let r = (base_color.r as f32 * (1.0 - opacity_clamped) + tint_color.r as f32 * opacity_clamped).round() as u8;
        let g = (base_color.g as f32 * (1.0 - opacity_clamped) + tint_color.g as f32 * opacity_clamped).round() as u8;
        let b = (base_color.b as f32 * (1.0 - opacity_clamped) + tint_color.b as f32 * opacity_clamped).round() as u8;
        let final_color = Color::from_rgba(r, g, b, base_color.a);
        self.fill_rounded_rect(rect, radius, final_color);
    }

    pub fn draw_text(
        &mut self,
        text: impl Into<String>,
        origin: Point,
        color: Color,
        font_size: f32,
        font_family: Option<String>,
    ) {
        self.commands.push(DrawCommand::DrawText {
            text: text.into(),
            origin,
            color,
            font_size,
            font_family,
        });
    }

    pub fn draw_line(&mut self, start: Point, end: Point, color: Color, width: f32) {
        self.commands.push(DrawCommand::DrawLine { start, end, color, width });
    }

    pub fn push_clip(&mut self, rect: Rect) {
        self.commands.push(DrawCommand::PushClip(rect));
    }

    pub fn pop_clip(&mut self) {
        self.commands.push(DrawCommand::PopClip);
    }

    pub fn save(&mut self) {
        self.commands.push(DrawCommand::Save);
    }

    pub fn restore(&mut self) {
        self.commands.push(DrawCommand::Restore);
    }

    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.commands.push(DrawCommand::Translate(dx, dy));
    }

    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.commands.push(DrawCommand::Scale(sx, sy));
    }

    pub fn commands(&self) -> &[DrawCommand] {
        &self.commands
    }

    /// Reset command list and frame arena in O(1) time
    pub fn reset(&mut self) {
        self.commands.clear();
        self.frame_arena.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_and_bump_arena() {
        let mut canvas = Canvas::new();
        canvas.clear(Color::BLACK);
        canvas.fill_rect(Rect::new(0.0, 0.0, 100.0, 100.0), Color::RED);

        assert_eq!(canvas.commands().len(), 2);

        // Test arena allocation
        let s = canvas.arena().alloc_str("temporary per-frame string");
        assert_eq!(s, "temporary per-frame string");

        canvas.reset();
        assert_eq!(canvas.commands().len(), 0);
    }

    #[test]
    fn test_canvas_elevation_shadow_and_surface_tint() {
        let mut canvas = Canvas::new();
        let tokens = ElevationTokens::default();

        // Level 0 should not produce shadow commands
        canvas.draw_elevation_shadow(Rect::new(0.0, 0.0, 100.0, 100.0), BorderRadius::all(16.0), 0, &tokens);
        assert_eq!(canvas.commands().len(), 0);

        // Level 1 produces 2 shadow commands (ambient + key)
        canvas.draw_elevation_shadow(Rect::new(0.0, 0.0, 100.0, 100.0), BorderRadius::all(16.0), 1, &tokens);
        assert_eq!(canvas.commands().len(), 2);

        // Fill surface tint
        canvas.fill_surface_tint(
            Rect::new(0.0, 0.0, 100.0, 100.0),
            BorderRadius::all(16.0),
            Color::from_rgb(30, 30, 30),
            Color::from_rgb(103, 80, 164),
            0.08,
        );
        assert_eq!(canvas.commands().len(), 3);
    }
}
