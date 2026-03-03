pub type ColorRGBA = u32; // Hexadecimal RRGGBBAA

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineDrawAlgorithm {
    #[default]
    Aliased, // Bresenham
    Antialiased, // Xiaolin Wu
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LineClipAlgorithm {
    None, // No clipping
    #[default]
    Parametric, // Liang-Barsky
    RegionCode, // Cohen-Sutherland
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LoopMode {
    /// Render as fast as the CPU allows. No sleep, no sync.
    Uncapped,

    /// Synchronise presentation with the monitor refresh rate.
    /// Relies on softbuffer/winit present timing — no busy-wait.
    #[default]
    Vsync,

    /// Cap rendering to at most `fps` frames per second.
    /// Sleeps the remainder of each frame budget if rendering finishes early.
    MaxFps(u32),

    /// Decouple update logic from rendering via a fixed timestep accumulator.
    /// `step_ms` is the fixed update step in milliseconds (e.g. 16 for ~60 Hz logic).
    /// Rendering happens as fast as possible; update runs N times per frame to catch up.
    DeltaTime { step_ms: u32 },
}
