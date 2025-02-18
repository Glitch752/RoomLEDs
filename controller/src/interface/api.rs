use std::sync::Arc;

use axum::{extract::{Path, State}, response::IntoResponse, routing::{get, post}, Json, Router};

use crate::{render::effects::AnyEffect, LightingState};

pub fn router() -> Router<Arc<LightingState>> {
    let api_router = Router::new()
        .route("/light_positions", get(light_positions_handler))
        .route("/run_effect", post(run_effect_handler))
        .route("/temporary_effect/:effect", post(temporary_effect_handler));

    api_router
}

async fn light_positions_handler(State(state): State<Arc<LightingState>>) -> impl IntoResponse {
    let pixel_locations = state.render_state.lock().info.pixel_locations.clone();
    let pixel_locations = pixel_locations.iter().map(|location| (location.x, location.y)).collect::<Vec<_>>();
    Json(pixel_locations)
}

async fn run_effect_handler(
    State(state): State<Arc<LightingState>>,
    Json(effect): Json<AnyEffect>
) -> impl IntoResponse {
    state.render_state.lock().effect = Box::new(effect);

    "OK"
}

async fn temporary_effect_handler(
    State(state): State<Arc<LightingState>>,
    Path(effect_name): Path<String>
) -> impl IntoResponse {
    let effect = state.presets.get_temporary_effect(&effect_name);
    
    if let Some(effect) = effect {
        state.render_state.lock().temporary_effect_compositor.add_effect(effect);
    }

    "OK"
}