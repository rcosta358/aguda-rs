mod lexer;
mod ast;

use std::fs;
use crate::lexer::Lexer;

fn main() {
    let src = fs::read_to_string("./main.aguda").unwrap();
    let mut lexer = Lexer::new(&src);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
}