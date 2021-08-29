use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::hash::Hash;

/// A graph structure that contains vertices and the edges between them...
pub struct Graph<T> {
  pub vertices: HashMap<T, Vec<T>>,
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
    self.vertices.insert(vertex, Vec::new());
  }
  /// Add and edge between two vertices...
  pub fn add_edge(&mut self, vertex_one: T, vertex_two: T) {
    if self.vertices.contains_key(&vertex_one) && self.vertices.contains_key(&vertex_two) {
      self.vertices.get_mut(&vertex_one).unwrap().push(vertex_two);
    } else {
      panic!("Vertex doesn't exist... ☠️")
    }
  }
  /// Gets all vertices connected to a given vertex...
  pub fn get_neighbors(&self, vertex: T) -> &Vec<T> {
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
    let mut visited: HashSet<T> = HashSet::new();
    let mut curr_item = Vec::new();
    while queue.len() > 0 {
      curr_item = queue.remove(0);
      println!("Current Item: {:#?}", curr_item);
      if !visited.contains(curr_item.last().unwrap()) {
        if curr_item.first().unwrap() == &starting_vertex
          && curr_item.last().unwrap() == &destination_vertex
        {
          println!("Complete!");
          break;
        }
        visited.insert(curr_item.last().unwrap().clone());
      }
      for (curr_vert, next_vert) in self.vertices.get_key_value(curr_item.last().unwrap()) {
        println!("Curr vert: {:#?}", curr_vert);
        println!("Next vert: {}", next_vert[0]);
        // for vert in next_vert.iter() {
        //   curr_item.push(self.vertices.get_key_value(vert).unwrap());
        // }

        // queue.push(curr_item.to_vec())
      }
    }
    curr_item
  }
  /// Takes in a starting node and travels to the destination node by traversing depth-first through the graph...
  #[allow(unused_variables)]
  pub fn depth_first_search(&self, starting_vertex: T, destination_vertex: T) {
    unimplemented!()
  }
}
