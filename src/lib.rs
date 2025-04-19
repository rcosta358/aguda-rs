use colored::Colorize;
use crate::semantic::type_checker::TypeChecker;
use crate::semantic::declaration_checker::DeclarationChecker;
use crate::syntax::lexer::Lexer;
use crate::syntax::parser::Parser;

pub mod syntax;
pub mod semantic;
pub mod utils;

pub fn compile_aguda_program(src: String) -> Result<String, String> {
    let tokens = Lexer::new(&src)
        .tokenize()
        .map_err(|e| format!("{} {}", "Lexical Error:".red().bold(), e))?;

    let ast = Parser::new(&src, tokens)
        .parse()
        .map_err(|e| format!("{} {}", "Syntax Error:".red().bold(), e))?;

    let result = DeclarationChecker::new().check(&ast);
    let symbol_table = match result {
        Ok(table) => table,
        Err(e) => {
            let errors = e
                .into_iter()
                .map(|e| format!("{} {}", "Declaration Error:".red().bold(), e.get_message(&src)))
                .collect::<Vec<_>>()
                .join("\n");
            return Err(errors);
        }
    };

    if let Err(e) = TypeChecker::new(symbol_table).check(&ast) {
        let errors = e
            .into_iter()
            .map(|e| format!("{} {}", "Type Error:".red().bold(), e.get_message(&src)))
            .collect::<Vec<_>>()
            .join("\n");
        return Err(errors);
    }

    Ok(ast.to_text())
}

