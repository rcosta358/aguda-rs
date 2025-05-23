use crate::diagnostics::errors::TypeError;
use crate::semantic::{get_init_symbols, Symbol};
use crate::semantic::symbol_table::SymbolTable;
use crate::syntax::ast::*;

#[derive(Debug)]
pub struct TypeChecker {
    symbols: SymbolTable<Symbol>,
    errors: Vec<TypeError>,
}

impl TypeChecker {

    pub fn new() -> Self {
        TypeChecker {
            symbols: SymbolTable::new(get_init_symbols()),
            errors: Vec::new()
        }
    }

    pub fn check(&mut self, prog: &Program) -> Result<(), Vec<TypeError>> {
        for decl in &prog.decls {
            match &decl.value {
                Decl::Var { id, ty, expr } => {
                    // variable scope
                    self.symbols.enter_scope();
                    self.check_against(expr, &ty.value);
                    self.symbols.exit_scope();
                    self.declare(&id, &ty.value);
                }
                Decl::Fun { id, params, ty, expr } => {
                    // function scope
                    self.symbols.enter_scope();
                    for (param_id, param_ty) in params.iter().zip(ty.value.params.iter()) {
                        self.declare(&param_id, &param_ty);
                    }
                    self.check_against(expr, &ty.value.ret);
                    self.symbols.exit_scope();
                    self.declare(&id, &Type::Fun(ty.value.clone()));

                    // check main function signature
                    if id.value == "main" && (
                        ty.value.params.len() != 1
                        || *ty.value.params.first().unwrap() != Type::Unit
                        || *ty.value.ret != Type::Unit
                    ) {
                        self.errors.push(TypeError::main_signature_mismatch(ty.span.clone()));
                    }
                }
            }
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn type_of(&mut self, expr: &Spanned<Expr>) -> Type {
        let span = expr.span.clone();
        match &expr.value {
            Expr::Chain { lhs, rhs } => {
                self.type_of(lhs);
                if let Expr::Let { id, ty, .. } = &lhs.value {
                    // declare the let binding in the scope
                    self.declare(&id, &ty.value);
                }
                self.type_of(rhs)
            }
            Expr::Let { ty, expr, .. } => {
                // let scope
                self.symbols.enter_scope();
                self.check_against(expr, &ty.value);
                self.symbols.exit_scope();
                Type::Unit
            }
            Expr::Set { lhs, expr } => {
                let lhs_type = self.type_of_lhs(lhs);
                self.check_against(expr, &lhs_type);
                Type::Unit
            }
            Expr::BinOp { lhs, op, rhs } => {
                match op {
                    Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Mod | Op::Pow => {
                        self.check_against(lhs, &Type::Int);
                        self.check_against(rhs, &Type::Int);
                        Type::Int
                    }
                    Op::And | Op::Or => {
                        self.check_against(lhs, &Type::Bool);
                        self.check_against(rhs, &Type::Bool);
                        Type::Bool
                    }
                    Op::Eq | Op::Neq | Op::Lt | Op::Leq | Op::Gt | Op::Geq => {
                        let left_type = self.type_of(lhs);
                        self.check_against(rhs, &left_type);
                        Type::Bool
                    }
                }
            }
            Expr::Not { expr } => {
                self.check_against(expr, &Type::Bool);
                Type::Bool
            }
            Expr::FunCall { id, args } => {
                let Some(fun) = self.symbols.lookup(&id.value) else {
                    return Type::Any; // undeclared symbol, error already reported
                };
                if let Type::Fun(ty) = fun.ty {
                    if ty.params.len() != args.len() {
                        self.errors.push(
                            TypeError::arg_count_mismatch(span.clone(), args.len(), ty.params.len())
                        )
                    }
                    for (arg, arg_type) in args.iter().zip(ty.params.iter()) {
                        self.check_against(arg, arg_type);
                    }
                    *ty.ret.clone()
                } else {
                    self.errors.push(TypeError::not_callable(span.clone(), fun.ty));
                    Type::Any // avoid error propagation
                }
            }
            Expr::IfElse { cond, then, els } => {
                self.check_against(cond, &Type::Bool);
                let then_type = self.type_of(then);
                self.check_against(els, &then_type);
                then_type
            }
            Expr::While { cond, expr } => {
                self.check_against(cond, &Type::Bool);
                self.type_of(expr);
                Type::Unit
            }
            Expr::NewArray { ty, size, init } => {
                self.check_against(size, &Type::Int);
                self.check_against(init, &ty.value);
                Type::Array(Box::new(ty.value.clone()))
            }
            Expr::ArrayIndex { lhs, index } => {
                let arr_type = self.type_of_lhs(lhs);
                self.check_against(index, &Type::Int);
                if let Type::Array(elem_ty) = arr_type {
                    *elem_ty.clone()
                } else {
                    self.errors.push(TypeError::not_indexable(span.clone(), arr_type));
                    Type::Any // avoid error propagation
                }
            }
            Expr::Id(id) => self.lookup(&id).unwrap_or(Type::Any),
            Expr::Int(_) => Type::Int,
            Expr::String(_) => Type::String,
            Expr::Bool(_) => Type::Bool,
            Expr::Unit => Type::Unit,
        }
    }

    fn type_of_lhs(&mut self, lhs: &Spanned<Lhs>) -> Type {
        match &lhs.value {
            Lhs::Var { id } => self.lookup(&id).unwrap_or(Type::Any),
            Lhs::Index { lhs, index } => {
                let arr_type = self.type_of_lhs(lhs);
                self.check_against(index, &Type::Int);
                if let Type::Array(elem) = arr_type {
                    *elem.clone()
                } else {
                    self.errors.push(TypeError::not_indexable(lhs.span.clone(), arr_type));
                    Type::Any // avoid error propagation
                }
            }
        }
    }

    fn check_against(&mut self, expr: &Spanned<Expr>, expected: &Type) {
        match expected {
            // any type matches any type
            Type::Any => return,
            Type::Array(expected_inner) if **expected_inner == Type::Any => {
                let found = self.type_of(expr);
                match found {
                    // array of any matches any array of any type
                    Type::Array(_) => return,
                    _ => {
                        // not an array
                        self.errors.push(
                            TypeError::type_mismatch(expr.span.clone(), found, expected.clone())
                        );
                    }
                }
            }
            _ => match &expr.value {
                Expr::IfElse { cond, then, els } => {
                    self.check_against(cond, &Type::Bool);
                    self.check_against(then, &expected);
                    self.check_against(els, &expected);
                }
                Expr::BinOp { lhs, op, rhs } if matches!(op, Op::Eq) || matches!(op, Op::Neq) => {
                    let lhs_ty = self.type_of(lhs);
                    self.check_against(rhs, &lhs_ty);
                }
                _ => {
                    let found = self.type_of(expr);
                    if found == Type::Any {
                        return; // avoid error propagation
                    }
                    if &found != expected {
                        self.errors.push(
                            TypeError::type_mismatch(expr.span.clone(), found, expected.clone())
                        );
                    }
                }
            }
        }
    }

    fn declare(&mut self, id: &Spanned<Id>, ty: &Type) {
        let symbol = Symbol {
            ty: ty.clone(),
            span: id.span.clone(),
        };
        self.symbols.declare(&id.value, &symbol);
    }

    fn lookup(&mut self, id: &Spanned<Id>) -> Option<Type> {
        self.symbols.lookup(&id.value).map(|s| s.ty.clone())
    }
}
