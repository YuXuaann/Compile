use crate::grammar::Grammar;
use crate::production::Production;
use crate::symbol::Symbol;
use crate::test::symbols::*;
use std::sync::LazyLock;

#[rustfmt::skip]
lazy_static::lazy_static! {
    pub static ref TEST: std::sync::Mutex<Grammar> = {
        let mut sys_y_grammar = Grammar::new();
        let mut make = |lhs: &LazyLock<Symbol>, rhs: Vec<&LazyLock<Symbol>>|  {
            sys_y_grammar.add_rule(Production::from((*lhs).clone(), rhs.iter().map(|s| (**s).clone()).collect()));
        };

        make(&letter_A, vec![&letter_B, &letter_B]);
        make(&letter_A, vec![&letter_B, &letter_a]);
        make(&letter_B, vec![&letter_A, &letter_A]);
        make(&letter_B, vec![&letter_A, &letter_b]);

        sys_y_grammar.set_start_symbol(letter_A.clone());

        std::sync::Mutex::new(sys_y_grammar)
    };
}
