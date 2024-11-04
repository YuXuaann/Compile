use crate::{graph::Graph_, nfa::NFA};
use std::collections::{HashMap, HashSet};

pub struct DFA {
    start_state: usize,
    end_states: Vec<usize>,
    graph: Graph_,
}

impl DFA {
    pub fn from(nfa: &NFA) -> Self {
        let graph = &nfa.graph;
        let terminal = graph.get_terminal();
        let start_state = graph.extend_from_point(0);
        let mut end_states = Vec::new();
        let mut states = HashSet::new();
        states.insert(start_state.clone());
        graph.dfs_get_dfa_states(&start_state, &terminal, &mut states);
        let states = states.into_iter().collect::<Vec<_>>();
        let mut index_of_state = HashMap::new();
        for (i, state) in states.iter().enumerate() {
            index_of_state.insert(state, i);
        }
        let mut graph_res = Graph_::new(states.len());

        for u in states.iter() {
            for &w in &terminal {
                let v = graph.extend_from_point_vec_and_val(u, w);
                if v.len() != 0 {
                    graph_res.add_edge(
                        *index_of_state.get(u).unwrap(),
                        *index_of_state.get(&v).unwrap(),
                        w,
                    );
                }
            }
            if u.contains(&(nfa.graph.edges.len() - 1)) {
                end_states.push(*index_of_state.get(u).unwrap());
            }
        }

        DFA {
            start_state: *index_of_state.get(&start_state).unwrap(),
            end_states: end_states,
            graph: graph_res,
        }
    }

    pub fn show(&self, id: usize) {
        println!("start state: {}", self.start_state);
        println!("end states: {:?}", self.end_states);
        self.graph.show_graph(&format!("dfa_{}.dot", id));
    }

    pub fn contains(&self, exp: &str) -> bool {
        let mut succeed = false;
        self.graph
            .dfs_search(self.start_state, &self.end_states, 0, exp, &mut succeed)
    }
}
