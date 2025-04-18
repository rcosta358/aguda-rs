use crate::semantic::SemanticError;
use crate::syntax::ast::{Program, Decl, Expr, Lhs, Type};
use crate::semantic::symbol_table::{SymbolTable};

pub struct VariableChecker<'a> {
    prog: &'a Program,
    table: SymbolTable,
    errors: Vec<SemanticError>,
}

impl <'a> VariableChecker<'a> {
    pub fn new(prog: &'a Program) -> Self {
        Self {
            prog,
            table: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn check(&mut self) -> Result<(), Vec<SemanticError>> {
        for decl in &self.prog.decls {
            self.check_globals(&decl.value.clone());
        }

        for decl in &self.prog.decls {
            self.check_decl(&decl.value.clone());
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_globals(&mut self, decl: &Decl) {
        match decl {
            Decl::Var { id, ty, .. } => {
                if self.table.declare(id.value.clone(), ty.value.clone()).is_err() {
                    self.errors.push(SemanticError::DuplicateDeclaration(id.clone()));
                }
            }
            Decl::Fun { id, param_types, ret_type, .. } => {
                let ty = Type::Fun(
                    param_types.iter().map(|p| p.value.clone()).collect(),
                    Box::new(ret_type.value.clone())
                );
                if self.table.declare(id.value.clone(), ty).is_err() {
                    self.errors.push(SemanticError::DuplicateDeclaration(id.clone()));
                }
            }
        }
    }

    fn check_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Var { expr, .. } => {
                self.table.enter_scope();
                self.check_expr(&expr.value);
                self.table.exit_scope();
            }
            Decl::Fun { param_ids, param_types, expr, .. } => {
                self.table.enter_scope();
                for (pid, pty) in param_ids.iter().zip(param_types.iter()) {
                    if self.table.declare(pid.value.clone(), pty.value.clone()).is_err() {
                        self.errors.push(SemanticError::DuplicateDeclaration(pid.clone()));
                    }
                }
                self.check_expr(&expr.value);
                self.table.exit_scope();
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Id(id) => {
                if self.table.lookup(&id.value).is_none() {
                    self.errors.push(SemanticError::UndeclaredVariable(id.clone()));
                }
            }
            Expr::Let { id, ty, expr } => {
                if self.table.declare(id.value.clone(), ty.value.clone()).is_err() {
                    self.errors.push(SemanticError::DuplicateDeclaration(id.clone()));
                }
                self.table.enter_scope();
                self.check_expr(&expr.value);
                self.table.exit_scope();
            }
            Expr::Set { lhs, expr } => {
                self.check_lhs(&lhs.value);
                self.check_expr(&expr.value);
            }
            Expr::FunCall { id, args } => {
                if self.table.lookup(&id.value).is_none() {
                    self.errors.push(SemanticError::UndeclaredVariable(id.clone()));
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
                self.check_expr(&cond.value);
                self.check_expr(&expr.value);
            }
            Expr::NewArray { size, init, .. } => {
                self.check_expr(&size.value);
                self.check_expr(&init.value);
            }
            Expr::IfElse { cond, then, els } => {
                self.check_expr(&cond.value);
                self.check_expr(&then.value);
                self.check_expr(&els.value);
            }
            Expr::Num(_) | Expr::Bool(_) | Expr::Str(_) | Expr::Unit => {}
        }
    }

    fn check_lhs(&mut self, lhs: &Lhs) {
        match lhs {
            Lhs::Var { id } => {
                if self.table.lookup(&id.value).is_none() {
                    self.errors.push(SemanticError::UndeclaredVariable(id.clone()));
                }
            }
            Lhs::Index { lhs, index } => {
                self.check_lhs(&lhs.value);
                self.check_expr(&index.value);
            }
        }
    }
}