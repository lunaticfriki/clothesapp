use crate::domain::entities::{Outfit, Pants, Price, Shirt};
use crate::domain::repositories::{OutfitRepository, PantsRepository, ShirtRepository};
use uuid::Uuid;

pub fn seed_data(
    pants_repo: &dyn PantsRepository,
    shirt_repo: &dyn ShirtRepository,
    outfit_repo: &dyn OutfitRepository,
) -> Result<(), String> {
    let pants_data = vec![
        ("Classic Blue Jeans", 79.99, Some(59.99), "Navy Blue", "P-001"),
        ("Black Chinos", 69.99, None, "Black", "P-002"),
        ("Khaki Cargo Pants", 89.99, Some(69.99), "Khaki", "P-003"),
        ("Grey Dress Pants", 99.99, None, "Charcoal Grey", "P-004"),
        ("Olive Green Joggers", 54.99, Some(44.99), "Olive Green", "P-005"),
        ("Dark Denim Jeans", 84.99, None, "Dark Blue", "P-006"),
        ("Beige Linen Pants", 74.99, Some(59.99), "Beige", "P-007"),
        ("White Chinos", 69.99, None, "White", "P-008"),
        ("Brown Corduroy Pants", 79.99, Some(64.99), "Brown", "P-009"),
        ("Navy Dress Pants", 94.99, None, "Navy", "P-010"),
        ("Light Blue Jeans", 79.99, Some(59.99), "Light Blue", "P-011"),
        ("Black Joggers", 49.99, None, "Black", "P-012"),
        ("Tan Cargo Pants", 89.99, Some(74.99), "Tan", "P-013"),
        ("Burgundy Chinos", 74.99, None, "Burgundy", "P-014"),
        ("Forest Green Pants", 84.99, Some(69.99), "Forest Green", "P-015"),
        ("Stone Grey Jeans", 79.99, None, "Stone Grey", "P-016"),
        ("Cream Linen Pants", 74.99, Some(59.99), "Cream", "P-017"),
        ("Midnight Blue Chinos", 69.99, None, "Midnight Blue", "P-018"),
        ("Rust Orange Pants", 79.99, Some(64.99), "Rust Orange", "P-019"),
        ("Slate Grey Dress Pants", 99.99, None, "Slate Grey", "P-020"),
    ];

    for (name, price, discounted, color, reference) in pants_data {
        let pants = Pants::new(
            Uuid::new_v4().to_string(),
            name.to_string(),
            Price::new(price, discounted),
            color.to_string(),
            reference.to_string(),
        );
        pants_repo.create(pants)?;
    }

    let shirts_data = vec![
        ("White Oxford Shirt", 59.99, Some(44.99), "White", "S-001"),
        ("Blue Denim Shirt", 64.99, None, "Light Blue", "S-002"),
        ("Black T-Shirt", 29.99, Some(19.99), "Black", "S-003"),
        ("Grey Polo Shirt", 49.99, None, "Heather Grey", "S-004"),
        ("Navy Henley", 39.99, Some(29.99), "Navy", "S-005"),
        ("Red Flannel Shirt", 69.99, None, "Red Plaid", "S-006"),
        ("Green Linen Shirt", 54.99, Some(44.99), "Sage Green", "S-007"),
        ("Pink Button-Down", 59.99, None, "Light Pink", "S-008"),
        ("Yellow Polo", 49.99, Some(39.99), "Mustard Yellow", "S-009"),
        ("Purple Dress Shirt", 74.99, None, "Deep Purple", "S-010"),
        ("Striped Blue Shirt", 64.99, Some(49.99), "Blue/White Stripe", "S-011"),
        ("Olive Green T-Shirt", 29.99, None, "Olive", "S-012"),
        ("Charcoal Henley", 39.99, Some(29.99), "Charcoal", "S-013"),
        ("Beige Linen Shirt", 54.99, None, "Beige", "S-014"),
        ("Teal Polo Shirt", 49.99, Some(39.99), "Teal", "S-015"),
        ("Burgundy Flannel", 69.99, None, "Burgundy Plaid", "S-016"),
        ("Cream Oxford Shirt", 59.99, Some(44.99), "Cream", "S-017"),
        ("Sky Blue Dress Shirt", 74.99, None, "Sky Blue", "S-018"),
        ("Forest Green Polo", 49.99, Some(39.99), "Forest Green", "S-019"),
        ("Maroon T-Shirt", 29.99, None, "Maroon", "S-020"),
    ];

    let mut shirt_ids = Vec::new();
    for (name, price, discounted, color, reference) in shirts_data {
        let shirt = Shirt::new(
            Uuid::new_v4().to_string(),
            name.to_string(),
            Price::new(price, discounted),
            color.to_string(),
            reference.to_string(),
        );
        let id = shirt.id.clone();
        shirt_repo.create(shirt)?;
        shirt_ids.push(id);
    }

    let all_pants = pants_repo.get_all()?;
    let pants_ids: Vec<String> = all_pants.iter().map(|p| p.id.clone()).collect();

    let outfit_data = vec![
        (Some(0), Some(0)),
        (Some(1), Some(2)),
        (Some(3), None),
        (None, Some(10)),
        (None, None),
    ];

    for (shirt_idx, pants_idx) in outfit_data {
        let shirt_id = shirt_idx.and_then(|idx| {
            if idx < shirt_ids.len() {
                Some(shirt_ids[idx].clone())
            } else {
                None
            }
        });
        
        let pants_id = pants_idx.and_then(|idx| {
            if idx < pants_ids.len() {
                Some(pants_ids[idx].clone())
            } else {
                None
            }
        });

        let outfit = Outfit::new(
            Uuid::new_v4().to_string(),
            shirt_id,
            pants_id,
        );
        outfit_repo.create(outfit)?;
    }

    Ok(())
}
