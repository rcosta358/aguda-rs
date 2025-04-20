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
            Decl::Fun { id, params, ty, .. } => {
                if RESERVED_IDENTIFIERS.contains(&id.value) {
                    self.errors.push(DeclarationError::ReservedIdentifier(id.clone()));
                    return
                }
                for param_id in params {
                    if RESERVED_IDENTIFIERS.contains(&param_id.value) {
                        self.errors.push(DeclarationError::ReservedIdentifier(param_id.clone()));
                        return
                    }
                }
                if params.len() != ty.params.len() {
                    self.errors.push(
                        DeclarationError::WrongFunctionSignature {
                            span: id.span.clone(),
                            params_found: params.len(),
                            types_found: ty.params.len(),
                        }
                    );
                }
                if self.table.declare(id.value.clone(), Type::Fun(ty.clone())).is_err() {
                    self.errors.push(DeclarationError::DuplicateDeclaration(id.clone()));
                }
            }
        }
    }

    fn check_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Var { expr, .. } => {
                // var scope
                self.table.enter_scope();
                self.check_expr(&expr.value);
                self.table.exit_scope();
            }
            Decl::Fun { params, ty, expr, .. } => {
                // function scope
                self.table.enter_scope();
                for (pid, pty) in params.iter().zip(ty.params.iter()) {
                    if self.table.declare(pid.value.clone(), pty.clone()).is_err() {
                        self.errors.push(DeclarationError::DuplicateDeclaration(pid.clone()));
                    }
                }
                self.check_expr(&expr.value);
                self.table.exit_scope();
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Chain { lhs, rhs } => {
                self.check_expr(&lhs.value);
                self.check_expr(&rhs.value);

                // if lhs was a let, we need to exit the scope
                if matches!(lhs.value, Expr::Let { .. }) {
                    self.table.exit_scope();
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
                // let scope
                self.table.enter_scope();
                self.check_expr(&expr.value);
                // keep scope open, which will be closed in the chain later
                // this is needed so the variable is in scope for rest of the expression
            }
            Expr::Set { lhs, expr } => {
                self.check_lhs(&lhs.value);
                self.check_expr(&expr.value);
            }
            Expr::BinOp { lhs, rhs, .. } => {
                self.check_expr(&lhs.value);
                self.check_expr(&rhs.value);
            }
            Expr::Not { expr } => {
                self.check_expr(&expr.value)
            },
            Expr::FunCall { id, args } => {
                if self.table.lookup(&id.value).is_none() {
                    self.errors.push(DeclarationError::UndeclaredIdentifier(id.clone()));
                }
                for arg in args {
                    self.check_expr(&arg.value);
                }
            }
            Expr::IfElse { cond, then, els } => {
                self.check_expr(&cond.value);

                // then scope
                self.table.enter_scope();
                self.check_expr(&then.value);
                self.table.exit_scope();

                // else scope
                self.table.enter_scope();
                self.check_expr(&els.value);
                self.table.exit_scope();
            }
            Expr::While { cond, expr } => {
                self.check_expr(&cond.value);

                // while body scope
                self.table.enter_scope();
                self.check_expr(&expr.value);
                self.table.exit_scope();
            }
            Expr::NewArray { size, init, .. } => {
                self.check_expr(&size.value);
                self.check_expr(&init.value);
            }
            Expr::ArrayIndex { lhs, index } => {
                self.check_lhs(&lhs.value);
                self.check_expr(&index.value);
            }
            Expr::Id(id) => {
                if self.table.lookup(&id.value).is_none() {
                    self.errors.push(DeclarationError::UndeclaredIdentifier(id.clone()));
                }
            }
            Expr::Number(_) | Expr::Bool(_) | Expr::String(_) | Expr::Unit => {}
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