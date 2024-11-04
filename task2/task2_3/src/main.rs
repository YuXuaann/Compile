#[macro_use]
extern crate lazy_static;
use dfa::DFA;
use nfa::NFA;
use std::sync::Mutex;
mod dfa;
mod dsu;
mod graph;
mod nfa;

lazy_static! {
    static ref TEST_ID: Mutex<usize> = Mutex::new(0);
}

struct Test {
    nfa: NFA,
    dfa: DFA,
    regular_expression: String,
    expression: Vec<String>,
}

impl Test {
    pub fn from(regular_expression: String, expression: Vec<&str>) -> Self {
        let expression = expression.iter().map(|x| x.to_string()).collect();
        let nfa = NFA::from(&regular_expression);
        let dfa = DFA::from(&nfa);
        Test {
            nfa,
            dfa,
            regular_expression,
            expression,
        }
    }

    pub fn run(&self) {
        println!(
            "========test {} for nfa begin!========",
            TEST_ID.lock().unwrap()
        );
        println!("regular expression: {}\n", self.regular_expression);
        self.nfa.show(*TEST_ID.lock().unwrap());

        for exp in &self.expression {
            println!("expression: {}", exp);
            let identified = self.nfa.contains(exp);
            println!("identified: {}\n", identified);
        }

        println!(
            "========test {} for dfa begin!========",
            TEST_ID.lock().unwrap()
        );
        println!("regular expression: {}\n", self.regular_expression);
        self.dfa.show(*TEST_ID.lock().unwrap());

        for exp in &self.expression {
            println!("expression: {}", exp);
            let identified = self.dfa.contains(exp);
            println!("identified: {}\n", identified);
        }

        *TEST_ID.lock().unwrap() += 1;
    }
}

fn test1() {
    let regular_expression = "(a(ab|c))*d*";
    let expression = vec![
        "aabacacaabddd",
        "aabacacabdd",
        "ddddddd",
        "aaaaaaaaaa",
        "",
        "hello world",
    ];

    let test = Test::from(regular_expression.to_string(), expression);
    test.run();
    let mut test = test;
    test.dfa.minimize();
    test.run();
}

fn test2() {
    let regular_expression = "woc*";
    let expression = vec![
        "wocccccc",
        "woc",
        "wo",
        "woccc",
        "wocccccc",
        "woccccccwocccccc",
    ];
    let test = Test::from(regular_expression.to_string(), expression);
    test.run();
    let mut test = test;
    test.dfa.minimize();
    test.run();
}

fn test3() {
    let regular_expression = "((ab)*|aaa)";
    let expression = vec!["", "aaa", "ab", "abababa", "abababab"];
    let test = Test::from(regular_expression.to_string(), expression);
    test.run();
    let mut test = test;
    test.dfa.minimize();
    test.run();
}

fn test4() {
    let regular_expression = "b(a|b)*bab";
    let expression = vec![
        "",
        "bbbbbbbbbbbbbbbbbbbbbbbbab",
        "ab",
        "bbab",
        "babab",
        "bab",
    ];
    let test = Test::from(regular_expression.to_string(), expression);
    test.run();
    let mut test = test;
    test.dfa.minimize();
    test.run();
}

fn main() {
    test1();
    test2();
    test3();
    test4();
}
