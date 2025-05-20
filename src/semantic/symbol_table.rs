use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
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
pub struct Scope {
    symbols: HashMap<Id, Symbol>,
    parent: Option<ScopeRef>,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    curr_scope: ScopeRef,
    unused_symbols: Vec<(Id, Symbol)>,
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
                (id.to_string(), symbol)
            })
            .collect::<HashMap<_, _>>();
        let root = Rc::new(RefCell::new(Scope {
            symbols,
            parent: None,
        }));
        SymbolTable {
            curr_scope: root,
            unused_symbols: Vec::new(),
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
        self.update_unused_symbols(scope);

        // go back to parent scope
        if let Some(parent) = parent_opt {
            self.curr_scope = parent;
        } else {
            panic!("cannot exit the root scope");
        }
    }

    // returns true if the identifier was declared
    pub fn declare(&mut self, id: Spanned<Id>, ty: Type) -> bool {
        if id.value == "_" {
            // wildcards are not declared (ignored)
            return true
        }
        let mut scope = self.curr_scope.borrow_mut();
        if scope.symbols.contains_key(&id.value) { // redefinition
            if scope.parent.is_none() { // global scope
                return false // cannot redeclare global declarations
            }
        }
        let symbol = Symbol {
            ty,
            span: id.span,
            used: false,
        };
        scope.symbols.insert(id.value, symbol);
        true
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

    pub fn get_symbols_in_scope(&self) -> Vec<(Id, Symbol)> {
        let mut map = HashMap::new();
        let mut scope_opt = Some(self.curr_scope.clone());
        while let Some(scope_ref) = scope_opt {
            let scope = scope_ref.borrow();
            for name in scope.symbols.keys() {
                map.insert(name.clone(), scope.symbols[name].clone());
            }
            scope_opt = scope.parent.clone();
        }
        map.into_iter().collect()
    }

    pub fn get_scope(&self) -> ScopeRef {
        self.curr_scope.clone()
    }

    pub fn get_unused_symbols(&self) -> Vec<(Id, Symbol)> {
        self.unused_symbols.clone()
    }

    fn update_unused_symbols(&mut self, scope: Scope) {
        self.unused_symbols.extend(
            scope.symbols
                .iter()
                .filter(|(id, symbol)| !symbol.used && !id.starts_with("_"))
                .map(|(id, symbol)| (id.clone(), symbol.clone()))
                .collect::<Vec<_>>()
        );
    }
}
