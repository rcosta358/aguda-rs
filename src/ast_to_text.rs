use crate::rustlr_ast::*;

impl<'lt> Program<'lt> {
    pub fn to_text(&self) -> String {
        self.0
            .iter()
            .map(|decl| decl.value().to_text(0))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl<'lt> Decl<'lt> {
    pub fn to_text(&self, level: usize) -> String {
        match self {
            Decl::Fun { id, params, ty, expr, .. } => {
                format!(
                    "let {} ({}) : {} =\n{}{}",
                    id,
                    params.to_text(),
                    ty.to_text(),
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
            Decl::Decl_Nothing => "".to_string(),
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
                let else_str = match els.as_ref() {
                    Else::Else(_, expr) =>
                        format!(
                            "\n{}else\n{}{}",
                            indent(level + 1),
                            indent(level + 2),
                            expr.to_text(level + 2)
                        ),
                    Else::Unit => format!("\n{}", indent(level)),
                    _ => "".to_string(),
                };
                format!(
                    "if {} then\n{}{}{}",
                    cond.to_text(level),
                    indent(level + 2),
                    then.to_text(level + 2),
                    else_str
                )
            }
            Expr::While { cond, expr, .. } => {
                format!(
                    "while {} do\n{}{}",
                    cond.to_text(level),
                    indent(level + 1),
                    expr.to_text(level + 1)
                )
            }
            Expr::Chain(head, rest) => {
                let mut s = head.to_text(level);
                for expr in rest {
                    s.push_str(&format!(
                        ";\n{}{}",
                        indent(level),
                        expr.value().to_text(level)
                    ));
                }
                s
            }
            // binary operations
            Expr::Add(l, r) => format!("{} + {}", l.to_text(level), r.to_text(level)),
            Expr::Sub(l, r) => format!("{} - {}", l.to_text(level), r.to_text(level)),
            Expr::Mul(l, r) => format!("{} * {}", l.to_text(level), r.to_text(level)),
            Expr::Div(l, r) => format!("{} / {}", l.to_text(level), r.to_text(level)),
            Expr::Mod(l, r) => format!("{} % {}", l.to_text(level), r.to_text(level)),
            Expr::Pow(l, r) => format!("{} ** {}", l.to_text(level), r.to_text(level)),
            Expr::And(l, r) => format!("{} && {}", l.to_text(level), r.to_text(level)),
            Expr::Or(l, r) => format!("{} || {}", l.to_text(level), r.to_text(level)),
            Expr::Lt(l, r) => format!("{} < {}", l.to_text(level), r.to_text(level)),
            Expr::Gt(l, r) => format!("{} > {}", l.to_text(level), r.to_text(level)),
            Expr::Eq(l, r) => format!("{} == {}", l.to_text(level), r.to_text(level)),
            Expr::Leq(l, r) => format!("{} <= {}", l.to_text(level), r.to_text(level)),
            Expr::Geq(l, r) => format!("{} >= {}", l.to_text(level), r.to_text(level)),
            Expr::Neq(l, r) => format!("{} != {}", l.to_text(level), r.to_text(level)),

            // unary operations
            Expr::Not(e) => format!("!{}", e.to_text(level)),
            Expr::Neg(e) => format!("-{}", e.to_text(level)),

            // literals
            Expr::Num(n) => n.to_string(),
            Expr::True(_) => "true".to_string(),
            Expr::False(_) => "false".to_string(),
            Expr::Unit(_) => "unit".to_string(),
            Expr::String(s) => format!("{}", s),
            Expr::Id(id) => id.to_string(),
            Expr::Paren(expr) => format!("{}", expr.to_text(level)),
            Expr::FunCall(id, args) => format!("{}({})", id, args.to_text(level)),
            Expr::Index(lhs, expr) => format!("{}[{}]", lhs.to_text(), expr.to_text(level)),
            _ => "".to_string(),
        }
    }
}

impl<'lt> ExprList<'lt> {
    pub fn to_text(&self, level: usize) -> String {
        let mut s = self.0.to_text(level);
        for expr in &self.1 {
            s.push_str(&format!(", {}", expr.value().to_text(level)));
        }
        s
    }
}

impl<'lt> Type<'lt> {
    pub fn to_text(&self) -> String {
        match self {
            Type::Int(_) => "Int".to_string(),
            Type::Unit(_) => "Unit".to_string(),
            Type::Bool(_) => "Bool".to_string(),
            Type::String(_) => "String".to_string(),
            Type::Array(inner) => format!("{}[]", inner.to_text()),
            _ => "".to_string(),
        }
    }
}

impl<'lt> FunType<'lt> {
    pub fn to_text(&self) -> String {
        match self {
            FunType::SingleParam { ty, ret } => format!("{} -> {}", ty.to_text(), ret.to_text()),
            FunType::MultiParam { ty, ret } => format!("({}) -> {}", ty.to_text(), ret.to_text()),
            _ => "".to_string(),
        }
    }
}

impl<'lt> TypeList<'lt> {
    pub fn to_text(&self) -> String {
        match self {
            TypeList::TypeList(head, rest) => {
                let mut s = head.to_text();
                for t in rest {
                    s.push_str(&format!(", {}", t.value().to_text()));
                }
                s
            }
            _ => "".to_string(),
        }
    }
}

impl<'lt> ParamList<'lt> {
    pub fn to_text(&self) -> String {
        let mut params = vec![self.0];
        for p in &self.1 {
            params.push(p.value());
        }
        params.join(", ")
    }
}

impl<'lt> Lhs<'lt> {
    pub fn to_text(&self) -> String {
        match self {
            Lhs::Index(lhs, expr) => format!("{}[{}]", lhs.to_text(), expr.to_text(0)),
            Lhs::Id(id) => id.to_string(),
            _ => "".to_string(),
        }
    }
}

fn indent(level: usize) -> String {
    "  ".repeat(level)
}
