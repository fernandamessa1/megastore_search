# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## Descrição
Sistema de busca de produtos usando Rust e HashMap para indexação rápida.

## Tecnologias
- Rust
- Cargo
- Crates padrão (HashMap)

## Como executar
1. Clonar o projeto
2. Rodar: cargo run
3. Rodar testes: cargo test

## Exemplo de busca
Query: "Galaxy"
Resultados: Smartphone Galaxy, etc.

## Arquitetura
- src/product.rs -> Estrutura Product
- src/search.rs -> Funções de busca e índice
- src/main.rs -> Programa principal

## Algoritmos
- HashMap para indexação rápida de palavras-chave
- Busca direta em O(1) para cada termo
