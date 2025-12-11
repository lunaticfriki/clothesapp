use clothes::domain::entities::Pants;
use super::price_mother::PriceMother;

pub struct PantsMother;

impl PantsMother {
    pub fn default() -> Pants {
        Pants::new(
            "pants-1".to_string(),
            "Classic Blue Jeans".to_string(),
            PriceMother::default(),
            "Blue".to_string(),
            "P-001".to_string(),
        )
    }

    pub fn with_discount() -> Pants {
        Pants::new(
            "pants-2".to_string(),
            "Black Chinos".to_string(),
            PriceMother::with_discount(),
            "Black".to_string(),
            "P-002".to_string(),
        )
    }

    pub fn empty() -> Pants {
        Pants::empty()
    }

    pub fn custom(id: &str, name: &str, color: &str, reference: &str) -> Pants {
        Pants::new(
            id.to_string(),
            name.to_string(),
            PriceMother::default(),
            color.to_string(),
            reference.to_string(),
        )
    }

    pub fn with_id(id: &str) -> Pants {
        Pants::new(
            id.to_string(),
            "Test Pants".to_string(),
            PriceMother::default(),
            "Test Color".to_string(),
            "TEST-001".to_string(),
        )
    }
}
