use inkwell::{context::Context, builder::Builder, module::Module, values::*, types::BasicTypeEnum, IntPredicate};
use std::collections::HashMap;
use std::path::Path;
use inkwell::memory_buffer::MemoryBuffer;
use inkwell::types::BasicType;
use crate::syntax::ast::*;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {

    pub fn new(module_name: &str, context: &'ctx Context) -> Self {
        let module = context.create_module(module_name);
        let aguda_buf = MemoryBuffer::create_from_file(Path::new("aguda.ll")).expect("could not read aguda.ll");
        let aguda_mod = context.create_module_from_ir(aguda_buf).expect("failed to parse aguda.ll");
        module.link_in_module(aguda_mod).expect("failed to link aguda module");
        let builder = context.create_builder();
        CodeGen {
            context,
            module,
            builder,
            variables: HashMap::new(),
        }
    }

    pub fn output_to_file(&self) {
        let file_name = self.module.get_name().to_str().unwrap().replace(".agu", ".ll");
        self.module.print_to_file(file_name).expect("failed to generate to file");
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
            if let Decl::Fun { params, ty, expr, .. } = decl {
                let params = params.iter().map(|param| param.value.clone()).collect::<Vec<_>>();
                self.gen_fun(fun, &params, &ty.value, &expr.value);
            }
        }
    }

    fn gen_decl(&mut self, decl: &Decl) {
        if let Decl::Var { id, ty, expr } = decl {
            match &expr.value {
                Expr::Unit | Expr::String(_) | Expr::Bool(_) | Expr::Int(_) => {}
                _ => unimplemented!("not implemented expression for top-level declaration at line {}", expr.span.start),
            }
            let llvm_ty = self.llvm_type(&ty.value);
            let val = self.gen_expr(&expr.value);
            let global = self.module.add_global(llvm_ty, None, &id.value);
            global.set_initializer(&val);
        }
    }

    fn gen_fun_signature(
        &mut self,
        id: &Id,
        fun_ty: &FunType,
    ) -> FunctionValue<'ctx> {
        let ret_ty = self.llvm_type(&fun_ty.ret);
        let params_ty = fun_ty
            .params
            .iter()
            .map(|ty| self.llvm_type(ty).into())
            .collect::<Vec<_>>();
        let fn_type = ret_ty.fn_type(&params_ty, false);
        self.module.add_function(&id, fn_type, None)
    }

    fn gen_fun(
        &mut self,
        fun: &FunctionValue<'ctx>,
        params: &[Id],
        fun_ty: &FunType,
        body: &Expr,
    ) {
        // function body
        let entry = self.context.append_basic_block(fun.clone(), "entry");
        self.builder.position_at_end(entry);
        self.variables.clear();

        // allocate space for params
        for (index, param) in fun.get_param_iter().enumerate() {
            let name = &params[index];
            let ty = self.llvm_type(&fun_ty.params[index]);
            let pointer = self.builder.build_alloca(ty, name).expect("failed to allocate parameter");
            self.builder.build_store(pointer, param).expect("failed to store parameter");
            self.variables.insert(name.clone(), pointer);
        }

        // generate function body
        let ret_val = self.gen_expr(body);
        self.builder.build_return(Some(&ret_val)).expect("failed to return from function");
    }

    fn gen_expr(&mut self, expr: &Expr) -> BasicValueEnum<'ctx> {
        match expr {
            Expr::Chain { lhs, rhs } => {
                self.gen_expr(&lhs.value);
                self.gen_expr(&rhs.value)
            }
            Expr::Let { id, ty, expr } => {
                let val = self.gen_expr(&expr.value);
                if id.value == "_" {
                    // dont store wildcards
                    return val;
                }
                let llvm_ty = self.llvm_type(&ty.value);
                let pointer = self.builder.build_alloca(llvm_ty, &id.value).expect("failed to allocate variable");
                self.builder.build_store(pointer, val).expect("failed to store value");
                self.variables.insert(id.value.clone(), pointer);
                val
            }
            Expr::Set { lhs, expr } => {
                let pointer = self.gen_lhs(&lhs.value);
                let val = self.gen_expr(&expr.value);
                self.builder.build_store(pointer, val).expect("failed to store value");
                val
            }
            Expr::BinOp { lhs, op, rhs } => {
                let l = self.gen_expr(&lhs.value).into_int_value();
                let r = self.gen_expr(&rhs.value).into_int_value();
                let res = match op {
                    Op::Add => self.builder.build_int_add(l, r, "add"),
                    Op::Sub => self.builder.build_int_sub(l, r, "sub"),
                    Op::Mul => self.builder.build_int_mul(l, r, "mul"),
                    Op::Div => self.builder.build_int_signed_div(l, r, "div"),
                    Op::Mod => self.builder.build_int_signed_rem(l, r, "mod"),
                    Op::Eq => self.builder.build_int_compare(IntPredicate::EQ, l, r, "eq"),
                    Op::Neq => self.builder.build_int_compare(IntPredicate::NE, l, r, "neq"),
                    Op::Lt => self.builder.build_int_compare(IntPredicate::SLT, l, r, "lt"),
                    Op::Leq => self.builder.build_int_compare(IntPredicate::SLE, l, r, "leq"),
                    Op::Gt => self.builder.build_int_compare(IntPredicate::SGT, l, r, "gt"),
                    Op::Geq => self.builder.build_int_compare(IntPredicate::SGE, l, r, "geq"),
                    Op::And => self.builder.build_and(l, r, "and"),
                    Op::Or => self.builder.build_or(l, r, "or"),
                    Op::Pow => Ok({
                        let pow_fn = self.module.get_function("__pow__").expect("pow function not found");
                        let args = [l.into(), r.into()];
                        let call = self.builder.build_call(pow_fn, &args, "pow").expect("failed to build pow call");
                        call.try_as_basic_value().left().expect("failed to get pow result").into_int_value()
                    })
                };
                res.unwrap().into()
            }
            Expr::Not { expr } => {
                let val = self.gen_expr(&expr.value).into_int_value();
                self.builder.build_not(val, "not").expect("failed to build not").into()
            }
            Expr::While { cond, expr } => {
                let parent = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                let cond_block = self.context.append_basic_block(parent, "cond");
                let body_block = self.context.append_basic_block(parent, "body");
                let after_block = self.context.append_basic_block(parent, "after");

                // while block
                self.builder.build_unconditional_branch(cond_block).expect("failed to build unconditional branch");

                // cond block
                self.builder.position_at_end(cond_block);
                let cond_val = self.gen_expr(&cond.value).into_int_value();
                let zero = self.context.bool_type().const_int(0, false);
                let cmp = self.builder
                    .build_int_compare(IntPredicate::NE, cond_val, zero, "while_cond")
                    .expect("failed to build comparison");

                self.builder.build_conditional_branch(cmp, body_block, after_block)
                    .expect("failed to build conditional branch");

                // body block
                self.builder.position_at_end(body_block);
                self.gen_expr(&expr.value);
                self.builder.build_unconditional_branch(cond_block).expect("failed to build unconditional branch");

                // after block
                self.builder.position_at_end(after_block);

                // return value
                self.context.i32_type().const_int(0, false).into()
            }
            Expr::IfElse { cond, then, els } => {
                let parent = self.builder.get_insert_block().unwrap().get_parent().unwrap();
                let then_block = self.context.append_basic_block(parent, "then");
                let else_block = self.context.append_basic_block(parent, "else");
                let merge_block = self.context.append_basic_block(parent, "merge");
                let cond_val = self.gen_expr(&cond.value).into_int_value();
                let zero = self.context.bool_type().const_int(0, false);
                let cmp = self.builder.build_int_compare(IntPredicate::NE, cond_val, zero, "cond").expect("failed to build comparison");
                self.builder.build_conditional_branch(cmp, then_block, else_block).expect("failed to build conditional branch");

                // then block
                self.builder.position_at_end(then_block);
                let then_val = self.gen_expr(&then.value);
                self.builder.build_unconditional_branch(merge_block).expect("failed to build unconditional branch");

                // else block
                self.builder.position_at_end(else_block);
                let else_val = self.gen_expr(&els.value);
                self.builder.build_unconditional_branch(merge_block).expect("failed to build unconditional branch");

                // merge block
                self.builder.position_at_end(merge_block);
                let phi = self.builder.build_phi(then_val.get_type(), "phi").expect("failed to build phi");
                phi.add_incoming(&[(&then_val, then_block), (&else_val, else_block)]);
                phi.as_basic_value()
            }
            Expr::FunCall { id, args } => {
                let fun_name = match id.value.as_str() {
                    "print" => "__print_int__",
                    _ => &id.value,
                };
                let fun = self.module.get_function(fun_name).expect(format!("undefined function {}", fun_name).as_str());
                let args = args
                    .iter()
                    .map(|arg| BasicMetadataValueEnum::from(self.gen_expr(&arg.value)))
                    .collect::<Vec<_>>();
                let call = self.builder.build_call(fun, &args, "call").expect("failed to call function");
                call.try_as_basic_value().left().expect("failed to get return value")
            }
            Expr::Id(id) => {
                let pointer = self.variables.get(&id.value).expect("undefined variable");
                let i32_ty = self.context.i32_type();
                self.builder.build_load(i32_ty, *pointer, &id.value).expect("failed to load variable")
            }
            Expr::Int(n) => self.context.i32_type().const_int(*n as u64, true).into(),
            Expr::Bool(b) => self.context.bool_type().const_int(*b as u64, false).into(),
            Expr::Unit => self.context.i32_type().const_zero().into(),
            _ => unimplemented!("not implemented expression: {:?}", expr),
        }
    }

    fn gen_lhs(&mut self, lhs: &Lhs) -> PointerValue<'ctx> {
       match lhs {
           Lhs::Var { id } => *self.variables.get(&id.value).expect("undefined variable"),
           Lhs::Index { .. } => unimplemented!("not implemented expression: array index")
       }
    }

    fn llvm_type(&self, ty: &Type) -> BasicTypeEnum<'ctx> {
        match ty {
            Type::Int => self.context.i32_type().into(),
            Type::Bool => self.context.bool_type().into(),
            Type::Unit => self.context.i32_type().into(),
            _ => unimplemented!("not implemented type: {:?}", ty),
        }
    }
}
