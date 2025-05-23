use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::syntax::ast::Id;

// wrapper for shared and mutable access
type ScopeRef<T> = Rc<RefCell<Scope<T>>>;

#[derive(Debug, Clone)]
pub struct Scope<T> {
    symbols: HashMap<Id, T>,
    parent: Option<ScopeRef<T>>,
}

#[derive(Debug, Clone)]
pub struct SymbolTable<T> {
    curr_scope: ScopeRef<T>,
}

impl <T: Clone> SymbolTable <T> {

    pub fn new(symbols: HashMap<Id, T>) -> Self {
        let root = Rc::new(RefCell::new(Scope {
            symbols,
            parent: None,
        }));
        SymbolTable {
            curr_scope: root,
        }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Rc::new(RefCell::new(Scope {
            symbols: HashMap::new(),
            parent: Some(self.curr_scope.clone()),
        }));
        // enter new nested scope
        self.curr_scope = new_scope;
    }

    pub fn exit_scope(&mut self) {
        let parent_opt = self.curr_scope.borrow().parent.clone();
        // go to parent scope
        if let Some(parent) = parent_opt {
            self.curr_scope = parent;
        } else {
            panic!("cannot exit the root scope");
        }
    }

    // returns true if the identifier was declared
    pub fn declare(&mut self, id: &str, val: &T) -> bool {
        if id == "_" {
            // wildcards are not declared (ignored)
            return true
        }
        let mut scope = self.curr_scope.borrow_mut();
        if scope.symbols.contains_key(id) { // redefinition
            if scope.parent.is_none() { // global scope
                return false // cannot redeclare global declarations
            }
        }
        scope.symbols.insert(id.to_string(), val.clone());
        true
    }

    pub fn lookup(&self, id: &str) -> Option<T> {
        if id == "_" {
            // wildcards cannot be looked up
            return None;
        }
        // look for the identifier in the current scope and its parents
        let mut scope_opt = Some(self.curr_scope.clone());
        while let Some(scope_ref) = scope_opt {
            let scope = scope_ref.borrow();
            if let Some(val) = scope.symbols.get(id) {
                return Some(val.clone());
            }
            scope_opt = scope.parent.clone();
        }
        None
    }

    pub fn get_symbols_in_scope(&self) -> Vec<(Id, T)> {
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
}
