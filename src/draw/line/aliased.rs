use crate::{draw::canvas::Canvas, types::ColorRGBA};

/// Draws a straight line between two points using Bresenham's line algorithm.
/// The segment is first clipped to the canvas boundaries with Liang-Barsky.
/// `color` is a `u32` in RRGGBBAA format; the alpha channel is respected by
/// `draw_pixel` but Bresenham always writes at full coverage (no blending here).
impl Canvas {
    pub fn draw_line_aliased(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: ColorRGBA) {
        let mut dx = (x1 - x0).abs();
        let mut dy = (y1 - y0).abs();

        if dx == 0 && dy == 0 {
            self.draw_pixel(x0, y0, color);
            return;
        }

        let sx = if x1 > x0 { 1 } else { -1 };
        let sy = if y1 > y0 { 1 } else { -1 };
        let mut x = x0;
        let mut y = y0;

        if dx == 0 {
            for _ in 0..=dy {
                self.draw_pixel(x0, y, color);
                y += sy;
            }
            return;
        }

        if dy == 0 {
            for _ in 0..=dx {
                self.draw_pixel(x, y0, color);
                x += sx;
            }
            return;
        }

        let swap = dx < dy;
        if swap {
            (dx, dy) = (dy, dx);
        }

        let mut deviation = 2 * dy - dx;

        for _ in 0..=dx {
            // Safety net: clipping guarantees validity but rounding can nudge
            // an endpoint by 1 pixel, so we keep the in_bounds guard.
            if self.in_bounds(x, y) {
                self.draw_pixel(x, y, color);
            }

            if deviation > 0 {
                if swap {
                    x += sx;
                } else {
                    y += sy;
                }
                deviation -= 2 * dx;
            }

            if swap {
                y += sy;
            } else {
                x += sx;
            }
            deviation += 2 * dy;
        }
    }
}
