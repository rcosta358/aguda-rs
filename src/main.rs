use std::fs;
use aguda_rs::agudaparser::*;

fn main() {
    let src = fs::read_to_string("./main.agu").expect("couldn't read source file");
    let tokenizer1 = agudalexer::from_str(&src);
    let mut parser = make_parser(tokenizer1);
    let result = parse_with(&mut parser);
    match result {
        Ok(ast) => {
            println!("Parsed successfully: {:?}", ast);
        }
        Err(err) => {
            println!("Error parsing: {:?}", err);
        }
    }
}