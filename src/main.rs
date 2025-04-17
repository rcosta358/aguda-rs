use std::fs;
use aguda_rs::lexer::Lexer;
use aguda_rs::parser::Parser;

fn main() {
    let src = fs::read_to_string("./main.agu").expect("couldn't read source file");
    let mut lexer = Lexer::new(&src);
    match lexer.tokenize() {
        Ok(tokens) => {
            let parser = Parser::new(&src, tokens);
            match parser.parse() {
                Ok(ast) => println!("{}", ast.to_text()),
                Err(e) => eprintln!("Syntax Error: {}", e),
            }
        }
        Err(e) => eprintln!("Lexical Error: {}", e)
    }
}
