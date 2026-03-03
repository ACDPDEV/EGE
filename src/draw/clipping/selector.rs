use crate::draw::canvas::Canvas;
use crate::types::LineClipAlgorithm;

impl Canvas {
    pub fn line_clip(&self, x0: i32, y0: i32, x1: i32, y1: i32) -> Option<(i32, i32, i32, i32)> {
        match self.line_clip_algorithm {
            LineClipAlgorithm::None => Some((x0, y0, x1, y1)),
            LineClipAlgorithm::Parametric => self.parametric_clip_line(x0, y0, x1, y1),
            LineClipAlgorithm::RegionCode => self.region_code_clip_line(x0, y0, x1, y1),
        }
    }
    pub fn set_line_clip_algorithm(&mut self, algorithm: LineClipAlgorithm) {
        self.line_clip_algorithm = algorithm;
    }
}
