use crate::domain::{entities::Outfit, repositories::DynOutfitRepository};

#[derive(Clone)]
pub struct OutfitReadService {
    repository: DynOutfitRepository,
}

impl OutfitReadService {
    pub fn new(repository: DynOutfitRepository) -> Self {
        Self { repository }
    }

    pub fn get_outfit_by_id(&self, id: &str) -> Result<Option<Outfit>, String> {
        self.repository.get_by_id(id)
    }

    pub fn get_all_outfits(&self) -> Result<Vec<Outfit>, String> {
        self.repository.get_all()
    }
}
