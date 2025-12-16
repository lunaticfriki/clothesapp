use crate::domain::{entities::Shirt, repositories::DynShirtRepository};

#[derive(Clone)]
pub struct ShirtWriteService {
    repository: DynShirtRepository,
}

impl ShirtWriteService {
    pub fn new(repository: DynShirtRepository) -> Self {
        Self { repository }
    }

    pub fn create_shirt(&self, shirt: Shirt) -> Result<Shirt, String> {
        self.repository.create(shirt)
    }

    pub fn update_shirt(&self, id: &str, shirt: Shirt) -> Result<Shirt, String> {
        self.repository.update(id, shirt)
    }

    pub fn delete_shirt(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
