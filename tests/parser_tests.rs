use std::fs;
use std::path::Path;
use aguda_rs::{Lexer, Parser};

#[test]
fn test_all_valid_agu_files() {
    let base_dir = Path::new("./tests/valid");
    assert!(base_dir.exists(), "Valid test directory not found");

    let mut passed = 0;
    let mut failed = 0;
    let mut failures = vec![];

    for entry in fs::read_dir(base_dir).expect("Failed to read base test directory") {
        let path = entry.expect("Invalid entry").path();
        if path.is_dir() {
            match test_agu_file_in_dir(&path) {
                Ok(_) => passed += 1,
                Err(err) => {
                    failed += 1;
                    failures.push(err);
                }
            }
        }
    }

    if !failures.is_empty() {
        println!("\n❌ Failure details:");
        for err in &failures {
            println!("{}", err);
        }
    }

    println!("\n📊 Test Summary:");
    println!("✅ Passed: {}", passed);
    println!("❌ Failed: {}", failed);
}

fn test_agu_file_in_dir(dir: &Path) -> Result<(), String> {
    let agu_file = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read dir {:?}: {}", dir, e))?
        .map(|entry| entry.expect("Invalid entry").path())
        .find(|p| p.extension().map_or(false, |ext| ext == "agu"));

    let agu_path = agu_file.ok_or_else(|| format!("No .agu file found in {:?}", dir))?;

    let src = fs::read_to_string(&agu_path)
        .map_err(|e| format!("Failed to read file {:?}: {}", agu_path, e))?;

    let mut lexer = Lexer::new(&src);
    match lexer.tokenize() {
        Ok(tokens) => {
            let parser = Parser::new(&src, tokens);
            match parser.parse() {
                Ok(_) => {
                    println!("✅ Parsed: {:?}", dir.file_name().unwrap());
                    Ok(())
                }
                Err(e) => Err(format!(
                    "❌ Parser error in {:?}: {}",
                    dir.file_name().unwrap(),
                    e
                )),
            }
        }
        Err(e) => Err(format!(
            "❌ Lexer error in {:?}: {}",
            dir.file_name().unwrap(),
            e
        )),
    }
}
