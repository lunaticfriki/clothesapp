use serde::{Deserialize, Serialize};
use crate::domain::entities::{Outfit, Pants, Price, Shirt};

#[derive(Debug, Deserialize, Serialize)]
pub struct PantsDTO {
    pub name: String,
    pub price: Price,
    pub color: String,
    pub reference: String,
}

impl PantsDTO {
    pub fn to_pants(self) -> Pants {
        Pants::new(
            String::new(), 
            self.name,
            self.price,
            self.color,
            self.reference,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ShirtDTO {
    pub name: String,
    pub price: Price,
    pub color: String,
    pub reference: String,
}

impl ShirtDTO {
    pub fn to_shirt(self) -> Shirt {
        Shirt::new(
            String::new(),
            self.name,
            self.price,
            self.color,
            self.reference,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutfitDTO {
    pub shirt: Option<Shirt>,
    pub pants: Option<Pants>,
}

impl OutfitDTO {
    pub fn to_outfit(self) -> Outfit {
        Outfit::new(
            String::new(), 
            self.shirt,
            self.pants,
        )
    }
}
