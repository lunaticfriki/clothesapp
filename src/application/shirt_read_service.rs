use crate::domain::{entities::Shirt, repositories::ShirtRepository};
use std::sync::Arc;

pub trait ShirtReadServiceInterface: shaku::Interface {
    fn get_shirt_by_id(&self, id: &str) -> Result<Option<Shirt>, String>;
    fn get_all_shirts(&self) -> Result<Vec<Shirt>, String>;
}

#[derive(Clone, shaku::Component)]
#[shaku(interface = ShirtReadServiceInterface)]
pub struct ShirtReadService {
    #[shaku(inject)]
    repository: Arc<dyn ShirtRepository>,
}

impl ShirtReadServiceInterface for ShirtReadService {
    fn get_shirt_by_id(&self, id: &str) -> Result<Option<Shirt>, String> {
        self.repository.get_by_id(id)
    }

    fn get_all_shirts(&self) -> Result<Vec<Shirt>, String> {
        self.repository.get_all()
    }
}
