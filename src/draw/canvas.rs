use crate::types::{ColorRGBA, LineClipAlgorithm, LineDrawAlgorithm};

const DEFAULT_BACKGROUND: ColorRGBA = 0x000000FF;

pub struct Canvas {
    pub(crate) pixel_buffer: Vec<ColorRGBA>,
    pub line_draw_algorithm: LineDrawAlgorithm,
    pub line_clip_algorithm: LineClipAlgorithm,
    pub background_color: ColorRGBA,
    pub width: u32,
    pub height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32, background_color: Option<ColorRGBA>) -> Self {
        let background_color = match background_color {
            Some(v) => v,
            None => DEFAULT_BACKGROUND,
        };
        Self {
            pixel_buffer: vec![background_color; (width * height) as usize],
            line_draw_algorithm: LineDrawAlgorithm::default(),
            line_clip_algorithm: LineClipAlgorithm::default(),
            background_color,
            width,
            height,
        }
    }

    /// Returns the total number of pixels in the buffer (`width * height`).
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.pixel_buffer.len()
    }

    /// Returns `true` if the buffer contains no pixels.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.pixel_buffer.is_empty()
    }
}
