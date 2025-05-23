use std::collections::HashMap;
use inkwell::{context::Context, builder::Builder, module::Module, values::*, types::BasicTypeEnum, IntPredicate};
use std::ops::Deref;
use std::path::Path;
use inkwell::builder::BuilderError;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::types::{BasicType, IntType, StructType};
use crate::semantic::symbol_table::SymbolTable;
use crate::syntax::ast::*;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    symbols: SymbolTable<(PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,
}

impl<'ctx> CodeGen<'ctx> {

    pub fn new(module_name: &str, context: &'ctx Context) -> Self {
        if module_name == "lib.agu" {
            panic!("invalid module name"); // avoid collision with lib.ll
        }
        let module = context.create_module(module_name);
        let aguda_buf = MemoryBuffer::create_from_file(Path::new("lib.ll")).expect("could not read lib.ll");
        let aguda_mod = context.create_module_from_ir(aguda_buf).expect("failed to parse lib.ll");
        module.link_in_module(aguda_mod).expect("failed to link aguda module");
        let builder = context.create_builder();
        CodeGen {
            context,
            module,
            builder,
            symbols: SymbolTable::new(HashMap::new()),
        }
    }

    pub fn gen_ll(&self, path: Box<Path>) {
        self.module.print_to_file(path.with_extension("ll")).expect("failed to generate .ll file");
    }

    pub fn gen_program(&mut self, prog: &Program) {
        // declare all functions first to allow for mutually recursive function calls
        let functions = &prog.decls.iter().filter_map(|decl| {
            if let Decl::Fun { id, ty, .. } = &decl.value {
                Some((&decl.value, self.gen_fun_signature(&id.value, &ty.value)))
            } else {
                None
            }
        }).collect::<Vec<_>>();

        // declare globals
        for decl in prog.decls.iter().filter(|d| matches!(d.value, Decl::Var { .. })) {
            self.gen_decl(&decl.value);
        }

        // generate functions
        for (decl, fun) in functions {
            if let Decl::Fun { id, params, ty, expr } = decl {
                let params = params.iter().map(|param| param.value.clone()).collect::<Vec<_>>();
                self.gen_fun(&id.value, fun, &params, &ty.value, &expr.value);
            }
        }
    }

    fn gen_decl(&mut self, decl: &Decl) {
        if let Decl::Var { id, ty, expr } = decl {
            match &expr.value {
                Expr::Unit | Expr::String(_) | Expr::Bool(_) | Expr::Int(_) => {}
                _ => unimplemented!("expression for top-level declaration"),
            }
            let llvm_ty = self.llvm_type(&ty.value);
            self.symbols.enter_scope();
            let val = self.gen_expr(&expr.value);
            self.symbols.exit_scope();
            let global = self.module.add_global(llvm_ty, None, &id.value);
            self.symbols.declare(&id.value, &(global.as_pointer_value(), llvm_ty));
            global.set_initializer(&val);
        }
    }

    fn gen_fun_signature(
        &mut self,
        id: &Id,
        fun_ty: &FunType,
    ) -> FunctionValue<'ctx> {
        let params_ty = fun_ty
            .params
            .iter()
            .map(|t| self.llvm_type(t).into())
            .collect::<Vec<_>>();

        let fn_type = match fun_ty.ret.deref() {
            Type::Unit =>
                if id == "main" {
                    self.context.i32_type().fn_type(&params_ty, false)
                } else {
                    self.context.void_type().fn_type(&params_ty, false)
                },
            _ => self.llvm_type(&fun_ty.ret).fn_type(&params_ty, false),
        };
        self.module.add_function(&id, fn_type, None)
    }


    fn gen_fun(
        &mut self,
        id: &Id,
        fun: &FunctionValue<'ctx>,
        params: &[Id],
        fun_ty: &FunType,
        body: &Expr,
    ) {
        // function body
        let entry = self.context.append_basic_block(fun.clone(), "entry");
        self.builder.position_at_end(entry);
        self.symbols.enter_scope();

        // allocate space for params
        for (index, param) in fun.get_param_iter().enumerate() {
            let name = &params[index];
            let llvm_ty = self.llvm_type(&fun_ty.params[index]);
            let pointer = self.builder.build_alloca(llvm_ty, name).unwrap();
            self.builder.build_store(pointer, param).unwrap();
            self.symbols.declare(name, &(pointer, llvm_ty));
        }

        // generate function body
        let ret_val = self.gen_expr(body);
        self.symbols.exit_scope();
        if matches!(fun_ty.ret.deref(), Type::Unit) {
            if id == "main" {
                self.builder.build_return(Some(&self.int_type().const_int(0, false))).unwrap();
            } else {
                self.builder.build_return(None).unwrap();
            }
        } else {
            self.builder.build_return(Some(&ret_val)).unwrap();
        }
    }

    fn gen_expr(&mut self, expr: &Expr) -> BasicValueEnum<'ctx> {
        match expr {
            Expr::Chain { lhs, rhs } => {
                self.gen_expr(&lhs.value);
                let rhs = self.gen_expr(&rhs.value);
                if matches!(lhs.value, Expr::Let { .. }) {
                    self.symbols.exit_scope();
                }
                rhs
            }
            Expr::Let { id, ty, expr } => {
                let val = self.gen_expr(&expr.value);
                self.symbols.enter_scope();
                // chain expression is responsible for exiting the scope

                // only allocate space if not a wildcard
                if id.value != "_" {
                    let llvm_ty = self.llvm_type(&ty.value);
                    let pointer = self.builder.build_alloca(llvm_ty, &id.value).unwrap();
                    self.builder.build_store(pointer, val).unwrap();
                    self.symbols.declare(&id.value, &(pointer, llvm_ty));
                };

                self.unit_type().const_zero().into()
            }
            Expr::Set { lhs, expr } => {
                let pointer = self.gen_lhs(&lhs.value);
                let val = self.gen_expr(&expr.value);
                self.builder.build_store(pointer, val).unwrap();
                self.unit_type().const_zero().into()
            }
            Expr::BinOp { lhs, op, rhs } => {
                match op {
                    // short circuit operators
                    Op::And => self.build_short_circuit_op(&lhs.value, &rhs.value, true),
                    Op::Or => self.build_short_circuit_op(&lhs.value, &rhs.value, false),
                    _ => {
                        let l = self.gen_expr(&lhs.value).into_int_value();
                        let r = self.gen_expr(&rhs.value).into_int_value();
                        let res = match op {
                            Op::Add => self.builder.build_int_add(l, r, "add"),
                            Op::Sub => self.builder.build_int_sub(l, r, "sub"),
                            Op::Mul => self.builder.build_int_mul(l, r, "mul"),
                            Op::Mod => self.builder.build_int_signed_rem(l, r, "mod"),
                            Op::Eq => self.builder.build_int_compare(IntPredicate::EQ, l, r, "eq"),
                            Op::Neq => self.builder.build_int_compare(IntPredicate::NE, l, r, "neq"),
                            Op::Lt => self.builder.build_int_compare(IntPredicate::SLT, l, r, "lt"),
                            Op::Leq => self.builder.build_int_compare(IntPredicate::SLE, l, r, "leq"),
                            Op::Gt => self.builder.build_int_compare(IntPredicate::SGT, l, r, "gt"),
                            Op::Geq => self.builder.build_int_compare(IntPredicate::SGE, l, r, "geq"),
                            Op::Div => self.call_binop_fun(l, r, "div"),
                            Op::Pow => self.call_binop_fun(l, r, "pow"),
                            Op::And | Op::Or => unreachable!()
                        };
                        res.unwrap().into()
                    }
                }

            }
            Expr::Not { expr } => {
                let val = self.gen_expr(&expr.value).into_int_value();
                self.builder.build_not(val, "not").unwrap().into()
            }
            Expr::While { cond, expr } => {
                let parent = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                let cond_block = self.context.append_basic_block(parent, "cond");
                let body_block = self.context.append_basic_block(parent, "body");
                let after_block = self.context.append_basic_block(parent, "after");

                // while block
                self.builder.build_unconditional_branch(cond_block).unwrap();

                // cond block
                self.builder.position_at_end(cond_block);
                let cond_val = self.gen_expr(&cond.value).into_int_value();
                let zero = self.context.bool_type().const_zero();
                let cmp = self.builder.build_int_compare(IntPredicate::NE, cond_val, zero, "while_cond").unwrap();
                self.builder.build_conditional_branch(cmp, body_block, after_block).unwrap();

                // body block
                self.builder.position_at_end(body_block);
                self.gen_expr(&expr.value);
                self.builder.build_unconditional_branch(cond_block).unwrap();

                // after block
                self.builder.position_at_end(after_block);

                // return value
                self.unit_type().const_zero().into()
            }
            Expr::IfElse { cond, then, els } => {
                let parent = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                let then_block = self.context.append_basic_block(parent, "then");
                let else_block = self.context.append_basic_block(parent, "else");
                let merge_block = self.context.append_basic_block(parent, "merge");

                // condition
                let cond_val = self.gen_expr(&cond.value).into_int_value();
                let zero = cond_val.get_type().const_zero();
                let cmp = self.builder.build_int_compare(IntPredicate::NE, cond_val, zero, "if_cond").unwrap();
                self.builder.build_conditional_branch(cmp, then_block, else_block).unwrap();

                // then
                self.builder.position_at_end(then_block);
                let then_val = self.gen_expr(&then.value);
                let then_end = self.builder.get_insert_block().unwrap();
                self.builder.build_unconditional_branch(merge_block).unwrap();

                // else
                self.builder.position_at_end(else_block);
                let else_val = self.gen_expr(&els.value);
                let else_end = self.builder.get_insert_block().unwrap();
                self.builder.build_unconditional_branch(merge_block).unwrap();

                // merge
                self.builder.position_at_end(merge_block);

                // return branch result
                let phi = self.builder.build_phi(then_val.get_type(), "phi").unwrap();
                phi.add_incoming(&[(&then_val, then_end), (&else_val, else_end)]);
                phi.as_basic_value()

            }
            Expr::FunCall { id, args } => {
                let fun_name = match id.value.as_str() {
                    "print" => {
                        // get the bit width of the first argument to know which print function to call
                        let arg = self.gen_expr(&args.first().unwrap().value).get_type();
                        match arg {
                            BasicTypeEnum::IntType(ty) => {
                                match ty.get_bit_width() {
                                    1 => "__print_bool__",
                                    32 => "__print_int__",
                                    _ => panic!("unsupported bit width for print: {}", ty.get_bit_width()),
                                }
                            }
                            BasicTypeEnum::StructType(ty) if ty.get_field_types().is_empty() =>
                                "__print_unit__",
                            _ => panic!("unsupported type for print: {:?}", arg),
                        }
                    }
                    _ => &id.value
                };
                let fun = self.module.get_function(fun_name)
                    .expect(format!("undefined function {}", fun_name).as_str());
                let args = args
                    .iter()
                    .map(|arg| BasicMetadataValueEnum::from(self.gen_expr(&arg.value)))
                    .collect::<Vec<_>>();
                let call_site = self.builder.build_call(fun, &args, "call").unwrap();
                call_site
                    .try_as_basic_value()
                    .left()
                    .unwrap_or_else(|| self.unit_type().const_zero().into())
            }
            Expr::Id(id) => {
                let (pointer, ty) = self.symbols.lookup(&id.value)
                    .expect(format!("undefined variable {}", id.value).as_str());
                self.builder.build_load(ty, pointer, &id.value).unwrap()
            }
            Expr::Int(n) => self.int_type().const_int(*n as u64, true).into(),
            Expr::Bool(b) => self.bool_type().const_int(*b as u64, false).into(),
            Expr::Unit => self.unit_type().const_zero().into(),
            _ => unimplemented!("expression {:?}", expr),
        }
    }

    fn gen_lhs(&mut self, lhs: &Lhs) -> PointerValue<'ctx> {
       match lhs {
           Lhs::Var { id } => self.symbols.lookup(&id.value)
               .expect(format!("undefined variable {}", id.value).as_str()).0,
           Lhs::Index { .. } => unimplemented!("array index")
       }
    }

    fn llvm_type(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Int => self.int_type().into(),
            Type::Bool => self.bool_type().into(),
            Type::Unit => self.unit_type().into(),
            _ => unimplemented!("type {:?}", ty),
        }
    }

    fn int_type(&self) -> IntType<'ctx> {
        self.context.i32_type()
    }

    fn bool_type(&self) -> IntType<'ctx> {
        self.context.bool_type()
    }

    fn unit_type(&self) -> StructType<'ctx> {
        self.context.struct_type(&[], false)
    }

    fn build_short_circuit_op(&mut self, lhs: &Expr, rhs: &Expr, and_op: bool) -> BasicValueEnum<'ctx> {
        let fun = self.builder.get_insert_block().unwrap().get_parent().unwrap();

        // lhs
        let lhs_val = self.gen_expr(lhs).into_int_value();
        let lhs_block = self.builder.get_insert_block().unwrap();

        // rhs and merge blocks
        let rhs_block = self.context.append_basic_block(fun, "rhs");
        let merge_block = self.context.append_basic_block(fun, "merge");

        // and => branch to rhs when lhs is true
        // or => branch to rhs only when lhs is false
        let cmp = if and_op { lhs_val } else { self.builder.build_not(lhs_val, "not").unwrap() };
        self.builder.build_conditional_branch(cmp, rhs_block, merge_block).unwrap();

        // rhs
        self.builder.position_at_end(rhs_block);
        let rhs_val = self.gen_expr(rhs).into_int_value();
        self.builder.build_unconditional_branch(merge_block).unwrap();
        let rhs_end = self.builder.get_insert_block().unwrap();

        // merge
        self.builder.position_at_end(merge_block);
        let phi = self.builder.build_phi(self.bool_type(), "logical_op").unwrap();
        phi.add_incoming(&[(&lhs_val, lhs_block), (&rhs_val, rhs_end)]);
        phi.as_basic_value().as_basic_value_enum()
    }

    fn call_binop_fun(
        &self,
        l: IntValue<'ctx>,
        r: IntValue<'ctx>,
        name: &str
    ) -> Result<IntValue<'ctx>, BuilderError> {
        let fun_name = format!("__{}__", name);
        let fun = self.module.get_function(&fun_name)
            .expect(format!("undefined function {}", fun_name).as_str());
        let args = [l.into(), r.into()];
        let call = self.builder.build_call(fun, &args, &name)?;
        let ret = call.try_as_basic_value().left().unwrap().into_int_value();
        Ok(ret)
    }
}
