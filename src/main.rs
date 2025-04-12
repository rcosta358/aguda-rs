use std::{env, fs};
use aguda_rs::ast::Program;
use aguda_rs::rustlrparser::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = if args.len() > 1 { &args[1] } else { "./main.agu" };
    let src = fs::read_to_string(&filepath).expect("Couldn't read source file");
    let lexer = rustlrlexer::from_str(&src);
    let mut parser = make_parser(lexer);
    let result = parse_with(&mut parser);
    match result {
        Ok(raw_ast) => {
            let ast = Program::convert(raw_ast);
            println!("{}", ast.to_text());
        }
        Err(_) => eprintln!("Error parsing file"),
    }
}
