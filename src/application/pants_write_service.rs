use crate::domain::{entities::Pants, repositories::DynPantsRepository};
use uuid::Uuid;

#[derive(Clone)]
pub struct PantsWriteService {
    repository: DynPantsRepository,
}

impl PantsWriteService {
    pub fn new(repository: DynPantsRepository) -> Self {
        Self { repository }
    }

    pub fn create_pants(&self, mut pants: Pants) -> Result<Pants, String> {
        pants.id = Uuid::new_v4().to_string();
        self.repository.create(pants)
    }

    pub fn update_pants(&self, id: &str, pants: Pants) -> Result<Pants, String> {
        self.repository.update(id, pants)
    }

    pub fn delete_pants(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
