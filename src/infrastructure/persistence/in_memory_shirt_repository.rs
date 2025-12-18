use crate::domain::{entities::Shirt, repositories::ShirtRepository};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(shaku::Component)]
#[shaku(interface = ShirtRepository)]
pub struct InMemoryShirtRepository {
    #[shaku(default)]
    storage: Arc<RwLock<HashMap<String, Shirt>>>,
}

impl Default for InMemoryShirtRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryShirtRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl ShirtRepository for InMemoryShirtRepository {
    fn create(&self, shirt: Shirt) -> Result<Shirt, String> {
        let mut storage = self.storage.write().map_err(|e| e.to_string())?;
        
        if storage.contains_key(&shirt.id) {
            return Err(format!("Shirt with id {} already exists", shirt.id));
        }
        
        storage.insert(shirt.id.clone(), shirt.clone());
        Ok(shirt)
    }

    fn get_by_id(&self, id: &str) -> Result<Option<Shirt>, String> {
        let storage = self.storage.read().map_err(|e| e.to_string())?;
        Ok(storage.get(id).cloned())
    }

    fn get_all(&self) -> Result<Vec<Shirt>, String> {
        let storage = self.storage.read().map_err(|e| e.to_string())?;
        Ok(storage.values().cloned().collect())
    }

    fn update(&self, id: &str, shirt: Shirt) -> Result<Shirt, String> {
        let mut storage = self.storage.write().map_err(|e| e.to_string())?;
        
        if !storage.contains_key(id) {
            return Err(format!("Shirt with id {} not found", id));
        }
        
        storage.insert(id.to_string(), shirt.clone());
        Ok(shirt)
    }

    fn delete(&self, id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().map_err(|e| e.to_string())?;
        
        if storage.remove(id).is_none() {
            return Err(format!("Shirt with id {} not found", id));
        }
        
        Ok(())
    }
}
