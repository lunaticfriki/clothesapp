use crate::domain::entities::Outfit;
use std::sync::Arc;

pub trait OutfitRepository: shaku::Interface + Send + Sync {
    fn create(&self, outfit: Outfit) -> Result<Outfit, String>;
    fn get_by_id(&self, id: &str) -> Result<Option<Outfit>, String>;
    fn get_all(&self) -> Result<Vec<Outfit>, String>;
    fn update(&self, id: &str, outfit: Outfit) -> Result<Outfit, String>;
    fn delete(&self, id: &str) -> Result<(), String>;
}

