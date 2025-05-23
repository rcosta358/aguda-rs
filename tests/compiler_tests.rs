use std::fs;
use std::path::Path;
use aguda_rs::{compile_aguda_program, run_aguda_program};
use aguda_rs::diagnostics::errors::AgudaError;
use aguda_rs::diagnostics::formatting::format_aguda_errors;

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

fn test_agu_files_in_dir(dir: &Path, valid: bool) -> (i32, i32) {
    assert!(dir.exists(), "Test directory not found");
    let mut passed = 0;
    let mut failed = 0;
    for entry in fs::read_dir(dir).expect("failed to read base test directory") {
        let path = entry.expect("invalid entry").path();
        if path.is_dir() {
            match test_agu_file_in_dir(&path) {
                Ok(_) => {
                    passed += 1;
                    if !valid {
                        println!("❌ Test shouldn't have passed in {:?}", path);
                    }
                },
                Err(err) => {
                    if valid {
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
        .map_err(|e| format!("failed to read dir {:?}: {}", dir, e))?
        .map(|entry| entry.expect("invalid entry").path())
        .find(|p| p.extension().map_or(false, |ext| ext == "agu"));

    let agu_path = agu_file.ok_or_else(|| format!("no .agu file found in {:?}", dir))?;
    let src = fs::read_to_string(&agu_path)
        .map_err(|e| format!("failed to read file {:?}: {}", agu_path, e))?;

    let result = compile_aguda_program(&src, &agu_path.to_string_lossy(), 0);
    match result {
        Ok(_) => {
            let expected_file = agu_path.with_extension("expect");
            let expected = fs::read_to_string(&expected_file)
                .map_err(|e| format!("failed to read expected output file {:?}: {}", expected_file, e))?;

            let output = run_aguda_program(&agu_path.to_str().unwrap());
            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    if stdout.trim() == expected.trim() {
                        Ok(())
                    } else {
                        Err(format!("wrong output for: {:?}:\nexpected: {}\ngot: {}", agu_path, expected, stdout))
                    }
                },
                Err(e) => {
                    let errors = vec![AgudaError::from(e)];
                    Err(format_aguda_errors(errors, 1, true, &agu_path.to_string_lossy(), &src))
                }
            }
        },
        Err(e) => {
            let errors = e.iter().map(|e| AgudaError::from(e.clone())).collect::<Vec<_>>();
            Err(format_aguda_errors(errors, 1, true, &agu_path.to_string_lossy(), &src))
        }
    }
}