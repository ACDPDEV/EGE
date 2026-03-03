use crate::draw::canvas::Canvas;
use crate::types::ColorRGBA;

impl Canvas {
    /// Returns a read-only slice of the raw pixel buffer.
    ///
    /// Useful for passing the frame to a display backend (e.g. `minifb`, `softbuffer`).
    /// The slice is in row-major order: pixel `(x, y)` is at index `y * width + x`.
    pub fn pixels(&self) -> &[ColorRGBA] {
        &self.pixel_buffer
    }

    /// Returns a mutable slice of the raw pixel buffer.
    pub fn pixels_mut(&mut self) -> &mut [ColorRGBA] {
        &mut self.pixel_buffer
    }

    /// Fills the entire buffer with `color`, replacing every pixel.
    ///
    /// This is a raw overwrite — it does **not** blend. Intended to reset
    /// the canvas at the start of each frame.
    pub fn clear(&mut self, color: ColorRGBA) {
        self.pixel_buffer.fill(color);
    }

    /// Resizes the canvas to `new_width × new_height`, filled with `background`.
    ///
    /// All existing pixel data is discarded. The underlying allocation is
    /// reused when possible to avoid unnecessary heap traffic.
    pub fn resize(&mut self, new_width: u32, new_height: u32, background: ColorRGBA) {
        self.width = new_width;
        self.height = new_height;

        let new_len = (new_width * new_height) as usize;

        self.pixel_buffer.clear();
        self.pixel_buffer.resize(new_len, background);
    }
}
