use std::env;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    lexer::tokenize(args[1].clone()).iter().for_each(|x| {
        println!("{:?}", x);
    });
}
