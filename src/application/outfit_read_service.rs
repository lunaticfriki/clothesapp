use crate::domain::{entities::Outfit, repositories::OutfitRepository};
use std::sync::Arc;

pub trait OutfitReadServiceInterface: shaku::Interface {
    fn get_outfit_by_id(&self, id: &str) -> Result<Option<Outfit>, String>;
    fn get_all_outfits(&self) -> Result<Vec<Outfit>, String>;
}

#[derive(Clone, shaku::Component)]
#[shaku(interface = OutfitReadServiceInterface)]
pub struct OutfitReadService {
    #[shaku(inject)]
    repository: Arc<dyn OutfitRepository>,
}

impl OutfitReadServiceInterface for OutfitReadService {
    fn get_outfit_by_id(&self, id: &str) -> Result<Option<Outfit>, String> {
        self.repository.get_by_id(id)
    }

    fn get_all_outfits(&self) -> Result<Vec<Outfit>, String> {
        self.repository.get_all()
    }
}
