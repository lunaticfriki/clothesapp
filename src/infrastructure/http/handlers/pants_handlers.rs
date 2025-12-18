use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use std::sync::Arc;

use crate::{
    infrastructure::http::dto::PantsDTO,
    application::{
        pants_read_service::PantsReadServiceInterface,
        pants_write_service::PantsWriteServiceInterface,
    },
    domain::entities::Pants
};

pub trait PantsHandlersInterface: shaku::Interface {}

#[derive(Clone, shaku::Component)]
#[shaku(interface = PantsHandlersInterface)]
pub struct PantsHandlers {
    #[shaku(inject)]
    read_service: Arc<dyn PantsReadServiceInterface>,
    #[shaku(inject)]
    write_service: Arc<dyn PantsWriteServiceInterface>,
}

impl PantsHandlersInterface for PantsHandlers {}

pub async fn create_pants(
    State(handlers): State<PantsHandlers>,
    Json(request): Json<PantsDTO>,
) -> impl IntoResponse {
    let pants = request.to_pants();
    match handlers.write_service.create_pants(pants) {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn get_pants_by_id(
    State(handlers): State<PantsHandlers>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match handlers.read_service.get_pants_by_id(&id) {
        Ok(Some(pants)) => (StatusCode::OK, Json(pants)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Pants not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn get_all_pants(State(handlers): State<PantsHandlers>) -> impl IntoResponse {
    match handlers.read_service.get_all_pants() {
        Ok(pants) => (StatusCode::OK, Json(pants)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn update_pants(
    State(handlers): State<PantsHandlers>,
    Path(id): Path<String>,
    Json(pants): Json<Pants>,
) -> impl IntoResponse {
    match handlers.write_service.update_pants(&id, pants) {
        Ok(updated) => (StatusCode::OK, Json(updated)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn delete_pants(
    State(handlers): State<PantsHandlers>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match handlers.write_service.delete_pants(&id) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}
