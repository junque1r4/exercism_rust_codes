pub mod graph {
    pub use std::collections::HashMap;
    pub type Attrs = [( &'static str, &'static str)];
    use graph_items::{edge::Edge, node::Node};

    pub struct Graph {
        pub nodes: Vec<graph_items::node::Node>,
        pub edges: Vec<graph_items::edge::Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::<Node>::new(),
                edges: Vec::<Edge>::new(),
                attrs: HashMap::<String, String>::new(),
            }
        }
    }

    pub mod graph_items {

        pub mod node {
            use std::{collections::HashMap};

            pub struct Node {
                name: &'static str,
                attr: HashMap<&'static str, &'static str>,
            }

            impl Node {
                pub fn new(name: &'static str) -> Self {
                    let attrs = HashMap::<&'static str, &'static str>::new();
                    Node { name: name, attr: attrs }
                }
            }
        }

        pub mod edge {
            pub struct Edge {
                // Add fields and implement methods for Edge
            }

            impl Edge {
                // Implement methods for Edge
            }
        }
    }
}
