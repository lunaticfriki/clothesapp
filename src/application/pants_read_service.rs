use crate::domain::{entities::Pants, repositories::DynPantsRepository};

#[derive(Clone)]
pub struct PantsReadService {
    repository: DynPantsRepository,
}

impl PantsReadService {
    pub fn new(repository: DynPantsRepository) -> Self {
        Self { repository }
    }

    pub fn get_pants_by_id(&self, id: &str) -> Result<Option<Pants>, String> {
        self.repository.get_by_id(id)
    }

    pub fn get_all_pants(&self) -> Result<Vec<Pants>, String> {
        self.repository.get_all()
    }
}
