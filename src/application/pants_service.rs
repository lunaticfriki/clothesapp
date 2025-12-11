use crate::domain::{entities::Pants, repositories::DynPantsRepository};

#[derive(Clone)]
pub struct PantsService {
    repository: DynPantsRepository,
}

impl PantsService {
    pub fn new(repository: DynPantsRepository) -> Self {
        Self { repository }
    }

    pub fn create_pants(&self, pants: Pants) -> Result<Pants, String> {
        self.repository.create(pants)
    }

    pub fn get_pants_by_id(&self, id: &str) -> Result<Option<Pants>, String> {
        self.repository.get_by_id(id)
    }

    pub fn get_all_pants(&self) -> Result<Vec<Pants>, String> {
        self.repository.get_all()
    }

    pub fn update_pants(&self, id: &str, pants: Pants) -> Result<Pants, String> {
        self.repository.update(id, pants)
    }

    pub fn delete_pants(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
