use super::object_mothers::ShirtMother;

#[test]
fn test_shirt_creation() {
    let shirt = ShirtMother::default();
    assert_eq!(shirt.id, "shirt-1");
    assert_eq!(shirt.name, "White Oxford Shirt");
    assert_eq!(shirt.color, "White");
    assert_eq!(shirt.reference, "S-001");
}

#[test]
fn test_shirt_with_discount() {
    let shirt = ShirtMother::with_discount();
    assert_eq!(shirt.id, "shirt-2");
    assert_eq!(shirt.name, "Blue Denim Shirt");
    assert!(shirt.price.discounted_price.is_some());
}

#[test]
fn test_custom_shirt() {
    let shirt = ShirtMother::custom("custom-1", "Custom Shirt", "Green", "C-001");
    assert_eq!(shirt.id, "custom-1");
    assert_eq!(shirt.name, "Custom Shirt");
    assert_eq!(shirt.color, "Green");
    assert_eq!(shirt.reference, "C-001");
}

#[test]
fn test_shirt_with_specific_id() {
    let shirt = ShirtMother::with_id("test-id-456");
    assert_eq!(shirt.id, "test-id-456");
}
