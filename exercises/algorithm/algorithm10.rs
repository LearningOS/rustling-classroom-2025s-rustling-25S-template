use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        if self.adjacency_table().contains_key(node) {
            false
        } else {
            self.adjacency_table_mutable()
                .insert(node.to_string(), Vec::new());
            true
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        self.add_node(from);
        self.add_node(to);

        let adj_table = self.adjacency_table_mutable();
        adj_table.get_mut(from).unwrap().push((to.to_string(), weight));
        adj_table.get_mut(to).unwrap().push((from.to_string(), weight));
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().contains_key(node)
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, neighbors) in self.adjacency_table() {
            for (to_node, weight) in neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::*;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&"a".to_string(), &"b".to_string(), 5),
            (&"b".to_string(), &"a".to_string(), 5),
            (&"b".to_string(), &"c".to_string(), 10),
            (&"c".to_string(), &"b".to_string(), 10),
            (&"c".to_string(), &"a".to_string(), 7),
            (&"a".to_string(), &"c".to_string(), 7),
        ];

        for edge in &expected_edges {
            assert!(graph.edges().contains(edge));
        }
    }
}
