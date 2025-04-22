use colored::Colorize;
use crate::semantic::type_checker::TypeChecker;
use crate::semantic::declaration_checker::DeclarationChecker;
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;
use crate::utils::format_checker_errors;

pub mod syntax;
pub mod semantic;
pub mod utils;
pub mod cli;

pub fn compile_aguda_program(
    src: String,
    max_errors: usize,
    print_ast: bool
) -> Result<String, String> {
    let tokens = Lexer::new(&src)
        .tokenize()
        .map_err(|e| format!("{} {}", "Lexical Error:".red().bold(), e))?;

    let ast = Parser::new(&src, tokens)
        .parse()
        .map_err(|e| format!("{} {}", "Syntax Error:".red().bold(), e))?;

    let symbol_table = DeclarationChecker::new()
        .check(&ast)
        .map_err(|e| format_checker_errors(e, &src, "Declaration Error:", max_errors))?;

    TypeChecker::new(symbol_table)
        .check(&ast)
        .map_err(|e| format_checker_errors(e, &src, "Type Error:", max_errors))?;

    let output = if print_ast { ast.to_text() } else { "".to_string() };
    Ok(output)
}
