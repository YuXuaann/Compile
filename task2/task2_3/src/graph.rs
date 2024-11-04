use crate::dsu::DSU;
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::{fs, usize};

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

    fn alloc(point: usize, allocotor: &mut HashMap<usize, usize>) -> usize {
        if !allocotor.contains_key(&point) {
            let len = allocotor.len();
            allocotor.insert(point, len);
        }
        *allocotor.get(&point).unwrap()
    }

    fn remove_multiple_edges(&mut self) {
        for (_u, edges) in self.edges.iter_mut().enumerate() {
            let mut set = HashSet::new();
            let mut result = Vec::new();
            for item in edges.iter() {
                if set.insert(item) {
                    result.push(*item);
                }
            }
            *edges = result;
        }
    }

    pub fn minimize(
        &mut self,
        dsu: &mut DSU,
        start_state: &mut usize,
        end_states: &mut Vec<usize>,
    ) {
        let mut point_alloctor: HashMap<usize, usize> = HashMap::new();
        let mut p = |x| Graph_::alloc(x, &mut point_alloctor);
        let mut graph = Graph_::new(dsu.count());
        dsu.show();

        for (u, edges) in self.edges.iter().enumerate() {
            for (v, w) in edges {
                let x = dsu.find(u);
                let y = dsu.find(*v);
                graph.add_edge(p(x), p(y), *w);
            }
        }

        graph.remove_multiple_edges();

        self.edges = graph.edges.clone();
        *start_state = p(*start_state);
        for end_state in end_states {
            *end_state = p(*end_state);
        }
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

    pub fn show_graph(&self, filename: &str, start_state: usize, accept_states: &[usize]) {
        let mut graph = DiGraph::new();

        let mut node_indices = vec![];
        for i in 0..self.edges.len() {
            node_indices.push(graph.add_node(format!("{}", i)));
        }
        for (from, edges) in self.edges.iter().enumerate() {
            for &(to, label) in edges {
                graph.add_edge(node_indices[from], node_indices[to], label.to_string());
            }
        }

        // 添加开始状态
        let start_node = graph.add_node("start".to_string());
        graph.add_edge(start_node, node_indices[start_state], "".to_string());

        // 生成 DOT 格式的字符串
        let dot = Dot::new(&graph);
        let mut dot_string = format!("{:?}", dot);

        // 设置图的方向为从左到右
        dot_string = dot_string.replace("digraph {", "digraph {\n    rankdir=LR;");
        dot_string = dot_string.replace("\\\"", "");

        // 标记接受状态
        for &accept_state in accept_states {
            // let node_label = format!("\"{}\"", accept_state);
            // println!("# accept_state: {}", node_label);
            dot_string = dot_string.replace(
                &format!("    {} [ ", accept_state),
                &format!("    {} [ shape=doublecircle, ", accept_state),
            );
        }

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
