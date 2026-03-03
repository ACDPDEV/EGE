use std::num::NonZeroU32;
use std::time::{Duration, Instant};

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use crate::draw::canvas::Canvas;
use crate::types::LoopMode;

// ─────────────────────────────────────────────────────────────────────────────
// Public entry point
// ─────────────────────────────────────────────────────────────────────────────

/// Opens a window and runs the main loop.
///
/// # Arguments
/// - `title`     — window title.
/// - `width`     — initial window width in physical pixels.
/// - `height`    — initial window height in physical pixels.
/// - `loop_mode` — timing strategy (see [`LoopMode`]).
/// - `update`    — called once per logical update step.
///                 Receives the canvas and the elapsed time since the last
///                 update in seconds (`delta`). Draw into the canvas here.
///
/// # Example
/// ```rust
/// run("Motor", 800, 600, LoopMode::Vsync, |canvas, _delta| {
///     canvas.clear(0x1a1a2eff);
///     canvas.draw_line(10, 10, 200, 180, 0x00ffe0ff);
/// });
/// ```
pub fn run<F>(title: &str, width: u32, height: u32, loop_mode: LoopMode, update: F)
where
    F: FnMut(&mut Canvas, f64) + 'static,
{
    let event_loop = EventLoop::new().expect("failed to create event loop");

    // LoopMode::Vsync lets winit block on present; everything else polls.
    event_loop.set_control_flow(match loop_mode {
        LoopMode::Vsync => ControlFlow::Wait,
        _ => ControlFlow::Poll,
    });

    let mut app = App::new(title, width, height, loop_mode, update);
    event_loop.run_app(&mut app).expect("event loop error");
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal application state
// ─────────────────────────────────────────────────────────────────────────────

struct App<F: FnMut(&mut Canvas, f64)> {
    // Window / surface — populated on `resumed`.
    window: Option<Window>,
    surface: Option<Surface<&'static Window, &'static Window>>,

    canvas: Canvas,
    loop_mode: LoopMode,
    update: F,

    // Timing state.
    last_frame: Instant,
    /// Accumulator for DeltaTime mode (seconds).
    accumulator: f64,
    title: String,
}

impl<F: FnMut(&mut Canvas, f64)> App<F> {
    fn new(title: &str, width: u32, height: u32, loop_mode: LoopMode, update: F) -> Self {
        Self {
            window: None,
            surface: None,
            canvas: Canvas::new(width, height, None),
            loop_mode,
            update,
            last_frame: Instant::now(),
            accumulator: 0.0,
            title: title.to_owned(),
        }
    }

    // ── Core per-frame work ───────────────────────────────────────────────────

    fn render(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame).as_secs_f64();

        match self.loop_mode {
            // ── Uncapped: run update once, no sleep. ─────────────────────────
            LoopMode::Uncapped => {
                self.last_frame = now;
                (self.update)(&mut self.canvas, delta);
            }

            // ── Vsync: winit already blocks on present; one update per frame. ─
            LoopMode::Vsync => {
                self.last_frame = now;
                (self.update)(&mut self.canvas, delta);
            }

            // ── MaxFps: sleep if the frame finished faster than the budget. ───
            LoopMode::MaxFps(fps) => {
                let budget = Duration::from_secs_f64(1.0 / fps as f64);
                let elapsed = now.duration_since(self.last_frame);

                if elapsed < budget {
                    // Sleep the remaining time so we don't busy-wait.
                    std::thread::sleep(budget - elapsed);
                }

                self.last_frame = Instant::now();
                (self.update)(&mut self.canvas, delta);
            }

            // ── DeltaTime: fixed-step accumulator. ───────────────────────────
            // The update callback is invoked with a constant `step` until the
            // accumulator is drained. Rendering happens once after all steps.
            // This keeps physics/logic deterministic regardless of frame rate.
            LoopMode::DeltaTime { step_ms } => {
                let step = step_ms as f64 / 1000.0;
                self.last_frame = now;
                self.accumulator += delta;

                while self.accumulator >= step {
                    (self.update)(&mut self.canvas, step);
                    self.accumulator -= step;
                }
            }
        }

        self.present();
    }

    // ── Blit canvas buffer → softbuffer surface ───────────────────────────────

    fn present(&mut self) {
        let (Some(window), Some(surface)) = (&self.window, &mut self.surface) else {
            return;
        };

        let size = window.inner_size();
        let (Some(w), Some(h)) = (NonZeroU32::new(size.width), NonZeroU32::new(size.height)) else {
            return;
        };

        // Resize the softbuffer surface to match the window.
        if surface.resize(w, h).is_err() {
            return;
        }

        let Ok(mut sb_buffer) = surface.buffer_mut() else {
            return;
        };

        // softbuffer expects pixels in 0x00RRGGBB (native endian u32).
        // Our canvas uses 0xRRGGBBAA, so we shift right by 8 bits.
        let canvas_pixels = self.canvas.pixels();
        for (dst, &src) in sb_buffer.iter_mut().zip(canvas_pixels.iter()) {
            *dst = src >> 8;
        }

        let _ = sb_buffer.present();
    }

    // ── Handle window resize ──────────────────────────────────────────────────

    fn on_resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.canvas
                .resize(new_size.width, new_size.height, 0x000000FF);
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// winit ApplicationHandler
// ─────────────────────────────────────────────────────────────────────────────

impl<F: FnMut(&mut Canvas, f64)> ApplicationHandler for App<F> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create the window the first time the event loop resumes.
        if self.window.is_some() {
            return;
        }

        let size = PhysicalSize::new(self.canvas.width, self.canvas.height);
        let attrs = Window::default_attributes()
            .with_title(&self.title)
            .with_inner_size(size)
            .with_resizable(true);

        let window = event_loop
            .create_window(attrs)
            .expect("failed to create window");

        // SAFETY: The window is stored in `self` and outlives the surface.
        // softbuffer requires a `&'static` reference via this pattern.
        let window_ref: &'static Window = unsafe { &*(&window as *const Window) };

        let context = Context::new(window_ref).expect("failed to create softbuffer context");
        let surface = Surface::new(&context, window_ref).expect("failed to create surface");

        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(new_size) => {
                self.on_resize(new_size);
            }

            // RedrawRequested fires when winit decides it's time to draw
            // (e.g. after a present in Vsync mode, or on Poll).
            WindowEvent::RedrawRequested => {
                self.render();

                // Request the next frame immediately in non-Vsync modes.
                if let LoopMode::Vsync = self.loop_mode {
                } else if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // In Poll mode winit calls this when the event queue is empty.
        // We request a redraw so RedrawRequested fires next iteration.
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }
}
