use std::ffi::CString;

use llvm::prelude::LLVMBuilderRef;
use llvm::prelude::LLVMContextRef;
use llvm::prelude::LLVMModuleRef;
use llvm::prelude::LLVMValueRef;
use llvm::LLVMBuilder;
use llvm::LLVMContext;
use llvm::LLVMModule;
use llvm::LLVMType;

use std::collections::HashMap;

use crate::util::{BinaryOp, Node, Type, UnaryOp};

unsafe fn codegen_expr(
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    node: Node,
    mut namespace_items: &mut HashMap<String, LLVMValueRef>,
) -> Result<LLVMValueRef, std::io::Error> {
    match node {
        Node::Int(value) => {
            let int_type = llvm::core::LLVMInt32TypeInContext(context);
            Ok(llvm::core::LLVMConstInt(int_type, value as u64, 0))
        }

        Node::Bool(value) => {
            // Booleans are really just `i1`s
            let bool_type = llvm::core::LLVMInt1TypeInContext(context);

            Ok(llvm::core::LLVMConstInt(bool_type, value as u64, 0))
        }

        Node::Ref(value) => Ok(*namespace_items.get(&value).unwrap()),

        Node::FnRef(..) => unimplemented!(),

        Node::Assign(name, value) => {
            let new_value = codegen_expr(context, module, builder, *value, namespace_items)?;
            namespace_items.insert(name, new_value);
            Ok(new_value)
        }

        Node::Function {
            name,
            arguments: _,
            return_value,
            children,
        } => {
            let return_type = match return_value {
                Type::Bool => llvm::core::LLVMInt1TypeInContext(context),
                Type::Int => llvm::core::LLVMInt32TypeInContext(context),
            };

            let fn_type =
                llvm::core::LLVMFunctionType(return_type, return_type as *mut *mut LLVMType, 0, 0);

            // We turn `name` into a CString to null-terminate it
            let name_as_ptr = CString::new(name).unwrap();
            let function_name = name_as_ptr.as_ptr() as *const _;

            let function = llvm::core::LLVMAddFunction(module, function_name, fn_type);

            let block = llvm::core::LLVMAppendBasicBlockInContext(
                context,
                function,
                b"entry\0".as_ptr() as *const _,
            );
            llvm::core::LLVMPositionBuilderAtEnd(builder, block);

            for expr in children {
                codegen_expr(context, module, builder, *expr, &mut namespace_items)?;
            }

            Ok(llvm::core::LLVMGetNamedFunction(module, function_name))
        }

        Node::BinaryExpr { op, lhs, rhs } => {
            let lhs = codegen_expr(context, module, builder, *lhs, namespace_items)?;
            let rhs = codegen_expr(context, module, builder, *rhs, namespace_items)?;

            match op {
                BinaryOp::Plus => {
                    let name = CString::new("addtmp").unwrap();
                    Ok(llvm::core::LLVMBuildAdd(builder, lhs, rhs, name.as_ptr()))
                }
                BinaryOp::Minus => {
                    let name = CString::new("subtmp").unwrap();
                    Ok(llvm::core::LLVMBuildSub(builder, lhs, rhs, name.as_ptr()))
                }
            }
        }

        Node::UnaryExpr { op, child } => match op {
            UnaryOp::Minus => {
                let lhs = codegen_expr(context, module, builder, Node::Int(0), namespace_items)?;
                let rhs = codegen_expr(context, module, builder, *child, namespace_items)?;

                let name = CString::new("subtmp").unwrap();
                Ok(llvm::core::LLVMBuildSub(builder, lhs, rhs, name.as_ptr()))
            }
            UnaryOp::Return => {
                let child = codegen_expr(context, module, builder, *child, namespace_items)?;

                Ok(llvm::core::LLVMBuildRet(builder, child))
            }
        },
    }
}

pub unsafe fn codegen(input: Vec<Node>) -> (*mut LLVMBuilder, *mut LLVMModule, *mut LLVMContext) {
    let context = llvm::core::LLVMContextCreate();
    let module = llvm::core::LLVMModuleCreateWithName(b"jargono_compiled\0".as_ptr() as *const _);
    let builder = llvm::core::LLVMCreateBuilderInContext(context);

    let mut namespace_items = HashMap::new();
    for expr in input {
        codegen_expr(context, module, builder, expr, &mut namespace_items)
            .expect("Error generating code.");
    }

    (builder, module, context)
}
