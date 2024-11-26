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
    pub is_terminal: bool,
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
            is_terminal,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.name == ""
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
        self.name.len() == 1 && self.name[0].is_empty()
    }
    pub fn contains(&self, name: &str) -> bool {
        for symbol in &self.name {
            if symbol.name == name {
                return true;
            }
        }
        false
    }
    pub fn from(name: Vec<Symbol>) -> Self {
        Expression { name: name }
    }
    pub fn construct(name: Vec<&Symbol>) -> Self {
        Expression {
            name: name.into_iter().cloned().collect(),
        }
    }
    pub fn remove_last(&mut self) {
        // println!("[remove_last]: len is {}", self.name.len());
        if self.name.len() == 0 {
            return;
        }
        while let Some(symbol) = self.name.last() {
            if symbol.is_empty() {
                self.name.pop();
            } else {
                break;
            }
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

    pub fn construct_rule(&mut self, lhs: &Symbol, rhs: Vec<&Symbol>) {
        self.add_symbol(lhs.clone());
        for symbol in rhs.clone() {
            self.add_symbol(symbol.clone());
        }
        self.productions
            .entry(lhs.clone())
            .or_insert(Vec::new())
            .push(Expression::construct(rhs.clone()));
    }

    fn delete_rule(&mut self, lhs: &Symbol, rhs: &Vec<Symbol>) {
        if let Some(expressions) = self.productions.get_mut(lhs) {
            expressions.retain(|expression| expression.name != *rhs);
        }
    }

    fn clear_rule(&mut self, lhs: &Symbol) {
        self.productions.remove(lhs);
    }

    pub fn set_start(&mut self, symbol: &Symbol) {
        if !self.non_terminals.contains(symbol) {
            panic!("Non_terminals don't contain {}!", symbol.name);
        }
        self.start = symbol.clone();
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

        for lhs in non_terminals {
            let rhs = self.productions.get(lhs);
            if rhs.is_none() {
                warn!("{} has no production!", lhs.name);
                continue;
            }
            print!("  {} -> ", lhs.name);
            let mut sorted_rhs = rhs.unwrap().clone();
            sorted_rhs.sort();
            for expression in sorted_rhs.iter() {
                for symbol in expression.name.iter() {
                    if symbol.name == "" {
                        print!("ε ");
                    } else {
                        print!("{} ", symbol.name);
                    }
                }
                if expression != sorted_rhs.last().unwrap() {
                    print!("| ");
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
            info!(
                "Extracting common left factor for {}, {}/{}",
                non_terminals[i].name,
                i + 1,
                non_terminals.len()
            );
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
            trie.show_graph(&format!("trie_{}.dot", i), &lhs.name);
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
            info!(
                "Transfer to direct left recursion for {}, {}/{}",
                non_terminals[i].name,
                i + 1,
                non_terminals.len()
            );
            let lhs = &non_terminals[i];
            let rhss = self.productions.get(lhs).cloned();
            if rhss.is_none() {
                warn!("No production for {}", lhs.name);
                continue;
            }
            for expression in rhss.unwrap() {
                if expression.name.is_empty() {
                    continue;
                }
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
            info!(
                "Eliminate left recursion for {}, {}/{}",
                non_terminals[i].name,
                i + 1,
                non_terminals.len()
            );
            let lhs = &non_terminals[i];
            let rhss = self.productions.get(lhs).cloned();
            if rhss.is_none() {
                warn!("No production for {}", lhs.name);
                continue;
            }
            let mut alpha = Vec::new();
            let mut beta = Vec::new();
            for expression in rhss.unwrap() {
                if expression.name.len() == 0 {
                    continue;
                }
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

        info!("[cal_first] symbol is {}", symbol.name);

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
                if symbol.is_terminal {
                    break;
                }
                if !sub_first_result.contains(&Symbol::from("", true)) {
                    break;
                }
                result.remove(&Symbol::from("", true));
            }
        }

        saved_first.entry(symbol.clone()).or_insert(result.clone());
        result
    }

    fn first_non_empty(&self, symbols: &[Symbol]) -> Symbol {
        for symbol in symbols {
            let production = self.productions.get(symbol);
            if production.is_none() {
                return symbol.clone();
            }
            for expression in production.unwrap() {
                if !expression.contains("") {
                    return symbol.clone();
                }
            }
        }
        Symbol::from("", true)
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
                                if symbol.is_empty() {
                                    continue;
                                }
                                result
                                    .entry(rhs_symbol.clone())
                                    .or_insert(HashSet::new())
                                    .insert(symbol.clone());
                            }
                            continue;
                        }

                        for symbol in first
                            .get(&self.first_non_empty(&expression.name[i + 1..]))
                            .unwrap()
                            .iter()
                        {
                            if symbol.is_empty() {
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
                                    if symbol.is_empty() {
                                        continue;
                                    }
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
        &self,
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
        info!("first calculated");

        // follow
        let follow = self.cal_follow(&first);
        (first, follow)
    }

    pub fn show_cal(&self) {
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
                error!("Terminal symbol in follow set!");
                panic!();
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

    pub fn cal_select(&self) -> HashMap<(Symbol, Expression), HashSet<Symbol>> {
        let (first, follow) = self.cal();
        let mut result = HashMap::new();
        for (lhs, expressions) in &self.productions {
            for expression in expressions {
                let mut select = first
                    .get(&self.first_non_empty(&expression.name[..]))
                    .unwrap()
                    .clone();
                if select.contains(&Symbol::from("", true)) {
                    select.remove(&Symbol::from("", true));
                    select.extend(follow.get(lhs).unwrap().clone());
                }
                result.insert((lhs.clone(), expression.clone()), select);
            }
        }
        result
    }

    pub fn show_select(&self) {
        info!("Select:");
        let select = self.cal_select();
        for (symbol, symbols) in &select {
            print!("select({} -> ", symbol.0.name);
            for s in &symbol.1.name {
                if s.name == "" {
                    print!("ε");
                } else {
                    print!("{}", s.name);
                }
            }
            print!(") = {{ ");
            for s in symbols {
                if s.name == "" {
                    print!("ε");
                } else {
                    print!("{}", s.name);
                }
                if s != symbols.into_iter().last().unwrap() {
                    print!(", ");
                }
            }
            println!(" }}");
        }
    }

    pub fn show_table(&self) {
        info!("Table:");
        let select = self.cal_select();
        let mut table = HashMap::new();
        let mut is_ll1_flag = true;
        for ((symbol, expression), symbols) in &select {
            for s in symbols {
                if table.contains_key(&(symbol.clone(), s.clone())) {
                    is_ll1_flag = false;
                }
                table.insert((symbol.clone(), s.clone()), expression.clone());
            }
        }

        let mut terminals = self.terminals.clone();
        terminals.insert(Symbol::from("$", true));

        print!("{:<15}", " ");
        for terminal in &terminals {
            if !terminal.is_empty() {
                print!("{:<15}", terminal.name);
            }
        }
        println!();

        for non_terminal in &self.non_terminals {
            print!("{:<15}", non_terminal.name);
            for terminal in &terminals {
                if terminal.is_empty() {
                    continue;
                }
                if let Some(expression) = table.get(&(non_terminal.clone(), terminal.clone())) {
                    let production = non_terminal.name.clone() + "->";
                    let expression_name: String = expression
                        .name
                        .iter()
                        .map(|x| {
                            if x.is_empty() {
                                "ε".to_string()
                            } else {
                                x.name.clone()
                            }
                        })
                        .collect();
                    print!("{:<15}", production + &expression_name);
                } else {
                    print!("{:<15}", "");
                }
            }
            println!();
        }
        info!("Is LL(1): {}", is_ll1_flag);
        if !is_ll1_flag {
            warn!("Table after is wrong!");
        }
    }

    /// 自顶向下的预测语法分析算法
    pub fn predict(&self, input: &str) -> bool {
        let mut stack = Vec::new();
        stack.push(Symbol::from("$", true));
        stack.push(self.start.clone());

        let mut input = input.chars().collect::<Vec<_>>();
        input.push('$');

        let select = self.cal_select();
        let mut input_index = 0;

        while let Some(top) = stack.pop() {
            if top.is_terminal {
                if top.name == input[input_index].to_string() {
                    input_index += 1;
                } else {
                    return false;
                }
            } else {
                let mut found = false;
                for ((lhs, expression), symbols) in &select {
                    if lhs == &top
                        && symbols.contains(&Symbol::from(&input[input_index].to_string(), true))
                    {
                        for symbol in expression.name.iter().rev() {
                            if !symbol.is_empty() {
                                stack.push(symbol.clone());
                            }
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    return false;
                }
            }
        }

        input_index == input.len()
    }

    pub fn identity(&self, path: &str) -> Result<(), String> {
        let code = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        info!("code: \n{}", code);
        let code = code.replace("\n", "");
        let code = code.replace(" ", "");
        info!("code after flaten: \n{}", code);

        if self.predict(&code) {
            Ok(())
        } else {
            Err("Identified Failed".to_string())
        }
    }
}
