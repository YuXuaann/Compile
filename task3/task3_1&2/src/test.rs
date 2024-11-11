use log::debug;

use crate::grammar::{Expression, Symbol, CFG};
use std::collections::HashMap;

lazy_static! {
    static ref SYMBOL: HashMap<char, Symbol> = {
        let mut symbols = HashMap::new();
        for i in 0..26 {
            let c1 = (b'A' + i) as char;
            let s1 = Symbol::from(&c1.to_string(), false);
            symbols.insert(c1, s1);
            let c2 = (b'a' + i) as char;
            let s2 = Symbol::from(&c2.to_string(), true);
            symbols.insert(c2, s2);
        }
        for i in 0..10 {
            let c = (b'0' + i) as char;
            let s = Symbol::from(&c.to_string(), true);
            symbols.insert(c, s);
        }
        symbols.insert('+', Symbol::from("+", true));
        symbols.insert('-', Symbol::from("-", true));
        symbols.insert('*', Symbol::from("*", true));
        symbols.insert('/', Symbol::from("/", true));
        symbols.insert('(', Symbol::from("(", true));
        symbols.insert(')', Symbol::from(")", true));
        symbols
    };
}

fn s2symbol(c: &str) -> Vec<Symbol> {
    let mut v = Vec::new();
    for i in c.chars() {
        if let Some(symbol) = SYMBOL.get(&i) {
            v.push(symbol.clone());
        } else {
            panic!("Invalid symbol: {}", i);
        }
    }
    v
}

fn c2symbol(c: char) -> Symbol {
    if let Some(symbol) = SYMBOL.get(&c) {
        symbol.clone()
    } else {
        panic!("Invalid symbol: {}", c);
    }
}

pub struct Test {
    cfg: CFG,
}

impl Test {
    pub fn new() -> Self {
        debug!("Test begin!!!");
        Test { cfg: CFG::new() }
    }

    pub fn add_rule(&mut self, lhs: char, rhs: &str) {
        self.cfg
            .add_rule(&c2symbol(lhs), &Expression::from(s2symbol(rhs)));
    }

    pub fn special_add_rule(&mut self, lhs: char, rhs: &str) {
        self.cfg.add_rule(
            &c2symbol(lhs),
            &Expression::from(vec![Symbol::from(rhs, true)]),
        );
    }

    pub fn set_start(&mut self, name: &str) {
        self.cfg.set_start(name);
    }

    pub fn extract_common_left_factor(&mut self) {
        self.cfg.extract_common_left_factor();
    }

    pub fn transfer_to_direct_left_recursion(&mut self) {
        self.cfg.transfer_to_direct_left_recursion();
    }

    pub fn eliminate_left_recursion(&mut self) {
        self.cfg.eliminate_left_recursion();
    }

    pub fn show(&self) {
        self.cfg.show();
    }
}
