use std::process::Command;

fn main() {
    lalrpop::process_src().expect("failed to process LALRPOP grammar file");

    // compile the library to then link with generated code
    Command::new("clang")
        .arg("-S")
        .arg("-emit-llvm")
        .arg("lib.c")
        .arg("-o")
        .arg("lib.ll")
        .output()
        .expect("failed to compile C library");
}