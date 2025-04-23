use crate::semantic::type_checker::TypeChecker;
use crate::semantic::declaration_checker::DeclarationChecker;
use crate::syntax::ast::Program;
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
use crate::errors::CompileError;

pub mod syntax;
pub mod semantic;
pub mod utils;
pub mod cli;
pub mod errors;

pub fn compile_aguda_program(
    src: &str,
) -> Result<Program, Vec<CompileError>> {
    let tokens = Lexer::new(src)
        .tokenize()
        .map_err(|e| vec![CompileError::from(e)])?;

    let ast = Parser::new(tokens)
        .parse()
        .map_err(|e| vec![CompileError::from(e)])?;

    let symbol_table = DeclarationChecker::new()
        .check(&ast)
        .map_err(|errs| errs.into_iter().map(CompileError::from).collect::<Vec<_>>())?;

    TypeChecker::new(symbol_table)
        .check(&ast)
        .map_err(|errs| errs.into_iter().map(CompileError::from).collect::<Vec<_>>())?;

    Ok(ast)
}