use std::sync::Arc;

use axum::{extract::{Path, Query, State}, response::{IntoResponse, Response}, routing::{delete, get, post}, Json, Router};

use crate::{render::effects::{AnyEffect, AnyTemporaryEffect}, LightingState};

// TODO: Proper shared types for the API

pub fn router() -> Router<Arc<LightingState>> {
    let api_router = Router::new()
        .route("/light_positions", get(light_positions_handler))
        .route("/temporary_effects", get(get_temporary_effects_handler))
        .route("/temporary_effects/:effect", delete(delete_temporary_effect_handler))
        .route("/temporary_effects/:effect", post(create_temporary_effect_handler))
        .route("/effect_presets", get(get_effect_presets_handler))
        .route("/effect_presets/:preset", post(create_effect_preset_handler))
        .route("/effect_presets/:preset", delete(delete_effect_preset_handler))
        .route("/run_temporary_effect/:effect", get(run_temporary_effect_handler))
        .route("/run_effect", post(run_effect_handler));

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

async fn run_temporary_effect_handler(
    State(state): State<Arc<LightingState>>,
    Path(effect_name): Path<String>
) -> impl IntoResponse {
    let effect_presets = state.presets.read().await;
    let effect = effect_presets.get_temporary_effect(&effect_name);
    
    if let Some(effect) = effect {
        state.render_state.lock().temporary_effect_compositor.add_effect(effect);
    }

    "OK"
}

async fn get_temporary_effects_handler(
    State(state): State<Arc<LightingState>>
) -> Response {
    let effect_presets = state.presets.read().await;
    let effects = effect_presets.get_temporary_effect_list();
    Json(effects).into_response()
}

async fn create_temporary_effect_handler(
    State(state): State<Arc<LightingState>>,
    Path(effect_name): Path<String>,
    Json(effect): Json<AnyTemporaryEffect>
) -> impl IntoResponse {
    let mut effect_presets = state.presets.write().await;
    match effect_presets.add_temporary_effect(effect_name, effect) {
        Ok(_) => "OK",
        Err(_) => "Effect already exists"
    }
}

async fn delete_temporary_effect_handler(
    State(state): State<Arc<LightingState>>,
    Path(effect_name): Path<String>
) -> Response {
    let mut effect_presets = state.presets.write().await;
    effect_presets.remove_temporary_effect(&effect_name).unwrap();
    "OK".into_response()
}

async fn get_effect_presets_handler(
    State(state): State<Arc<LightingState>>
) -> Response {
    let effect_presets = state.presets.read().await;
    let presets = effect_presets.get_preset_list();
    Json(presets).into_response()
}

async fn create_effect_preset_handler(
    State(state): State<Arc<LightingState>>,
    Path(preset_name): Path<String>,
    Query(icon): Query<String>,
    Json(preset): Json<AnyEffect>
) -> impl IntoResponse {
    let mut effect_presets = state.presets.write().await;
    match effect_presets.add_preset(preset_name, icon, preset) {
        Ok(_) => "OK",
        Err(_) => "Preset already exists"
    }
}

async fn delete_effect_preset_handler(
    State(state): State<Arc<LightingState>>,
    Path(preset_name): Path<String>
) -> Response {
    let mut effect_presets = state.presets.write().await;
    effect_presets.remove_preset(&preset_name).unwrap();
    "OK".into_response()
}