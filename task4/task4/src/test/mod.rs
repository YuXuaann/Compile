use log::info;
use rules::TEST;
use symbols::*;

mod rules;
pub mod symbols;

fn show() {
    info!("[test] show grammar");
    let test = TEST.lock().unwrap();
    println!("{}", test);
}

fn test_productions_from(symbol: Symbol) {
    info!("[test] productions from symbol: {}", symbol);
    let test = TEST.lock().unwrap();
    let productions = test.productions_from(&symbol);
    productions.iter().for_each(|p| println!("{}", p));
}

fn test_production_substitute(production: Production, symbol: Symbol, replacement: Vec<Symbol>) {
    info!(
        "[test] test {} substitute from symbol: {} to {:?}",
        production, symbol, replacement
    );
    let mut production = production;
    println!("before substitute: {:?}", production);
    production.substitute(&symbol, &replacement);
    println!("after substitute: {:?}", production);
}

pub fn test_main() {
    show();
    test_productions_from(letter_A.clone());
    test_productions_from(letter_B.clone());
    test_production_substitute(
        Production::from(
            letter_A.clone(),
            vec![letter_H.clone(), Epsilon.clone(), digit_1.clone()],
        ),
        digit_1.clone(),
        vec![letter_A.clone(), letter_B.clone()],
    );
    test_production_substitute(
        Production::from(
            letter_A.clone(),
            vec![letter_H.clone(), Epsilon.clone(), digit_1.clone()],
        ),
        Epsilon.clone(),
        vec![letter_A.clone(), letter_B.clone()],
    );
    test_production_substitute(
        Production::from(
            letter_A.clone(),
            vec![letter_H.clone(), Epsilon.clone(), digit_1.clone()],
        ),
        letter_B.clone(),
        vec![letter_A.clone(), letter_B.clone()],
    );
}
