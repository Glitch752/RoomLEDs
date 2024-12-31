use std::sync::Arc;

use assert_no_alloc::AllocDisabler;
use parking_lot::Mutex;

mod output;
mod interface;

#[cfg(debug_assertions)] // required when disable_release is set (default)
#[global_allocator]
static A: AllocDisabler = AllocDisabler;

static FRAME_TIMES_STORED: usize = 100;

// State for rendering the lights that needs to be shared between the web server and the output thread
#[derive(Clone)]
struct RenderState {
    // Temporary: the speed of the lights
    speed: f64,
    start_hue: f64,

    // Statistics we collect to display on the web interface
    // We can't use a dynamic array here because allocating in the output thread is not allowed
    frame_times: [f64; FRAME_TIMES_STORED],
    frames: usize,
}

// Shared global state for the web application
struct LightingState {
    render_state: Arc<Mutex<RenderState>>,
}

#[tokio::main]
async fn main() {
    let lighting_state = Arc::new(LightingState {
        render_state: Arc::new(Mutex::new(RenderState {
            speed: 1.0,
            start_hue: 0.0,

            frame_times: [0.0; FRAME_TIMES_STORED],
            frames: 0,
        })),
    });

    output::start_output_thread(Arc::clone(&lighting_state.render_state));

    interface::serve(lighting_state).await;
}