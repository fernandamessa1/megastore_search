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
Busca: "Galaxy"
Resultados: Smartphone Galaxy, etc.

## Arquitetura
- src/product.rs -> Estrutura Product
- src/search.rs -> Funções de busca e índice
- src/main.rs -> Programa principal

## Algoritmos
- HashMap para indexação rápida de palavras-chave
- Busca direta em O(1) para cada termo

## Grafos e Recomendação

Para complementar a busca, foi implementado o módulo "graph.rs" que utiliza grafos para simular relações entre produtos.  
Essa estrutura permite sugerir itens relacionados a partir de conexões entre nós, melhorando a recomendação de produtos.  

### Pontos positivos
- Melhora a experiência do usuário com sugestões inteligentes.
- Estrutura flexível para representar relações complexas.
- Facilmente escalável para bases maiores.

### Pontos negativos
- Consome mais memória à medida que o grafo cresce.
- Pode aumentar o tempo de processamento em buscas complexas.
- Requer manutenção da estrutura de arestas para manter relevância.

