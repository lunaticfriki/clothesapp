mod application;
mod domain;
mod infrastructure;
mod di;

use std::sync::Arc;

use infrastructure::http::handlers::{OutfitHandlers, PantsHandlers, ShirtHandlers};
use di::AppModule;
use infrastructure::seed_data;
use shaku::HasComponent;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let module = AppModule::builder().build();

    {
        use crate::domain::repositories::{PantsRepository, ShirtRepository, OutfitRepository};
        
        let pants_repo: Arc<dyn PantsRepository> = module.resolve();
        let shirt_repo: Arc<dyn ShirtRepository> = module.resolve();
        let outfit_repo: Arc<dyn OutfitRepository> = module.resolve();
        
        if let Err(e) = seed_data(pants_repo.as_ref(), shirt_repo.as_ref(), outfit_repo.as_ref()) {
            eprintln!("Failed to seed data: {}", e);
            return;
        }
        tracing::info!("Database seeded successfully");
    }

    use crate::infrastructure::http::handlers::{
        pants_handlers::PantsHandlersInterface,
        shirt_handlers::ShirtHandlersInterface,
        outfit_handlers::OutfitHandlersInterface,
    };
    
    let pants_trait: Arc<dyn PantsHandlersInterface> = module.resolve();
    let shirt_trait: Arc<dyn ShirtHandlersInterface> = module.resolve();
    let outfit_trait: Arc<dyn OutfitHandlersInterface> = module.resolve();
    
    let pants_handlers: PantsHandlers = unsafe {
        let ptr = Arc::into_raw(pants_trait);
        let concrete_ptr = ptr as *const PantsHandlers;
        (*concrete_ptr).clone()
    };
    
    let shirt_handlers: ShirtHandlers = unsafe {
        let ptr = Arc::into_raw(shirt_trait);
        let concrete_ptr = ptr as *const ShirtHandlers;
        (*concrete_ptr).clone()
    };
    
    let outfit_handlers: OutfitHandlers = unsafe {
        let ptr = Arc::into_raw(outfit_trait);
        let concrete_ptr = ptr as *const OutfitHandlers;
        (*concrete_ptr).clone()
    };

    let app = infrastructure::create_router(
        pants_handlers,
        shirt_handlers,
        outfit_handlers,
    );

    
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
