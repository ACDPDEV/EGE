use crate::draw::canvas::Canvas;

// ─────────────────────────────────────────────────────────────────────────────
// Region codes for Cohen-Sutherland
// ─────────────────────────────────────────────────────────────────────────────
const INSIDE: u8 = 0b0000;
const LEFT: u8 = 0b0001;
const RIGHT: u8 = 0b0010;
const BOTTOM: u8 = 0b0100;
const TOP: u8 = 0b1000;

impl Canvas {
    /// Computes the Cohen-Sutherland region code for point (x, y)
    /// relative to the canvas clip rectangle [0, width) x [0, height).
    fn get_region_code(&self, x: f64, y: f64) -> u8 {
        let mut code = INSIDE;
        let xmax = (self.width as f64) - 1.0;
        let ymax = (self.height as f64) - 1.0;

        if x < 0.0 {
            code |= LEFT;
        } else if x > xmax {
            code |= RIGHT;
        }
        if y < 0.0 {
            code |= BOTTOM;
        } else if y > ymax {
            code |= TOP;
        }

        code
    }

    /// Cohen-Sutherland line clipping.
    ///
    /// Returns `Some((x0, y0, x1, y1))` with the clipped integer endpoints if
    /// any part of the segment is visible, or `None` if it is entirely outside.
    pub fn region_code_clip_line(
        &self,
        x0: i32,
        y0: i32,
        x1: i32,
        y1: i32,
    ) -> Option<(i32, i32, i32, i32)> {
        let (mut x0, mut y0) = (x0 as f64, y0 as f64);
        let (mut x1, mut y1) = (x1 as f64, y1 as f64);

        let xmax = (self.width as f64) - 1.0;
        let ymax = (self.height as f64) - 1.0;

        let mut code0 = self.get_region_code(x0, y0);
        let mut code1 = self.get_region_code(x1, y1);

        loop {
            if code0 | code1 == INSIDE {
                return Some((
                    x0.round() as i32,
                    y0.round() as i32,
                    x1.round() as i32,
                    y1.round() as i32,
                ));
            }
            if code0 & code1 != 0 {
                return None;
            }

            let code_out = if code0 != INSIDE { code0 } else { code1 };
            let (x, y);

            if code_out & TOP != 0 {
                x = x0 + (x1 - x0) * (ymax - y0) / (y1 - y0);
                y = ymax;
            } else if code_out & BOTTOM != 0 {
                x = x0 + (x1 - x0) * (0.0 - y0) / (y1 - y0);
                y = 0.0;
            } else if code_out & RIGHT != 0 {
                y = y0 + (y1 - y0) * (xmax - x0) / (x1 - x0);
                x = xmax;
            } else {
                y = y0 + (y1 - y0) * (0.0 - x0) / (x1 - x0);
                x = 0.0;
            }

            if code_out == code0 {
                (x0, y0) = (x, y);
                code0 = self.get_region_code(x0, y0);
            } else {
                (x1, y1) = (x, y);
                code1 = self.get_region_code(x1, y1);
            }
        }
    }
}
