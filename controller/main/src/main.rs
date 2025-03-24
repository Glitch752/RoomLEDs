use std::sync::Arc;

use interface::presets::EffectPresets;
use parking_lot::Mutex;
use render::{effects::{self, TemporaryEffectCompositor}, frame::Pixel, spatial_map::{Location, SpatialMap}, RenderInfo, RenderState};
use tokio::sync::RwLock;

mod output;
mod interface;
mod render;

static FRAME_TIMES_STORED: usize = 100;

static TOTAL_PIXELS: u32 = 812;

// Shared global state for the web application
struct LightingState {
    render_state: Arc<Mutex<RenderState>>,
    presets: RwLock<EffectPresets>
}

#[tokio::main]
async fn main() {
    let pixel_locations = SpatialMap::new(TOTAL_PIXELS)
        .add_span(-14, 187, Location::from_inches(0., 0.), Location::from_inches(0., 132.))
        .add_span(187, 406, Location::from_inches(0., 132.), Location::from_inches(144., 132.))
        .add_span(406, 558, Location::from_inches(144., 132.), Location::from_inches(144., 32.))
        .add_span(558, 623, Location::from_inches(144., 32.), Location::from_inches(114., 0.))
        .add_span(623, 798, Location::from_inches(114., 0.), Location::from_inches(0., 0.))
        .get_individual_pixel_locations()
        .try_into().unwrap();

    let lighting_state = Arc::new(LightingState {
        render_state: Arc::new(Mutex::new(RenderState {
            info: RenderInfo::new(pixel_locations),
            temporary_effect_compositor: TemporaryEffectCompositor::new(vec![]),
            effect: effects::SolidColorEffect::new(Pixel::new(0, 0, 0, 1.0), 0, TOTAL_PIXELS).into()
        })),
        presets: RwLock::new(EffectPresets::load())
    });

    let (render_thread, render_consumer) =
        render::start_render_thread(Arc::clone(&lighting_state.render_state));
    output::start_output_thread(render_thread.thread().clone(), render_consumer);

    interface::serve(lighting_state).await;
}