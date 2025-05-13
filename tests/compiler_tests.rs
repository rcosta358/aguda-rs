use std::fs;
use std::path::Path;
use aguda_rs::compile_aguda_program;
use aguda_rs::diagnostics::formatting::format_errors;

#[test]
fn test_compiler() {
    let base_dir = Path::new("./aguda-testing/test");
    let valid_dir = base_dir.join("valid");
    let invalid_syntax_dir = base_dir.join("invalid-syntax");
    let invalid_semantic_dir = base_dir.join("invalid-semantic");

    let (valid_passed, valid_failed) = test_agu_files_in_dir(&valid_dir, true);
    let valid_tests = valid_passed + valid_failed;
    let (invalid_syntax_passed, invalid_syntax_failed) = test_agu_files_in_dir(&invalid_syntax_dir, false);
    let invalid_syntax_tests = invalid_syntax_passed + invalid_syntax_failed;
    let (invalid_semantic_passed, invalid_semantic_failed) = test_agu_files_in_dir(&invalid_semantic_dir, false);
    let invalid_semantic_tests = invalid_semantic_passed + invalid_semantic_failed;
    let total_tests = valid_tests + invalid_syntax_tests + invalid_semantic_tests;
    let failed_tests = valid_failed + invalid_syntax_passed + invalid_semantic_passed;

    println!("\n📊 Test Summary");
    println!("========================");
    println!("Valid tests ({})", valid_tests);
    println!("✅  Passed: {}", valid_passed);
    println!("❌  Failed: {}", valid_failed);
    println!("========================");
    println!("Invalid syntax tests ({})", invalid_syntax_tests);
    println!("✅  Passed: {}", invalid_syntax_passed);
    println!("❌  Failed: {}", invalid_syntax_failed);
    println!("========================");
    println!("Invalid semantic tests ({})", invalid_semantic_tests);
    println!("✅  Passed: {}", invalid_semantic_passed);
    println!("❌  Failed: {}", invalid_semantic_failed);
    println!("========================");
    println!("📝  Total tests: {}", total_tests);
    println!("⚠️  Failures: {}", failed_tests);
    println!("========================");

    assert_eq!(valid_failed, 0, "Some valid tests failed");
    assert_eq!(invalid_syntax_passed, 0, "Some invalid syntax tests passed");
    assert_eq!(invalid_semantic_passed, 0, "Some invalid semantic tests passed");
}

fn test_agu_files_in_dir(dir: &Path, should_pass: bool) -> (i32, i32) {
    assert!(dir.exists(), "Test directory not found");
    let mut passed = 0;
    let mut failed = 0;
    for entry in fs::read_dir(dir).expect("Failed to read base test directory") {
        let path = entry.expect("Invalid entry").path();
        if path.is_dir() {
            match test_agu_file_in_dir(&path) {
                Ok(_) => {
                    passed += 1;
                    if !should_pass {
                        println!("❌ Test shouldn't have passed in {:?}", path);
                    }
                },
                Err(err) => {
                    if should_pass {
                        println!("{}", err);
                    }
                    failed += 1;
                }
            }
        }
    }
    (passed, failed)
}

fn test_agu_file_in_dir(dir: &Path) -> Result<(), String> {
    let agu_file = fs::read_dir(dir)
        .map_err(|e| format!("Failed to read dir {:?}: {}", dir, e))?
        .map(|entry| entry.expect("Invalid entry").path())
        .find(|p| p.extension().map_or(false, |ext| ext == "agu"));

    let agu_path = agu_file.ok_or_else(|| format!("No .agu file found in {:?}", dir))?;
    let src = fs::read_to_string(&agu_path)
        .map_err(|e| format!("Failed to read file {:?}: {}", agu_path, e))?;

    let result = compile_aguda_program(&src);
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format_errors(e, 1, true, &agu_path.to_string_lossy(), &src))
    }
}