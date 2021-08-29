use graphs::Graph;
// use std::collections::HashSet;

fn main() {
  let mut graph_i32 = Graph::new();
  graph_i32.add_vertex(1);
  graph_i32.add_vertex(2);
  graph_i32.add_vertex(3);
  graph_i32.add_vertex(4);
  graph_i32.add_vertex(5);
  graph_i32.add_vertex(6);
  graph_i32.add_vertex(7);
  graph_i32.add_edge(5, 3);
  graph_i32.add_edge(6, 3);
  graph_i32.add_edge(7, 1);
  graph_i32.add_edge(4, 7);
  graph_i32.add_edge(1, 2);
  graph_i32.add_edge(7, 6);
  graph_i32.add_edge(2, 4);
  graph_i32.add_edge(3, 5);
  graph_i32.add_edge(2, 3);
  graph_i32.add_edge(4, 6);

  let check_node = 1;
  println!(
    "Graph i32: {:#?}, Neighbors of {}: {:#?}, BFS: {:#?}",
    &graph_i32.vertices,
    check_node,
    graph_i32.get_neighbors(1),
    graph_i32.breath_first_search(1, 6)
  );

  //   let mut graph_str = Graph::new();
  //   graph_str.add_vertex("1");
  //   graph_str.add_vertex("2");
  //   graph_str.add_vertex("3");
  //   graph_str.add_vertex("4");
  //   graph_str.add_vertex("5");
  //   graph_str.add_edge("1", "2");
  //   graph_str.add_edge("1", "3");
  //   graph_str.add_edge("2", "4");
  //   graph_str.add_edge("2", "5");

  //   println!(
  //     "Graph str: {:#?}, Neighbors of 1: {:#?}",
  //     &graph_str.vertices,
  //     graph_str.get_neighbors("1")
  //   );
}
