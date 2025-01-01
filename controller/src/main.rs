use std::sync::Arc;

use parking_lot::Mutex;
use render::frame::PresentedFrame;

mod output;
mod interface;
mod render;

static FRAME_TIMES_STORED: usize = 100;

static TOTAL_PIXELS: u32 = 812;

// State for rendering the lights that needs to be shared between the web server and the output thread
#[derive(Debug, Clone)]
struct RenderState {
    time: f64,

    // Statistics we collect to display on the web interface
    // We can't use a dynamic array here because allocating in the output thread is not allowed
    frame_times: [f64; FRAME_TIMES_STORED],
    frames: usize,

    current_presented_frame: Option<PresentedFrame>,
}

// Shared global state for the web application
struct LightingState {
    render_state: Arc<Mutex<RenderState>>,
}

#[tokio::main]
async fn main() {
    let lighting_state = Arc::new(LightingState {
        render_state: Arc::new(Mutex::new(RenderState {
            time: 0.0,

            frame_times: [0.0; FRAME_TIMES_STORED],
            frames: 0,

            current_presented_frame: None,
        })),
    });

    let (render_thread, render_consumer) = render::start_render_thread(Arc::clone(&lighting_state.render_state));
    output::start_output_thread(render_thread.thread().clone(), render_consumer);

    interface::serve(lighting_state).await;
}