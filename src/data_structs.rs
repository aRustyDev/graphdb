use std::collections::HashMap;

struct Node {
    id: u64,
    label: String,
    properties: HashMap<String, String>,
}

struct NodeList {
    nodes: Vec<Node>,
}

struct Edge {
    id: u64,
    label: String,
    source: u64,
    target: u64,
    properties: HashMap<String, String>,
}

struct EdgeList {
    edges: Vec<Edge>,
}

struct Property {
    key: String,
    value: String,
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
}