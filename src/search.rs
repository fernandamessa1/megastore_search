use std::collections::HashMap;
use crate::product::Product;

pub struct SearchIndex {
    index: HashMap<String, Vec<u32>>,
    products: HashMap<u32, Product>,
}

impl SearchIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            products: HashMap::new(),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        let id = product.id;
        self.products.insert(id, product.clone());

        for word in product.name.to_lowercase().split_whitespace() {
            self.index.entry(word.to_string()).or_default().push(id);
        }
        for word in product.brand.to_lowercase().split_whitespace() {
            self.index.entry(word.to_string()).or_default().push(id);
        }
        for word in product.category.to_lowercase().split_whitespace() {
            self.index.entry(word.to_string()).or_default().push(id);
        }
    }

    pub fn search(&self, query: &str) -> Vec<&Product> {
        let mut results = Vec::new();
        for word in query.to_lowercase().split_whitespace() {
            if let Some(ids) = self.index.get(word) {
                for id in ids {
                    if let Some(prod) = self.products.get(id) {
                        results.push(prod);
                    }
                }
            }
        }
        results
    }
}
