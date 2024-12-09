mod lexer;
mod syntax;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file>", args[0]);
        std::process::exit(1);
    }

    let tokens = lexer::tokenize(args[1].clone());
    tokens.iter().for_each(|x| {
        println!("{:?}", x);
    });

    let ast = syntax::parse(tokens);
    ast.iter().for_each(|x| {
        println!("{:?}", x);
    });
}
