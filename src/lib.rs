use colored::Colorize;
use crate::semantic::type_checker::TypeChecker;
use crate::semantic::declaration_checker::DeclarationChecker;
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
use crate::utils::format_checker_errors;

pub mod syntax;
pub mod semantic;
pub mod utils;

const MAX_ERRORS: usize = 10;

pub fn compile_aguda_program(src: String) -> Result<String, String> {
    let tokens = Lexer::new(&src)
        .tokenize()
        .map_err(|e| format!("{} {}", "Lexical Error:".red().bold(), e))?;

    let ast = Parser::new(&src, tokens)
        .parse()
        .map_err(|e| format!("{} {}", "Syntax Error:".red().bold(), e))?;

    let symbol_table = DeclarationChecker::new()
        .check(&ast)
        .map_err(|e| format_checker_errors(e, &src, "Declaration Error:", MAX_ERRORS))?;

    TypeChecker::new(symbol_table)
        .check(&ast)
        .map_err(|e| format_checker_errors(e, &src, "Type Error:", MAX_ERRORS))?;

    Ok(ast.to_text())
}
