use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::infrastructure::http::handlers::{
    outfit_handlers::{
        create_outfit, delete_outfit, get_all_outfits, get_outfit_by_id, update_outfit,
        OutfitHandlers,
    },
    pants_handlers::{
        create_pants, delete_pants, get_all_pants, get_pants_by_id, update_pants, PantsHandlers,
    },
    shirt_handlers::{
        create_shirt, delete_shirt, get_all_shirts, get_shirt_by_id, update_shirt, ShirtHandlers,
    },
};

pub fn create_router(
    pants_handlers: PantsHandlers,
    shirt_handlers: ShirtHandlers,
    outfit_handlers: OutfitHandlers,
) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/pants", get(get_all_pants).post(create_pants))
        .route(
            "/pants/:id",
            get(get_pants_by_id).put(update_pants).delete(delete_pants),
        )
        .with_state(pants_handlers)
        .route("/shirts", get(get_all_shirts).post(create_shirt))
        .route(
            "/shirts/:id",
            get(get_shirt_by_id).put(update_shirt).delete(delete_shirt),
        )
        .with_state(shirt_handlers)
        .route("/outfits", get(get_all_outfits).post(create_outfit))
        .route(
            "/outfits/:id",
            get(get_outfit_by_id)
                .put(update_outfit)
                .delete(delete_outfit),
        )
        .with_state(outfit_handlers)
        .layer(cors)
}
