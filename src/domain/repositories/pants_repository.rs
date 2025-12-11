use crate::domain::entities::Pants;
use std::sync::Arc;

pub trait PantsRepository: Send + Sync {
    fn create(&self, pants: Pants) -> Result<Pants, String>;
    fn get_by_id(&self, id: &str) -> Result<Option<Pants>, String>;
    fn get_all(&self) -> Result<Vec<Pants>, String>;
    fn update(&self, id: &str, pants: Pants) -> Result<Pants, String>;
    fn delete(&self, id: &str) -> Result<(), String>;
}

pub type DynPantsRepository = Arc<dyn PantsRepository>;
