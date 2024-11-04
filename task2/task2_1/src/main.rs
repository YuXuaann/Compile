use core::panic;
use petgraph::dot::Dot;
use petgraph::graph::DiGraph;
use rand::seq::SliceRandom;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fs;
use std::path::Path;

struct Graph_ {
    edges: Vec<Vec<(usize, char)>>,
}

#[allow(unused)]
impl Graph_ {
    pub fn new() -> Graph_ {
        Graph_ { edges: vec![] }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, c: char) {
        self.edges[from].push((to, c));
    }

    fn show_graph(&self, filename: &str) {
        let mut graph = DiGraph::new();

        // 添加节点
        let mut node_indices = vec![];
        for i in 0..self.edges.len() {
            node_indices.push(graph.add_node(i));
        }

        // 添加边
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

pub struct NFA {
    /// 确保起点为0，终点为 len - 1
    graph: Graph_,
    /// 是否已经找到
    has_succeed: bool,
}

impl Debug for NFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.graph.edges)
    }
}

impl NFA {
    pub fn new() -> NFA {
        NFA {
            graph: Graph_::new(),
            has_succeed: false,
        }
    }

    pub fn from(exp: &str) -> NFA {
        let transfer_exp = add_connect(exp);
        println!("transferd expression is {}", transfer_exp);
        let mut nfa_stack = Vec::new();
        let mut op_stack = Vec::new();
        // ( ) > * > + > |
        for c in transfer_exp.chars() {
            match c {
                '(' => {
                    op_stack.push(c);
                }
                ')' => {
                    while let Some(op) = op_stack.pop() {
                        if op == '(' {
                            break;
                        }
                        let b = nfa_stack.pop().unwrap();
                        let a = nfa_stack.pop().unwrap();
                        match op {
                            '*' => {
                                nfa_stack.push(repeat(&a));
                            }
                            '|' => {
                                nfa_stack.push(or(&a, &b));
                            }
                            '+' => {
                                nfa_stack.push(and(&a, &b));
                            }
                            _ => {
                                panic!("error: unknown operator {}", op);
                            }
                        }
                    }
                }
                '*' => {
                    let a = nfa_stack.pop().unwrap();
                    nfa_stack.push(repeat(&a));
                }
                '+' => {
                    while let Some(op) = op_stack.pop() {
                        match op {
                            '*' => {
                                panic!("error: * can't be here");
                            }
                            '+' => {
                                let b = nfa_stack.pop().unwrap();
                                let a = nfa_stack.pop().unwrap();
                                nfa_stack.push(and(&a, &b));
                            }
                            '|' => {
                                op_stack.push(op);
                                break;
                            }
                            '(' => {
                                op_stack.push(op);
                                break;
                            }
                            _ => {
                                panic!("error: unknown operator {}", op);
                            }
                        }
                    }
                    op_stack.push(c);
                }
                '|' => {
                    while let Some(op) = op_stack.pop() {
                        match op {
                            '*' => {
                                panic!("error: * can't be here");
                            }
                            '+' => {
                                let b = nfa_stack.pop().unwrap();
                                let a = nfa_stack.pop().unwrap();
                                nfa_stack.push(and(&a, &b));
                            }
                            '|' => {
                                let b = nfa_stack.pop().unwrap();
                                let a = nfa_stack.pop().unwrap();
                                nfa_stack.push(or(&a, &b));
                            }
                            '(' => {
                                op_stack.push(op);
                                break;
                            }
                            _ => {
                                panic!("error: unknown operator {}", op);
                            }
                        }
                    }
                    op_stack.push(c);
                }
                _ => {
                    assert!(c.is_alphanumeric());
                    nfa_stack.push(teminate(c));
                }
            }
            // println!("============ c is: {} ===============", c);
            // println!("nfa_stack: {:?}", nfa_stack);
            // println!("op_stack: {:?}", op_stack);
            // println!("");
        }

        while let Some(op) = op_stack.pop() {
            match op {
                '*' => {
                    panic!("error: * can't be here");
                }
                '+' => {
                    let b = nfa_stack.pop().unwrap();
                    let a = nfa_stack.pop().unwrap();
                    nfa_stack.push(and(&a, &b));
                }
                '|' => {
                    let b = nfa_stack.pop().unwrap();
                    let a = nfa_stack.pop().unwrap();
                    nfa_stack.push(or(&a, &b));
                }
                '(' => {
                    panic!("error: ( can't be here");
                }
                ')' => {
                    panic!("error: ) can't be here");
                }
                _ => {
                    panic!("error: unknown operator {}", op);
                }
            }
        }

        assert_eq!(nfa_stack.len(), 1);
        assert_eq!(op_stack.len(), 0);
        nfa_stack.pop().unwrap()
    }

    pub fn show(&self, id: usize) {
        self.graph.show_graph(&format!("nfa_{}.dot", id));
    }

    fn dfs(&mut self, u: usize, now_index: usize, exp: &str) -> bool {
        // println!("u: {}, now_index: {}, exp: {}", u, now_index, exp);
        if now_index == exp.len() {
            if u == self.graph.edges.len() - 1 {
                self.has_succeed = true;
                return true;
            }
        }
        if self.has_succeed {
            return true;
        }
        let mut result = false;

        let mut edges = self.graph.edges[u].clone();
        let mut rng = rand::thread_rng();
        edges.shuffle(&mut rng);

        for (v, w) in &edges {
            if *w == 'ε' {
                result |= self.dfs(*v, now_index, exp);
            }
            if now_index == exp.len() {
                continue;
            }
            if *w == exp.chars().nth(now_index).unwrap() {
                result |= self.dfs(*v, now_index + 1, exp);
            }
        }
        result
    }

    pub fn contains(&mut self, exp: &str) -> bool {
        self.has_succeed = false;
        self.dfs(0, 0, exp)
    }
}

pub fn offset(edge: &Vec<(usize, char)>, offset: usize) -> Vec<(usize, char)> {
    let mut result = Vec::new();
    for i in 0..edge.len() {
        result.push((edge[i].0 + offset, edge[i].1));
    }
    result
}

/// connect a and b
pub fn and(a: &NFA, b: &NFA) -> NFA {
    let a_edges = a.graph.edges.clone();
    let b_edges = b.graph.edges.clone();
    let mut result = a_edges.clone();

    for i in 0..b_edges.len() {
        let mut tmp = Vec::new();
        for j in 0..b_edges[i].len() {
            tmp.push((b_edges[i][j].0 + a_edges.len(), b_edges[i][j].1));
        }
        result.push(tmp);
    }
    result[a_edges.len() - 1].push((a_edges.len(), 'ε'));
    NFA {
        graph: Graph_ { edges: result },
        has_succeed: false,
    }
}

/// or a and b
pub fn or(a: &NFA, b: &NFA) -> NFA {
    let a_edges = a.graph.edges.clone();
    let b_edges = b.graph.edges.clone();
    let mut result = Vec::new();

    // point 0
    result.push(vec![(1, 'ε'), (a_edges.len() + 1, 'ε')]);

    // point 1 to a_edges.len()
    for edge in &a_edges {
        result.push(offset(edge, 1));
    }

    // point a_edges.len() + 1 to a_edges.len() + b_edges.len()
    for edge in &b_edges {
        result.push(offset(edge, a_edges.len() + 1));
    }

    // point a_edges.len() + b_edges.len() + 1
    result.push(vec![]);

    result[a_edges.len()].push((a_edges.len() + b_edges.len() + 1, 'ε'));
    result[a_edges.len() + b_edges.len()].push((a_edges.len() + b_edges.len() + 1, 'ε'));

    NFA {
        graph: Graph_ { edges: result },
        has_succeed: false,
    }
}

/// repeat a
pub fn repeat(a: &NFA) -> NFA {
    let a_edges = a.graph.edges.clone();
    let mut result = Vec::new();

    // point 0
    result.push(vec![(1, 'ε'), (a_edges.len() + 1, 'ε')]);

    // point 1 to a_edges.len()
    for edge in &a_edges {
        result.push(offset(edge, 1));
    }

    // point a_edges.len() + 1
    result.push(vec![]);

    assert_eq!(result.len(), a_edges.len() + 2);

    result[a_edges.len()].push((1, 'ε'));
    result[a_edges.len()].push((a_edges.len() + 1, 'ε'));

    NFA {
        graph: Graph_ { edges: result },
        has_succeed: false,
    }
}

/// teminate
pub fn teminate(a: char) -> NFA {
    let mut result = Vec::new();
    result.push(vec![(1, a)]);
    result.push(vec![]);
    NFA {
        graph: Graph_ { edges: result },
        has_succeed: false,
    }
}

/// transfer the raw expression
fn add_connect(exp: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = exp.chars().collect();
    let len = chars.len();
    for i in 0..len {
        result.push(chars[i]);
        if i + 1 < len {
            let current = chars[i];
            let next = chars[i + 1];
            if (current.is_alphanumeric() || current == ')' || current == '*')
                && (next.is_alphanumeric() || next == '(')
            {
                result.push('+');
            }
        }
    }
    result
}

fn test1() {
    let regular_expression = "(a(ab|c))*d*";
    println!("regular expression: {}\n", regular_expression);
    let mut nfa = NFA::from(&regular_expression);
    nfa.show(1);

    let expression = "aabacacaabddd";
    println!("expression: {}", expression);
    let identified = nfa.contains(expression);
    println!("identified: {}\n", identified);

    let expression = "aabacacabdd";
    println!("expression: {}", expression);
    let identified = nfa.contains(expression);
    println!("identified: {}\n", identified);

    let expression = "ddddddd";
    println!("expression: {}", expression);
    let identified = nfa.contains(expression);
    println!("identified: {}\n", identified);

    let expression = "aaaaaaaaaa";
    println!("expression: {}", expression);
    let identified = nfa.contains(expression);
    println!("identified: {}\n", identified);

    let expression = "";
    println!("expression: {}", expression);
    let identified = nfa.contains(expression);
    println!("identified: {}\n", identified);

    let expression = "hello world";
    println!("expression: {}", expression);
    let identified = nfa.contains(expression);
    println!("identified: {}\n", identified);
}

fn test2() {
    let regular_expression = "woc*";
    println!("regular expression: {}\n", regular_expression);
    let mut nfa = NFA::from(&regular_expression);
    nfa.show(2);

    let expression = "wocccccc";
    println!("expression: {}", expression);
    let identified = nfa.contains(expression);
    println!("identified: {}\n", identified);
}

fn test3() {
    let regular_expression = "(0|1)*101";
    println!("regular expression: {}\n", regular_expression);
    let mut nfa = NFA::from(&regular_expression);
    nfa.show(3);

    let expression = "11111111111111111000000101";
    println!("expression: {}", expression);
    let identified = nfa.contains(expression);
    println!("identified: {}\n", identified);

    let expression = "101";
    println!("expression: {}", expression);
    let identified = nfa.contains(expression);
    println!("identified: {}\n", identified);
}

fn main() {
    test1();
    test2();
    test3();
}
