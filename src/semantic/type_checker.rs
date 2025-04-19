use std::collections::HashMap;
use crate::semantic::TypeError;
use crate::semantic::symbol_table::SymbolTable;
use crate::syntax::ast::*;

pub type Ctx = HashMap<String, Type>;

#[derive(Debug)]
pub struct TypeChecker {
    table: SymbolTable,
    errors: Vec<TypeError>,
}

impl TypeChecker {

    pub fn new(table: SymbolTable) -> Self {
        TypeChecker {
            table,
            errors: Vec::new()
        }
    }

    pub fn check(&mut self, prog: &Program) -> Result<(), Vec<TypeError>> {
        for decl in &prog.decls {
            match &decl.value {
                Decl::Var { ty, expr, ..} => {
                    self.check_against(expr, &ty.value);
                }
                Decl::Fun { param_ids, param_types, ret_type, expr, .. } => {
                    self.table.enter_scope();
                    for (param_id, param_ty) in param_ids.iter().zip(param_types.iter()) {
                        self.table.declare(param_id.value.clone(), param_ty.value.clone()).unwrap();
                    }
                    self.check_against(expr, &ret_type.value);
                    self.table.exit_scope()
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
            Expr::Id(id) => self.table.lookup(&id.value).unwrap(),
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
                self.table.enter_scope();
                self.table.declare(id.value.clone(), ty.value.clone()).unwrap();
                // keep scope open, which will be closed in the chain
                // this is needed so the variable is in scope for rest of the expression

                Type::Unit
            }
            Expr::Set { lhs, expr } => {
                let lhs_type = self.type_of_lhs(lhs);
                self.check_against(expr, &lhs_type);
                Type::Unit
            }
            Expr::FunCall { id, args } => {
                let fun_type = self.table.lookup(&id.value).unwrap();
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
                let ty = self.type_of(rhs);

                // if lhs was a let, we need to exit the scope
                if matches!(lhs.value, Expr::Let { .. }) {
                    self.table.exit_scope();
                }
                ty
            }
        }
    }

    fn type_of_lhs(&mut self, lhs: &Spanned<Lhs>) -> Type {
        match &lhs.value {
            Lhs::Var { id } => {
                self.table.lookup(&id.value).expect("undefined identifier")
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
        let found = self.type_of(expr);
        match expected {
            Type::Any => {},
            Type::Array(inner) if **inner == Type::Any => {},
            _ => {
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