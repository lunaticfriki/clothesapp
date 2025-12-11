use crate::domain::{entities::Shirt, repositories::DynShirtRepository};

#[derive(Clone)]
pub struct ShirtService {
    repository: DynShirtRepository,
}

impl ShirtService {
    pub fn new(repository: DynShirtRepository) -> Self {
        Self { repository }
    }

    pub fn create_shirt(&self, shirt: Shirt) -> Result<Shirt, String> {
        self.repository.create(shirt)
    }

    pub fn get_shirt_by_id(&self, id: &str) -> Result<Option<Shirt>, String> {
        self.repository.get_by_id(id)
    }

    pub fn get_all_shirts(&self) -> Result<Vec<Shirt>, String> {
        self.repository.get_all()
    }

    pub fn update_shirt(&self, id: &str, shirt: Shirt) -> Result<Shirt, String> {
        self.repository.update(id, shirt)
    }

    pub fn delete_shirt(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
