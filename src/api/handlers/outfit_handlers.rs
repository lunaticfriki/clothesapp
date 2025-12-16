use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{application::{OutfitReadService, OutfitWriteService}, domain::entities::Outfit};

#[derive(Clone)]
pub struct OutfitHandlers {
    read_service: OutfitReadService,
    write_service: OutfitWriteService,
}

impl OutfitHandlers {
    pub fn new(read_service: OutfitReadService, write_service: OutfitWriteService) -> Self {
        Self { read_service, write_service }
    }
}

pub async fn create_outfit(
    State(handlers): State<OutfitHandlers>,
    Json(outfit): Json<Outfit>,
) -> impl IntoResponse {
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
