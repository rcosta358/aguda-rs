use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::errors::Warning;
use crate::semantic::INIT_SYMBOLS;
use crate::syntax::ast::{Id, Span, Spanned, Type};

// wrapper for shared and mutable access
type ScopeRef = Rc<RefCell<Scope>>;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub ty: Type,
    pub span: Span,
    pub used: bool,
}

#[derive(Debug, Clone)]
struct Scope {
    symbols: HashMap<String, Symbol>,
    parent: Option<ScopeRef>,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    curr_scope: ScopeRef,
    warnings: Vec<Warning>,
}

impl SymbolTable {

    pub fn new() -> Self {
        let symbols = INIT_SYMBOLS
            .iter()
            .cloned()
            .map(|(id, ty)| {
                let symbol = Symbol {
                    ty,
                    span: Span::default(),
                    used: true,
                };
                (id, symbol)
            })
            .collect::<HashMap<_, _>>();
        let root = Rc::new(RefCell::new(Scope {
            symbols,
            parent: None,
        }));
        SymbolTable {
            curr_scope: root,
            warnings: Vec::new()
        }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Rc::new(RefCell::new(Scope {
            symbols: HashMap::new(),
            parent: Some(self.curr_scope.clone()),
        }));
        self.curr_scope = new_scope; // enter new nested scope
    }

    pub fn exit_scope(&mut self) {
        let (parent_opt, scope) = {
            let curr = self.curr_scope.borrow();
            (curr.parent.clone(), curr.clone())
        };

        self.check_unused_symbols(scope);

        // go back to parent scope
        if let Some(parent) = parent_opt {
            self.curr_scope = parent;
        } else {
            panic!("cannot exit the root scope");
        }
    }

    pub fn declare(&mut self, id: Spanned<String>, ty: Type) -> Result<(), ()> {
        if id.value == "_" {
            // wildcards are not declared (ignored)
            return Ok(());
        }
        let mut scope = self.curr_scope.borrow_mut();
        if scope.symbols.contains_key(&id.value) {
            return Err(()); // duplicate declaration
        }
        let symbol = Symbol {
            ty,
            span: id.span,
            used: false,
        };
        scope.symbols.insert(id.value, symbol);
        Ok(())
    }

    pub fn lookup(&self, id: &str) -> Option<Type> {
        if id == "_" {
            // wildcards cannot be looked up
            return None;
        }
        // look for the identifier in the current scope and its parents
        let mut scope_opt = Some(self.curr_scope.clone());
        while let Some(scope_ref) = scope_opt {
            let mut scope = scope_ref.borrow_mut();
            if let Some(symbol) = scope.symbols.get_mut(id) {
                symbol.used = true; // mark as used
                return Some(symbol.ty.clone());
            }
            scope_opt = scope.parent.clone();
        }
        None
    }

    pub fn get_warnings(&self) -> Vec<Warning> {
        // declaration scopes
        let mut all = self.warnings.clone();

        // global scope
        let curr = self.curr_scope.borrow();
        for (id, sym) in curr.symbols.iter() {
            if !sym.used {
                all.push(Warning::UnusedSymbol(Spanned {
                    value: id.clone(),
                    span:  sym.span.clone(),
                }));
            }
        }
        all.reverse(); // return errors from top to bottom
        all
    }

    fn check_unused_symbols(&mut self, scope: Scope) {
        // collect all unused symbols in the current scope
        let unused_symbols: Vec<(Id, Symbol)> = scope.symbols
            .iter()
            .filter(|(_, symbol)| !symbol.used)
            .map(|(id, symbol)| (id.clone(), symbol.clone()))
            .collect();

        // add warnings
        for (id, sym) in unused_symbols.iter() {
            let warning = Warning::UnusedSymbol(Spanned {
                value: id.clone(),
                span: sym.span.clone(),
            });
            self.warnings.push(warning);
        }
    }
}
