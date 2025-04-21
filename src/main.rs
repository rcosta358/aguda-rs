use std::{env, fs};
use std::path::Path;
use aguda_rs::compile_aguda_program;

fn main() {
    let result = read_source_file().and_then(compile_aguda_program);
    match result {
        Ok(output) => println!("{}", output),
        Err(err) => eprintln!("{}", err),
    }
}

fn read_source_file() -> Result<String, String> {
    let filepath = env::args().nth(1).unwrap_or_else(|| String::from("./main.agu"));
    if filepath.is_empty() || !filepath.ends_with(".agu") {
        return Err("Invalid aguda file".to_string());
    }
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
