use clothes::domain::entities::Shirt;
use super::price_mother::PriceMother;

pub struct ShirtMother;

impl ShirtMother {
    pub fn default() -> Shirt {
        Shirt::new(
            "shirt-1".to_string(),
            "White Oxford Shirt".to_string(),
            PriceMother::default(),
            "White".to_string(),
            "S-001".to_string(),
        )
    }

    pub fn with_discount() -> Shirt {
        Shirt::new(
            "shirt-2".to_string(),
            "Blue Denim Shirt".to_string(),
            PriceMother::with_discount(),
            "Blue".to_string(),
            "S-002".to_string(),
        )
    }

    pub fn empty() -> Shirt {
        Shirt::empty()
    }

    pub fn custom(id: &str, name: &str, color: &str, reference: &str) -> Shirt {
        Shirt::new(
            id.to_string(),
            name.to_string(),
            PriceMother::default(),
            color.to_string(),
            reference.to_string(),
        )
    }

    pub fn with_id(id: &str) -> Shirt {
        Shirt::new(
            id.to_string(),
            "Test Shirt".to_string(),
            PriceMother::default(),
            "Test Color".to_string(),
            "TEST-001".to_string(),
        )
    }
}
