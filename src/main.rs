use std::{env, fs};
use std::path::Path;
use aguda_rs::parser::parse_aguda_program;

fn main() {
    match read_source_file() {
        Ok(src) => {
            let result = parse_aguda_program(&src);
            match result {
                Ok(ast) => println!("\n✅ Parsed successfully\n{}", ast.to_text()),
                Err(e) => eprintln!("\n❌ Couldn't parse file\n{}", e),
            }
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn read_source_file() -> Result<String, String> {
    let filepath = env::args().nth(1).unwrap_or_else(|| String::from("./main.agu"));
    let path = Path::new(&filepath);
    if !path.exists() {
        return Err(format!("The source file '{}' does not exist", filepath));
    }
    match fs::read_to_string(path) {
        Ok(content) if content.trim().is_empty() => {
            Err(format!("The source file '{}' is empty", filepath))
        }
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Error reading file '{}': {}", filepath, e)),
    }
}