use megastore_search::product::Product;
use megastore_search::search::SearchIndex;

use std::time::Instant;

fn main() {
    // Criar índice e adicionar um produto de exemplo
    let mut index = SearchIndex::new();
    let product = Product {
        id: 1,
        name: "Smartphone Galaxy".to_string(),
        category: "Eletrônicos".to_string(),
        brand: "Samsung".to_string(),
        price: 2500.0,
    };
    index.add_product(product.clone());

    // Executar busca normal
    let results = index.search("Galaxy");
    println!("Resultados para 'Galaxy': {:?}", results);

    // -----------------------------
    // Medição de performance simples
    // -----------------------------
    let mut big_index = SearchIndex::new();
    for i in 0..10_000 {
        big_index.add_product(Product {
            id: i,
            name: format!("Produto {}", i),
            category: "Categoria".to_string(),
            brand: "Marca".to_string(),
            price: (i % 100) as f64,
        });
    }

    let start = Instant::now();
    let resultados = big_index.search("Produto 9999");
    let duration = start.elapsed();

    println!(
        "Busca retornou {} resultados em {:?}",
        resultados.len(),
        duration
    );
}
