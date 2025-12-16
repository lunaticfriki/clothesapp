use crate::domain::{entities::Shirt, repositories::DynShirtRepository};

#[derive(Clone)]
pub struct ShirtReadService {
    repository: DynShirtRepository,
}

impl ShirtReadService {
    pub fn new(repository: DynShirtRepository) -> Self {
        Self { repository }
    }

    pub fn get_shirt_by_id(&self, id: &str) -> Result<Option<Shirt>, String> {
        self.repository.get_by_id(id)
    }

    pub fn get_all_shirts(&self) -> Result<Vec<Shirt>, String> {
        self.repository.get_all()
    }
}
