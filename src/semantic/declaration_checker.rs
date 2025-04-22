use crate::semantic::{DeclarationError, RESERVED_IDENTIFIERS};
use crate::syntax::ast::{Program, Decl, Expr, Lhs, Type};
use crate::semantic::symbol_table::SymbolTable;

pub struct DeclarationChecker {
    symbols: SymbolTable,
    errors: Vec<DeclarationError>,
}

impl DeclarationChecker {
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(),
            errors: Vec::new(),
        }
    }

    pub fn check(&mut self, prog: &Program) -> Result<SymbolTable, Vec<DeclarationError>> {
        // declare functions first to allow mutually recursive function calls
        for decl in &prog.decls {
            self.declare_fun_decl(&decl.value.clone());
        }
        // check each declaration
        for decl in &prog.decls {
            self.check_decl(&decl.value.clone());
        }
        if self.errors.is_empty() {
            Ok(self.symbols.clone())
        } else {
            Err(self.errors.clone())
        }
    }

    fn declare_fun_decl(&mut self, decl: &Decl) {
        if let Decl::Fun { id, params, ty, .. } = decl {
            if RESERVED_IDENTIFIERS.contains(&id.value) {
                self.errors.push(DeclarationError::ReservedIdentifier(id.clone()));
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
            for param_id in params {
                if RESERVED_IDENTIFIERS.contains(&param_id.value) {
                    self.errors.push(DeclarationError::ReservedIdentifier(param_id.clone()));
                }
            }
            if self.symbols.declare(id.value.clone(), Type::Fun(ty.clone())).is_err() {
                self.errors.push(DeclarationError::DuplicateDeclaration(id.clone()));
            }
        }
    }

    fn check_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Var { expr, id, ty } => {
                if RESERVED_IDENTIFIERS.contains(&id.value) {
                    self.errors.push(DeclarationError::ReservedIdentifier(id.clone()));
                }
                // variable scope
                self.symbols.enter_scope();
                self.check_expr(&expr.value);
                self.symbols.exit_scope();

                // only declare after inner scope so it's not visible inside the let scope
                if self.symbols.declare(id.value.clone(), ty.value.clone()).is_err() {
                    self.errors.push(DeclarationError::DuplicateDeclaration(id.clone()));
                }
            }
            Decl::Fun { params, ty, expr, .. } => {
                // function scope
                self.symbols.enter_scope();
                for (param_id, param_ty) in params.iter().zip(ty.params.iter()) {
                    if self.symbols.declare(param_id.value.clone(), param_ty.clone()).is_err() {
                        self.errors.push(DeclarationError::DuplicateDeclaration(param_id.clone()));
                    }
                }
                self.check_expr(&expr.value);
                self.symbols.exit_scope();
            }
        }
    }

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Chain { lhs, rhs } => {
                self.check_expr(&lhs.value);
                self.check_expr(&rhs.value);
            }
            Expr::Let { id, ty, expr } => {
                if RESERVED_IDENTIFIERS.contains(&id.value) {
                    self.errors.push(DeclarationError::ReservedIdentifier(id.clone()));
                }
                // let scope
                self.symbols.enter_scope();
                self.check_expr(&expr.value);
                self.symbols.exit_scope();

                // only declare after inner scope so it's not visible inside the let scope
                if self.symbols.declare(id.value.clone(), ty.value.clone()).is_err() {
                    self.errors.push(DeclarationError::DuplicateDeclaration(id.clone()));
                }
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
                if self.symbols.lookup(&id.value).is_none() {
                    self.errors.push(DeclarationError::UndeclaredIdentifier(id.clone()));
                }
                for arg in args {
                    self.check_expr(&arg.value);
                }
            }
            Expr::IfElse { cond, then, els } => {
                self.check_expr(&cond.value);

                // then scope
                self.symbols.enter_scope();
                self.check_expr(&then.value);
                self.symbols.exit_scope();

                // else scope
                self.symbols.enter_scope();
                self.check_expr(&els.value);
                self.symbols.exit_scope();
            }
            Expr::While { cond, expr } => {
                self.check_expr(&cond.value);

                // while body scope
                self.symbols.enter_scope();
                self.check_expr(&expr.value);
                self.symbols.exit_scope();
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
                if self.symbols.lookup(&id.value).is_none() {
                    self.errors.push(DeclarationError::UndeclaredIdentifier(id.clone()));
                }
            }
            Expr::Int(_) | Expr::Bool(_) | Expr::String(_) | Expr::Unit => {}
        }
    }

    fn check_lhs(&mut self, lhs: &Lhs) {
        match lhs {
            Lhs::Var { id } => {
                if self.symbols.lookup(&id.value).is_none() {
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
