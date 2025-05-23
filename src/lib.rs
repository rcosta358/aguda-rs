use std::process::{Command, Output};
use inkwell::context::Context;
use crate::codegen::codegen::CodeGen;
use crate::semantic::type_checker::TypeChecker;
use crate::semantic::declaration_checker::DeclarationChecker;
use crate::syntax::ast::Program;
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
use crate::diagnostics::errors::{CompileError, RuntimeError, SemanticError};
use crate::diagnostics::warnings::Warning;
use std::path::Path;

pub mod syntax;
pub mod semantic;
pub mod codegen;
pub mod utils;
pub mod cli;
pub mod diagnostics;

pub fn compile_aguda_program(
    src: &str,
    file: &str,
    opt: u32,
) -> Result<(Program, Vec<Warning>), Vec<CompileError>> {

    // lexing
    let tokens = Lexer::new(src)
        .tokenize()
        .map_err(|e| vec![CompileError::from(e)])?;

    // parsing
    let ast = Parser::new(tokens)
        .parse()
        .map_err(|e| vec![CompileError::from(e)])?;

    // declaration and type checking
    let (decl_errors, warnings) = DeclarationChecker::new().check(&ast);
    let _ = TypeChecker::new()
        .check(&ast)
        .map_err(|type_errors|
            SemanticError::from_both(decl_errors.clone(), type_errors.clone())
                .into_iter()
                .map(|e| CompileError::from(e))
                .collect::<Vec<_>>()
        )?;

    if !decl_errors.is_empty() {
        return Err(decl_errors.iter().map(|e| CompileError::from(e.to_owned())).collect::<Vec<_>>());
    }

    // code generation
    let context = Context::create();
    let mut codegen = CodeGen::new(file, &context);
    let ll_path = Path::new(file);
    codegen.gen_program(&ast);
    codegen.gen_ll(Box::from(ll_path));

    // llvm optimization
    if opt > 0 {
        let ll_path = ll_path.with_extension("ll");
        Command::new("opt")
            .arg(format!("-O{}", opt))
            .arg(&ll_path)
            .arg("-o")
            .arg(&ll_path)
            .output()
            .expect("failed to optimize LLVM IR");
    }

    Ok((ast, warnings))
}

pub fn run_aguda_program(path: &str) -> Result<Output, RuntimeError> {
    // run the program with lli
    let ll_path = Path::new(path).with_extension("ll");
    let output = Command::new("lli")
        .arg(&ll_path)
        .output()
        .expect("failed to execute command");

    if output.status.success() {
        Ok(output)
    } else {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(RuntimeError { message })
    }
}