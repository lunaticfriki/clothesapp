use crate::domain::{entities::Pants, repositories::DynPantsRepository};

#[derive(Clone)]
pub struct PantsWriteService {
    repository: DynPantsRepository,
}

impl PantsWriteService {
    pub fn new(repository: DynPantsRepository) -> Self {
        Self { repository }
    }

    pub fn create_pants(&self, pants: Pants) -> Result<Pants, String> {
        self.repository.create(pants)
    }

    pub fn update_pants(&self, id: &str, pants: Pants) -> Result<Pants, String> {
        self.repository.update(id, pants)
    }

    pub fn delete_pants(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
