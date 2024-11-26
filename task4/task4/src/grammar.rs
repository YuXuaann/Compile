use log::warn;

use crate::symbol::Symbol;
use std::collections::{HashMap, HashSet};

pub struct Grammar {
    pub start_symbol: Symbol,
    pub terminals: HashSet<Symbol>,
    pub non_terminals: HashSet<Symbol>,
    pub productions: HashMap<Symbol, Vec<Symbol>>,
}

/// 构建文法
impl Grammar {
    /// 新建空白文法
    pub fn new() -> Self {
        Grammar {
            start_symbol: Symbol::epsilon(),
            terminals: HashSet::new(),
            non_terminals: HashSet::new(),
            productions: HashMap::new(),
        }
    }

    /// 添加产生式
    pub fn add_rule(&mut self, lhs: Symbol, rhs: Vec<Symbol>) {}
    pub fn set_start_symbol(&mut self, symbol: Symbol) {
        self.start_symbol = symbol;
    }
}

/// 实现功能
impl Grammar {
    pub fn substitute_all_before(&mut self) {}
}

impl std::fmt::Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        log::info!("Grammar Display");
        log::info!("Start: {}", self.start_symbol);

        log::info!("Terminals:");
        for symbol in self.terminals.iter() {
            write!(f, "  {}", symbol)?;
        }
        write!(f, "\n")?;

        log::info!("Non-Terminals:");
        for symbol in self.non_terminals.iter() {
            write!(f, "  {}", symbol)?;
        }
        write!(f, "\n")?;

        log::info!("Rules:");
        for (lhs, rhs) in self.productions.iter() {
            write!(f, "  {} -> ", lhs)?;
            for symbol in rhs.iter() {
                write!(f, "{} ", symbol)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;

        Ok(())
    }
}
