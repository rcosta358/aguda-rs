use std::collections::HashMap;
use crate::semantic::{TypeError, INIT_SYMBOLS};
use crate::syntax::ast::*;

pub type Ctx = HashMap<String, Type>;

#[derive(Debug)]
pub struct TypeChecker {
    pub ctx: Ctx,
    pub errors: Vec<TypeError>,
}

impl TypeChecker {

    pub fn new() -> Self {
        let symbols = INIT_SYMBOLS.iter().cloned().collect::<HashMap<_, _>>();
        TypeChecker {
            ctx: symbols,
            errors: Vec::new()
        }
    }

    pub fn check(&mut self, prog: &Program) -> Result<(), Vec<TypeError>> {
        // collect all function signatures
        for decl in &prog.decls {
            if let Decl::Fun { id, param_types, ret_type, .. } = &decl.value {
                let name = id.value.clone();
                let params = param_types.iter()
                    .map(|t| t.value.clone())
                    .collect::<Vec<_>>();
                let ret = ret_type.value.clone();
                self.ctx.insert(name, Type::Fun(params, Box::new(ret)));
            }
        }

        // type checking of the program
        for decl in &prog.decls {
            match &decl.value {
                Decl::Var { id, ty, expr } => {
                    // add variable to the context
                    self.ctx.insert(id.value.clone(), ty.value.clone());
                    // expression must match the declared type
                    self.check_against(expr, &ty.value);

                }

                Decl::Fun { param_ids, param_types, ret_type, expr, .. } => {
                    // bind parameters in the context
                    for (param_id, param_type) in param_ids.iter().zip(param_types.iter()) {
                        self.ctx.insert(param_id.value.clone(), param_type.value.clone());
                    }
                    // expression must match the declared return type
                    self.check_against(expr, &ret_type.value);
                    // remove parameters from the context
                    for pid in param_ids {
                        self.ctx.remove(&pid.value);
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

    pub fn type_of(&mut self, expr: &Spanned<Expr>) -> Type {
        let span = expr.span.clone();
        match &expr.value {
            Expr::Num(_) => Type::Int,
            Expr::Str(_) => Type::String,
            Expr::Bool(_) => Type::Bool,
            Expr::Unit => Type::Unit,
            Expr::Id(id) => {
                let name = &id.value;
                self.ctx.get(name).cloned().expect("undefined identifier")
            }
            Expr::BinOp { lhs, op, rhs } => {
                match op.get_type() {
                    OpType::Numerical => {
                        self.check_against(lhs, &Type::Int);
                        self.check_against(rhs, &Type::Int);
                        Type::Int
                    }
                    OpType::Logical => {
                        self.check_against(lhs, &Type::Bool);
                        self.check_against(rhs, &Type::Bool);
                        Type::Bool
                    }
                    OpType::Comparison => {
                        let left_type = self.type_of(lhs);
                        self.check_against(rhs, &left_type);
                        Type::Bool
                    }
                }
            }
            Expr::Not { expr: e } => {
                self.check_against(e, &Type::Bool);
                Type::Bool
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
            Expr::Let { id, ty, expr } => {
                self.check_against(expr, &ty.value);
                self.ctx.insert(id.value.clone(), ty.value.clone());
                Type::Unit
            }
            Expr::Set { lhs, expr } => {
                let lhs_type = self.type_of_lhs(lhs);
                self.check_against(expr, &lhs_type);
                Type::Unit
            }
            Expr::FunCall { id, args } => {
                let name = &id.value;
                let fun_type = self.ctx.get(name).cloned().unwrap();
                if let Type::Fun(param_types, ret_type) = fun_type {
                    if param_types.len() != args.len() {
                        self.errors.push(
                            TypeError::WrongNumberOfArguments {
                                span: span.clone(),
                                expected: param_types.len(),
                                found: args.len(),
                            }
                        )
                    }
                    for (arg, arg_type) in args.iter().zip(param_types.iter()) {
                        self.check_against(arg, arg_type);
                    }
                    *ret_type.clone()
                } else {
                    self.errors.push(
                        TypeError::NotCallable {
                            span: span.clone(),
                            found: fun_type,
                        }
                    );
                    Type::Unit
                }

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
                    self.errors.push(
                        TypeError::NotIndexable {
                            span: span.clone(),
                            found: arr_type,
                        }
                    );
                    Type::Unit
                }
            }
            Expr::Chain { lhs, rhs } => {
                self.type_of(lhs);
                self.type_of(rhs)
            }
        }
    }

    fn type_of_lhs(&mut self, lhs: &Spanned<Lhs>) -> Type {
        match &lhs.value {
            Lhs::Var { id } => {
                self.ctx.get(&id.value).cloned().expect("undefined identifier")
            }
            Lhs::Index { lhs, index } => {
                let arr_type = self.type_of_lhs(lhs);
                self.check_against(index, &Type::Int);
                if let Type::Array(elem) = arr_type {
                    *elem.clone()
                } else {
                    self.errors.push(
                        TypeError::NotIndexable {
                            span: lhs.span.clone(),
                            found: arr_type,
                        }
                    );
                    Type::Unit
                }
            }
        }
    }

    pub fn check_against(&mut self, expr: &Spanned<Expr>, expected: &Type) {
        match expected {
            Type::Any => {},
            Type::Array(inner) if **inner == Type::Any => {},
            _ => {
                let found = self.type_of(expr);
                if &found != expected {
                    self.errors.push(
                        TypeError::TypeMismatch {
                            span: expr.span.clone(),
                            expected: expected.clone(),
                            found,
                        }
                    );
                }
            }
        }
    }
}