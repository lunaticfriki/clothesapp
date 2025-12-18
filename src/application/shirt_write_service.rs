use crate::domain::{entities::Shirt, repositories::ShirtRepository};
use std::sync::Arc;
use uuid::Uuid;

pub trait ShirtWriteServiceInterface: shaku::Interface {
    fn create_shirt(&self, shirt: Shirt) -> Result<Shirt, String>;
    fn update_shirt(&self, id: &str, shirt: Shirt) -> Result<Shirt, String>;
    fn delete_shirt(&self, id: &str) -> Result<(), String>;
}

#[derive(Clone, shaku::Component)]
#[shaku(interface = ShirtWriteServiceInterface)]
pub struct ShirtWriteService {
    #[shaku(inject)]
    repository: Arc<dyn ShirtRepository>,
}

impl ShirtWriteServiceInterface for ShirtWriteService {
    fn create_shirt(&self, mut shirt: Shirt) -> Result<Shirt, String> {
        shirt.id = Uuid::new_v4().to_string();
        self.repository.create(shirt)
    }

    fn update_shirt(&self, id: &str, shirt: Shirt) -> Result<Shirt, String> {
        self.repository.update(id, shirt)
    }

    fn delete_shirt(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id)
    }
}
