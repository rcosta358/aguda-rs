use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use lazy_static::lazy_static;
use crate::semantic::symbol_table::VarInfo::FunType;
use crate::syntax::ast::Type;

#[derive(Debug, Clone)]
pub enum VarInfo {
    VarType(Type),
    FunType {
        param_types: Vec<Type>,
        ret_type: Type,
    },
}

type ScopeRef = Rc<RefCell<Scope>>;

#[derive(Debug)]
struct Scope {
    symbols: HashMap<String, VarInfo>,
    parent: Option<ScopeRef>,
}

#[derive(Debug)]
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
        SymbolTable {
            current_scope: root,
        }
    }

    pub fn enter_scope(&mut self) {
        let new_scope = Rc::new(RefCell::new(Scope {
            symbols: HashMap::new(),
            parent: Some(self.current_scope.clone()),
        }));
        self.current_scope = new_scope;
    }

    pub fn exit_scope(&mut self) {
        let parent = {
            let current = self.current_scope.borrow();
            if let Some(ref parent) = current.parent {
                Some(parent.clone())
            } else {
                None
            }
        };
        if let Some(parent) = parent {
            self.current_scope = parent;
        } else {
            panic!("tried to exit the root scope");
        }
    }

    pub fn declare(&mut self, id: String, ty: VarInfo) -> Result<(), String> {
        let mut scope = self.current_scope.borrow_mut();
        if scope.symbols.contains_key(&id) {
            Err(format!("variable '{}' already declared in this scope", id))
        } else {
            scope.symbols.insert(id, ty);
            Ok(())
        }
    }

    pub fn lookup(&self, name: &str) -> Option<VarInfo> {
        let mut scope = Some(self.current_scope.clone());
        while let Some(s) = scope {
            let s_ref = s.borrow();
            if let Some(info) = s_ref.symbols.get(name) {
                return Some(info.clone());
            }
            scope = s_ref.parent.clone();
        }
        None
    }
}

lazy_static! {
    static ref INIT_SYMBOLS: [(String, VarInfo); 2] = [
        (
            "print".to_string(),
            FunType {
                param_types: vec![Type::Any],
                ret_type: Type::Unit
            }
        ),
        (
            "length".to_string(),
            FunType {
                param_types: vec![Type::Array(Box::new(Type::Any))],
                ret_type: Type::Int
            }
        )
    ];
}