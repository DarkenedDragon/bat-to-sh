use std::{fs, option};
use petgraph::{graph::Graph};
use petgraph::dot::{Config, Dot};
use std::collections::{HashMap, HashSet};



fn main() {
    let batch_file_path = "test.bat";
    let mut context_graph = Graph::<&str, ()>::default();
    let mut operators_set = HashSet::<&str>::new();
    let mut command_set = HashSet::<&str>::new();

    // Add all the operators to the set
    operators_set.insert("==");
    operators_set.insert("=");

    // Add all the commands
    command_set.insert("ASSOC");
    command_set.insert("BREAK");
    command_set.insert("CALL");
    command_set.insert("CD");
    command_set.insert("CHDIR");
    command_set.insert("CLS");
    command_set.insert("COLOR");
    command_set.insert("COPY");
    command_set.insert("DEL");
    command_set.insert("DIR");
    command_set.insert("DATE");
    command_set.insert("ECHO");
    command_set.insert("ELSE");
    command_set.insert("ENDLOCAL");
    command_set.insert("ERASE");
    command_set.insert("EXIT");
    command_set.insert("FOR");
    command_set.insert("FTYPE");
    command_set.insert("GOTO");
    command_set.insert("IF");
    command_set.insert("MD");
    command_set.insert("MKDIR");
    command_set.insert("MKLINK");
    command_set.insert("MOVE");
    command_set.insert("PATH");
    command_set.insert("PAUSE");
    command_set.insert("POPD");
    command_set.insert("RD");
    command_set.insert("REN");
    command_set.insert("RENAME");
    command_set.insert("REM");
    command_set.insert("RMDIR");
    command_set.insert("SET");
    command_set.insert("SETLOCAL");
    command_set.insert("SHIFT");
    command_set.insert("START");
    command_set.insert("TIME");
    command_set.insert("TITLE");
    command_set.insert("TYPE");
    command_set.insert("VER");
    command_set.insert("VERIFY");
    command_set.insert("VOL");

    // Go through the file word by word and build a context tree
    let batch_file = fs::read_to_string(batch_file_path).expect(&format!("Cannot open {}", batch_file_path));
    for line in batch_file.lines() {
        if let Some(word) = line.split_whitespace().next() {
            // If we have a command, add that as a node
            // that points to its values
            println!("Parsing {}", word);
            if command_set.contains(word.to_uppercase().as_str()) {
                let cmd_index = context_graph.add_node(word);
                let value_index = context_graph.add_node(line);
                context_graph.add_edge(cmd_index, value_index, ());
            }
        }
    }

    println!("{:?}", Dot::with_config(&context_graph, &[Config::EdgeNoLabel]));
}
