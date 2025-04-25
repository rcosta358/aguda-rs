use crate::semantic::type_checker::TypeChecker;
use crate::semantic::declaration_checker::DeclarationChecker;
use crate::syntax::ast::Program;
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
use crate::errors::{CompileError, SemanticError, Warning};

pub mod syntax;
pub mod semantic;
pub mod utils;
pub mod cli;
pub mod errors;

pub fn compile_aguda_program(
    src: &str,
) -> Result<(Program, Vec<Warning>), Vec<CompileError>> {

    let tokens = Lexer::new(src)
        .tokenize()
        .map_err(|e| vec![CompileError::from(e)])?;

    let ast = Parser::new(tokens)
        .parse()
        .map_err(|e| vec![CompileError::from(e)])?;

    let (symbol_table, decl_errors, warnings) = DeclarationChecker::new().check(&ast);
    let _ = TypeChecker::new(symbol_table)
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

    Ok((ast, warnings))
}