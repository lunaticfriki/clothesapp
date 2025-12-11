use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outfit {
    pub id: String,
    pub shirt_id: Option<String>,
    pub pants_id: Option<String>,
    pub is_completed: bool,
}

impl Outfit {
    pub fn new(id: String, shirt_id: Option<String>, pants_id: Option<String>) -> Self {
        let is_completed = shirt_id.is_some() && pants_id.is_some();
        Self {
            id,
            shirt_id,
            pants_id,
            is_completed,
        }
    }

    pub fn empty() -> Self {
        Self {
            id: String::new(),
            shirt_id: None,
            pants_id: None,
            is_completed: false,
        }
    }

    pub fn update_completion(&mut self) {
        self.is_completed = self.shirt_id.is_some() && self.pants_id.is_some();
    }
}
