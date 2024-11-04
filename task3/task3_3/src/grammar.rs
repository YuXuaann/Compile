use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[allow(unused_imports)]
use log::{self, info, warn};
#[allow(unused_imports)]
use log::{debug, error};

use crate::trie::Trie;

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Symbol {
    pub name: String,
    is_terminal: bool,
}

impl Symbol {
    pub fn new() -> Self {
        Symbol {
            name: String::new(),
            is_terminal: false,
        }
    }
    pub fn from(name: &str, is_terminal: bool) -> Self {
        Symbol {
            name: name.to_string(),
            is_terminal: is_terminal,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct Expression {
    pub name: Vec<Symbol>,
}

impl Expression {
    pub fn new() -> Self {
        Expression { name: Vec::new() }
    }
    pub fn empty() -> Self {
        Expression {
            name: vec![Symbol::from("", true)],
        }
    }
    pub fn is_empty(&self) -> bool {
        self.name.len() == 1 && self.name[0].name == ""
    }
    pub fn from(name: Vec<Symbol>) -> Self {
        Expression { name: name }
    }
    pub fn remove_last(&mut self) {
        if self.name.len() == 0 {
            return;
        }
        while self.name.last().unwrap().name == "" {
            self.name.pop();
        }
    }
    pub fn push(&mut self, symbol: Symbol) {
        self.name.push(symbol);
    }
    pub fn extend(&mut self, symbols: Vec<Symbol>) {
        self.name.extend(symbols);
    }
    #[allow(unused)]
    pub fn show(&self) {
        for symbol in self.name.iter() {
            print!("{}", symbol.name);
        }
    }
}

/// Context-Free Grammar
pub struct CFG {
    start: Symbol,
    terminals: HashSet<Symbol>,
    non_terminals: HashSet<Symbol>,
    productions: HashMap<Symbol, Vec<Expression>>,
}

impl CFG {
    pub fn new() -> CFG {
        CFG {
            start: Symbol::new(),
            terminals: HashSet::new(),
            non_terminals: HashSet::new(),
            productions: HashMap::new(),
        }
    }

    fn add_symbol(&mut self, symbol: Symbol) {
        if symbol.is_terminal {
            self.terminals.insert(symbol);
        } else {
            self.non_terminals.insert(symbol);
        }
    }

    pub fn add_rule(&mut self, lhs: &Symbol, rhs: &Expression) {
        self.add_symbol(lhs.clone());
        for symbol in rhs.name.clone() {
            self.add_symbol(symbol);
        }
        self.productions
            .entry(lhs.clone())
            .or_insert(Vec::new())
            .push(Expression::from(rhs.name.clone()));
    }

    fn delete_rule(&mut self, lhs: &Symbol, rhs: &Vec<Symbol>) {
        if let Some(expressions) = self.productions.get_mut(lhs) {
            expressions.retain(|expression| expression.name != *rhs);
        }
    }

    fn clear_rule(&mut self, lhs: &Symbol) {
        self.productions.remove(lhs);
    }

    pub fn set_start(&mut self, name: &str) {
        for symbol in self.non_terminals.iter() {
            if symbol.name == name {
                self.start = symbol.clone();
                return;
            }
        }
    }

    pub fn show(&self) {
        log::info!("Start: {}", self.start.name);
        log::info!("Terminals:");
        let mut terminals: Vec<&Symbol> = self.terminals.iter().clone().collect();
        terminals.sort();
        for symbol in terminals.iter() {
            print!("  {}", symbol.name);
        }
        println!();
        log::info!("Non-Terminals:");
        let mut non_terminals: Vec<&Symbol> = self.non_terminals.iter().clone().collect();
        non_terminals.sort();
        for symbol in non_terminals.iter() {
            print!("  {}", symbol.name);
        }
        println!();
        log::info!("Rules:");
        for (lhs, rhs) in self.productions.iter() {
            print!("  {} -> ", lhs.name);
            let mut sorted_rhs = rhs.clone();
            sorted_rhs.sort();
            for expression in sorted_rhs.iter() {
                for symbol in expression.name.iter() {
                    if symbol.name == "" {
                        print!("ε");
                    } else {
                        print!("{}", symbol.name);
                    }
                }
                if expression != sorted_rhs.last().unwrap() {
                    print!(" | ");
                }
            }
            println!();
        }
        println!();
    }

    /// 提取公共左因子
    pub fn extract_common_left_factor(&mut self) {
        let non_terminals: Vec<Symbol> = self.non_terminals.iter().map(|x| x.clone()).collect();

        for i in 0..non_terminals.len() {
            let lhs = &non_terminals[i];
            let rhss = self.productions.get(lhs).cloned();
            if rhss.is_none() {
                warn!("No production for {}", lhs.name);
                continue;
            }
            let mut trie = Trie::new();
            for expression in rhss.unwrap() {
                trie.insert(&expression);
            }
            trie.show_graph(&format!("trie_{}.dot", i));
            let prefixes = trie.prefix_and_suffix();

            self.clear_rule(lhs);
            let mut new_symbol = lhs.clone();
            for (pre, sufs) in prefixes {
                if sufs.is_empty() {
                    self.add_rule(lhs, &pre);
                    continue;
                }
                new_symbol = Symbol::from(&(new_symbol.name.clone() + &String::from("'")), false);
                let mut new_rhs = pre;
                new_rhs.push(new_symbol.clone());
                self.add_rule(lhs, &new_rhs);

                for suf in sufs {
                    let new_rhs = suf;
                    self.add_rule(&new_symbol, &new_rhs);
                }
            }
        }
    }

    /// 最坏情况下O(m²) 产生式集合总长度m，默认经过提取左公共因子
    pub fn transfer_to_direct_left_recursion(&mut self) {
        let non_terminals: Vec<Symbol> = self.non_terminals.iter().map(|x| x.clone()).collect();
        let non_terminals_index: HashMap<Symbol, usize> = non_terminals
            .iter()
            .enumerate()
            .map(|(i, x)| (x.clone(), i))
            .collect();

        for i in 0..non_terminals.len() {
            let lhs = &non_terminals[i];
            let rhss = self.productions.get(lhs).cloned();
            if rhss.is_none() {
                warn!("No production for {}", lhs.name);
                continue;
            }
            for expression in rhss.unwrap() {
                let first_symbol = &expression.name[0];
                if first_symbol.is_terminal {
                    continue;
                }
                let first_symbol_index = non_terminals_index.get(first_symbol);
                if let Some(j) = first_symbol_index {
                    if *j < i {
                        // 代换以消除间接左递归
                        let rest_rhs = expression.name[1..].to_vec();
                        let substitution =
                            self.productions.get(&non_terminals[*j]).cloned().unwrap();
                        for sub_expression in substitution {
                            let mut new_rhs = sub_expression.name.clone();
                            new_rhs.extend(rest_rhs.clone());
                            self.add_rule(lhs, &Expression::from(new_rhs));
                            // info!("Substitute {} for {}", non_terminals[*j].name, lhs.name);
                        }
                        self.delete_rule(lhs, &expression.name);
                        // info!("Delete rule: {} -> {}", lhs.name, expression.name[0].name);
                    }
                } else {
                    panic!("Symbol not found!");
                }
            }
        }
    }

    /// 默认已经过消除间接左递归
    pub fn eliminate_left_recursion(&mut self) {
        let non_terminals: Vec<Symbol> = self.non_terminals.iter().map(|x| x.clone()).collect();

        for i in 0..non_terminals.len() {
            let lhs = &non_terminals[i];
            let rhss = self.productions.get(lhs).cloned();
            if rhss.is_none() {
                warn!("No production for {}", lhs.name);
                continue;
            }
            let mut alpha = Vec::new();
            let mut beta = Vec::new();
            for expression in rhss.unwrap() {
                let first_symbol = &expression.name[0];
                if first_symbol == lhs {
                    alpha.push(expression.name[1..].to_vec());
                } else {
                    beta.push(expression.name);
                }
            }
            if !alpha.is_empty() {
                self.clear_rule(lhs);
                let new_symbol = Symbol::from(&(lhs.name.clone() + &String::from("'")), false);
                for b in beta {
                    let mut rhs = b.clone();
                    rhs.push(new_symbol.clone());
                    self.add_rule(lhs, &Expression::from(rhs));
                }
                for a in alpha {
                    let mut rhs = a.clone();
                    rhs.push(new_symbol.clone());
                    self.add_rule(&new_symbol, &Expression::from(rhs));
                }
                self.add_rule(&new_symbol, &Expression::from(vec![Symbol::from("", true)]));
            }
        }
    }

    fn cal_first(
        &self,
        symbol: &Symbol,
        saved_first: &mut HashMap<Symbol, HashSet<Symbol>>,
    ) -> HashSet<Symbol> {
        if saved_first.contains_key(symbol) {
            return saved_first.get(symbol).unwrap().clone();
        }

        if symbol.is_terminal {
            saved_first
                .entry(symbol.clone())
                .or_insert(vec![symbol.clone()].into_iter().collect());
            return saved_first.get(symbol).unwrap().clone();
        }

        let mut result = HashSet::new();

        let production = self.productions.get(&symbol);
        if production.is_none() {
            error!("No production for {}", symbol.name);
            panic!();
        }
        for expression in self.productions.get(&symbol).unwrap() {
            for symbol in &expression.name {
                let sub_first_result = self.cal_first(symbol, saved_first);
                result.extend(sub_first_result.clone());
                if !sub_first_result.contains(&Symbol::from("", true)) {
                    break;
                }
            }
        }

        saved_first.entry(symbol.clone()).or_insert(result.clone());
        result
    }

    pub fn cal_follow(
        &self,
        first: &HashMap<Symbol, HashSet<Symbol>>,
    ) -> HashMap<Symbol, HashSet<Symbol>> {
        let mut result = HashMap::new();
        result.insert(
            self.start.clone(),
            vec![Symbol::from("$", true)].into_iter().collect(),
        );

        loop {
            let old_result = result.clone();
            for (lhs_symbol, expressions) in &self.productions {
                let follow_set = result
                    .entry(lhs_symbol.clone())
                    .or_insert(HashSet::new())
                    .clone();
                for expression in expressions {
                    let len = expression.name.len();
                    for (i, rhs_symbol) in expression.name.iter().enumerate() {
                        if rhs_symbol.is_terminal {
                            continue;
                        }

                        if i == len - 1 {
                            for symbol in follow_set.iter() {
                                result
                                    .entry(rhs_symbol.clone())
                                    .or_insert(HashSet::new())
                                    .insert(symbol.clone());
                            }
                            continue;
                        }

                        for symbol in first.get(&expression.name[i + 1]).unwrap().iter() {
                            if symbol.name == "" {
                                continue;
                            }
                            result
                                .entry(rhs_symbol.clone())
                                .or_insert(HashSet::new())
                                .insert(symbol.clone());
                        }

                        if !expression.name[i + 1].is_terminal {
                            let production = &self.productions.get(&expression.name[i + 1]);
                            if production.is_none() {
                                error!("No production for {}", expression.name[i + 1].name);
                                panic!();
                            }
                            let mut flag = false;
                            for expression in production.unwrap() {
                                if expression.is_empty() {
                                    flag = true;
                                    break;
                                }
                            }
                            if flag {
                                for symbol in follow_set.iter() {
                                    result
                                        .entry(rhs_symbol.clone())
                                        .or_insert(HashSet::new())
                                        .insert(symbol.clone());
                                }
                            }
                        }
                    }
                }
            }
            if old_result == result {
                break;
            }
        }

        result
    }

    pub fn cal(
        &mut self,
    ) -> (
        HashMap<Symbol, HashSet<Symbol>>,
        HashMap<Symbol, HashSet<Symbol>>,
    ) {
        // first
        let mut saved_first: HashMap<Symbol, HashSet<Symbol>> = HashMap::new();
        let mut first = HashMap::new();
        for symbol in self.terminals.iter() {
            first.insert(symbol.clone(), self.cal_first(symbol, &mut saved_first));
        }
        for symbol in self.non_terminals.iter() {
            first.insert(symbol.clone(), self.cal_first(symbol, &mut saved_first));
        }

        // follow
        let follow = self.cal_follow(&first);
        (first, follow)
    }

    pub fn show_cal(&mut self) {
        let (first, follow) = self.cal();

        info!("First:");
        for (symbol, first_set) in first.iter() {
            if symbol.is_terminal {
                continue;
            }
            let mut symbol_name = symbol.name.clone();
            if symbol.name == "" {
                symbol_name = String::from("ε");
            }
            print!("First({}) = {{ ", symbol_name);
            for sub_symbol in first_set.iter() {
                if sub_symbol.name == "" {
                    print!("ε");
                } else {
                    print!("{}", sub_symbol.name);
                }
                if sub_symbol != first_set.into_iter().last().unwrap() {
                    print!(" ,");
                }
            }
            println!(" }}");
        }

        info!("Follow:");
        for (symbol, follow_set) in follow.iter() {
            if symbol.is_terminal {
                continue;
            }
            let mut symbol_name = symbol.name.clone();
            if symbol.name == "" {
                symbol_name = String::from("ε");
            }
            print!("Follow({}) = {{ ", symbol_name);
            for sub_symbol in follow_set.iter() {
                if sub_symbol.name == "" {
                    print!("ε");
                } else {
                    print!("{}", sub_symbol.name);
                }
                if sub_symbol != follow_set.into_iter().last().unwrap() {
                    print!(" ,");
                }
            }
            println!(" }}");
        }
        println!();
    }
}
