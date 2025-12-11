use super::object_mothers::PriceMother;

#[test]
fn test_price_creation() {
    let price = PriceMother::default();
    assert_eq!(price.price, 99.99);
    assert_eq!(price.discounted_price, None);
}

#[test]
fn test_price_with_discount() {
    let price = PriceMother::with_discount();
    assert_eq!(price.price, 99.99);
    assert_eq!(price.discounted_price, Some(79.99));
}

#[test]
fn test_effective_price_without_discount() {
    let price = PriceMother::default();
    assert_eq!(price.effective_price(), 99.99);
}

#[test]
fn test_effective_price_with_discount() {
    let price = PriceMother::with_discount();
    assert_eq!(price.effective_price(), 79.99);
}

#[test]
fn test_custom_price() {
    let price = PriceMother::custom(150.0, Some(120.0));
    assert_eq!(price.price, 150.0);
    assert_eq!(price.discounted_price, Some(120.0));
    assert_eq!(price.effective_price(), 120.0);
}
