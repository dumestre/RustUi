use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const RED: Color = Color {
        r: 244,
        g: 63,
        b: 94,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 16,
        g: 185,
        b: 129,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 79,
        g: 70,
        b: 229,
        a: 255,
    };
    pub const SLATE_800: Color = Color {
        r: 30,
        g: 41,
        b: 59,
        a: 255,
    };
    pub const SLATE_900: Color = Color {
        r: 15,
        g: 23,
        b: 42,
        a: 255,
    };
    pub const TRANSPARENT: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };

    pub fn lerp(a: Color, b: Color, t: f32) -> Color {
        Color {
            r: (a.r as f32 + (b.r as f32 - a.r as f32) * t) as u8,
            g: (a.g as f32 + (b.g as f32 - a.g as f32) * t) as u8,
            b: (a.b as f32 + (b.b as f32 - a.b as f32) * t) as u8,
            a: (a.a as f32 + (b.a as f32 - a.a as f32) * t) as u8,
        }
    }
    pub fn alpha(self, a_val: u8) -> Self {
        Self { a: a_val, ..self }
    }
}

pub struct CachedGlyph {
    pub width: u32,
    pub height: u32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub data: Vec<f32>,
}

pub struct FontAtlas {
    pub cache: HashMap<(char, u32), CachedGlyph>,
}

impl FontAtlas {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get_or_insert(&mut self, font: &FontArc, c: char, size: f32) -> &CachedGlyph {
        let key = (c, size as u32);
        if !self.cache.contains_key(&key) {
            let scale = PxScale::from(size);
            let scaled_font = font.as_scaled(scale);
            let glyph = scaled_font.scaled_glyph(c);
            if let Some(outlined) = font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                let width = bounds.width() as u32;
                let height = bounds.height() as u32;
                let mut data = vec![0.0; (width * height) as usize];
                outlined.draw(|px, py, v| {
                    if px < width && py < height {
                        data[(py * width + px) as usize] = v;
                    }
                });
                self.cache.insert(
                    key,
                    CachedGlyph {
                        width,
                        height,
                        offset_x: bounds.min.x,
                        offset_y: bounds.min.y,
                        data,
                    },
                );
            } else {
                self.cache.insert(
                    key,
                    CachedGlyph {
                        width: 0,
                        height: 0,
                        offset_x: 0.0,
                        offset_y: 0.0,
                        data: vec![],
                    },
                );
            }
        }
        self.cache.get(&key).unwrap()
    }
}

pub fn draw_text_smooth(
    frame: &mut [u8],
    atlas: &mut FontAtlas,
    font: &FontArc,
    size: f32,
    x: f32,
    y: f32,
    text: &str,
    col: Color,
    sw: u32,
    sh: u32,
) {
    let scale = PxScale::from(size);
    let scaled_font = font.as_scaled(scale);
    let mut caret = ab_glyph::point(x, y + scaled_font.ascent());

    for c in text.chars() {
        let cached = atlas.get_or_insert(font, c, size);
        if cached.width > 0 {
            let start_x = (caret.x + cached.offset_x) as i32;
            let start_y = (caret.y + cached.offset_y) as i32;
            for py in 0..cached.height {
                for px in 0..cached.width {
                    let v = cached.data[(py * cached.width + px) as usize];
                    if v > 0.01 {
                        // Otimização: Pular pixels vazios
                        let final_x = start_x + px as i32;
                        let final_y = start_y + py as i32;
                        if final_x >= 0
                            && final_x < sw as i32
                            && final_y >= 0
                            && final_y < sh as i32
                        {
                            let idx = ((final_y as u32 * sw + final_x as u32) * 4) as usize;
                            let alpha = v * (col.a as f32 / 255.0);
                            frame[idx] =
                                (col.r as f32 * alpha + frame[idx] as f32 * (1.0 - alpha)) as u8;
                            frame[idx + 1] = (col.g as f32 * alpha
                                + frame[idx + 1] as f32 * (1.0 - alpha))
                                as u8;
                            frame[idx + 2] = (col.b as f32 * alpha
                                + frame[idx + 2] as f32 * (1.0 - alpha))
                                as u8;
                            frame[idx + 3] = 255;
                        }
                    }
                }
            }
        }
        caret.x += scaled_font.h_advance(font.glyph_id(c));
    }
}

pub fn draw_icon_plus(f: &mut [u8], x: i32, y: i32, size: i32, col: Color, sw: u32, sh: u32) {
    let mid = size / 2;
    for i in 0..size {
        let px1 = x + mid;
        let py1 = y + i;
        let px2 = x + i;
        let py2 = y + mid;
        if px1 >= 0 && px1 < sw as i32 && py1 >= 0 && py1 < sh as i32 {
            let idx = ((py1 as u32 * sw + px1 as u32) * 4) as usize;
            f[idx] = col.r;
            f[idx + 1] = col.g;
            f[idx + 2] = col.b;
            f[idx + 3] = 255;
        }
        if px2 >= 0 && px2 < sw as i32 && py2 >= 0 && py2 < sh as i32 {
            let idx = ((py2 as u32 * sw + px2 as u32) * 4) as usize;
            f[idx] = col.r;
            f[idx + 1] = col.g;
            f[idx + 2] = col.b;
            f[idx + 3] = 255;
        }
    }
}

pub fn draw_icon_chart(f: &mut [u8], x: i32, y: i32, size: i32, col: Color, sw: u32, sh: u32) {
    for i in 0..3 {
        let h = (i + 1) * (size / 3);
        let wx = size / 4;
        draw_rect(f, x + i * (wx + 1), y + size - h, wx, h, col, sw, sh);
    }
}

// OTIMIZAÇÃO HÍBRIDA 🚀: Desenho de retângulos arredondados ultra-rápido
pub fn draw_rounded_rect(
    f: &mut [u8],
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    r: f32,
    c: Color,
    sw: u32,
    sh: u32,
) {
    if r <= 0.0 {
        draw_rect(f, x as i32, y as i32, w as i32, h as i32, c, sw, sh);
        return;
    }

    // 1. Centro Sólido (Fast Fill)
    draw_rect(
        f,
        (x + r) as i32,
        y as i32,
        (w - 2.0 * r) as i32,
        h as i32,
        c,
        sw,
        sh,
    );
    draw_rect(
        f,
        x as i32,
        (y + r) as i32,
        r as i32,
        (h - 2.0 * r) as i32,
        c,
        sw,
        sh,
    );
    draw_rect(
        f,
        (x + w - r) as i32,
        (y + r) as i32,
        r as i32,
        (h - 2.0 * r) as i32,
        c,
        sw,
        sh,
    );

    // 2. Cantos Arredondados (Math only for corners)
    let corners = [
        (x + r, y + r),
        (x + w - r, y + r),
        (x + r, y + h - r),
        (x + w - r, y + h - r),
    ];
    for (i, (cx, cy)) in corners.iter().enumerate() {
        let x_range = if i % 2 == 0 {
            (x as i32)..(cx.clone() as i32)
        } else {
            (cx.clone() as i32)..(x as i32 + w as i32)
        };
        let y_range = if i < 2 {
            (y as i32)..(cy.clone() as i32)
        } else {
            (cy.clone() as i32)..(y as i32 + h as i32)
        };

        for py in y_range {
            for px in x_range.clone() {
                if px < 0 || py < 0 || px >= sw as i32 || py >= sh as i32 {
                    continue;
                }
                let dx = px as f32 - cx;
                let dy = py as f32 - cy;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= r {
                    let idx = ((py as u32 * sw + px as u32) * 4) as usize;
                    let a = if dist <= r - 1.0 { 1.0 } else { r - dist };
                    let al = (c.a as f32 * a) / 255.0;
                    f[idx] = (c.r as f32 * al + f[idx] as f32 * (1.0 - al)) as u8;
                    f[idx + 1] = (c.g as f32 * al + f[idx + 1] as f32 * (1.0 - al)) as u8;
                    f[idx + 2] = (c.b as f32 * al + f[idx + 2] as f32 * (1.0 - al)) as u8;
                    f[idx + 3] = 255;
                }
            }
        }
    }
}

pub fn draw_shadow(f: &mut [u8], x: f32, y: f32, w: f32, h: f32, r: f32, b: f32, sw: u32, sh: u32) {
    draw_rounded_rect(
        f,
        x + 2.0,
        y + 4.0,
        w,
        h,
        r + b,
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 50,
        },
        sw,
        sh,
    );
}

pub fn clear(f: &mut [u8], c: Color) {
    for p in f.chunks_exact_mut(4) {
        p[0] = c.r;
        p[1] = c.g;
        p[2] = c.b;
        p[3] = c.a;
    }
}

pub fn draw_rect(f: &mut [u8], x: i32, y: i32, w: i32, h: i32, c: Color, sw: u32, sh: u32) {
    let x_start = x.max(0);
    let x_end = (x + w).min(sw as i32);
    let y_start = y.max(0);
    let y_end = (y + h).min(sh as i32);

    if x_start >= x_end || y_start >= y_end {
        return;
    }

    for py in y_start..y_end {
        let line_start = ((py as u32 * sw + x_start as u32) * 4) as usize;
        let line_end = ((py as u32 * sw + x_end as u32) * 4) as usize;
        for p in f[line_start..line_end].chunks_exact_mut(4) {
            if c.a == 255 {
                p[0] = c.r;
                p[1] = c.g;
                p[2] = c.b;
                p[3] = 255;
            } else {
                let al = c.a as f32 / 255.0;
                p[0] = (c.r as f32 * al + p[0] as f32 * (1.0 - al)) as u8;
                p[1] = (c.g as f32 * al + p[1] as f32 * (1.0 - al)) as u8;
                p[2] = (c.b as f32 * al + p[2] as f32 * (1.0 - al)) as u8;
                p[3] = 255;
            }
        }
    }
}

// ============================================================================
// DEBUG OVERLAY
// ============================================================================

/// Desenha overlay de debug com FPS e informações de renderização
pub fn draw_debug_overlay(
    frame: &mut [u8],
    atlas: &mut FontAtlas,
    font: &FontArc,
    fps: f64,
    frame_time_ms: f64,
    sw: u32,
    sh: u32,
) {
    let bg = Color { r: 0, g: 0, b: 0, a: 200 };
    
    // Background do overlay
    draw_rect(
        frame,
        0,
        0,
        180,
        80,
        bg,
        sw,
        sh,
    );

    // FPS
    let fps_text = format!("FPS: {:.0}", fps);
    draw_text_smooth(
        frame,
        atlas,
        font,
        14.0,
        8.0,
        10.0,
        &fps_text,
        Color::WHITE,
        sw,
        sh,
    );

    // Frame time
    let ft_text = format!("Frame: {:.2}ms", frame_time_ms);
    draw_text_smooth(
        frame,
        atlas,
        font,
        14.0,
        8.0,
        30.0,
        &ft_text,
        Color::WHITE,
        sw,
        sh,
    );

    // Resolução
    let res_text = format!("Res: {}x{}", sw, sh);
    draw_text_smooth(
        frame,
        atlas,
        font,
        14.0,
        8.0,
        50.0,
        &res_text,
        Color::WHITE,
        sw,
        sh,
    );

    // Indicador de cor baseado no FPS
    let indicator_color = if fps >= 55.0 {
        Color::GREEN
    } else if fps >= 30.0 {
        Color { r: 255, g: 200, b: 0, a: 255 }
    } else {
        Color::RED
    };

    draw_rect(
        frame,
        160,
        10,
        12,
        12,
        indicator_color,
        sw,
        sh,
    );
}

/// Desenha bounds de debug para um rect
pub fn draw_debug_bounds(
    frame: &mut [u8],
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: Color,
    sw: u32,
    sh: u32,
) {
    // Top
    draw_rect(frame, x as i32, y as i32, w as i32, 1, color, sw, sh);
    // Bottom
    draw_rect(frame, x as i32, (y + h) as i32 - 1, w as i32, 1, color, sw, sh);
    // Left
    draw_rect(frame, x as i32, y as i32, 1, h as i32, color, sw, sh);
    // Right
    draw_rect(frame, (x + w) as i32 - 1, y as i32, 1, h as i32, color, sw, sh);
}
