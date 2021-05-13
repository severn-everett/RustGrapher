use crate::lib::graph::Graph;

mod lib;

fn main() {
    match Graph::load("test_resources/tinyG.json") {
        Ok(_) => println!("Graph loaded!"),
        Err(e) => println!("Graph failed to load: {}", e)
    };
}