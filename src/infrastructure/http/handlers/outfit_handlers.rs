use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use std::sync::Arc;

use crate::{
    infrastructure::http::dto::OutfitDTO,
    application::{
        outfit_read_service::OutfitReadServiceInterface,
        outfit_write_service::OutfitWriteServiceInterface,
    },
    domain::entities::Outfit
};

pub trait OutfitHandlersInterface: shaku::Interface {}

#[derive(Clone, shaku::Component)]
#[shaku(interface = OutfitHandlersInterface)]
pub struct OutfitHandlers {
    #[shaku(inject)]
    read_service: Arc<dyn OutfitReadServiceInterface>,
    #[shaku(inject)]
    write_service: Arc<dyn OutfitWriteServiceInterface>,
}

impl OutfitHandlersInterface for OutfitHandlers {}

pub async fn create_outfit(
    State(handlers): State<OutfitHandlers>,
    Json(request): Json<OutfitDTO>,
) -> impl IntoResponse {
    let outfit = request.to_outfit();
    match handlers.write_service.create_outfit(outfit) {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn get_outfit_by_id(
    State(handlers): State<OutfitHandlers>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match handlers.read_service.get_outfit_by_id(&id) {
        Ok(Some(outfit)) => (StatusCode::OK, Json(outfit)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Outfit not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn get_all_outfits(State(handlers): State<OutfitHandlers>) -> impl IntoResponse {
    match handlers.read_service.get_all_outfits() {
        Ok(outfits) => (StatusCode::OK, Json(outfits)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn update_outfit(
    State(handlers): State<OutfitHandlers>,
    Path(id): Path<String>,
    Json(outfit): Json<Outfit>,
) -> impl IntoResponse {
    match handlers.write_service.update_outfit(&id, outfit) {
        Ok(updated) => (StatusCode::OK, Json(updated)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn delete_outfit(
    State(handlers): State<OutfitHandlers>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match handlers.write_service.delete_outfit(&id) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}
