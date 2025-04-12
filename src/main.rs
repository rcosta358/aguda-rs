use std::fs;
use aguda_rs::ast::Program;
use aguda_rs::rustlrparser::*;

fn main() {
    let src = fs::read_to_string("./main.agu").expect("couldn't read source file");
    let tokenizer1 = rustlrlexer::from_str(&src);
    let mut parser = make_parser(tokenizer1);
    let result = parse_with(&mut parser);
    if let Ok(raw_ast) = result {
        let ast = Program::convert(raw_ast);
        println!("{}", ast.to_text());
    }
}
