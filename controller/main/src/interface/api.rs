use std::sync::Arc;

use axum::{extract::{Path, Query, State}, response::IntoResponse, routing::{delete, get, post, put}, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::{render::{effects::{AnyEffect, AnyTemporaryEffect, SolidColorEffect}, frame::PixelColor}, LightingState, TOTAL_PIXELS};

// TODO: Authentication

pub fn router() -> Router<Arc<LightingState>> {
    let api_router = Router::new()
        .route("/temporary_effects", get(get_temporary_effect_handlers))
        .route("/temporary_effect", post(create_temporary_effect_handler))
        .route("/temporary_effect/:effect_id", delete(delete_temporary_effect_handler))
        .route("/temporary_effect/:effect_id", put(update_temporary_effect_handler))
        .route("/effect_presets", get(get_effect_presets_handler))
        .route("/effect_preset", post(create_effect_preset_handler))
        .route("/effect_preset/:effect_id", get(get_effect_preset_handler))
        .route("/effect_preset/:effect_id", put(update_effect_preset_handler))
        .route("/effect_preset/:effect_id", delete(delete_effect_preset_handler))
        .route("/run_temporary_effect/:effect_id", post(run_temporary_effect_handler))
        .route("/run_effect", post(run_arbitrary_effect_handler))
        .route("/run_effect/:effect_id", post(run_effect_handler));

    api_router
}

async fn run_arbitrary_effect_handler(
    State(state): State<Arc<LightingState>>,
    Json(effect): Json<Option<AnyEffect>>
) -> impl IntoResponse {
    match effect {
        Some(e) => {
            state.render_state.lock().effect = Box::new(e);
        }
        None => {
            state.render_state.lock().effect = Box::new(SolidColorEffect::new(
                PixelColor::BLACK, 0, TOTAL_PIXELS
            ));
        }
    };

    "OK"
}

async fn run_effect_handler(
    State(state): State<Arc<LightingState>>,
    Path(effect_id): Path<String>
) -> impl IntoResponse {
    let effect_presets = state.presets.read().await;
    let id = match Uuid::parse_str(&effect_id) {
        Ok(id) => id,
        Err(_) => return json!({ "status": "Error", "message": "Invalid UUID" }).to_string(),
    };

    let effect = effect_presets.get_preset(id);
    
    if let Some(effect) = effect {
        state.render_state.lock().effect = Box::new(effect);
    }

    json!({ "status": "OK" }).to_string()
}

async fn run_temporary_effect_handler(
    State(state): State<Arc<LightingState>>,
    Path(effect_id): Path<String>
) -> impl IntoResponse {
    let effect_presets = state.presets.read().await;
    let id = match Uuid::parse_str(&effect_id) {
        Ok(id) => id,
        Err(_) => return json!({ "status": "Error", "message": "Invalid UUID" }).to_string(),
    };
    let effect = effect_presets.get_temporary_effect(id);
    
    if let Some(effect) = effect {
        state.render_state.lock().temporary_effect_compositor.add_effect(effect);
    }

    json!({ "status": "OK" }).to_string()
}

async fn get_temporary_effect_handlers(
    State(state): State<Arc<LightingState>>
) -> impl IntoResponse {
    let effect_presets = state.presets.read().await;
    let effects = effect_presets.get_temporary_effect_list();
    Json(shared::TemporaryEffectList { effects })
}

#[derive(Serialize, Deserialize)]
struct CreateTemporaryEffectParams {
    name: String
}

async fn create_temporary_effect_handler(
    State(state): State<Arc<LightingState>>,
    Query(params): Query<CreateTemporaryEffectParams>,
    Json(effect): Json<AnyTemporaryEffect>
) -> impl IntoResponse {
    let mut effect_presets = state.presets.write().await;
    match effect_presets.add_temporary_effect(params.name, effect) {
        Ok(_) => json!({ "status": "OK" }).to_string(),
        Err(e) => json!({ "status": "Error", "message": e.to_string() }).to_string(),
    }
}

async fn update_temporary_effect_handler(
    State(state): State<Arc<LightingState>>,
    Path(effect_id): Path<String>,
    Query(params): Query<CreateTemporaryEffectParams>,
    Json(effect): Json<AnyTemporaryEffect>
) -> impl IntoResponse {
    let mut effect_presets = state.presets.write().await;
    let id = match Uuid::parse_str(&effect_id) {
        Ok(id) => id,
        Err(_) => return json!({ "status": "Error", "message": "Invalid UUID" }).to_string(),
    };
    
    match effect_presets.update_temporary_effect(id, params.name, effect) {
        Ok(_) => json!({ "status": "OK" }).to_string(),
        Err(e) => json!({ "status": "Error", "message": e.to_string() }).to_string(),
    }
}

async fn delete_temporary_effect_handler(
    State(state): State<Arc<LightingState>>,
    Path(effect_id): Path<String>
) -> impl IntoResponse {
    let mut effect_presets = state.presets.write().await;
    let id = match Uuid::parse_str(&effect_id) {
        Ok(id) => id,
        Err(_) => return json!({ "status": "Error", "message": "Invalid UUID" }).to_string(),
    };
    effect_presets.remove_temporary_effect(id).unwrap();
    json!({ "status": "OK" }).to_string()
}

async fn get_effect_presets_handler(
    State(state): State<Arc<LightingState>>
) -> impl IntoResponse {
    let effect_presets = state.presets.read().await;
    let presets = effect_presets.get_preset_list();
    Json(shared::EffectPresetList { effects: presets })
}

async fn get_effect_preset_handler(
    State(state): State<Arc<LightingState>>,
    Path(preset_id): Path<String>
) -> impl IntoResponse {
    let effect_presets = state.presets.read().await;
    let id = match Uuid::parse_str(&preset_id) {
        Ok(id) => id,
        Err(_) => return Err("Invalid UUID"),
    };
    let preset = effect_presets.get_preset(id);
    if let Some(preset) = preset {
        Ok(Json(preset))
    } else {
        Err("Not found")
    }
}

#[derive(Serialize, Deserialize)]
struct CreateEffectParams {
    name: String,
    icon: String
}

async fn create_effect_preset_handler(
    State(state): State<Arc<LightingState>>,
    Query(params): Query<CreateEffectParams>,
    Json(preset): Json<AnyEffect>
) -> impl IntoResponse {
    let mut effect_presets = state.presets.write().await;
    match effect_presets.add_preset(params.name, params.icon, preset) {
        Ok(_) => Json(serde_json::json!({})),
        Err(e) => {
            let err = e.to_string();
            Json(serde_json::json!({
                "error": err
            }))
        }
    }
}

async fn update_effect_preset_handler(
    State(state): State<Arc<LightingState>>,
    Path(preset_id): Path<String>,
    Query(params): Query<CreateEffectParams>,
    Json(preset): Json<AnyEffect>
) -> impl IntoResponse {
    let mut effect_presets = state.presets.write().await;
    let id = match Uuid::parse_str(&preset_id) {
        Ok(id) => id,
        Err(_) => return json!({ "status": "Error", "message": "Invalid UUID" }).to_string(),
    };
    
    match effect_presets.update_preset(id, params.name, params.icon, preset) {
        Ok(_) => json!({ "status": "OK" }).to_string(),
        Err(e) => json!({ "status": "Error", "message": e.to_string() }).to_string(),
    }
}

async fn delete_effect_preset_handler(
    State(state): State<Arc<LightingState>>,
    Path(preset_id): Path<String>
) -> impl IntoResponse {
    let mut effect_presets = state.presets.write().await;
    let id = match Uuid::parse_str(&preset_id) {
        Ok(id) => id,
        Err(_) => return json!({ "status": "Error", "message": "Invalid UUID" }).to_string(),
    };
    effect_presets.remove_preset(id).unwrap();
    json!({ "status": "OK" }).to_string()
}