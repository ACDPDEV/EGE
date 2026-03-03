use crate::draw::canvas::Canvas;
use crate::types::ColorRGBA;

// ─────────────────────────────────────────────────────────────────────────────
// ColorRGBA helpers  (format: 0xRRGGBBAA)
// ─────────────────────────────────────────────────────────────────────────────

#[inline(always)]
fn unpack(c: ColorRGBA) -> (f64, f64, f64, f64) {
    (
        ((c >> 24) & 0xFF) as f64 / 255.0, // R
        ((c >> 16) & 0xFF) as f64 / 255.0, // G
        ((c >> 8) & 0xFF) as f64 / 255.0,  // B
        (c & 0xFF) as f64 / 255.0,         // A
    )
}

#[inline(always)]
fn pack(r: f64, g: f64, b: f64, a: f64) -> ColorRGBA {
    let ch = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u32;
    (ch(r) << 24) | (ch(g) << 16) | (ch(b) << 8) | ch(a)
}

/// Porter-Duff "over": composites `src` on top of `dst` using straight alpha.
///
///   out_a = src_a + dst_a · (1 − src_a)
///   out_c = (src_c · src_a + dst_c · dst_a · (1 − src_a)) / out_a
#[inline]
fn blend_over(src: ColorRGBA, dst: ColorRGBA) -> ColorRGBA {
    let (sr, sg, sb, sa) = unpack(src);
    let (dr, dg, db, da) = unpack(dst);

    let out_a = sa + da * (1.0 - sa);

    // Both pixels are fully transparent — result is transparent black.
    if out_a < f64::EPSILON {
        return 0x00000000;
    }

    let ch = |sc: f64, dc: f64| (sc * sa + dc * da * (1.0 - sa)) / out_a;

    pack(ch(sr, dr), ch(sg, dg), ch(sb, db), out_a)
}

// ─────────────────────────────────────────────────────────────────────────────
// Pixel-level canvas operations
// ─────────────────────────────────────────────────────────────────────────────

impl Canvas {
    /// Computes the flat buffer index for pixel `(x, y)`.
    /// Assumes `(x, y)` is already validated with `in_bounds`.
    #[inline(always)]
    pub(crate) fn pixel_index(&self, x: i32, y: i32) -> usize {
        y as usize * self.width as usize + x as usize
    }

    /// Writes `color` onto pixel `(x, y)` using Porter-Duff "over" blending.
    ///
    /// If `color` is fully opaque (`AA = 0xFF`) this is equivalent to a plain
    /// overwrite. If `color` is fully transparent (`AA = 0x00`) the existing
    /// pixel is left unchanged. Out-of-bounds coordinates are silently ignored.
    pub fn draw_pixel(&mut self, x: i32, y: i32, color: ColorRGBA) {
        if !self.in_bounds(x, y) {
            return;
        }

        let idx = self.pixel_index(x, y);
        self.pixel_buffer[idx] = blend_over(color, self.pixel_buffer[idx]);
    }

    /// Returns the color of pixel `(x, y)`, or `None` if out of bounds.
    pub fn get_pixel(&self, x: i32, y: i32) -> Option<ColorRGBA> {
        if !self.in_bounds(x, y) {
            return None;
        }

        Some(self.pixel_buffer[self.pixel_index(x, y)])
    }
}
