use lazy_static::lazy_static;
use crate::syntax::ast::{Id, Span, Spanned, Type};
use crate::utils::format_error;

pub mod symbol_table;
pub mod declaration_checker;
pub mod type_checker;

#[derive(Debug, Clone)]
pub enum DeclarationError {
    UndeclaredIdentifier(Spanned<Id>),
    DuplicateDeclaration(Spanned<Id>),
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
                    &format!("mismatched types, found {}, expected {}", found.to_text(), expected.to_text()),
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

lazy_static! {
    static ref INIT_SYMBOLS: [(String, Type); 2] = [
        (
            "print".to_string(),
            Type::Fun(vec![Type::Any], Box::new(Type::Unit))
        ),
        (
            "length".to_string(),
            Type::Fun(vec![Type::Array(Box::new(Type::Any))], Box::new(Type::Int))
        ),
    ];
}
