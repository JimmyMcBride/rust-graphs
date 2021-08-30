use crate::Graph;

pub fn tree_a() -> Graph<i32> {
  let mut graph = Graph::new();
  graph.add_vertex(1);
  graph.add_vertex(2);
  graph.add_vertex(3);
  graph.add_vertex(4);
  graph.add_vertex(5);
  graph.add_vertex(6);
  graph.add_vertex(7);
  graph.add_edge(1, 2);
  graph.add_edge(2, 3);
  graph.add_edge(2, 4);
  graph.add_edge(3, 5);
  graph.add_edge(4, 6);
  graph.add_edge(4, 7);
  graph.add_edge(5, 3);
  graph.add_edge(6, 3);
  graph.add_edge(7, 1);
  graph.add_edge(7, 6);
  graph
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::HashSet;

  #[test]
  fn tree_a_test() {
    let graph_a = tree_a();
    let mut expected_hash_one = HashSet::new();
    expected_hash_one.insert(2);
    let mut expected_hash_two = HashSet::new();
    expected_hash_two.insert(3);
    expected_hash_two.insert(4);
    assert_eq!(graph_a.get_neighbors(1), &expected_hash_one);
    assert_eq!(graph_a.get_neighbors(2), &expected_hash_two);
  }
}
