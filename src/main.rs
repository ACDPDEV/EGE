mod draw;
mod engine;
mod types;
mod utils;

use engine::loop_::run;
use types::LoopMode;
use utils::clear_console;

fn main() {
    let mut frame_count = 0u32;
    let mut fps_timer = 0.0f64;
    let mut fps = 0u32;

    run("Motor", 800, 600, LoopMode::Vsync, move |canvas, delta| {
        // Acumular tiempo y contar frames
        frame_count += 1;
        fps_timer += delta;

        if fps_timer >= 1.0 {
            fps = frame_count;
            frame_count = 0;
            fps_timer = 0.0;
        }

        canvas.clear(0x1a1a2eff);
        clear_console();
        println!("FPS: {}", fps);
        println!("Delta time: {:.4}", delta);

        canvas.draw_line(10, 10, 790, 590, 0x00ffe0ff);
        canvas.draw_line_antialiased(10, 10 + 20, 790, 590 + 20, 0x00ffe0ff);
    });
}
