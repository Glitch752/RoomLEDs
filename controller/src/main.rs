use std::sync::Arc;

use parking_lot::Mutex;
use render::{frame::PresentedFrame, spatial_map::{Location, SpatialMap}};

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

    debug_text: String,

    pixel_locations: [Location; TOTAL_PIXELS as usize],
}

// Shared global state for the web application
struct LightingState {
    render_state: Arc<Mutex<RenderState>>
}

#[tokio::main]
async fn main() {
    let pixel_locations = SpatialMap::new(TOTAL_PIXELS)
        // TODO: Find actual pixel locations
        .add_corner(0,   Location::new(0.0, 0.0))
        .add_corner(200, Location::new(1.0, 0.0))
        .add_corner(400, Location::new(1.0, 1.0))
        .add_corner(600, Location::new(0.0, 1.0))
        .add_corner(TOTAL_PIXELS, Location::new(0.0, 0.0))
        .get_individual_pixel_locations()
        .try_into().unwrap();

    let lighting_state = Arc::new(LightingState {
        render_state: Arc::new(Mutex::new(RenderState {
            time: 0.0,

            frame_times: [0.0; FRAME_TIMES_STORED],
            frames: 0,

            current_presented_frame: None,

            debug_text: String::new(),

            pixel_locations
        }))
    });

    let (render_thread, render_consumer) = render::start_render_thread(Arc::clone(&lighting_state.render_state));
    output::start_output_thread(render_thread.thread().clone(), render_consumer);

    interface::serve(lighting_state).await;
}