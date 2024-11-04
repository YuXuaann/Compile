use crate::graph::Graph_;
use core::panic;
use std::fmt::Debug;
use std::fmt::Formatter;

pub struct NFA {
    /// 确保起点为0，终点为 len - 1
    pub graph: Graph_,
}

impl Debug for NFA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.graph.edges)
    }
}

impl NFA {
    pub fn from(exp: &str) -> NFA {
        let transfer_exp = add_connect(exp);
        // println!("transferd expression is {}", transfer_exp);
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
        self.graph
            .show_graph(&format!("nfa_{}.dot", id), 0, &[self.graph.edges.len() - 1]);
    }

    pub fn contains(&self, exp: &str) -> bool {
        let mut succeed = false;
        self.graph
            .dfs_search(0, &vec![self.graph.edges.len() - 1], 0, exp, &mut succeed)
    }
}

fn offset(edge: &Vec<(usize, char)>, offset: usize) -> Vec<(usize, char)> {
    let mut result = Vec::new();
    for i in 0..edge.len() {
        result.push((edge[i].0 + offset, edge[i].1));
    }
    result
}

/// connect a and b
fn and(a: &NFA, b: &NFA) -> NFA {
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
    }
}

/// or a and b
fn or(a: &NFA, b: &NFA) -> NFA {
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
    }
}

/// repeat a
fn repeat(a: &NFA) -> NFA {
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
    }
}

/// teminate
fn teminate(a: char) -> NFA {
    let mut result = Vec::new();
    result.push(vec![(1, a)]);
    result.push(vec![]);
    NFA {
        graph: Graph_ { edges: result },
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
