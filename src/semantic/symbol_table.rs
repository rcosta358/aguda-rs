use lazy_static::lazy_static;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::syntax::ast::{FunType, Type};

type ScopeRef = Rc<RefCell<Scope>>;

#[derive(Debug, Clone)]
struct Scope {
    symbols: HashMap<String, Type>,
    parent: Option<ScopeRef>,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    current_scope: ScopeRef,
}

impl SymbolTable {

    pub fn new() -> Self {
        let symbols = INIT_SYMBOLS.iter().cloned().collect::<HashMap<_, _>>();
        let root = Rc::new(RefCell::new(Scope {
            symbols,
            parent: None,
        }));
        SymbolTable { current_scope: root }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Rc::new(RefCell::new(Scope {
            symbols: HashMap::new(),
            parent: Some(self.current_scope.clone()),
        }));
        self.current_scope = new_scope;
    }

    pub fn exit_scope(&mut self) {
        let parent_opt = {
            let curr = self.current_scope.borrow();
            curr.parent.clone()
        };
        if let Some(parent) = parent_opt {
            self.current_scope = parent;
        } else {
            panic!("tried to exit the root scope");
        }
    }

    pub fn declare(&mut self, id: String, ty: Type) -> Result<(), ()> {
        if id == "_" {
            // wildcards are not declared
            return Ok(());
        }
        let mut scope = self.current_scope.borrow_mut();
        if scope.symbols.contains_key(&id) {
            Err(())
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
        let mut scope_opt = Some(self.current_scope.clone());
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

lazy_static! {
    pub static ref INIT_SYMBOLS: [(String, Type); 2] = [
        (
            "print".to_string(),
            Type::Fun(
                FunType {
                    params: vec![Type::Any],
                    ret: Box::new(Type::Unit)
                }
            )
        ),
        (
            "length".to_string(),
            Type::Fun(
                FunType {
                    params: vec![Type::Array(Box::new(Type::Any))],
                    ret: Box::new(Type::Int)
                }
            )
        ),
    ];
    pub static ref RESERVED_IDENTIFIERS: Vec<String> =
        INIT_SYMBOLS.iter().map(|s| s.0.clone()).collect::<Vec<_>>();
}
