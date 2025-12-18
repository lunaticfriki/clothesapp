use crate::domain::{entities::Pants, repositories::PantsRepository};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(shaku::Component)]
#[shaku(interface = PantsRepository)]
pub struct InMemoryPantsRepository {
    #[shaku(default)]
    storage: Arc<RwLock<HashMap<String, Pants>>>,
}

impl Default for InMemoryPantsRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryPantsRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl PantsRepository for InMemoryPantsRepository {
    fn create(&self, pants: Pants) -> Result<Pants, String> {
        let mut storage = self.storage.write().map_err(|e| e.to_string())?;
        
        if storage.contains_key(&pants.id) {
            return Err(format!("Pants with id {} already exists", pants.id));
        }
        
        storage.insert(pants.id.clone(), pants.clone());
        Ok(pants)
    }

    fn get_by_id(&self, id: &str) -> Result<Option<Pants>, String> {
        let storage = self.storage.read().map_err(|e| e.to_string())?;
        Ok(storage.get(id).cloned())
    }

    fn get_all(&self) -> Result<Vec<Pants>, String> {
        let storage = self.storage.read().map_err(|e| e.to_string())?;
        Ok(storage.values().cloned().collect())
    }

    fn update(&self, id: &str, pants: Pants) -> Result<Pants, String> {
        let mut storage = self.storage.write().map_err(|e| e.to_string())?;
        
        if !storage.contains_key(id) {
            return Err(format!("Pants with id {} not found", id));
        }
        
        storage.insert(id.to_string(), pants.clone());
        Ok(pants)
    }

    fn delete(&self, id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().map_err(|e| e.to_string())?;
        
        if storage.remove(id).is_none() {
            return Err(format!("Pants with id {} not found", id));
        }
        
        Ok(())
    }
}
