use megastore_search::product::Product;
use megastore_search::search::SearchIndex;

fn main() {
    let mut index = SearchIndex::new();
    let product = Product {
        id: 1,
        name: "Smartphone Galaxy".to_string(),
        category: "Eletrônicos".to_string(),
        brand: "Samsung".to_string(),
        price: 2500.0,
    };
    index.add_product(product.clone());

    let results = index.search("Galaxy");
    println!("Resultados para 'Galaxy': {:?}", results);
}
