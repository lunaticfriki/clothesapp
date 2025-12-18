use clothes::domain::entities::Outfit;
use super::{PantsMother, ShirtMother};

pub struct OutfitMother;

impl OutfitMother {
    pub fn complete() -> Outfit {
        Outfit::new(
            "outfit-1".to_string(),
            Some(ShirtMother::default()),
            Some(PantsMother::default()),
        )
    }

    pub fn incomplete_no_shirt() -> Outfit {
        Outfit::new(
            "outfit-2".to_string(),
            None,
            Some(PantsMother::default()),
        )
    }

    pub fn incomplete_no_pants() -> Outfit {
        Outfit::new(
            "outfit-3".to_string(),
            Some(ShirtMother::default()),
            None,
        )
    }

    pub fn empty() -> Outfit {
        Outfit::empty()
    }

    pub fn custom(id: &str, shirt: Option<clothes::domain::entities::Shirt>, pants: Option<clothes::domain::entities::Pants>) -> Outfit {
        Outfit::new(id.to_string(), shirt, pants)
    }
    
    pub fn with_discounts() -> Outfit {
        Outfit::new(
            "outfit-discounted".to_string(),
            Some(ShirtMother::with_discount()),
            Some(PantsMother::with_discount()),
        )
    }
}
