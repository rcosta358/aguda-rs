use lazy_static::lazy_static;
use crate::syntax::ast::{FunType, Id, Span, Spanned, Type};
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

pub trait SemanticError {
    fn get_message(&self, src: &str) -> String;
}

impl SemanticError for DeclarationError {
    fn get_message(&self, src: &str) -> String {
        match self {
            DeclarationError::UndeclaredIdentifier(var) => {
                let msg = if var.value == "_" {
                    "wildcard identifier cannot be used"
                } else {
                    &format!("undeclared identifier '{}'", var.value)
                };
                format_error(
                    src,
                    var.span.clone(),
                    msg,
                    None
                )
            },
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
                    &format!("reserved identifier '{}' cannot be used", var.value),
                    None
                ),
            DeclarationError::WrongFunctionSignature { span, params_found, types_found } =>
                format_error(
                    src,
                    span.clone(),
                    &format!(
                        "wrong function signature, found {} parameter(s) and {} type(s)",
                        params_found,
                        types_found
                    ),
                    None
                ),
        }
    }
}

impl SemanticError for TypeError {
    fn get_message(&self, src: &str) -> String {
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

lazy_static! {
    pub static ref INIT_SYMBOLS: [(String, Type); 2] = [
        (
            // print: Any -> Unit
            "print".to_string(),
            Type::Fun(
                FunType {
                    params: vec![Type::Any],
                    ret: Box::new(Type::Unit)
                }
            )
        ),
        (
            // length: Any[] -> Int
            "length".to_string(),
            Type::Fun(
                FunType {
                    params: vec![Type::Array(Box::new(Type::Any))],
                    ret: Box::new(Type::Int)
                }
            )
        ),
    ];
    pub static ref RESERVED_IDENTIFIERS: Vec<String> =
        INIT_SYMBOLS.iter().map(|s| s.0.clone()).collect::<Vec<_>>();
}
