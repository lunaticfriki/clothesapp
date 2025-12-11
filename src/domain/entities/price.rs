use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub price: f64,
    pub discounted_price: Option<f64>,
}

impl Price {
    pub fn new(price: f64, discounted_price: Option<f64>) -> Self {
        Self {
            price,
            discounted_price,
        }
    }

    pub fn empty() -> Self {
        Self {
            price: 0.0,
            discounted_price: None,
        }
    }

    pub fn effective_price(&self) -> f64 {
        self.discounted_price.unwrap_or(self.price)
    }
}
