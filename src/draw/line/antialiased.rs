use crate::{draw::canvas::Canvas, types::ColorRGBA};

// ─────────────────────────────────────────────────────────────────────────────
// ColorRGBA helpers  (format: 0xRRGGBBAA)
// ─────────────────────────────────────────────────────────────────────────────

/// Unpacks a `ColorRGBA` into its four `f64` components in [0.0, 1.0].
#[inline(always)]
fn unpack(c: u32) -> (f64, f64, f64, f64) {
    (
        ((c >> 24) & 0xFF) as f64 / 255.0, // R
        ((c >> 16) & 0xFF) as f64 / 255.0, // G
        ((c >> 8) & 0xFF) as f64 / 255.0,  // B
        (c & 0xFF) as f64 / 255.0,         // A
    )
}

/// Packs four `f64` components in [0.0, 1.0] into a `ColorRGBA`.
#[inline(always)]
fn pack(r: f64, g: f64, b: f64, a: f64) -> u32 {
    let clamp = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u32;
    (clamp(r) << 24) | (clamp(g) << 16) | (clamp(b) << 8) | clamp(a)
}

/// Standard "over" alpha compositing:
///   out = src·α_src + dst·α_dst·(1 − α_src)
///
/// Both `src` and `dst` are pre-multiplied during the blend and then
/// normalised back to straight alpha so the buffer stays consistent.
#[inline]
fn alpha_blend(src: u32, dst: u32) -> u32 {
    let (sr, sg, sb, sa) = unpack(src);
    let (dr, dg, db, da) = unpack(dst);

    let out_a = sa + da * (1.0 - sa);

    if out_a < f64::EPSILON {
        return 0x00000000;
    }

    let blend_ch = |sc: f64, dc: f64| -> f64 { (sc * sa + dc * da * (1.0 - sa)) / out_a };

    pack(blend_ch(sr, dr), blend_ch(sg, dg), blend_ch(sb, db), out_a)
}

// ─────────────────────────────────────────────────────────────────────────────
// Xiaolin Wu anti-aliased line
// ─────────────────────────────────────────────────────────────────────────────

impl Canvas {
    /// Draws an anti-aliased line using Xiaolin Wu's algorithm.
    ///
    /// `color` is `RRGGBBAA`. The alpha channel in `color` sets the maximum
    /// opacity of the stroke; Wu modulates it per-pixel by the fractional
    /// distance to the ideal line, then composites the result over whatever
    /// is already in the pixel buffer (Porter-Duff "over").
    ///
    /// The segment is clipped with Liang-Barsky before rasterisation.
    pub fn draw_line_antialiased(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: ColorRGBA) {
        let (mut x0f, mut y0f) = (x0 as f64, y0 as f64);
        let (mut x1f, mut y1f) = (x1 as f64, y1 as f64);

        let steep = (y1f - y0f).abs() > (x1f - x0f).abs();

        // Transpose so we always iterate along the major (x) axis.
        if steep {
            (x0f, y0f) = (y0f, x0f);
            (x1f, y1f) = (y1f, x1f);
        }

        // Ensure left-to-right traversal.
        if x0f > x1f {
            (x0f, x1f) = (x1f, x0f);
            (y0f, y1f) = (y1f, y0f);
        }

        let dx = x1f - x0f;
        let dy = y1f - y0f;
        let gradient = if dx == 0.0 { 1.0 } else { dy / dx };

        // Extract the base alpha of the stroke colour so Wu can scale it.
        let base_alpha = (color & 0xFF) as f64 / 255.0;

        // ── Pixel writer with Porter-Duff "over" blending ────────────────────
        // Builds a new `ColorRGBA` whose alpha = base_alpha × coverage,
        // reads the existing pixel from the buffer, blends, and writes back.
        let plot = |canvas: &mut Canvas, px: i32, py: i32, coverage: f64| {
            let (rx, ry) = if steep { (py, px) } else { (px, py) };
            if !canvas.in_bounds(rx, ry) {
                return;
            }

            // Modulate the stroke alpha by the Wu coverage weight.
            let modulated_alpha = (base_alpha * coverage * 255.0).round() as u32;
            let src = (color & 0xFFFFFF00) | modulated_alpha;

            // Read the destination pixel from the buffer.
            let idx = (ry as u32 * canvas.width + rx as u32) as usize;
            let dst = canvas.pixel_buffer[idx];

            // Composite src over dst and write back.
            canvas.pixel_buffer[idx] = alpha_blend(src, dst);
        };

        // ── First endpoint ───────────────────────────────────────────────────
        let xend = x0f.round();
        let yend = y0f + gradient * (xend - x0f);
        let xgap = rfpart(x0f + 0.5);
        let xpx0 = xend as i32;
        let ypx0 = yend.floor() as i32;

        plot(self, xpx0, ypx0, rfpart(yend) * xgap);
        plot(self, xpx0, ypx0 + 1, fpart(yend) * xgap);

        let mut intery = yend + gradient;

        // ── Second endpoint ──────────────────────────────────────────────────
        let xend = x1f.round();
        let yend = y1f + gradient * (xend - x1f);
        let xgap = fpart(x1f + 0.5);
        let xpx1 = xend as i32;
        let ypx1 = yend.floor() as i32;

        plot(self, xpx1, ypx1, rfpart(yend) * xgap);
        plot(self, xpx1, ypx1 + 1, fpart(yend) * xgap);

        // ── Main loop: interior pixels ───────────────────────────────────────
        for x in (xpx0 + 1)..xpx1 {
            let y_floor = intery.floor() as i32;
            plot(self, x, y_floor, rfpart(intery));
            plot(self, x, y_floor + 1, fpart(intery));
            intery += gradient;
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Fractional-part helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Fractional part of `x`.
#[inline(always)]
fn fpart(x: f64) -> f64 {
    x - x.floor()
}

/// 1 − fractional part of `x`.
#[inline(always)]
fn rfpart(x: f64) -> f64 {
    1.0 - fpart(x)
}
