use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use std::sync::Arc;

use crate::{
    infrastructure::http::dto::ShirtDTO,
    application::{
        shirt_read_service::ShirtReadServiceInterface,
        shirt_write_service::ShirtWriteServiceInterface,
    },
    domain::entities::Shirt
};

pub trait ShirtHandlersInterface: shaku::Interface {}

#[derive(Clone, shaku::Component)]
#[shaku(interface = ShirtHandlersInterface)]
pub struct ShirtHandlers {
    #[shaku(inject)]
    read_service: Arc<dyn ShirtReadServiceInterface>,
    #[shaku(inject)]
    write_service: Arc<dyn ShirtWriteServiceInterface>,
}

impl ShirtHandlersInterface for ShirtHandlers {}

pub async fn create_shirt(
    State(handlers): State<ShirtHandlers>,
    Json(request): Json<ShirtDTO>,
) -> impl IntoResponse {
    let shirt = request.to_shirt();
    match handlers.write_service.create_shirt(shirt) {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn get_shirt_by_id(
    State(handlers): State<ShirtHandlers>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match handlers.read_service.get_shirt_by_id(&id) {
        Ok(Some(shirt)) => (StatusCode::OK, Json(shirt)).into_response(),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Shirt not found" })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn get_all_shirts(State(handlers): State<ShirtHandlers>) -> impl IntoResponse {
    match handlers.read_service.get_all_shirts() {
        Ok(shirts) => (StatusCode::OK, Json(shirts)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn update_shirt(
    State(handlers): State<ShirtHandlers>,
    Path(id): Path<String>,
    Json(shirt): Json<Shirt>,
) -> impl IntoResponse {
    match handlers.write_service.update_shirt(&id, shirt) {
        Ok(updated) => (StatusCode::OK, Json(updated)).into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}

pub async fn delete_shirt(
    State(handlers): State<ShirtHandlers>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match handlers.write_service.delete_shirt(&id) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )
            .into_response(),
    }
}
