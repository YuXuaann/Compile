use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub struct Graph_ {
    pub edges: Vec<Vec<(usize, char)>>,
}

impl Graph_ {
    pub fn new(len: usize) -> Self {
        let mut edges = Vec::new();
        for _ in 0..len {
            edges.push(Vec::new());
        }
        Graph_ { edges }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, label: char) {
        self.edges[from].push((to, label));
    }

    fn dfs_only_epsilon(&self, point: usize, visited: &mut Vec<bool>, result: &mut Vec<usize>) {
        visited[point] = true;
        for &(to, label) in &self.edges[point] {
            if label == 'ε' && !visited[to] {
                result.push(to);
                self.dfs_only_epsilon(to, visited, result);
            }
        }
    }

    pub fn extend_from_point(&self, point: usize) -> Vec<usize> {
        let mut result = Vec::new();
        result.push(point);
        let mut visited = vec![false; self.edges.len()];
        self.dfs_only_epsilon(point, &mut visited, &mut result);
        result
    }

    pub fn dfs_search(
        &self,
        u: usize,
        end_states: &Vec<usize>,
        now_index: usize,
        exp: &str,
        has_succeed: &mut bool,
    ) -> bool {
        // println!("u: {}, now_index: {}, exp: {}", u, now_index, exp);
        if now_index == exp.len() {
            if end_states.contains(&u) {
                *has_succeed = true;
                return true;
            }
        }
        if *has_succeed {
            return true;
        }
        let mut result = false;

        let mut edges = self.edges[u].clone();
        let mut rng = rand::thread_rng();
        edges.shuffle(&mut rng);

        for (v, w) in &edges {
            if *w == 'ε' {
                result |= self.dfs_search(*v, end_states, now_index, exp, has_succeed);
            }
            if now_index == exp.len() {
                continue;
            }
            if *w == exp.chars().nth(now_index).unwrap() {
                result |= self.dfs_search(*v, end_states, now_index + 1, exp, has_succeed);
            }
        }
        result
    }

    pub fn extend_from_point_vec_and_val(&self, point: &Vec<usize>, val: char) -> Vec<usize> {
        let mut result = HashSet::new();
        for &point in point {
            for (to, label) in &self.edges[point] {
                if *label == val {
                    for x in self.extend_from_point(*to) {
                        result.insert(x);
                    }
                }
            }
        }
        let mut to_state: Vec<usize> = result.clone().into_iter().collect();
        to_state.sort();
        to_state
    }

    pub fn dfs_get_dfa_states(
        &self,
        u: &Vec<usize>,
        terminate_states: &Vec<char>,
        states: &mut HashSet<Vec<usize>>,
    ) {
        for w in terminate_states {
            let to_state = self.extend_from_point_vec_and_val(u, *w);
            if !states.contains(&to_state) && !to_state.is_empty() {
                states.insert(to_state.clone());
                self.dfs_get_dfa_states(&to_state, terminate_states, states);
            }
        }
    }

    pub fn get_terminal(&self) -> Vec<char> {
        let mut terminal = HashSet::new();
        for edges in &self.edges {
            for (_, label) in edges {
                terminal.insert(*label);
            }
        }
        terminal.remove(&'ε');
        let mut res: Vec<char> = terminal.clone().into_iter().collect();
        res.sort();
        res
    }

    pub fn show_graph(&self, filename: &str) {
        let mut graph = DiGraph::new();

        let mut node_indices = vec![];
        for i in 0..self.edges.len() {
            node_indices.push(graph.add_node(i));
        }
        for (from, edges) in self.edges.iter().enumerate() {
            for &(to, label) in edges {
                graph.add_edge(node_indices[from], node_indices[to], label);
            }
        }

        let dot = Dot::new(&graph);

        let path = Path::new("result_pic");
        fs::create_dir_all(path).expect("Unable to create directory");

        let dot_file_path = path.join(filename);
        let dot_file_path_str = dot_file_path.to_str().unwrap();
        std::fs::write(dot_file_path_str, format!("{:?}", dot)).expect("Unable to write file");

        std::process::Command::new("sed")
            .args(&["-i", "s/'//g", dot_file_path_str])
            .output()
            .expect("Failed to execute sed command");

        let png_file_path = path.join(filename.replace(".dot", ".png"));
        let png_file_path_str = png_file_path.to_str().unwrap();
        std::process::Command::new("dot")
            .args(&["-Tpng", dot_file_path_str, "-o", png_file_path_str])
            .output()
            .expect("Failed to execute dot command");
    }
}
