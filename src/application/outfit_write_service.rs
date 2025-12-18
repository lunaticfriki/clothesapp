use crate::domain::{
    entities::Outfit,
    repositories::OutfitRepository
};
use std::sync::Arc;
use uuid::Uuid;

pub trait OutfitWriteServiceInterface: shaku::Interface {
    fn create_outfit(&self, outfit: Outfit) -> Result<Outfit, String>;
    fn update_outfit(&self, id: &str, outfit: Outfit) -> Result<Outfit, String>;
    fn delete_outfit(&self, id: &str) -> Result<(), String>;
}

#[derive(Clone, shaku::Component)]
#[shaku(interface = OutfitWriteServiceInterface)]
pub struct OutfitWriteService {
    #[shaku(inject)]
    repository: Arc<dyn OutfitRepository>,
}

impl OutfitWriteServiceInterface for OutfitWriteService {
    fn create_outfit(&self, mut outfit: Outfit) -> Result<Outfit, String> {
        outfit.id = Uuid::new_v4().to_string();
        outfit.update_completion();
        outfit.update_price();
        self.repository.create(outfit)
    }

    fn update_outfit(&self, id: &str, mut outfit: Outfit) -> Result<Outfit, String> {
        outfit.update_completion();
        outfit.update_price();
        self.repository.update(id, outfit)
    }

    fn delete_outfit(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
