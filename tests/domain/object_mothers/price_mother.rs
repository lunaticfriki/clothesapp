use clothes::domain::entities::Price;

pub struct PriceMother;

impl PriceMother {
    pub fn default() -> Price {
        Price::new(99.99, None)
    }

    pub fn with_discount() -> Price {
        Price::new(99.99, Some(79.99))
    }

    pub fn empty() -> Price {
        Price::empty()
    }

    pub fn cheap() -> Price {
        Price::new(29.99, None)
    }

    pub fn expensive() -> Price {
        Price::new(299.99, None)
    }

    pub fn custom(price: f64, discounted_price: Option<f64>) -> Price {
        Price::new(price, discounted_price)
    }
}
