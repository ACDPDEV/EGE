use crate::draw::canvas::Canvas;

impl Canvas {
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }
    pub fn in_bounds_y(&self, y: i32) -> bool {
        y >= 0 && y < self.height as i32
    }

    pub fn in_bounds_x(&self, x: i32) -> bool {
        x >= 0 && x < self.width as i32
    }
}
