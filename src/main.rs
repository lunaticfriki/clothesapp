mod api;
mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;

use api::handlers::{OutfitHandlers, PantsHandlers, ShirtHandlers};
use application::{OutfitService, PantsService, ShirtService};
use infrastructure::{seed_data, InMemoryOutfitRepository, InMemoryPantsRepository, InMemoryShirtRepository};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let pants_repo = Arc::new(InMemoryPantsRepository::new());
    let shirt_repo = Arc::new(InMemoryShirtRepository::new());  
    let outfit_repo = Arc::new(InMemoryOutfitRepository::new());

 
    if let Err(e) = seed_data(pants_repo.as_ref(), shirt_repo.as_ref(), outfit_repo.as_ref()) {
        eprintln!("Failed to seed data: {}", e);
        return;
    }
    tracing::info!("Database seeded successfully");

  
    let pants_service = PantsService::new(pants_repo);
    let shirt_service = ShirtService::new(shirt_repo);
    let outfit_service = OutfitService::new(outfit_repo);

  
    let pants_handlers = PantsHandlers::new(pants_service);
    let shirt_handlers = ShirtHandlers::new(shirt_service);
    let outfit_handlers = OutfitHandlers::new(outfit_service);

  
    let app = api::create_router(pants_handlers, shirt_handlers, outfit_handlers);

    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();
    
    tracing::info!("Server running on http://localhost:8080");
    tracing::info!("Available endpoints:");
    tracing::info!("  GET    /pants");
    tracing::info!("  POST   /pants");
    tracing::info!("  GET    /pants/:id");
    tracing::info!("  PUT    /pants/:id");
    tracing::info!("  DELETE /pants/:id");
    tracing::info!("  GET    /shirts");
    tracing::info!("  POST   /shirts");
    tracing::info!("  GET    /shirts/:id");
    tracing::info!("  PUT    /shirts/:id");
    tracing::info!("  DELETE /shirts/:id");
    tracing::info!("  GET    /outfits");
    tracing::info!("  POST   /outfits");
    tracing::info!("  GET    /outfits/:id");
    tracing::info!("  PUT    /outfits/:id");
    tracing::info!("  DELETE /outfits/:id");

    axum::serve(listener, app).await.unwrap();
}
