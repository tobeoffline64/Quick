#[cfg(feature = "skia")]
use crate::canvas::{Canvas, DrawCommand};

pub struct RenderPipeline;

impl RenderPipeline {
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "skia")]
    pub fn render_to_skia(&self, canvas_commands: &Canvas, skia_canvas: &mut skia_safe::Canvas) {
        use skia_safe::{Color4f, Paint, PaintStyle, Point as SkPoint, RRect, Rect as SkRect};

        for cmd in canvas_commands.commands() {
            match cmd {
                DrawCommand::Clear(c) => {
                    let c4f = Color4f::new(
                        c.r as f32 / 255.0,
                        c.g as f32 / 255.0,
                        c.b as f32 / 255.0,
                        c.a as f32 / 255.0,
                    );
                    skia_canvas.clear(c4f);
                }
                DrawCommand::FillRect(r, c) => {
                    let mut paint = Paint::default();
                    paint.set_style(PaintStyle::Fill);
                    paint.set_color4f(
                        Color4f::new(
                            c.r as f32 / 255.0,
                            c.g as f32 / 255.0,
                            c.b as f32 / 255.0,
                            c.a as f32 / 255.0,
                        ),
                        None,
                    );
                    let sk_rect = SkRect::from_xywh(r.origin.x, r.origin.y, r.size.width, r.size.height);
                    skia_canvas.draw_rect(sk_rect, &paint);
                }
                DrawCommand::StrokeRect(r, c, width) => {
                    let mut paint = Paint::default();
                    paint.set_style(PaintStyle::Stroke);
                    paint.set_stroke_width(*width);
                    paint.set_color4f(
                        Color4f::new(
                            c.r as f32 / 255.0,
                            c.g as f32 / 255.0,
                            c.b as f32 / 255.0,
                            c.a as f32 / 255.0,
                        ),
                        None,
                    );
                    let sk_rect = SkRect::from_xywh(r.origin.x, r.origin.y, r.size.width, r.size.height);
                    skia_canvas.draw_rect(sk_rect, &paint);
                }
                DrawCommand::FillRoundedRect(r, radius, c) => {
                    let mut paint = Paint::default();
                    paint.set_style(PaintStyle::Fill);
                    paint.set_anti_alias(true);
                    paint.set_color4f(
                        Color4f::new(
                            c.r as f32 / 255.0,
                            c.g as f32 / 255.0,
                            c.b as f32 / 255.0,
                            c.a as f32 / 255.0,
                        ),
                        None,
                    );
                    let sk_rect = SkRect::from_xywh(r.origin.x, r.origin.y, r.size.width, r.size.height);
                    let rrect = RRect::new_rect_radii(
                        sk_rect,
                        &[
                            skia_safe::Vector::new(radius.top_left, radius.top_left),
                            skia_safe::Vector::new(radius.top_right, radius.top_right),
                            skia_safe::Vector::new(radius.bottom_right, radius.bottom_right),
                            skia_safe::Vector::new(radius.bottom_left, radius.bottom_left),
                        ],
                    );
                    skia_canvas.draw_rrect(rrect, &paint);
                }
                DrawCommand::StrokeRoundedRect(r, radius, c, width) => {
                    let mut paint = Paint::default();
                    paint.set_style(PaintStyle::Stroke);
                    paint.set_stroke_width(*width);
                    paint.set_anti_alias(true);
                    paint.set_color4f(
                        Color4f::new(
                            c.r as f32 / 255.0,
                            c.g as f32 / 255.0,
                            c.b as f32 / 255.0,
                            c.a as f32 / 255.0,
                        ),
                        None,
                    );
                    let sk_rect = SkRect::from_xywh(r.origin.x, r.origin.y, r.size.width, r.size.height);
                    let rrect = RRect::new_rect_radii(
                        sk_rect,
                        &[
                            skia_safe::Vector::new(radius.top_left, radius.top_left),
                            skia_safe::Vector::new(radius.top_right, radius.top_right),
                            skia_safe::Vector::new(radius.bottom_right, radius.bottom_right),
                            skia_safe::Vector::new(radius.bottom_left, radius.bottom_left),
                        ],
                    );
                    skia_canvas.draw_rrect(rrect, &paint);
                }
                DrawCommand::DrawShadow { rect, radius, shadow } => {
                    use skia_safe::{BlurStyle, MaskFilter};
                    if shadow.color.a > 0 {
                        let mut paint = Paint::default();
                        paint.set_style(PaintStyle::Fill);
                        paint.set_anti_alias(true);
                        paint.set_color4f(
                            Color4f::new(
                                shadow.color.r as f32 / 255.0,
                                shadow.color.g as f32 / 255.0,
                                shadow.color.b as f32 / 255.0,
                                shadow.color.a as f32 / 255.0,
                            ),
                            None,
                        );

                        if shadow.blur_radius > 0.0 {
                            let sigma = shadow.blur_radius * 0.57735 + 0.5;
                            paint.set_mask_filter(MaskFilter::blur(BlurStyle::Normal, sigma, false));
                        }

                        let spread = shadow.spread_radius;
                        let sk_rect = SkRect::from_xywh(
                            rect.origin.x + shadow.offset_x - spread,
                            rect.origin.y + shadow.offset_y - spread,
                            rect.size.width + spread * 2.0,
                            rect.size.height + spread * 2.0,
                        );
                        let rrect = RRect::new_rect_radii(
                            sk_rect,
                            &[
                                skia_safe::Vector::new(radius.top_left, radius.top_left),
                                skia_safe::Vector::new(radius.top_right, radius.top_right),
                                skia_safe::Vector::new(radius.bottom_right, radius.bottom_right),
                                skia_safe::Vector::new(radius.bottom_left, radius.bottom_left),
                            ],
                        );
                        skia_canvas.draw_rrect(rrect, &paint);
                    }
                }
                DrawCommand::DrawText { text, origin, color, font_size, font_family: _ } => {
                    let mut paint = Paint::default();
                    paint.set_style(PaintStyle::Fill);
                    paint.set_anti_alias(true);
                    paint.set_color4f(
                        Color4f::new(
                            color.r as f32 / 255.0,
                            color.g as f32 / 255.0,
                            color.b as f32 / 255.0,
                            color.a as f32 / 255.0,
                        ),
                        None,
                    );
                    let font = skia_safe::Font::default().with_size(*font_size).unwrap_or_default();
                    skia_canvas.draw_str(text, SkPoint::new(origin.x, origin.y), &font, &paint);
                }
                DrawCommand::DrawLine { start, end, color, width } => {
                    let mut paint = Paint::default();
                    paint.set_style(PaintStyle::Stroke);
                    paint.set_stroke_width(*width);
                    paint.set_color4f(
                        Color4f::new(
                            color.r as f32 / 255.0,
                            color.g as f32 / 255.0,
                            color.b as f32 / 255.0,
                            color.a as f32 / 255.0,
                        ),
                        None,
                    );
                    skia_canvas.draw_line(
                        SkPoint::new(start.x, start.y),
                        SkPoint::new(end.x, end.y),
                        &paint,
                    );
                }
                DrawCommand::PushClip(r) => {
                    let sk_rect = SkRect::from_xywh(r.origin.x, r.origin.y, r.size.width, r.size.height);
                    skia_canvas.save();
                    skia_canvas.clip_rect(sk_rect, None, true);
                }
                DrawCommand::PopClip => {
                    skia_canvas.restore();
                }
                DrawCommand::Save => {
                    skia_canvas.save();
                }
                DrawCommand::Restore => {
                    skia_canvas.restore();
                }
                DrawCommand::Translate(dx, dy) => {
                    skia_canvas.translate((*dx, *dy));
                }
                DrawCommand::Scale(sx, sy) => {
                    skia_canvas.scale((*sx, *sy));
                }
            }
        }
    }
}
