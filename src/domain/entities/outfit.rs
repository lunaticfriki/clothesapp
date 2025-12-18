use serde::{Deserialize, Serialize};
use super::{Pants, Shirt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outfit {
    pub id: String,
    pub shirt: Option<Shirt>,
    pub pants: Option<Pants>,
    pub is_completed: bool,
    pub price: f64,
}

impl Outfit {
    pub fn new(id: String, shirt: Option<Shirt>, pants: Option<Pants>) -> Self {
        let is_completed = shirt.is_some() && pants.is_some();
        let price = Self::calculate_price_from_items(&shirt, &pants);
        Self {
            id,
            shirt,
            pants,
            is_completed,
            price,
        }
    }

    pub fn empty() -> Self {
        Self {
            id: String::new(),
            shirt: None,
            pants: None,
            is_completed: false,
            price: 0.0,
        }
    }

    pub fn update_completion(&mut self) {
        self.is_completed = self.shirt.is_some() && self.pants.is_some();
    }
    
    pub fn calculate_price(&self) -> f64 {
        Self::calculate_price_from_items(&self.shirt, &self.pants)
    }
    
    fn calculate_price_from_items(shirt: &Option<Shirt>, pants: &Option<Pants>) -> f64 {
        let mut total = 0.0;
        
        if let Some(shirt) = shirt {
            total += shirt.price.effective_price();
        }
        
        if let Some(pants) = pants {
            total += pants.price.effective_price();
        }
        
        total
    }
    
    pub fn update_price(&mut self) {
        self.price = self.calculate_price();
    }
}
