use crate::errors::{DeclarationError, Warning};
use crate::semantic::RESERVED_IDENTIFIERS;
use crate::syntax::ast::{Program, Decl, Expr, Lhs, Type, Id, Spanned};
use crate::semantic::symbol_table::SymbolTable;
use crate::utils::suggest_similar;

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

    pub fn check(&mut self, prog: &Program) -> (SymbolTable, Vec<DeclarationError>, Vec<Warning>) {
        // declare functions first to allow mutually recursive function calls
        for decl in &prog.decls {
            self.declare_fun_decl(&decl.value.clone());
        }

        // check each declaration
        for decl in &prog.decls {
            self.check_decl(&decl.value.clone());
        }

        // mark the last declaration as used (entry point)
        if let Some(last_decl) = prog.decls.last() {
            let id = match &last_decl.value {
                Decl::Fun { id, .. } => &id.value,
                Decl::Var { id, .. } => &id.value,
            };
            self.symbols.lookup(id); // lookup to mark as used
        }
        (self.symbols.clone(), self.errors.clone(), self.symbols.get_warnings())
    }

    fn declare_fun_decl(&mut self, decl: &Decl) {
        if let Decl::Fun { id, params, ty, .. } = decl {
            if RESERVED_IDENTIFIERS.contains(&id.value) {
                self.errors.push(DeclarationError::reserved_identifier(id.clone()));
            }
            if params.len() != ty.params.len() {
                self.errors.push(
                    DeclarationError::function_signature_mismatch(id.span.clone(), params.len(), ty.params.len())
                );
            }
            for param_id in params {
                if RESERVED_IDENTIFIERS.contains(&param_id.value) {
                    self.errors.push(DeclarationError::reserved_identifier(param_id.clone()));
                }
            }
            if !self.symbols.declare(id.clone(), Type::Fun(ty.clone())) {
                self.errors.push(DeclarationError::duplicate_declaration(id.clone()));
            }
        }
    }

    fn check_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Var { expr, id, ty } => {
                if RESERVED_IDENTIFIERS.contains(&id.value) {
                    self.errors.push(DeclarationError::reserved_identifier(id.clone()));
                }
                // variable scope
                self.symbols.enter_scope();
                self.check_expr(&expr.value);
                self.symbols.exit_scope();

                // only declare after exiting scope so it's not visible inside
                if !self.symbols.declare(id.clone(), ty.value.clone()) {
                    self.errors.push(DeclarationError::duplicate_declaration(id.clone()));
                }
            }
            Decl::Fun { params, ty, expr, .. } => {
                // function scope
                self.symbols.enter_scope();
                for (param_id, param_ty) in params.iter().zip(ty.params.iter()) {
                    self.symbols.declare(param_id.clone(), param_ty.clone());
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
                if let Expr::Let { id, ty, .. } = &lhs.value {
                    // declare the let binding in the scope
                    self.symbols.declare(id.clone(), ty.value.clone());
                }
                self.check_expr(&rhs.value);
            }
            Expr::Let { id, expr, .. } => {
                if RESERVED_IDENTIFIERS.contains(&id.value) {
                    self.errors.push(DeclarationError::reserved_identifier(id.clone()));
                }
                // let scope
                self.symbols.enter_scope();
                self.check_expr(&expr.value);
                self.symbols.exit_scope();
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
                self.check_id(id);
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
            Expr::Id(id) => self.check_id(id),
            Expr::Int(_) | Expr::Bool(_) | Expr::String(_) | Expr::Unit => {}
        }
    }

    fn check_lhs(&mut self, lhs: &Lhs) {
        match lhs {
            Lhs::Var { id } => self.check_id(id),
            Lhs::Index { lhs, index } => {
                self.check_lhs(&lhs.value);
                self.check_expr(&index.value);
            }
        }
    }

    fn check_id(&mut self, id: &Spanned<Id>) {
        if self.symbols.lookup(&id.value).is_none() {
            let all_symbols = self.symbols.get_visible_symbols();
            let similar = suggest_similar(all_symbols, &id.value);
            self.errors.push(DeclarationError::undeclared_identifier(id.clone(), similar));
        }
    }
}
