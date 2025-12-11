use crate::domain::{entities::Outfit, repositories::DynOutfitRepository};

#[derive(Clone)]
pub struct OutfitService {
    repository: DynOutfitRepository,
}

impl OutfitService {
    pub fn new(repository: DynOutfitRepository) -> Self {
        Self { repository }
    }

    pub fn create_outfit(&self, mut outfit: Outfit) -> Result<Outfit, String> {
        outfit.update_completion();
        self.repository.create(outfit)
    }

    pub fn get_outfit_by_id(&self, id: &str) -> Result<Option<Outfit>, String> {
        self.repository.get_by_id(id)
    }

    pub fn get_all_outfits(&self) -> Result<Vec<Outfit>, String> {
        self.repository.get_all()
    }

    pub fn update_outfit(&self, id: &str, mut outfit: Outfit) -> Result<Outfit, String> {
        outfit.update_completion();
        self.repository.update(id, outfit)
    }

    pub fn delete_outfit(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
