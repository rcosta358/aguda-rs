use crate::syntax::ast::{Id, Spanned};

#[derive(Debug, Clone)]
pub enum Warning {
    UnusedIdentifier(Spanned<Id>),
    RedefinedVariable(Spanned<Id>),
}
