use crate::domain::entities::Shirt;
use std::sync::Arc;

pub trait ShirtRepository: shaku::Interface + Send + Sync {
    fn create(&self, shirt: Shirt) -> Result<Shirt, String>;
    fn get_by_id(&self, id: &str) -> Result<Option<Shirt>, String>;
    fn get_all(&self) -> Result<Vec<Shirt>, String>;
    fn update(&self, id: &str, shirt: Shirt) -> Result<Shirt, String>;
    fn delete(&self, id: &str) -> Result<(), String>;
}
