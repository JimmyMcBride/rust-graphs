mod binary_trees;
use binary_trees::tree_a;
pub use graphs::Graph;

fn main() {
	let _graph_a = tree_a();
	println!("Graph_A: {:#?}", _graph_a.breath_first_search(1, 6));
}
