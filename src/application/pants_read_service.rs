use crate::domain::{entities::Pants, repositories::PantsRepository};
use std::sync::Arc;

pub trait PantsReadServiceInterface: shaku::Interface {
    fn get_pants_by_id(&self, id: &str) -> Result<Option<Pants>, String>;
    fn get_all_pants(&self) -> Result<Vec<Pants>, String>;
}

#[derive(Clone, shaku::Component)]
#[shaku(interface = PantsReadServiceInterface)]
pub struct PantsReadService {
    #[shaku(inject)]
    repository: Arc<dyn PantsRepository>,
}

impl PantsReadServiceInterface for PantsReadService {
    fn get_pants_by_id(&self, id: &str) -> Result<Option<Pants>, String> {
        self.repository.get_by_id(id)
    }

    fn get_all_pants(&self) -> Result<Vec<Pants>, String> {
        self.repository.get_all()
    }
}
