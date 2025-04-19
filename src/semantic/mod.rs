use crate::syntax::ast::{Id, Span, Spanned, Type};
use crate::utils::format_error;

pub mod symbol_table;
pub mod declaration_checker;
pub mod type_checker;

#[derive(Debug, Clone)]
pub enum DeclarationError {
    UndeclaredIdentifier(Spanned<Id>),
    DuplicateDeclaration(Spanned<Id>),
    ReservedIdentifier(Spanned<Id>),
    WrongFunctionSignature {
        span: Span,
        params_found: usize,
        types_found: usize,
    },
}

#[derive(Debug, Clone)]
pub enum TypeError {
    TypeMismatch {
        span: Span,
        found: Type,
        expected: Type,
    },
    WrongNumberOfArguments {
        span: Span,
        found: usize,
        expected: usize,
    },
    NotCallable {
        span: Span,
        found: Type
    },
    NotIndexable {
        span: Span,
        found: Type
    },
}

impl DeclarationError {
    pub fn get_message(&self, src: &str) -> String {
        match self {
            DeclarationError::UndeclaredIdentifier(var) =>
                format_error(
                    src,
                    var.span.clone(),
                    &format!("undeclared identifier '{}'", var.value),
                    None
                ),
            DeclarationError::DuplicateDeclaration(var) =>
                format_error(
                    src,
                    var.span.clone(),
                    &format!("duplicate identifier '{}' in the same scope", var.value),
                    None
                ),
            DeclarationError::ReservedIdentifier(var) =>
                format_error(
                    src,
                    var.span.clone(),
                    &format!("reserved identifier '{}'", var.value),
                    None
                ),
            DeclarationError::WrongFunctionSignature { span, params_found, types_found } =>
                format_error(
                    src,
                    span.clone(),
                    &format!("wrong function signature, found {} parameter(s) and {} type(s)", params_found, types_found),
                    None
                ),
        }
    }
}

impl TypeError {
    pub fn get_message(&self, src: &str) -> String {
        match self {
            TypeError::TypeMismatch { span, found, expected } => {
                format_error(
                    src,
                    span.clone(),
                    &format!("type mismatch, found {}, expected {}", found.to_text(), expected.to_text()),
                    None
                )
            }
            TypeError::WrongNumberOfArguments { span, found, expected } => {
                format_error(
                    src,
                    span.clone(),
                    &format!("wrong number of arguments, found {}, expected {}", found, expected),
                    None
                )
            }
            TypeError::NotCallable { span, found } => {
                format_error(
                    src,
                    span.clone(),
                    &format!("expression not callable, found {}, expected function", found.to_text()),
                    None
                )
            }
            TypeError::NotIndexable { span, found } => {
                format_error(
                    src,
                    span.clone(),
                    &format!("expression not indexable, found {}, expected array", found.to_text()),
                    None
                )
            }
        }
    }
}
