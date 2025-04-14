use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::dot::{Dot, Config};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

#[allow(dead_code)]
struct MyGraph {
    graph: Graph<&'static str, &'static str>,
    node_indices: HashMap<&'static str, NodeIndex>,
}

#[allow(dead_code)]
impl MyGraph {
    pub fn new() -> Self {
        let mut graph = Graph::<&'static str, &'static str>::new();
        let mut node_indices = HashMap::new();

        // Adiciona nós
        let n1 = graph.add_node("1");
        let n2 = graph.add_node("2");
        let n3 = graph.add_node("3");
        let n4 = graph.add_node("4");
        let n5 = graph.add_node("5");
        let n6 = graph.add_node("6");

        node_indices.insert("1", n1);
        node_indices.insert("2", n2);
        node_indices.insert("3", n3);
        node_indices.insert("4", n4);
        node_indices.insert("5", n5);
        node_indices.insert("6", n6);

        // Arestas com ciclos
        graph.add_edge(n1, n2, "");
        graph.add_edge(n2, n3, "");
        graph.add_edge(n3, n1, ""); // ciclo 1

        graph.add_edge(n2, n4, "");
        graph.add_edge(n4, n5, "");
        graph.add_edge(n5, n2, ""); // ciclo 2

        graph.add_edge(n3, n6, ""); // ligação extra

        MyGraph {
            graph,
            node_indices,
        }
    }

    pub fn dfs_from_node1(&self) -> Vec<&'static str> {
        let start = self.node_indices["1"];
        let mut dfs = Dfs::new(&self.graph, start);
        let mut visited = Vec::new();

        while let Some(node) = dfs.next(&self.graph) {
            visited.push(self.graph[node]);
        }

        visited
    }

    pub fn export_dot(&self, filename: &str) -> std::io::Result<()> {
        let dot_output = format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));
        let mut file = File::create(filename)?;
        file.write_all(dot_output.as_bytes())?;
        println!("Arquivo '{}' gerado com sucesso!", filename);
        Ok(())
    }
}

// -----------------------------------------------------------
// TESTES
// -----------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_visits_all_nodes() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();

        let mut sorted_result = result.clone();
        sorted_result.sort();

        let mut expected = vec!["1", "2", "3", "4", "5", "6"];
        expected.sort();

        assert_eq!(sorted_result, expected, "Todos os nós devem ser visitados");
    }

    #[test]
    fn test_dfs_starts_at_node1() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();
        assert_eq!(result.first(), Some(&"1"), "DFS deve começar pelo nó 1");
    }
}