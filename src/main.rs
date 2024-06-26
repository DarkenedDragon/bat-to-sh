use std::fs;
use petgraph::graph::Graph;
use std::collections::HashSet;

fn main() {
    let batch_file_path = "test.bat";
    let context_graph = Graph::<&str, ()>::default();
    let mut operators_set = HashSet::<&str>::new();

    // Add all the operators to the set
    operators_set.insert("set");
    operators_set.insert("if");
    operators_set.insert("==");
    operators_set.insert("=");
    operators_set.insert("echo");
    operators_set.insert("for");


    // Go through the file word by word and build a context tree
    let batch_file = fs::read_to_string(batch_file_path).expect(&format!("Cannot open {}", batch_file_path));
    for word in batch_file.split_whitespace() {

    }
}
