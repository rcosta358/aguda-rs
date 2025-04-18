use crate::semantic::SemanticError;
use crate::syntax::ast::{Program, Decl, Expr, Lhs};
use crate::semantic::symbol_table::{SymbolTable, VarInfo};
use crate::semantic::symbol_table::VarInfo::{FunType, VarType};

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

        // check all global declarations
        for decl in &self.prog.decls {
            match &decl.value {
                Decl::Var { id, ty, .. } => {
                    let var_ty = VarType(ty.value.clone());
                    if let Err(_) = &self.table.declare(id.value.clone(), var_ty) {
                        &self.errors.push(SemanticError::DuplicateDeclaration(id.clone()));
                    }
                }
                Decl::Fun { id, param_types, ret_type, .. } => {
                    let fun_type = FunType {
                        param_types: param_types.iter().map(|p| p.value.clone()).collect(),
                        ret_type: ret_type.value.clone(),
                    };
                    if let Err(_) = &self.table.declare(id.value.clone(), fun_type) {
                        &self.errors.push(SemanticError::DuplicateDeclaration(id.clone()));
                    }
                }
            }
        }

        // check all expressions
        for decl in &self.prog.decls {
            match &decl.value {
                Decl::Var { expr, .. } => {
                    &self.check_expr(&expr.value);
                }
                Decl::Fun { param_ids, param_types, expr, .. } => {
                    &self.table.enter_scope();
                    for (pid, pty) in param_ids.iter().zip(param_types.iter()) {
                        let var_ty = VarType(pty.value.clone());
                        if let Err(_) = &self.table.declare(pid.value.clone(), var_ty) {
                            &self.errors.push(SemanticError::DuplicateDeclaration(pid.clone()));
                        }
                    }
                    &self.check_expr(&expr.value);
                    &self.table.exit_scope();
                }
            }
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Id(id) => {
                if self.table.lookup(&id.value).is_none() {
                    &self.errors.push(SemanticError::UndeclaredVariable(id.clone()));
                }
            }
            Expr::Let { id, ty, expr } => {
                let var_ty = VarType(ty.value.clone());
                if let Err(_) = &self.table.declare(id.value.clone(), var_ty) {
                    &self.errors.push(SemanticError::DuplicateDeclaration(id.clone()));
                }
                &self.check_expr(&expr.value);
            }
            Expr::Set { lhs, expr } => {
                &self.check_lhs(&lhs.value);
                &self.check_expr(&expr.value);
            }
            Expr::FunCall { id, args } => {
                if self.table.lookup(&id.value).is_none() {
                    &self.errors.push(SemanticError::UndeclaredVariable(id.clone()));
                }
                for arg in args {
                    &self.check_expr(&arg.value);
                }
            }
            Expr::ArrayIndex { lhs, index } => {
                &self.check_lhs(&lhs.value);
                &self.check_expr(&index.value);
            }
            Expr::Chain { lhs, rhs } => {
                &self.check_expr(&lhs.value);
                &self.check_expr(&rhs.value);
            }
            Expr::BinOp { lhs, rhs, .. } => {
                &self.check_expr(&lhs.value);
                &self.check_expr(&rhs.value);
            }
            Expr::Not { expr } => {
                self.check_expr(&expr.value)
            },
            Expr::While { cond, expr } => {
                &self.check_expr(&cond.value);
                &self.check_expr(&expr.value);
            }
            Expr::NewArray { size, init, .. } => {
                &self.check_expr(&size.value);
                &self.check_expr(&init.value);
            }
            Expr::IfElse { cond, then, els } => {
                &self.check_expr(&cond.value);
                &self.check_expr(&then.value);
                &self.check_expr(&els.value);
            }
            Expr::Num(_) | Expr::Bool(_) | Expr::Str(_) | Expr::Unit => {}
        }
    }

    fn check_lhs(&mut self, lhs: &Lhs) {
        match lhs {
            Lhs::Var { id } => {
                if self.table.lookup(&id.value).is_none() {
                    &self.errors.push(SemanticError::UndeclaredVariable(id.clone()));
                }
            }
            Lhs::Index { lhs, index } => {
                &self.check_lhs(&lhs.value);
                &self.check_expr(&index.value);
            }
        }
    }
}