use super::object_mothers::{OutfitMother, PantsMother, ShirtMother};

#[test]
fn test_complete_outfit() {
    let outfit = OutfitMother::complete();
    assert_eq!(outfit.id, "outfit-1");
    assert!(outfit.shirt.is_some());
    assert!(outfit.pants.is_some());
    assert!(outfit.is_completed);
    assert!(outfit.price > 0.0);
}

#[test]
fn test_incomplete_outfit_no_shirt() {
    let outfit = OutfitMother::incomplete_no_shirt();
    assert!(outfit.shirt.is_none());
    assert!(outfit.pants.is_some());
    assert!(!outfit.is_completed);
}

#[test]
fn test_incomplete_outfit_no_pants() {
    let outfit = OutfitMother::incomplete_no_pants();
    assert!(outfit.shirt.is_some());
    assert!(outfit.pants.is_none());
    assert!(!outfit.is_completed);
}

#[test]
fn test_empty_outfit() {
    let outfit = OutfitMother::empty();
    assert!(outfit.shirt.is_none());
    assert!(outfit.pants.is_none());
    assert!(!outfit.is_completed);
    assert_eq!(outfit.price, 0.0);
}

#[test]
fn test_outfit_completion_logic() {
    let complete = OutfitMother::custom(
        "test-1",
        Some(ShirtMother::default()),
        Some(PantsMother::default()),
    );
    assert!(complete.is_completed);

    let incomplete = OutfitMother::custom(
        "test-2",
        Some(ShirtMother::default()),
        None,
    );
    assert!(!incomplete.is_completed);
}

#[test]
fn test_outfit_update_completion() {
    let mut outfit = OutfitMother::incomplete_no_pants();
    assert!(!outfit.is_completed);

    outfit.pants = Some(PantsMother::default());
    outfit.update_completion();
    assert!(outfit.is_completed);
}

#[test]
fn test_outfit_becomes_incomplete() {
    let mut outfit = OutfitMother::complete();
    assert!(outfit.is_completed);

    outfit.shirt = None;
    outfit.update_completion();
    assert!(!outfit.is_completed);
}

#[test]
fn test_outfit_price_calculation() {
    let outfit = OutfitMother::complete();
    let expected_price = outfit.shirt.as_ref().unwrap().price.effective_price() 
        + outfit.pants.as_ref().unwrap().price.effective_price();
    assert_eq!(outfit.price, expected_price);
}

#[test]
fn test_outfit_price_with_discounts() {
    let outfit = OutfitMother::with_discounts();
    let shirt_price = outfit.shirt.as_ref().unwrap().price.effective_price();
    let pants_price = outfit.pants.as_ref().unwrap().price.effective_price();
    assert_eq!(outfit.price, shirt_price + pants_price);
    assert!(outfit.shirt.as_ref().unwrap().price.discounted_price.is_some());
    assert!(outfit.pants.as_ref().unwrap().price.discounted_price.is_some());
}

#[test]
fn test_outfit_price_update() {
    let mut outfit = OutfitMother::incomplete_no_pants();
    let initial_price = outfit.price;
    
    outfit.pants = Some(PantsMother::default());
    outfit.update_price();
    
    assert!(outfit.price > initial_price);
}
