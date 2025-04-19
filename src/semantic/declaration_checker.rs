use crate::scope;
use crate::semantic::DeclarationError;
use crate::syntax::ast::{Program, Decl, Expr, Lhs, Type};
use crate::semantic::symbol_table::{SymbolTable, RESERVED_IDENTIFIERS};

pub struct DeclarationChecker {
    table: SymbolTable,
    errors: Vec<DeclarationError>,
}

impl DeclarationChecker {
    pub fn new() -> Self {
        Self {
            table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn check(&mut self, prog: &Program) -> Result<SymbolTable, Vec<DeclarationError>> {
        for decl in &prog.decls {
            self.check_globals(&decl.value.clone());
        }
        for decl in &prog.decls {
            self.check_decl(&decl.value.clone());
        }
        if self.errors.is_empty() {
            Ok(self.table.clone())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_globals(&mut self, decl: &Decl) {
        match decl {
            Decl::Var { id, ty, .. } => {
                if RESERVED_IDENTIFIERS.contains(&id.value) {
                    self.errors.push(DeclarationError::ReservedIdentifier(id.clone()));
                    return
                }
                if self.table.declare(id.value.clone(), ty.value.clone()).is_err() {
                    self.errors.push(DeclarationError::DuplicateDeclaration(id.clone()));
                }
            }
            Decl::Fun { id, param_ids, param_types, ret_type, .. } => {
                if RESERVED_IDENTIFIERS.contains(&id.value) {
                    self.errors.push(DeclarationError::ReservedIdentifier(id.clone()));
                    return
                }
                for param_id in param_ids {
                    if RESERVED_IDENTIFIERS.contains(&param_id.value) {
                        self.errors.push(DeclarationError::ReservedIdentifier(param_id.clone()));
                        return
                    }
                }
                if param_ids.len() != param_types.len() {
                    self.errors.push(
                        DeclarationError::WrongFunctionSignature {
                            span: id.span.clone(),
                            params_found: param_ids.len(),
                            types_found: param_types.len(),
                        }
                    );
                }
                let ty = Type::Fun(
                    param_types.iter().map(|p| p.value.clone()).collect(),
                    Box::new(ret_type.value.clone())
                );
                if self.table.declare(id.value.clone(), ty).is_err() {
                    self.errors.push(DeclarationError::DuplicateDeclaration(id.clone()));
                }
            }
        }
    }

    fn check_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Var { expr, .. } => {
                scope!(self.table, { self.check_expr(&expr.value) });
            }
            Decl::Fun { param_ids, param_types, expr, .. } => {
                scope!(self.table, {
                    for (pid, pty) in param_ids.iter().zip(param_types.iter()) {
                        if self.table.declare(pid.value.clone(), pty.value.clone()).is_err() {
                            self.errors.push(DeclarationError::DuplicateDeclaration(pid.clone()));
                        }
                    }
                    self.check_expr(&expr.value);
                });
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Id(id) => {
                if self.table.lookup(&id.value).is_none() {
                    self.errors.push(DeclarationError::UndeclaredIdentifier(id.clone()));
                }
            }
            Expr::Let { id, ty, expr } => {
                if RESERVED_IDENTIFIERS.contains(&id.value) {
                    self.errors.push(DeclarationError::ReservedIdentifier(id.clone()));
                    return
                }
                if self.table.declare(id.value.clone(), ty.value.clone()).is_err() {
                    self.errors.push(DeclarationError::DuplicateDeclaration(id.clone()));
                }
                scope!(self.table, { self.check_expr(&expr.value) });
            }
            Expr::Set { lhs, expr } => {
                self.check_lhs(&lhs.value);
                self.check_expr(&expr.value);
            }
            Expr::FunCall { id, args } => {
                if self.table.lookup(&id.value).is_none() {
                    self.errors.push(DeclarationError::UndeclaredIdentifier(id.clone()));
                }
                for arg in args {
                    self.check_expr(&arg.value);
                }
            }
            Expr::ArrayIndex { lhs, index } => {
                self.check_lhs(&lhs.value);
                self.check_expr(&index.value);
            }
            Expr::Chain { lhs, rhs } => {
                self.check_expr(&lhs.value);
                self.check_expr(&rhs.value);
            }
            Expr::BinOp { lhs, rhs, .. } => {
                self.check_expr(&lhs.value);
                self.check_expr(&rhs.value);
            }
            Expr::Not { expr } => {
                self.check_expr(&expr.value)
            },
            Expr::While { cond, expr } => {
                scope!(self.table, { self.check_expr(&cond.value) });
                scope!(self.table, { self.check_expr(&expr.value) });
            }
            Expr::NewArray { size, init, .. } => {
                self.check_expr(&size.value);
                self.check_expr(&init.value);
            }
            Expr::IfElse { cond, then, els } => {
                scope!(self.table, { self.check_expr(&cond.value) });
                scope!(self.table, { self.check_expr(&then.value) });
                scope!(self.table, { self.check_expr(&els.value) });
            }
            Expr::Num(_) | Expr::Bool(_) | Expr::Str(_) | Expr::Unit => {}
        }
    }

    fn check_lhs(&mut self, lhs: &Lhs) {
        match lhs {
            Lhs::Var { id } => {
                if self.table.lookup(&id.value).is_none() {
                    self.errors.push(DeclarationError::UndeclaredIdentifier(id.clone()));
                }
            }
            Lhs::Index { lhs, index } => {
                self.check_lhs(&lhs.value);
                self.check_expr(&index.value);
            }
        }
    }
}