use crate::semantic::TypeError;
use crate::semantic::symbol_table::SymbolTable;
use crate::syntax::ast::*;

#[derive(Debug)]
pub struct TypeChecker {
    symbols: SymbolTable,
    errors: Vec<TypeError>,
}

impl TypeChecker {

    pub fn new(symbols: SymbolTable) -> Self {
        TypeChecker {
            symbols, // already with global declarations
            errors: Vec::new()
        }
    }

    pub fn check(&mut self, prog: &Program) -> Result<(), Vec<TypeError>> {
        for decl in &prog.decls {
            match &decl.value {
                Decl::Var { ty, expr, ..} => {
                    // var scope
                    self.symbols.enter_scope();
                    self.check_against(expr, &ty.value);
                    self.symbols.exit_scope();
                }
                Decl::Fun { params, ty, expr, .. } => {
                    // function scope
                    self.symbols.enter_scope();
                    for (param_id, param_ty) in params.iter().zip(ty.params.iter()) {
                        self.symbols.declare(param_id.value.clone(), param_ty.clone()).unwrap();
                    }
                    self.check_against(expr, &ty.ret);
                    self.symbols.exit_scope()
                }
            }
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    pub fn type_of(&mut self, expr: &Spanned<Expr>) -> Type {
        let span = expr.span.clone();
        match &expr.value {
            Expr::Chain { lhs, rhs } => {
                self.type_of(lhs);
                self.type_of(rhs)
            }
            Expr::Let { id, ty, expr } => {

                // let scope
                self.symbols.enter_scope();
                self.check_against(expr, &ty.value);
                self.symbols.exit_scope();

                // only declare after inner scope
                self.symbols.declare(id.value.clone(), ty.value.clone()).unwrap();
                Type::Unit
            }
            Expr::Set { lhs, expr } => {
                let lhs_type = self.type_of_lhs(lhs);
                self.check_against(expr, &lhs_type);
                Type::Unit
            }
            Expr::BinOp { lhs, op, rhs } => {
                match op.get_type() {
                    OpType::Numerical => {
                        self.check_against(lhs, &Type::Int);
                        self.check_against(rhs, &Type::Int);
                        Type::Int
                    }
                    OpType::Logical => {
                        self.check_against(lhs, &Type::Bool);
                        self.check_against(rhs, &Type::Bool);
                        Type::Bool
                    }
                    OpType::Comparison => {
                        let left_type = self.type_of(lhs);
                        self.check_against(rhs, &left_type);
                        Type::Bool
                    }
                    OpType::String => {
                        self.check_against(lhs, &Type::String);
                        self.check_against(rhs, &Type::String);
                        Type::String
                    }
                }
            }
            Expr::Not { expr: e } => {
                self.check_against(e, &Type::Bool);
                Type::Bool
            }
            Expr::FunCall { id, args } => {
                let fun_type = self.symbols.lookup(&id.value).unwrap();
                if let Type::Fun(ty) = fun_type {
                    if ty.params.len() != args.len() {
                        self.errors.push(
                            TypeError::WrongNumberOfArguments {
                                span: span.clone(),
                                expected: ty.params.len(),
                                found: args.len(),
                            }
                        )
                    }
                    for (arg, arg_type) in args.iter().zip(ty.params.iter()) {
                        self.check_against(arg, arg_type);
                    }
                    *ty.ret.clone()
                } else {
                    self.errors.push(
                        TypeError::NotCallable {
                            span: span.clone(),
                            found: fun_type,
                        }
                    );
                    Type::Unit
                }
            }
            Expr::IfElse { cond, then, els } => {
                self.check_against(cond, &Type::Bool);

                // then scope
                self.symbols.enter_scope();
                let then_type = self.type_of(then);
                self.symbols.exit_scope();

                // else scope
                self.symbols.enter_scope();
                self.check_against(els, &then_type);
                self.symbols.exit_scope();

                then_type
            }
            Expr::While { cond, expr } => {
                self.check_against(cond, &Type::Bool);

                // while body scope
                self.symbols.enter_scope();
                self.type_of(expr);
                self.symbols.exit_scope();

                Type::Unit
            }
            Expr::NewArray { ty, size, init } => {
                self.check_against(size, &Type::Int);
                self.check_against(init, &ty.value);
                Type::Array(Box::new(ty.value.clone()))
            }
            Expr::ArrayIndex { lhs, index } => {
                let arr_type = self.type_of_lhs(lhs);
                self.check_against(index, &Type::Int);
                if let Type::Array(elem_ty) = arr_type {
                    *elem_ty.clone()
                } else {
                    self.errors.push(
                        TypeError::NotIndexable {
                            span: span.clone(),
                            found: arr_type,
                        }
                    );
                    Type::Unit
                }
            }
            Expr::Id(id) => self.symbols.lookup(&id.value).unwrap(),
            Expr::Number(_) => Type::Int,
            Expr::String(_) => Type::String,
            Expr::Bool(_) => Type::Bool,
            Expr::Unit => Type::Unit,
        }
    }

    fn type_of_lhs(&mut self, lhs: &Spanned<Lhs>) -> Type {
        match &lhs.value {
            Lhs::Var { id } => {
                self.symbols.lookup(&id.value).expect("undefined identifier")
            }
            Lhs::Index { lhs, index } => {
                let arr_type = self.type_of_lhs(lhs);
                self.check_against(index, &Type::Int);
                if let Type::Array(elem) = arr_type {
                    *elem.clone()
                } else {
                    self.errors.push(
                        TypeError::NotIndexable {
                            span: lhs.span.clone(),
                            found: arr_type,
                        }
                    );
                    Type::Unit
                }
            }
        }
    }

    pub fn check_against(&mut self, expr: &Spanned<Expr>, expected: &Type) {
        let found = self.type_of(expr);
        match expected {
            // any type matches any type
            Type::Any => return,

            // array of any should match an array of any type
            Type::Array(expected_inner) if **expected_inner == Type::Any => {
                match found {
                    Type::Array(_) => return,
                    _ => {
                        // not an array
                        self.errors.push(TypeError::TypeMismatch {
                            span: expr.span.clone(),
                            expected: expected.clone(),
                            found,
                        });
                    }
                }
            }
            // all other cases must match exactly
            _ => {
                if &found != expected {
                    self.errors.push(TypeError::TypeMismatch {
                        span: expr.span.clone(),
                        expected: expected.clone(),
                        found,
                    });
                }
            }
        }
    }

}