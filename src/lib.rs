use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;
pub mod binary_trees;

/// A graph structure that contains vertices and the edges between them...
pub struct Graph<T> {
  pub vertices: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
  T: Eq + Hash + Debug + Copy + Display,
{
  /// Creates a new graph...
  pub fn new() -> Graph<T> {
    Graph {
      vertices: HashMap::new(),
    }
  }
  /// Adds a new vertex to the graph...
  pub fn add_vertex(&mut self, vertex: T) {
    self.vertices.insert(vertex, HashSet::new());
  }
  /// Add and edge from one vertex to the next...
  pub fn add_edge(&mut self, vertex_one: T, vertex_two: T) {
    if self.vertices.contains_key(&vertex_one) && self.vertices.contains_key(&vertex_two) {
      self
        .vertices
        .get_mut(&vertex_one)
        .unwrap()
        .insert(vertex_two);
    } else {
      panic!("Vertex doesn't exist... ☠️")
    }
  }
  /// Gets all vertices connected to a given vertex...
  pub fn get_neighbors(&self, vertex: T) -> &HashSet<T> {
    match self.vertices.get(&vertex) {
      Some(n) => n,
      None => panic!("Vertex '{:#?}' does not exist", vertex),
    }
  }
  /// Takes in a starting node and travels to the destination node by traversing breath-first through the graph...
  #[allow(unused_variables)]
  pub fn breath_first_search(&self, starting_vertex: T, destination_vertex: T) -> Vec<T> {
    let mut queue = Vec::new();
    queue.push(vec![starting_vertex]);
    println!("--Queue *init: {:#?}--", queue);

    let mut visited: HashSet<T> = HashSet::new();
    let mut curr_item = Vec::new();
    println!("--visited and current items *init--");

    while queue.len() > 0 {
      curr_item = queue.remove(0);
      println!("Current list (dequeued and active): {:#?}", curr_item);
      if !visited.contains(curr_item.last().unwrap()) {
        if curr_item.first().unwrap() == &starting_vertex
          && curr_item.last().unwrap() == &destination_vertex
        {
          println!("Complete!");
          break;
        }
        visited.insert(curr_item.last().unwrap().clone());
      }
      for next_vertex in self.get_neighbors(curr_item.last().unwrap().clone()) {
        println!("Next vertex: {:#?}", next_vertex);
        let mut vertex_list = curr_item.clone();
        vertex_list.push(next_vertex.clone());
        println!(
          "Creating new list for queue (new list + next vertex): {:#?}",
          vertex_list
        );

        queue.push(vertex_list);

        println!("Adding new vertex list to queue: {:#?}", queue);
      }
    }
    curr_item
  }
  /// Takes in a starting node and travels to the destination node by traversing depth-first through the graph...
  #[allow(unused_variables)]
  pub fn depth_first_search(&self, starting_vertex: T, destination_vertex: T) {
    // init_binary_tree_a();
    unimplemented!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use binary_trees::tree_a;

  #[test]
  fn breath_first_search_test() {
    let graph_a = tree_a();
    assert_eq!(graph_a.breath_first_search(1, 7), [1, 2, 4, 7]);
    assert_eq!(graph_a.breath_first_search(1, 6), [1, 2, 4, 6]);
    assert_eq!(graph_a.breath_first_search(2, 7), [2, 4, 7]);
    assert_eq!(graph_a.breath_first_search(2, 5), [2, 3, 5]);
  }
}
