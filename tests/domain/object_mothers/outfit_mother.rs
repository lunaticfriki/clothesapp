use clothes::domain::entities::Outfit;

pub struct OutfitMother;

impl OutfitMother {
    pub fn complete() -> Outfit {
        Outfit::new(
            "outfit-1".to_string(),
            Some("shirt-1".to_string()),
            Some("pants-1".to_string()),
        )
    }

    pub fn incomplete_no_shirt() -> Outfit {
        Outfit::new(
            "outfit-2".to_string(),
            None,
            Some("pants-1".to_string()),
        )
    }

    pub fn incomplete_no_pants() -> Outfit {
        Outfit::new(
            "outfit-3".to_string(),
            Some("shirt-1".to_string()),
            None,
        )
    }

    pub fn empty() -> Outfit {
        Outfit::empty()
    }

    pub fn custom(id: &str, shirt_id: Option<String>, pants_id: Option<String>) -> Outfit {
        Outfit::new(id.to_string(), shirt_id, pants_id)
    }
}
