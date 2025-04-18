use crate::syntax::ast::Spanned;
use crate::utils::format_error;

pub mod symbol_table;
pub mod variable_checker;
pub mod type_checker;

#[derive(Debug, Clone)]
pub enum SemanticError {
    UndeclaredVariable(Spanned<String>),
    DuplicateDeclaration(Spanned<String>),
}

impl SemanticError {
    pub fn get_error(&self, src: &str) -> String {
        match self {
            SemanticError::UndeclaredVariable(var) =>
                format_error(src, var.span.clone(), "undeclared variable", None),
            SemanticError::DuplicateDeclaration(var) =>
                format_error(src, var.span.clone(), "duplicate declaration in the same scope", None),
        }
    }
}

