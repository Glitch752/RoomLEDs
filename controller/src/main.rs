use std::sync::Arc;

use parking_lot::Mutex;
use render::{effects::{self, TemporaryEffectCompositor}, frame::Pixel, spatial_map::{Location, SpatialMap}, RenderInfo, RenderState};

mod output;
mod interface;
mod render;

static FRAME_TIMES_STORED: usize = 100;

static TOTAL_PIXELS: u32 = 812;

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
            info: RenderInfo::new(pixel_locations),
            temporary_effect_compositor: TemporaryEffectCompositor::new(vec![]),
            effect: effects::SolidColorEffect::new(Pixel::new(0, 0, 0, 1.0), 0, TOTAL_PIXELS)
        }))
    });

    let (render_thread, render_consumer) =
        render::start_render_thread(Arc::clone(&lighting_state.render_state));
    output::start_output_thread(render_thread.thread().clone(), render_consumer);

    interface::serve(lighting_state).await;
}