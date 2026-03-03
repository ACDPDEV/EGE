use crate::draw::canvas::Canvas;

impl Canvas {
    // ─────────────────────────────────────────────────────────────────────────
    // Liang-Barsky
    // ─────────────────────────────────────────────────────────────────────────

    /// Liang-Barsky line clipping.
    ///
    /// Returns `Some((x0, y0, x1, y1))` with the clipped integer endpoints if
    /// any part of the segment is visible, or `None` if it is entirely outside.
    ///
    /// Parametrises the segment as P(t) = P0 + t·(P1−P0), t ∈ [0,1], and
    /// solves cuatro inequalities (one per boundary) to narrow the valid t range.
    /// Faster than Cohen-Sutherland because it never re-tests clipped endpoints.
    pub fn parametric_clip_line(
        &self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    ) -> Option<(i32, i32, i32, i32)> {
        let (x0f, y0f) = (x0 as f64, y0 as f64);
        let dx = (x1 - x0) as f64;
        let dy = (y1 - y0) as f64;

        let xmax = (self.width as f64) - 1.0;
        let ymax = (self.height as f64) - 1.0;

        // p[i] / q[i] encode the four boundary constraints.
        //   p < 0  → entering half-plane
        //   p > 0  → exiting  half-plane
        //   p = 0  → parallel to boundary (reject if q < 0, i.e. outside)
        let p = [-dx, dx, -dy, dy];
        let q = [x0f, xmax - x0f, y0f, ymax - y0f];

        let mut t0: f64 = 0.0; // entry parameter
        let mut t1: f64 = 1.0; // exit  parameter

        for i in 0..4 {
            if p[i] == 0.0 {
                if q[i] < 0.0 {
                    return None;
                }
                continue;
            }

            let t = q[i] / p[i];

            if p[i] < 0.0 {
                if t > t0 {
                    t0 = t;
                }
            } else {
                if t < t1 {
                    t1 = t;
                }
            }

            if t0 > t1 {
                return None;
            }
        }

        Some((
            (x0f + t0 * dx).round() as i32,
            (y0f + t0 * dy).round() as i32,
            (x0f + t1 * dx).round() as i32,
            (y0f + t1 * dy).round() as i32,
        ))
    }
}
