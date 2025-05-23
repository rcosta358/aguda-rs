use std::collections::HashMap;
use lazy_static::lazy_static;
use crate::syntax::ast::{FunType, Span, Type};

pub mod symbol_table;
pub mod declaration_checker;
pub mod type_checker;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ty: Type,
    pub span: Span,
}

pub fn get_init_symbols() -> HashMap<String, Symbol> {
    INIT_SYMBOLS
        .iter()
        .cloned()
        .map(|(id, ty)| { (id.to_string(), Symbol { ty, span: Span::default() }) })
        .collect::<HashMap<_, _>>()
}

lazy_static! {
    pub static ref INIT_SYMBOLS: [(&'static str, Type); 2] = [
        (
            // print: Any -> Unit
            "print",
            Type::Fun(
                FunType {
                    params: vec![Type::Any],
                    ret: Box::new(Type::Unit)
                }
            )
        ),
        (
            // length: Any[] -> Int
            "length",
            Type::Fun(
                FunType {
                    params: vec![Type::Array(Box::new(Type::Any))],
                    ret: Box::new(Type::Int)
                }
            )
        ),
    ];
    pub static ref RESERVED_IDENTIFIERS: Vec<String> =
        INIT_SYMBOLS.iter().map(|s| s.0.to_string()).collect::<Vec<_>>();
}
