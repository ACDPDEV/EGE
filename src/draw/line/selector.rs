use crate::draw::canvas::Canvas;
use crate::types::{ColorRGBA, LineDrawAlgorithm};

impl Canvas {
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: ColorRGBA) {
        let Some((x0, y0, x1, y1)) = self.line_clip(x0, y0, x1, y1) else {
            return;
        };
        match self.line_draw_algorithm {
            LineDrawAlgorithm::Aliased => self.draw_line_aliased(x0, y0, x1, y1, color),
            LineDrawAlgorithm::Antialiased => self.draw_line_antialiased(x0, y0, x1, y1, color),
        }
    }

    pub fn set_line_draw_algorithm(&mut self, algorithm: LineDrawAlgorithm) {
        self.line_draw_algorithm = algorithm;
    }
}
