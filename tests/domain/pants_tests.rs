use super::object_mothers::PantsMother;

#[test]
fn test_pants_creation() {
    let pants = PantsMother::default();
    assert_eq!(pants.id, "pants-1");
    assert_eq!(pants.name, "Classic Blue Jeans");
    assert_eq!(pants.color, "Blue");
    assert_eq!(pants.reference, "P-001");
}

#[test]
fn test_pants_with_discount() {
    let pants = PantsMother::with_discount();
    assert_eq!(pants.id, "pants-2");
    assert_eq!(pants.name, "Black Chinos");
    assert!(pants.price.discounted_price.is_some());
}

#[test]
fn test_custom_pants() {
    let pants = PantsMother::custom("custom-1", "Custom Pants", "Red", "C-001");
    assert_eq!(pants.id, "custom-1");
    assert_eq!(pants.name, "Custom Pants");
    assert_eq!(pants.color, "Red");
    assert_eq!(pants.reference, "C-001");
}

#[test]
fn test_pants_with_specific_id() {
    let pants = PantsMother::with_id("test-id-123");
    assert_eq!(pants.id, "test-id-123");
}
