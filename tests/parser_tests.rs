use std::fs;
use std::path::Path;
use aguda_rs::lexer::Lexer;
use aguda_rs::parser::Parser;

#[test]
fn test_parser() {
    let base_dir = Path::new("./tests/");
    let valid_dir = base_dir.join("valid");
    let invalid_syntax_dir = base_dir.join("invalid-syntax");
    let invalid_semantic_dir = base_dir.join("invalid-semantic");

    let (valid_passed, valid_failed) = test_agu_files_in_dir(&valid_dir, true);
    let valid_tests = valid_passed + valid_failed;
    let (invalid_syntax_passed, invalid_syntax_failed) = test_agu_files_in_dir(&invalid_syntax_dir, false);
    let invalid_syntax_tests = invalid_syntax_passed + invalid_syntax_failed;
    let (invalid_semantic_passed, invalid_semantic_failed) = test_agu_files_in_dir(&invalid_semantic_dir, true);
    let invalid_semantic_tests = invalid_semantic_passed + invalid_semantic_failed;
    let total_tests = valid_tests + invalid_syntax_tests + invalid_semantic_tests;

    println!("\n📊 Test Summary ({})", total_tests);
    println!("========================");
    println!("Valid tests ({})", valid_tests);
    println!("✅ Passed: {}", valid_passed);
    println!("❌ Failed: {}", valid_failed);
    println!("========================");
    println!("Invalid syntax tests ({})", invalid_syntax_tests);
    println!("✅ Passed: {}", invalid_syntax_passed);
    println!("❌ Failed: {}", invalid_syntax_failed);
    println!("========================");
    println!("Invalid semantic tests ({})", invalid_semantic_tests);
    println!("✅ Passed: {}", invalid_semantic_passed);
    println!("❌ Failed: {}", invalid_semantic_failed);
    println!("========================");

    assert_eq!(valid_failed, 0, "Some valid tests failed");
    assert_ne!(invalid_syntax_failed, 0, "Some invalid syntax tests passed");
    assert_eq!(invalid_semantic_passed, 0, "Some invalid semantic tests failed");
}

fn test_agu_files_in_dir(dir: &Path, show_err: bool) -> (i32, i32) {
    assert!(dir.exists(), "Test directory not found");
    let mut passed = 0;
    let mut failed = 0;
    for entry in fs::read_dir(dir).expect("Failed to read base test directory") {
        let path = entry.expect("Invalid entry").path();
        if path.is_dir() {
            match test_agu_file_in_dir(&path) {
                Ok(_) => passed += 1,
                Err(err) => {
                    if show_err {
                        println!("{}", err);
                    }
                    failed += 1;
                }
            }
        }
    }
    (passed, failed)
}

fn test_agu_file_in_dir(dir: &Path) -> Result<String, String> {
    let agu_file = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read dir {:?}: {}", dir, e))?
        .map(|entry| entry.expect("Invalid entry").path())
        .find(|p| p.extension().map_or(false, |ext| ext == "agu"));

    let agu_path = agu_file.ok_or_else(|| format!("No .agu file found in {:?}", dir))?;
    let src = fs::read_to_string(&agu_path)
        .map_err(|e| format!("Failed to read file {:?}: {}", agu_path, e))?;

    let mut lexer = Lexer::new(&src);
    let result = match lexer.tokenize() {
        Ok(tokens) => {
            let parser = Parser::new(&src, tokens);
            match parser.parse() {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Syntax Error: {}", e)),
            }
        }
        Err(e) => Err(format!("Lexical Error: {}", e))
    };
    match result {
        Ok(_) => Ok(format!("✅ PARSED: {:?}", dir.file_name().unwrap())),
        Err(e) => Err(format!("❌ {}: {:?}\n", e, dir.file_name().unwrap()))
    }
}