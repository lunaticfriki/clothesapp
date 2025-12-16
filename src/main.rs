mod api;
mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;

use api::handlers::{OutfitHandlers, PantsHandlers, ShirtHandlers};
use application::{
    OutfitReadService, OutfitWriteService, 
    PantsReadService, PantsWriteService, 
    ShirtReadService, ShirtWriteService
};
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

  
    let pants_read_service = PantsReadService::new(pants_repo.clone());
    let pants_write_service = PantsWriteService::new(pants_repo);
    let shirt_read_service = ShirtReadService::new(shirt_repo.clone());
    let shirt_write_service = ShirtWriteService::new(shirt_repo);
    let outfit_read_service = OutfitReadService::new(outfit_repo.clone());
    let outfit_write_service = OutfitWriteService::new(outfit_repo);

  
    let pants_handlers = PantsHandlers::new(pants_read_service, pants_write_service);
    let shirt_handlers = ShirtHandlers::new(shirt_read_service, shirt_write_service);
    let outfit_handlers = OutfitHandlers::new(outfit_read_service, outfit_write_service);

  
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
