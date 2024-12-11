// use std::fs::File;
// use std::io::Write;

mod lexer;
mod semantic;
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

    println!();

    let ast = syntax::parse(tokens);
    // ast.iter().for_each(|x| {
    //     println!("{:?}", x);
    // });

    //Semantic Analysis
    let optim_ast = semantic::semantic(&ast, &args[1].clone());

    // let dot_string = ast[0].to_dot();
    // let mut file = File::create("result_pic/output.dot").expect("Unable to create file");
    // file.write_all(dot_string.as_bytes())
    //     .expect("Unable to write data");
    // std::process::Command::new("dot")
    //     .args(&[
    //         "-Tpng",
    //         "result_pic/output.dot",
    //         "-o",
    //         "result_pic/output.png",
    //     ])
    //     .output()
    //     .expect("Failed to execute dot command");
}
