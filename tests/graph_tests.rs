use megastore_search::graph::Graph;

#[test]
fn graph_recommendation_simple() {
    let mut g = Graph::new();
    g.add_edge(1, 2);
    g.add_edge(1, 3);
    let recs = g.recommend(1, 2);
    assert_eq!(recs.len(), 2);
    assert!(recs.contains(&2));
    assert!(recs.contains(&3));
}
