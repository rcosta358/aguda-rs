use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::semantic::INIT_SYMBOLS;
use crate::syntax::ast::Type;

// wrapper for shared and mutable access
type ScopeRef = Rc<RefCell<Scope>>;

#[derive(Debug, Clone)]
struct Scope {
    symbols: HashMap<String, Type>,
    parent: Option<ScopeRef>,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    curr_scope: ScopeRef,
}

impl SymbolTable {

    pub fn new() -> Self {
        let symbols = INIT_SYMBOLS.iter().cloned().collect::<HashMap<_, _>>();
        let root = Rc::new(RefCell::new(Scope {
            symbols,
            parent: None,
        }));
        SymbolTable { curr_scope: root }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Rc::new(RefCell::new(Scope {
            symbols: HashMap::new(),
            parent: Some(self.curr_scope.clone()),
        }));
        self.curr_scope = new_scope; // enter new nested scope
    }

    pub fn exit_scope(&mut self) {
        let parent_opt = {
            let curr = self.curr_scope.borrow();
            curr.parent.clone()
        };
        // go back to parent scope
        if let Some(parent) = parent_opt {
            self.curr_scope = parent;
        } else {
            panic!("cannot exit the root scope");
        }
    }

    pub fn declare(&mut self, id: String, ty: Type) -> Result<(), ()> {
        if id == "_" {
            // wildcards are not declared (ignored)
            return Ok(());
        }
        let mut scope = self.curr_scope.borrow_mut();
        if scope.symbols.contains_key(&id) {
            Err(()) // duplicate declaration
        } else {
            scope.symbols.insert(id, ty);
            Ok(())
        }
    }

    pub fn lookup(&self, id: &str) -> Option<Type> {
        if id == "_" {
            // wildcards cannot be looked up
            return None;
        }
        let mut scope_opt = Some(self.curr_scope.clone());

        // look for the identifier in the current scope and its parents
        while let Some(scope_ref) = scope_opt {
            let scope = scope_ref.borrow();
            if let Some(info) = scope.symbols.get(id) {
                return Some(info.clone());
            }
            scope_opt = scope.parent.clone();
        }
        None
    }
}
