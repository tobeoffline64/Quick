use bumpalo::Bump;
use quick_core::geometry::{BorderRadius, Color, Point, Rect};

#[derive(Debug, Clone)]
pub enum DrawCommand {
    Clear(Color),
    FillRect(Rect, Color),
    StrokeRect(Rect, Color, f32),
    FillRoundedRect(Rect, BorderRadius, Color),
    StrokeRoundedRect(Rect, BorderRadius, Color, f32),
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
