use serde::{Deserialize, Serialize};
use super::price::Price;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pants {
    pub id: String,
    pub name: String,
    pub price: Price,
    pub color: String,
    pub reference: String,
}

impl Pants {
    pub fn new(id: String, name: String, price: Price, color: String, reference: String) -> Self {
        Self {
            id,
            name,
            price,
            color,
            reference,
        }
    }

    pub fn empty() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            price: Price::empty(),
            color: String::new(),
            reference: String::new(),
        }
    }
}
