use crate::grammar::{Expression, Symbol};
use petgraph::{dot::Dot, graph::DiGraph};
use std::{collections::HashMap, fs, path::Path};

struct Node {
    count: usize,
    children: HashMap<Symbol, Node>,
}

impl Node {
    fn new() -> Self {
        Node {
            count: 0,
            children: HashMap::new(),
        }
    }

    fn get_suffix(node: &Node) -> Vec<Expression> {
        if node.children.is_empty() {
            return vec![];
        }
        let mut result = Vec::new();
        let mut child_count = 0;
        for (symbol, child) in &node.children {
            child_count += child.count;
            let suffixes = Node::get_suffix(child);
            if suffixes.is_empty() {
                let mut new_expression = Expression::new();
                new_expression.push(symbol.clone());
                new_expression.remove_last();
                result.push(new_expression);
                continue;
            }
            for suffix in Node::get_suffix(child) {
                let mut new_expression = Expression::new();
                new_expression.push(symbol.clone());
                new_expression.extend(suffix.name);
                new_expression.remove_last();
                result.push(new_expression);
            }
        }
        for _ in child_count..node.count {
            result.push(Expression::empty());
        }
        result
    }
}

pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: Node::new() }
    }

    pub fn insert(&mut self, word: &Expression) {
        let mut node = &mut self.root;
        node.count += 1;
        for symbol in word.name.iter() {
            node = node
                .children
                .entry(symbol.clone())
                .or_insert_with(Node::new);
            node.count += 1;
        }
    }

    pub fn prefix_and_suffix(&self) -> Vec<(Expression, Vec<Expression>)> {
        let mut result = Vec::new();
        for (symbol, first_node) in &self.root.children {
            let mut node = first_node;
            let mut prefix = Expression::new();
            prefix.push(symbol.clone());
            while node.children.len() == 1 {
                let (ch, next_node) = node.children.iter().next().unwrap();
                prefix.push(ch.clone());
                node = next_node;
            }
            let suffix = Node::get_suffix(node);
            result.push((prefix, suffix));
        }
        result
    }

    pub fn show_graph(&self, filename: &str) {
        let mut graph = DiGraph::new();
        let mut node_indices = HashMap::new();
        let add_count_label = format!("root: {}", &self.root.count);
        let root_index = graph.add_node(add_count_label.clone());
        node_indices.insert(add_count_label, root_index);

        fn add_edges(
            graph: &mut DiGraph<String, String>,
            node_indices: &mut HashMap<String, petgraph::graph::NodeIndex>,
            parent_index: petgraph::graph::NodeIndex,
            node: &Node,
            prefix: &str,
        ) {
            for (symbol, child) in &node.children {
                let node_label = format!("{}{}", prefix, symbol.name);
                let add_count_label = format!("{}: {}", node_label.clone(), child.count);
                let child_index = graph.add_node(add_count_label.clone());
                node_indices.insert(node_label.clone(), child_index);
                graph.add_edge(parent_index, child_index, symbol.name.clone());
                add_edges(graph, node_indices, child_index, child, &node_label);
            }
        }

        add_edges(&mut graph, &mut node_indices, root_index, &self.root, "");

        let dot = Dot::new(&graph);
        let mut dot_string = format!("{:?}", dot);

        dot_string = dot_string.replace("\\\"", "");

        let path = Path::new("result_pic");
        fs::create_dir_all(path).expect("Unable to create directory");

        let dot_file_path = path.join(filename);
        let dot_file_path_str = dot_file_path.to_str().unwrap();
        std::fs::write(dot_file_path_str, dot_string).expect("Unable to write file");

        let png_file_path = path.join(filename.replace(".dot", ".png"));
        let png_file_path_str = png_file_path.to_str().unwrap();
        std::process::Command::new("dot")
            .args(&["-Tpng", dot_file_path_str, "-o", png_file_path_str])
            .output()
            .expect("Failed to execute dot command");
    }
}
