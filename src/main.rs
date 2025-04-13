use std::{env, fs};
use aguda_rs::parser::parse_aguda_program;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = if args.len() > 1 { &args[1] } else { "./main.agu" };
    let src = fs::read_to_string(&filepath).expect("Couldn't read source file");
    let result = parse_aguda_program(&src);
    match result {
        Ok(ast) => println!("{}", ast.to_text()),
        Err(e) => eprintln!("{}", e),
    }
}
