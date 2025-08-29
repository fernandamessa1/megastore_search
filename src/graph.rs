use std::collections::HashMap;

pub struct Graph {
    pub adj: HashMap<u32, Vec<u32>>, // product_id -> lista de product_id vizinhos
}

impl Graph {
    pub fn new() -> Self {
        Self { adj: HashMap::new() }
    }

    pub fn add_edge(&mut self, a: u32, b: u32) {
        self.adj.entry(a).or_default().push(b);
        self.adj.entry(b).or_default().push(a);
    }

    // recomendação simples: retorna até k vizinhos diretos (sem ranking sofisticado)
    pub fn recommend(&self, id: u32, k: usize) -> Vec<u32> {
        match self.adj.get(&id) {
            None => Vec::new(),
            Some(neigh) => neigh.iter().take(k).cloned().collect(),
        }
    }
}
