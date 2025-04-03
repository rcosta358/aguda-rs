mod lexer;
mod ast;
mod parser;
mod utils;

use std::fs;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let src = fs::read_to_string("./main.agu").expect("couldn't read source file");
    let mut lexer = Lexer::new(&src);
    match lexer.tokenize() {
        Ok(tokens) => {
            let parser = Parser::new(&src, tokens);
            match parser.parse() {
                Ok(ast) => println!("{:#?}", ast),
                Err(e) => eprintln!("Syntax Error: {}", e),
            }
        }
        Err(e) => eprintln!("Lexical Error: {}", e)
    }
}
