use crate::canvas::{Canvas, DrawCommand};
use fontdue::{Font as FFont, FontSettings};
use std::sync::{Arc, OnceLock};
use vello::kurbo::{Affine, Line, Point as KPoint, Rect as KRect, RoundedRect, RoundedRectRadii, Stroke};
use vello::peniko::{Blob, BlendMode, Color as PColor, Fill, Font as VFont};
use vello::{Glyph, Scene};

static VELLO_FONT: OnceLock<VFont> = OnceLock::new();
static FONTDUE_FONT: OnceLock<FFont> = OnceLock::new();

fn get_default_vello_font() -> &'static VFont {
    VELLO_FONT.get_or_init(|| {
        VFont::new(Blob::new(Arc::new(quick_style::fonts::INTER_REGULAR.to_vec())), 0)
    })
}

fn get_default_fontdue_font() -> &'static FFont {
    FONTDUE_FONT.get_or_init(|| {
        let settings = FontSettings {
            scale: 40.0,
            ..FontSettings::default()
        };
        FFont::from_bytes(quick_style::fonts::INTER_REGULAR, settings)
            .expect("Failed to load Inter font for Vello")
    })
}

pub struct VelloSceneBuilder;

impl VelloSceneBuilder {
    pub fn build(canvas: &Canvas, scene: &mut Scene, scale_factor: f32) {
        let root_scale = if scale_factor > 0.0 { scale_factor as f64 } else { 1.0 };
        let mut transform_stack = vec![Affine::scale(root_scale)];
        let v_font = get_default_vello_font();
        let f_font = get_default_fontdue_font();

        for cmd in canvas.commands() {
            let current_transform = *transform_stack.last().unwrap_or(&Affine::IDENTITY);

            match cmd {
                DrawCommand::Clear(c) => {
                    let p_color = PColor::rgba8(c.r, c.g, c.b, c.a);
                    let big_rect = KRect::new(-10000.0, -10000.0, 10000.0, 10000.0);
                    scene.fill(Fill::NonZero, Affine::IDENTITY, p_color, None, &big_rect);
                }
                DrawCommand::FillRect(r, c) => {
                    let p_color = PColor::rgba8(c.r, c.g, c.b, c.a);
                    let k_rect = KRect::new(r.min_x() as f64, r.min_y() as f64, r.max_x() as f64, r.max_y() as f64);
                    scene.fill(Fill::NonZero, current_transform, p_color, None, &k_rect);
                }
                DrawCommand::StrokeRect(r, c, w) => {
                    let p_color = PColor::rgba8(c.r, c.g, c.b, c.a);
                    let k_rect = KRect::new(r.min_x() as f64, r.min_y() as f64, r.max_x() as f64, r.max_y() as f64);
                    let stroke = Stroke::new(*w as f64);
                    scene.stroke(&stroke, current_transform, p_color, None, &k_rect);
                }
                DrawCommand::FillRoundedRect(r, rad, c) => {
                    let p_color = PColor::rgba8(c.r, c.g, c.b, c.a);
                    let k_rect = KRect::new(r.min_x() as f64, r.min_y() as f64, r.max_x() as f64, r.max_y() as f64);
                    let radii = RoundedRectRadii::new(
                        rad.top_left as f64,
                        rad.top_right as f64,
                        rad.bottom_right as f64,
                        rad.bottom_left as f64,
                    );
                    let rounded = RoundedRect::from_rect(k_rect, radii);
                    scene.fill(Fill::NonZero, current_transform, p_color, None, &rounded);
                }
                DrawCommand::StrokeRoundedRect(r, rad, c, w) => {
                    let p_color = PColor::rgba8(c.r, c.g, c.b, c.a);
                    let k_rect = KRect::new(r.min_x() as f64, r.min_y() as f64, r.max_x() as f64, r.max_y() as f64);
                    let radii = RoundedRectRadii::new(
                        rad.top_left as f64,
                        rad.top_right as f64,
                        rad.bottom_right as f64,
                        rad.bottom_left as f64,
                    );
                    let rounded = RoundedRect::from_rect(k_rect, radii);
                    let stroke = Stroke::new(*w as f64);
                    scene.stroke(&stroke, current_transform, p_color, None, &rounded);
                }
                DrawCommand::DrawShadow { rect, radius, shadow } => {
                    if shadow.color.a == 0 { continue; }
                    let p_color = PColor::rgba8(shadow.color.r, shadow.color.g, shadow.color.b, (shadow.color.a as f32 * 0.7) as u8);
                    let s_rect = KRect::new(
                        (rect.min_x() + shadow.offset_x - shadow.spread_radius) as f64,
                        (rect.min_y() + shadow.offset_y - shadow.spread_radius) as f64,
                        (rect.max_x() + shadow.offset_x + shadow.spread_radius) as f64,
                        (rect.max_y() + shadow.offset_y + shadow.spread_radius) as f64,
                    );
                    let radii = RoundedRectRadii::new(
                        radius.top_left as f64 + shadow.spread_radius as f64,
                        radius.top_right as f64 + shadow.spread_radius as f64,
                        radius.bottom_right as f64 + shadow.spread_radius as f64,
                        radius.bottom_left as f64 + shadow.spread_radius as f64,
                    );
                    let rounded = RoundedRect::from_rect(s_rect, radii);
                    scene.fill(Fill::NonZero, current_transform, p_color, None, &rounded);
                }
                DrawCommand::DrawLine { start, end, color, width } => {
                    let p_color = PColor::rgba8(color.r, color.g, color.b, color.a);
                    let line = Line::new(
                        KPoint::new(start.x as f64, start.y as f64),
                        KPoint::new(end.x as f64, end.y as f64),
                    );
                    let stroke = Stroke::new(*width as f64);
                    scene.stroke(&stroke, current_transform, p_color, None, &line);
                }
                DrawCommand::PushClip(r) => {
                    let k_rect = KRect::new(r.min_x() as f64, r.min_y() as f64, r.max_x() as f64, r.max_y() as f64);
                    scene.push_layer(BlendMode::default(), 1.0, current_transform, &k_rect);
                }
                DrawCommand::PopClip => {
                    scene.pop_layer();
                }
                DrawCommand::Translate(dx, dy) => {
                    if let Some(top) = transform_stack.last_mut() {
                        *top = top.then_translate((*dx as f64, *dy as f64).into());
                    }
                }
                DrawCommand::Scale(sx, sy) => {
                    if let Some(top) = transform_stack.last_mut() {
                        *top = top.then_scale_non_uniform(*sx as f64, *sy as f64);
                    }
                }
                DrawCommand::Save => {
                    let top = *transform_stack.last().unwrap_or(&Affine::IDENTITY);
                    transform_stack.push(top);
                }
                DrawCommand::Restore => {
                    if transform_stack.len() > 1 {
                        transform_stack.pop();
                    }
                }
                DrawCommand::DrawText { text, origin, color, font_size, .. } => {
                    let p_color = PColor::rgba8(color.r, color.g, color.b, color.a);
                    let scale = *font_size;
                    let line_height = scale * 1.35;
                    let mut cur_x = origin.x;
                    let mut cur_y = origin.y;
                    let start_x = cur_x;
                    let mut glyphs = Vec::with_capacity(text.len());

                    for ch in text.chars() {
                        if ch == '\n' {
                            cur_x = start_x;
                            cur_y += line_height;
                            continue;
                        }

                        let glyph_idx = f_font.lookup_glyph_index(ch);
                        let metrics = f_font.metrics(ch, scale);
                        if glyph_idx > 0 {
                            glyphs.push(Glyph {
                                id: glyph_idx as u32,
                                x: cur_x,
                                y: cur_y,
                            });
                            cur_x += metrics.advance_width;
                        } else if ch == ' ' {
                            cur_x += scale * 0.28;
                        } else {
                            cur_x += scale * 0.55;
                        }
                    }

                    if !glyphs.is_empty() {
                        scene
                            .draw_glyphs(v_font)
                            .font_size(scale)
                            .transform(current_transform)
                            .brush(p_color)
                            .draw(Fill::NonZero, glyphs.into_iter());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_core::geometry::{BorderRadius, Color, Point, Rect};
    use quick_style::theme::tokens::Shadow;

    #[test]
    fn test_vello_scene_builder_all_commands() {
        let mut canvas = Canvas::new();
        canvas.clear(Color::from_rgba(20, 20, 20, 255));
        canvas.fill_rect(Rect::new(10.0, 10.0, 100.0, 50.0), Color::from_rgba(255, 0, 0, 255));
        canvas.stroke_rect(Rect::new(10.0, 10.0, 100.0, 50.0), Color::WHITE, 2.0);
        canvas.fill_rounded_rect(
            Rect::new(20.0, 20.0, 200.0, 80.0),
            BorderRadius::all(8.0),
            Color::from_rgba(0, 120, 212, 255),
        );
        canvas.stroke_rounded_rect(
            Rect::new(20.0, 20.0, 200.0, 80.0),
            BorderRadius::all(8.0),
            Color::WHITE,
            1.5,
        );
        canvas.draw_shadow(
            Rect::new(50.0, 50.0, 150.0, 100.0),
            BorderRadius::all(12.0),
            Shadow {
                offset_x: 0.0,
                offset_y: 4.0,
                blur_radius: 12.0,
                spread_radius: 0.0,
                color: Color::from_rgba(0, 0, 0, 80),
            },
        );
        canvas.draw_line(Point::new(0.0, 0.0), Point::new(100.0, 100.0), Color::RED, 1.0);
        canvas.draw_text("Quick Vello Rendering Engine", Point::new(30.0, 40.0), Color::WHITE, 16.0, None);
        canvas.push_clip(Rect::new(0.0, 0.0, 500.0, 500.0));
        canvas.translate(10.0, 20.0);
        canvas.scale(1.5, 1.5);
        canvas.save();
        canvas.restore();
        canvas.pop_clip();

        let mut scene = Scene::new();
        VelloSceneBuilder::build(&canvas, &mut scene, 1.0);
    }
}
