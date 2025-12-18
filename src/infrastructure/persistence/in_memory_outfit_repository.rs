use crate::domain::{entities::Outfit, repositories::OutfitRepository};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(shaku::Component)]
#[shaku(interface = OutfitRepository)]
pub struct InMemoryOutfitRepository {
    #[shaku(default)]
    storage: Arc<RwLock<HashMap<String, Outfit>>>,
}

impl Default for InMemoryOutfitRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryOutfitRepository {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl OutfitRepository for InMemoryOutfitRepository {
    fn create(&self, outfit: Outfit) -> Result<Outfit, String> {
        let mut storage = self.storage.write().map_err(|e| e.to_string())?;
        
        if storage.contains_key(&outfit.id) {
            return Err(format!("Outfit with id {} already exists", outfit.id));
        }
        
        storage.insert(outfit.id.clone(), outfit.clone());
        Ok(outfit)
    }

    fn get_by_id(&self, id: &str) -> Result<Option<Outfit>, String> {
        let storage = self.storage.read().map_err(|e| e.to_string())?;
        Ok(storage.get(id).cloned())
    }

    fn get_all(&self) -> Result<Vec<Outfit>, String> {
        let storage = self.storage.read().map_err(|e| e.to_string())?;
        Ok(storage.values().cloned().collect())
    }

    fn update(&self, id: &str, outfit: Outfit) -> Result<Outfit, String> {
        let mut storage = self.storage.write().map_err(|e| e.to_string())?;
        
        if !storage.contains_key(id) {
            return Err(format!("Outfit with id {} not found", id));
        }
        
        storage.insert(id.to_string(), outfit.clone());
        Ok(outfit)
    }

    fn delete(&self, id: &str) -> Result<(), String> {
        let mut storage = self.storage.write().map_err(|e| e.to_string())?;
        
        if storage.remove(id).is_none() {
            return Err(format!("Outfit with id {} not found", id));
        }
        
        Ok(())
    }
}
