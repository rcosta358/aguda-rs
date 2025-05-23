use crate::diagnostics::errors::DeclarationError;
use crate::diagnostics::warnings::Warning;
use crate::semantic::{get_init_symbols, Symbol, RESERVED_IDENTIFIERS};
use crate::syntax::ast::{Program, Decl, Expr, Lhs, Type, Id, Spanned};
use crate::semantic::symbol_table::SymbolTable;
use crate::utils::get_similar;

pub struct DeclarationChecker {
    symbols: SymbolTable<Symbol>,
    unused_symbols: Vec<Spanned<Id>>,
    errors: Vec<DeclarationError>,
    main_declared: bool,
}

impl DeclarationChecker {
    pub fn new() -> Self {
        Self {
            symbols: SymbolTable::new(get_init_symbols()),
            unused_symbols: Vec::new(),
            errors: Vec::new(),
            main_declared: false,
        }
    }

    pub fn check(&mut self, prog: &Program) -> (Vec<DeclarationError>, Vec<Warning>) {
        // declare functions first to allow mutually recursive function calls
        for decl in &prog.decls {
            self.declare_fun(&decl.value);
        }
        // check each declaration
        for decl in &prog.decls {
            self.check_decl(&decl.value);
        }
        if self.main_declared {
            // mark main function as used
            self.lookup(&"main".to_string());
        } else {
            self.errors.push(DeclarationError::missing_main());
        }
        (self.errors.clone(), self.get_warnings())
    }

    fn declare_fun(&mut self, decl: &Decl) {
        if let Decl::Fun { id, params, ty, .. } = decl {
            if params.len() != ty.value.params.len() {
                self.errors.push(DeclarationError::function_signature_mismatch(id.span.clone(), params.len(), ty.value.params.len()));
            }
            self.declare(&id, &Type::Fun(ty.value.clone()));
            // check main declaration
            if id.value == "main" {
                if self.main_declared {
                    self.errors.push(DeclarationError::duplicate_main(id.span.clone()));
                } else {
                    self.main_declared = true;
                }
            }
        }
    }

    fn check_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Var { expr, id, ty } => {
                // variable scope
                self.symbols.enter_scope();
                self.check_expr(&expr.value);
                self.symbols.exit_scope();

                // only declare after exiting scope so it's not visible inside
                self.declare(&id, &ty.value);
            }
            Decl::Fun { params, ty, expr, .. } => {
                // function scope
                self.symbols.enter_scope();
                for (param_id, param_ty) in params.iter().zip(ty.value.params.iter()) {
                    self.declare(&param_id, &param_ty);
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
                    self.declare(&id, &ty.value);
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

                self.check_expr(&then.value);

                // else scope
                self.check_expr(&els.value);
            }
            Expr::While { cond, expr } => {
                self.check_expr(&cond.value);

                // while body scope
                self.check_expr(&expr.value);
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
        if self.lookup(&id.value).is_none() {
            let all_symbols = self.symbols.get_symbols_in_scope().iter().map(|(id, _)| id.to_owned()).collect::<Vec<_>>();
            let similar = get_similar(all_symbols, &id.value);
            self.errors.push(DeclarationError::undeclared_identifier(id.clone(), similar));
        }
    }

    fn declare(&mut self, id: &Spanned<Id>, ty: &Type) {
        if RESERVED_IDENTIFIERS.contains(&id.value) {
            self.errors.push(DeclarationError::reserved_identifier(id.clone()));
        }
        let symbol = Symbol {
            ty: ty.clone(),
            span: id.span.clone(),
        };
        self.symbols.declare(&id.value, &symbol);
        if !id.value.starts_with("_") {
            self.unused_symbols.push(id.clone());
        }
    }

    fn lookup(&mut self, id: &Id) -> Option<Symbol> {
        let symbol = self.symbols.lookup(&id);
        if let Some(symbol) = &symbol {
            let span = Spanned {
                value: id.clone(),
                span: symbol.span.clone(),
            };
            // remove from unused symbols
            self.unused_symbols = self.unused_symbols
                .clone()
                .into_iter()
                .filter(|s| *s != span)
                .collect::<Vec<_>>()
        }
        symbol
    }

    fn get_warnings(&self) -> Vec<Warning> {
        let mut warnings = Vec::new();
        for id in &self.unused_symbols {
            warnings.push(Warning::UnusedIdentifier(id.clone()));
        }
        warnings
    }
}
