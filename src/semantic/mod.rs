use lazy_static::lazy_static;
use crate::syntax::ast::{FunType, Type};

pub mod symbol_table;
pub mod declaration_checker;
pub mod type_checker;

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
