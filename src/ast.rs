use std::fmt;
pub use rustlr::LC;
use rustlr::LBox;
use crate::rustlr_ast;
use crate::rustlr_ast::*;

#[derive(Debug)]
pub struct Program<'lt>(pub Vec<Decl<'lt>>);

#[derive(Debug)]
pub enum Decl<'lt> {
    Var {
        id: &'lt str,
        ty: Type,
        expr: Expr<'lt>,
    },
    Fun {
        id: &'lt str,
        params: Vec<Param<'lt>>,
        ret_ty: Type,
        expr: Expr<'lt>,
    },
}

#[derive(Debug)]
pub enum Expr<'lt> {
    Binary {
        left: LBox<Expr<'lt>>,
        op: BinOp,
        right: LBox<Expr<'lt>>,
    },
    Set {
        lhs: LBox<Lhs<'lt>>,
        expr: LBox<Expr<'lt>>,
    },
    If {
        cond: LBox<Expr<'lt>>,
        then: LBox<Expr<'lt>>,
        els: LBox<Expr<'lt>>,
    },
    Let {
        id: &'lt str,
        ty: Type,
        expr: LBox<Expr<'lt>>,
    },
    New {
        ty: Type,
        size: LBox<Expr<'lt>>,
        init: LBox<Expr<'lt>>,
    },
    While {
        cond: LBox<Expr<'lt>>,
        body: LBox<Expr<'lt>>,
    },
    Not(LBox<Expr<'lt>>),
    FunCall {
        name: &'lt str,
        args: Vec<LBox<Expr<'lt>>>,
    },
    Paren(LBox<Expr<'lt>>),
    Id(&'lt str),
    Index {
        lhs: LBox<Lhs<'lt>>,
        index: LBox<Expr<'lt>>,
    },
    Num(i64),
    Unit,
    String(&'lt str),
    True,
    False,
    Null,
    Chain(Vec<LBox<Expr<'lt>>>),
}

#[derive(Debug)]
pub enum Lhs<'lt> {
    Index(LBox<Lhs<'lt>>, LBox<Expr<'lt>>),
    Id(&'lt str),
}

#[derive(Debug)]
pub struct Param<'lt> {
    pub id: &'lt str,
    pub ty: Type,
}

#[derive(Debug)]
pub enum Type {
    String,
    Unit,
    Int,
    Bool,
    Array(LBox<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::String => write!(f, "String"),
            Type::Unit => write!(f, "Unit"),
            Type::Int => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
            Type::Array(ty) => write!(f, "{}[]", ty.exp),
        }
    }
}

#[derive(Debug)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod, Pow, Eq, Neq, Lt, Leq, Gt, Geq, And, Or,
}

impl<'lt> Decl<'lt> {
    pub fn convert(src: &rustlr_ast::Decl<'lt>) -> Decl<'lt> {
        match src {
            rustlr_ast::Decl::Var { id, ty, expr, .. } => Decl::Var {
                id,
                ty: Type::convert(&ty),
                expr: Expr::convert(&expr),
            },
            rustlr_ast::Decl::Fun { id, params, ty, expr, .. } => {
                let (param_tys, ret_ty) = FunType::convert(ty);
                let param_ids = ParamList::convert(params);
                let params = param_ids.into_iter()
                    .zip(param_tys)
                    .map(|(id, ty)| Param { id, ty })
                    .collect();
                Decl::Fun {
                    id,
                    params,
                    ret_ty,
                    expr: Expr::convert(&expr),
                }
            }
            _ => panic!(),
        }
    }
}

impl <'lt> Type {
    pub fn convert(src: &rustlr_ast::Type<'lt>) -> Type {
        match src {
            rustlr_ast::Type::String(_) => Type::String,
            rustlr_ast::Type::Unit(_) => Type::Unit,
            rustlr_ast::Type::Int(_) => Type::Int,
            rustlr_ast::Type::Bool(_) => Type::Bool,
            rustlr_ast::Type::Array(ty) => Type::Array(LBox::new(Type::convert(&ty), ty.line(), ty.column())),
            _ => panic!(),
        }
    }
}

impl FunType<'_> {
    pub fn convert(src: &FunType) -> (Vec<Type>, Type) {
        match src {
            FunType::SingleParam { ty, ret } => (
                vec![Type::convert(&ty)],
                Type::convert(&ret),
            ),
            FunType::MultiParam { ty, ret } => (
                TypeList::convert(ty),
                Type::convert(&ret),
            ),
            _ => panic!(),
        }
    }
}

impl BinOp {
    fn from_expr(expr: &rustlr_ast::Expr) -> BinOp {
        match expr {
            rustlr_ast::Expr::Add(_, _) => BinOp::Add,
            rustlr_ast::Expr::Sub(_, _) => BinOp::Sub,
            rustlr_ast::Expr::Mul(_, _) => BinOp::Mul,
            rustlr_ast::Expr::Div(_, _) => BinOp::Div,
            rustlr_ast::Expr::Mod(_, _) => BinOp::Mod,
            rustlr_ast::Expr::Pow(_, _) => BinOp::Pow,
            rustlr_ast::Expr::Eq(_, _) => BinOp::Eq,
            rustlr_ast::Expr::Neq(_, _) => BinOp::Neq,
            rustlr_ast::Expr::Lt(_, _) => BinOp::Lt,
            rustlr_ast::Expr::Leq(_, _) => BinOp::Leq,
            rustlr_ast::Expr::Gt(_, _) => BinOp::Gt,
            rustlr_ast::Expr::Geq(_, _) => BinOp::Geq,
            rustlr_ast::Expr::And(_, _) => BinOp::And,
            rustlr_ast::Expr::Or(_, _) => BinOp::Or,
            _ => panic!(),
        }
    }
}

impl<'lt> Expr<'lt> {
    pub fn convert(src: &rustlr_ast::Expr<'lt>) -> Expr<'lt> {
        match src {
            rustlr_ast::Expr::Add(l, r) |
            rustlr_ast::Expr::Sub(l, r) |
            rustlr_ast::Expr::Mul(l, r) |
            rustlr_ast::Expr::Div(l, r) |
            rustlr_ast::Expr::Mod(l, r) |
            rustlr_ast::Expr::Pow(l, r) |
            rustlr_ast::Expr::Eq(l, r) |
            rustlr_ast::Expr::Neq(l, r) |
            rustlr_ast::Expr::Lt(l, r) |
            rustlr_ast::Expr::Leq(l, r) |
            rustlr_ast::Expr::Gt(l, r) |
            rustlr_ast::Expr::Geq(l, r) |
            rustlr_ast::Expr::And(l, r) |
            rustlr_ast::Expr::Or(l, r) => {
                let op = BinOp::from_expr(&src);
                Expr::Binary {
                    left: LBox::new(Expr::convert(l), l.line(), l.column()),
                    op,
                    right: LBox::new(Expr::convert(r), r.line(), r.column()),
                }
            }
            rustlr_ast::Expr::Neg(e) => Expr::Binary {
                left: LBox::new(Expr::Num(0), e.line(), e.column()),
                op: BinOp::Sub,
                right: LBox::new(Expr::convert(e), e.line(), e.column()),
            },
            rustlr_ast::Expr::Not(e) => Expr::Not(LBox::new(Expr::convert(e), e.line(), e.column())),
            rustlr_ast::Expr::If { cond, then, els, .. } => Expr::If {
                cond: LBox::new(Expr::convert(cond), cond.line(), cond.column()),
                then: LBox::new(Expr::convert(then), then.line(), then.column()),
                els: LBox::new(Else::convert(els), els.line(), els.column()),
            },
            rustlr_ast::Expr::Id(id) => Expr::Id(id),
            rustlr_ast::Expr::Num(n) => Expr::Num(*n),
            rustlr_ast::Expr::True(_) => Expr::True,
            rustlr_ast::Expr::False(_) => Expr::False,
            rustlr_ast::Expr::Unit(_) => Expr::Unit,
            rustlr_ast::Expr::String(s) => Expr::String(s),
            rustlr_ast::Expr::Null(_) => Expr::Null,
            rustlr_ast::Expr::Chain(head, tail) => {
                let mut exprs = vec![LBox::new(Expr::convert(head), head.line(), head.column())];
                exprs.extend(tail.into_iter().map(|lc| LBox::new(Expr::convert(&lc.0), lc.line(), lc.column())));
                Expr::Chain(exprs)
            }
            rustlr_ast::Expr::FunCall(name, args) => Expr::FunCall {
                name,
                args: ExprList::convert(&args.exp),
            },
            rustlr_ast::Expr::Set { lhs, expr, .. } => Expr::Set {
                lhs: LBox::new(Lhs::convert(lhs), lhs.line(), lhs.column()),
                expr: LBox::new(Expr::convert(expr), expr.line(), expr.column()),
            },
            rustlr_ast::Expr::Let { id, ty, expr, .. } => Expr::Let {
                id,
                ty: Type::convert(ty),
                expr: LBox::new(Expr::convert(expr), expr.line(), expr.column()),
            },
            rustlr_ast::Expr::New { ty, size, init, .. } => Expr::New {
                ty: Type::convert(ty),
                size: LBox::new(Expr::convert(size), size.line(), size.column()),
                init: LBox::new(Expr::convert(init), init.line(), init.column()),
            },
            rustlr_ast::Expr::While { cond, expr, .. } => Expr::While {
                cond: LBox::new(Expr::convert(cond), cond.line(), cond.column()),
                body: LBox::new(Expr::convert(expr), expr.line(), expr.column()),
            },
            rustlr_ast::Expr::Paren(expr) => Expr::Paren(LBox::new(Expr::convert(expr), expr.line(), expr.column())),
            rustlr_ast::Expr::Index(lhs, idx) => Expr::Index {
                lhs: LBox::new(Lhs::convert(lhs), lhs.line(), lhs.column()),
                index: LBox::new(Expr::convert(idx), idx.line(), idx.column()),
            },
            _ => panic!(),
        }
    }
}

impl<'lt> Lhs<'lt> {
    pub fn convert(src: &rustlr_ast::Lhs<'lt>) -> Lhs<'lt> {
        match src {
            rustlr_ast::Lhs::Index(lhs, idx) => Lhs::Index(
                LBox::new(Lhs::convert(lhs), lhs.line(), lhs.column()),
                LBox::new(Expr::convert(idx), idx.line(), idx.column()),
            ),
            rustlr_ast::Lhs::Id(id) => Lhs::Id(id),
            _ => panic!(),
        }
    }
}

impl<'lt> Else<'lt> {
    pub fn convert(src: &Else<'lt>) -> Expr<'lt> {
        match src {
            Else::Else(_, expr) => Expr::convert(&expr),
            _ => Expr::Unit,
        }
    }
}

impl<'lt> ParamList<'lt> {
    pub fn convert(src: &ParamList<'lt>) -> Vec<&'lt str> {
        let mut params = vec![src.0];
        params.extend(src.1.iter().map(|lc| lc.0));
        params
    }
}

impl<'lt> ExprList<'lt> {
    pub fn convert(src: &ExprList<'lt>) -> Vec<LBox<Expr<'lt>>> {
        let mut exprs = vec![LBox::new(Expr::convert(&src.0), src.0.line(), src.0.column())];
        exprs.extend(src.1.iter().map(|lc| LBox::new(Expr::convert(&lc.0), lc.line(), lc.column())));
        exprs
    }
}

impl TypeList<'_> {
    pub fn convert(src: &TypeList) -> Vec<Type> {
        match src {
            TypeList::TypeList(head, tail) => {
                let mut types = vec![Type::convert(&head)];
                types.extend(tail.into_iter().map(|lc| Type::convert(&lc.0)));
                types
            }
            _ => vec![],
        }
    }
}

impl<'lt> Program<'lt> {
    pub fn convert(src: rustlr_ast::Program<'lt>) -> Program<'lt> {
        Program(
            src.0.into_iter()
                .filter_map(|lc| Option::from(Decl::convert(&lc.0)))
                .collect()
        )
    }
}

impl<'lt> Program<'lt> {
    pub fn to_text(&self) -> String {
        self.0
            .iter()
            .map(|decl| decl.to_text(0))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl<'lt> Decl<'lt> {
    pub fn to_text(&self, level: usize) -> String {
        match self {
            Decl::Fun { id, params, ret_ty, expr, .. } => {
                format!(
                    "let {} {} -> {} =\n{}{}",
                    id,
                    format_params(params),
                    ret_ty.to_text(),
                    indent(level + 1),
                    expr.to_text(level + 1)
                )
            }
            Decl::Var { id, ty, expr, .. } => {
                format!(
                    "let {} : {} =\n{}{}",
                    id,
                    ty.to_text(),
                    indent(level + 1),
                    expr.to_text(level + 1)
                )
            }
        }
    }
}

impl<'lt> Expr<'lt> {
    pub fn to_text(&self, level: usize) -> String {
        match self {
            Expr::Let { id, ty, expr, .. } => {
                format!(
                    "let {} : {} = {}",
                    id,
                    ty.to_text(),
                    expr.to_text(level)
                )
            }
            Expr::Set { lhs, expr, .. } => {
                format!(
                    "set {} = {}",
                    lhs.to_text(),
                    expr.to_text(level)
                )
            }
            Expr::New { ty, size, init, .. } => {
                format!(
                    "new {}[{} | {}]",
                    ty.to_text(),
                    size.to_text(level),
                    init.to_text(level)
                )
            }
            Expr::If { cond, then, els, .. } => {
                format!(
                    "if {} then\n{}{}\n{}else\n{}{}",
                    cond.to_text(level),
                    indent(level + 1),
                    then.to_text(level + 1),
                    indent(level),
                    indent(level + 1),
                    els.to_text(level + 1)
                )
            }
            Expr::While { cond, body, .. } => {
                format!(
                    "while {} do\n{}{}",
                    cond.to_text(level),
                    indent(level + 1),
                    body.to_text(level + 1)
                )
            }
            Expr::Chain(chain) => {
                let mut s = chain[0].to_text(level);
                for expr in chain.iter().skip(1) {
                    s.push_str(&format!(
                        ";\n{}{}",
                        indent(level),
                        expr.to_text(level)
                    ));
                }
                s
            }
            Expr::Binary { left:l, op, right:r } => match op {
                BinOp::Add => format!("{} + {}", l.to_text(level), r.to_text(level)),
                BinOp::Sub => format!("{} - {}", l.to_text(level), r.to_text(level)),
                BinOp::Mul => format!("{} * {}", l.to_text(level), r.to_text(level)),
                BinOp::Div => format!("{} / {}", l.to_text(level), r.to_text(level)),
                BinOp::Mod => format!("{} % {}", l.to_text(level), r.to_text(level)),
                BinOp::Pow => format!("{} ** {}", l.to_text(level), r.to_text(level)),
                BinOp::And => format!("{} && {}", l.to_text(level), r.to_text(level)),
                BinOp::Or => format!("{} || {}", l.to_text(level), r.to_text(level)),
                BinOp::Lt => format!("{} < {}", l.to_text(level), r.to_text(level)),
                BinOp::Gt => format!("{} > {}", l.to_text(level), r.to_text(level)),
                BinOp::Eq => format!("{} == {}", l.to_text(level), r.to_text(level)),
                BinOp::Leq => format!("{} <= {}", l.to_text(level), r.to_text(level)),
                BinOp::Geq => format!("{} >= {}", l.to_text(level), r.to_text(level)),
                BinOp::Neq => format!("{} != {}", l.to_text(level), r.to_text(level)),
            }
            Expr::Not(e) => format!("!{}", e.to_text(level)),
            Expr::Num(n) => n.to_string(),
            Expr::True => "true".to_string(),
            Expr::False => "false".to_string(),
            Expr::Unit => "unit".to_string(),
            Expr::Null => "null".to_string(),
            Expr::String(s) => format!("{}", s),
            Expr::Id(id) => id.to_string(),
            Expr::Paren(expr) => format!("{}", expr.to_text(level)),
            Expr::FunCall { name, args } => format!("{}({})", name, format_vec(&args.iter().map(|x| x.to_text(level)).collect::<Vec<_>>())),
            Expr::Index { lhs, index } => format!("{}[{}]", lhs.to_text(), index.to_text(level)),
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

impl<'lt> Lhs<'lt> {
    pub fn to_text(&self) -> String {
        match self {
            Lhs::Index(lhs, expr) => format!("{}[{}]", lhs.to_text(), expr.to_text(0)),
            Lhs::Id(id) => id.to_string(),
        }
    }
}

fn format_vec<T: fmt::Display>(vec: &Vec<T>) -> String {
    vec.iter()
        .map(|item| format!("{}", item))
        .collect::<Vec<String>>()
        .join(", ")
}

fn format_params(params: &Vec<Param>) -> String {
    if params.len() == 1 {
        format!("{:?}", params[0])
    } else {
        let ids: Vec<String> = params.iter().map(|p| p.id.to_string()).collect();
        let tys: Vec<String> = params.iter().map(|p| format!("{}", p.ty)).collect();
        format!("({}) : ({})", ids.join(", "), tys.join(", "))
    }
}

fn indent(level: usize) -> String {
    "  ".repeat(level)
}
