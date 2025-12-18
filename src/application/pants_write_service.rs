use crate::domain::{entities::Pants, repositories::PantsRepository};
use std::sync::Arc;
use uuid::Uuid;

pub trait PantsWriteServiceInterface: shaku::Interface {
    fn create_pants(&self, pants: Pants) -> Result<Pants, String>;
    fn update_pants(&self, id: &str, pants: Pants) -> Result<Pants, String>;
    fn delete_pants(&self, id: &str) -> Result<(), String>;
}

#[derive(Clone, shaku::Component)]
#[shaku(interface = PantsWriteServiceInterface)]
pub struct PantsWriteService {
    #[shaku(inject)]
    repository: Arc<dyn PantsRepository>,
}

impl PantsWriteServiceInterface for PantsWriteService {
    fn create_pants(&self, mut pants: Pants) -> Result<Pants, String> {
        pants.id = Uuid::new_v4().to_string();
        self.repository.create(pants)
    }

    fn update_pants(&self, id: &str, pants: Pants) -> Result<Pants, String> {
        self.repository.update(id, pants)
    }

    fn delete_pants(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
