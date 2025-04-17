use std::fmt;
use std::ops::Range;
use crate::utils::indent;

#[derive(Debug)]
pub struct Spanned<T> {
    pub value: T,
    pub span: Range<usize>,
}

#[derive(Debug)]
pub struct Program {
    pub decls: Vec<Spanned<Decl>>,
}

#[derive(Debug)]
pub enum Decl {
    Var {
        id: Spanned<String>,
        ty: Spanned<Type>,
        expr: Spanned<Expr>
    },
    Fun{
        id: Spanned<String>,
        param_ids: Vec<Spanned<String>>,
        param_types: Vec<Spanned<Type>>,
        ret_ty: Spanned<Type>,
        expr: Spanned<Expr>
    }
}

#[derive(Debug)]
pub enum Expr {
    Chain {
        lhs: Box<Spanned<Expr>>,
        rhs: Box<Spanned<Expr>>
    },
    Let {
        id: Spanned<String>,
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
        id: Spanned<String>,
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
    Id(Spanned<String>),
    Num(i64),
    Str(String),
    Bool(bool),
    Unit,
}

#[derive(Debug)]
pub enum Lhs {
    Var {
        id: Spanned<String>
    },
    Index {
        lhs: Box<Spanned<Lhs>>,
        index: Box<Spanned<Expr>>
    },
}

#[derive(Debug)]
pub enum Op {
    Add, Sub, Mul, Div, Mod, Pow, And, Or, Eq, Neq, Lt, Leq, Gt, Geq,
}

#[derive(Debug)]
pub enum Type {
    Int,
    Bool,
    String,
    Unit,
    Array(Box<Type>),
}

#[derive(Debug)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod, Pow, Eq, Neq, Lt, Leq, Gt, Geq, And, Or,
}

impl fmt::Display for Spanned<String> {
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
            Decl::Fun { id, param_ids, param_types, ret_ty, expr } => {
                format!(
                    "let {} ({}) : ({}) -> {} =\n{}{}",
                    id,
                    param_ids
                        .iter()
                        .map(|id| id.value.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    param_types.iter()
                        .map(|ty| ty.value.to_text())
                        .collect::<Vec<_>>()
                        .join(", "),
                    ret_ty.value.to_text(),
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
            Expr::BinOp { lhs, op, rhs } => match op {
                Op::Add => format!("{} + {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Sub => format!("{} - {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Mul => format!("{} * {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Div => format!("{} / {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Mod => format!("{} % {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Pow => format!("{} ** {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::And => format!("{} && {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Or => format!("{} || {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Lt => format!("{} < {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Gt => format!("{} > {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Eq => format!("{} == {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Leq => format!("{} <= {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Geq => format!("{} >= {}", lhs.value.to_text(level), rhs.value.to_text(level)),
                Op::Neq => format!("{} != {}", lhs.value.to_text(level), rhs.value.to_text(level)),
            }
            Expr::Not { expr } => format!("!{}", expr.value.to_text(level)),
            Expr::Num(n) => n.to_string(),
            Expr::Bool(b) => format!("{}", b),
            Expr::Unit => "unit".to_string(),
            Expr::Str(s) => format!("{}", s),
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
