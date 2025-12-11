use super::object_mothers::OutfitMother;

#[test]
fn test_complete_outfit() {
    let outfit = OutfitMother::complete();
    assert_eq!(outfit.id, "outfit-1");
    assert_eq!(outfit.shirt_id, Some("shirt-1".to_string()));
    assert_eq!(outfit.pants_id, Some("pants-1".to_string()));
    assert!(outfit.is_completed);
}

#[test]
fn test_incomplete_outfit_no_shirt() {
    let outfit = OutfitMother::incomplete_no_shirt();
    assert_eq!(outfit.shirt_id, None);
    assert_eq!(outfit.pants_id, Some("pants-1".to_string()));
    assert!(!outfit.is_completed);
}

#[test]
fn test_incomplete_outfit_no_pants() {
    let outfit = OutfitMother::incomplete_no_pants();
    assert_eq!(outfit.shirt_id, Some("shirt-1".to_string()));
    assert_eq!(outfit.pants_id, None);
    assert!(!outfit.is_completed);
}

#[test]
fn test_empty_outfit() {
    let outfit = OutfitMother::empty();
    assert_eq!(outfit.shirt_id, None);
    assert_eq!(outfit.pants_id, None);
    assert!(!outfit.is_completed);
}

#[test]
fn test_outfit_completion_logic() {
    let complete = OutfitMother::custom(
        "test-1",
        Some("shirt-1".to_string()),
        Some("pants-1".to_string()),
    );
    assert!(complete.is_completed);

    let incomplete = OutfitMother::custom(
        "test-2",
        Some("shirt-1".to_string()),
        None,
    );
    assert!(!incomplete.is_completed);
}

#[test]
fn test_outfit_update_completion() {
    let mut outfit = OutfitMother::incomplete_no_pants();
    assert!(!outfit.is_completed);

    outfit.pants_id = Some("pants-1".to_string());
    outfit.update_completion();
    assert!(outfit.is_completed);
}

#[test]
fn test_outfit_becomes_incomplete() {
    let mut outfit = OutfitMother::complete();
    assert!(outfit.is_completed);

    outfit.shirt_id = None;
    outfit.update_completion();
    assert!(!outfit.is_completed);
}
