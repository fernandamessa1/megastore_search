#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
    pub brand: String,
    pub price: f64,
}

impl Product {
    pub fn new(id: u32, name: &str, category: &str, brand: &str, price: f64) -> Self {
        Self {
            id,
            name: name.to_string(),
            category: category.to_string(),
            brand: brand.to_string(),
            price,
        }
    }
}
