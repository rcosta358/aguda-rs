use std::fmt;
use std::ops::Range;
use crate::utils::indent;

pub type Span = Range<usize>;
pub type Id = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub decls: Vec<Spanned<Decl>>,
}

#[derive(Debug, Clone)]
pub enum Decl {
    Var {
        id: Spanned<Id>,
        ty: Spanned<Type>,
        expr: Spanned<Expr>
    },
    Fun {
        id: Spanned<Id>,
        params: Vec<Spanned<Id>>,
        ty: FunType,
        expr: Spanned<Expr>
    }
}

#[derive(Debug, Clone)]
pub enum Expr {
    Chain {
        lhs: Box<Spanned<Expr>>,
        rhs: Box<Spanned<Expr>>
    },
    Let {
        id: Spanned<Id>,
        ty: Spanned<Type>,
        expr: Box<Spanned<Expr>>
    },
    Set {
        lhs: Spanned<Lhs>,
        expr: Box<Spanned<Expr>>
    },
    BinOp {
        lhs: Box<Spanned<Expr>>,
        op: Op,
        rhs: Box<Spanned<Expr>>
    },
    Not {
        expr: Box<Spanned<Expr>>
    },
    While {
        cond: Box<Spanned<Expr>>,
        expr: Box<Spanned<Expr>>
    },
    IfElse {
        cond: Box<Spanned<Expr>>,
        then: Box<Spanned<Expr>>,
        els: Box<Spanned<Expr>>
    },
    FunCall {
        id: Spanned<Id>,
        args: Vec<Spanned<Expr>>
    },
    NewArray {
        ty: Spanned<Type>,
        size: Box<Spanned<Expr>>,
        init: Box<Spanned<Expr>>,
    },
    ArrayIndex {
        lhs: Spanned<Lhs>,
        index: Box<Spanned<Expr>>
    },
    Id(Spanned<Id>),
    Number(i64),
    String(String),
    Bool(bool),
    Unit,
}

#[derive(Debug, Clone)]
pub enum Lhs {
    Var {
        id: Spanned<Id>
    },
    Index {
        lhs: Box<Spanned<Lhs>>,
        index: Box<Spanned<Expr>>
    },
}

#[derive(Debug, Clone)]
pub enum Op {
    Add, Sub, Mul, Div, Mod, Pow, And, Or, Eq, Neq, Lt, Leq, Gt, Geq, Concat
}

impl Op {
    pub fn to_text(&self) -> String {
        let text: &str = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
            Op::Mod => "%",
            Op::Pow => "^",
            Op::And => "&&",
            Op::Or => "||",
            Op::Eq => "==",
            Op::Neq => "!=",
            Op::Lt => "<",
            Op::Leq => "<=",
            Op::Gt => ">",
            Op::Geq => ">=",
            Op::Concat => "++"
        };
        text.to_string()
    }

    pub fn get_type(&self) -> OpType {
        match self {
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow => OpType::Numerical,
            Op::And | Op::Or => OpType::Logical,
            Op::Eq | Op::Neq | Op::Lt | Op::Leq | Op::Gt | Op::Geq => OpType::Comparison,
            Op::Concat => OpType::String,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpType {
    Numerical,
    Logical,
    Comparison,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Bool,
    String,
    Unit,
    Array(Box<Type>),
    Fun(FunType),
    Any,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunType {
    pub params: Vec<Type>,
    pub ret: Box<Type>,
}

impl<T: fmt::Display> fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Program {
    pub fn to_text(&self) -> String {
        self.decls
            .iter()
            .map(|decl| decl.value.to_text(0))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl Decl {
    pub fn to_text(&self, level: usize) -> String {
        match self {
            Decl::Fun { id, params, ty, expr } => {
                format!(
                    "let {} ({}) : ({}) -> {} =\n{}{}",
                    id,
                    params
                        .iter()
                        .map(|id| id.value.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    ty.params.iter()
                        .map(|ty| ty.to_text())
                        .collect::<Vec<_>>()
                        .join(", "),
                    ty.ret.to_text(),
                    indent(level + 1),
                    expr.value.to_text(level + 1)
                )
            }
            Decl::Var { id, ty, expr } => {
                format!(
                    "let {} : {} =\n{}{}",
                    id,
                    ty.value.to_text(),
                    indent(level + 1),
                    expr.value.to_text(level + 1)
                )
            }
        }
    }
}

impl Expr {
    pub fn to_text(&self, level: usize) -> String {
        match self {
            Expr::Chain { lhs, rhs } => {
                format!(
                    "{};\n{}{}",
                    lhs.value.to_text(level),
                    indent(level),
                    rhs.value.to_text(level)
                )
            }
            Expr::Let { id, ty, expr } => {
                format!(
                    "let {} : {} = {}",
                    id,
                    ty.value.to_text(),
                    expr.value.to_text(level)
                )
            }
            Expr::Set { lhs, expr } => {
                format!(
                    "set {} = {}",
                    lhs.value.to_text(),
                    expr.value.to_text(level)
                )
            }
            Expr::NewArray { ty, size, init } => {
                format!(
                    "new {}[{} | {}]",
                    ty.value.to_text(),
                    size.value.to_text(level),
                    init.value.to_text(level)
                )
            }
            Expr::IfElse { cond, then, els } => {
                format!(
                    "if {} then\n{}{}\n{}else\n{}{}",
                    cond.value.to_text(level),
                    indent(level + 1),
                    then.value.to_text(level + 1),
                    indent(level),
                    indent(level + 1),
                    els.value.to_text(level + 1)
                )
            }
            Expr::While { cond, expr } => {
                format!(
                    "while {} do\n{}{}",
                    cond.value.to_text(level),
                    indent(level + 1),
                    expr.value.to_text(level + 1)
                )
            }
            Expr::BinOp { lhs, op, rhs } => {
                format!(
                    "{} {} {}",
                    lhs.value.to_text(level),
                    op.to_text(),
                    rhs.value.to_text(level)
                )
            }
            Expr::Not { expr } => format!("!{}", expr.value.to_text(level)),
            Expr::Number(n) => n.to_string(),
            Expr::Bool(b) => format!("{}", b),
            Expr::Unit => "unit".to_string(),
            Expr::String(s) => format!("{}", s),
            Expr::Id(id) => id.to_string(),
            Expr::FunCall { id, args } => {
                format!(
                    "{}({})",
                    id,
                    args.iter()
                        .map(|arg| arg.value.to_text(level))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },
            Expr::ArrayIndex { lhs, index } => format!("{}[{}]", lhs.value.to_text(), index.value.to_text(level)),
        }
    }
}

impl Type {
    pub fn to_text(&self) -> String {
        match self {
            Type::Int => "Int".to_string(),
            Type::Unit => "Unit".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::String => "String".to_string(),
            Type::Array(inner) => format!("{}[]", inner.to_text()),
            _ => panic!()
        }
    }
}

impl Lhs {
    pub fn to_text(&self) -> String {
        match self {
            Lhs::Index { lhs, index } => format!("{}[{}]", lhs.value.to_text(), index.value.to_text(0)),
            Lhs::Var { id } => id.to_string(),
        }
    }
}
