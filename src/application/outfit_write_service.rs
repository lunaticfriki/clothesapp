use crate::domain::{
    entities::Outfit, 
    repositories::DynOutfitRepository
};
use uuid::Uuid;

#[derive(Clone)]
pub struct OutfitWriteService {
    repository: DynOutfitRepository,
}

impl OutfitWriteService {
    pub fn new(repository: DynOutfitRepository) -> Self {
        Self { repository }
    }

    pub fn create_outfit(&self, mut outfit: Outfit) -> Result<Outfit, String> {
        // Auto-generate ID
        outfit.id = Uuid::new_v4().to_string();
        outfit.update_completion();
        outfit.update_price();
        self.repository.create(outfit)
    }

    pub fn update_outfit(&self, id: &str, mut outfit: Outfit) -> Result<Outfit, String> {
        outfit.update_completion();
        outfit.update_price();
        self.repository.update(id, outfit)
    }

    pub fn delete_outfit(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
