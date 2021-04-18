pub mod codegen;
pub mod parser;
pub mod util;

use std::ffi::CString;
use std::process::Command;
use std::{fs, ptr};

pub fn build(contents: String, output: String) -> std::io::Result<()> {
    let output = CString::new(output).expect("Invalid CString.");

    let ast = crate::parser::parse(contents);
    let (builder, module, context) = unsafe { crate::codegen::codegen(ast) };

    unsafe {
        llvm::core::LLVMPrintModuleToFile(module, output.as_ptr(), ptr::null_mut());

        llvm::core::LLVMDisposeBuilder(builder);
        llvm::core::LLVMDisposeModule(module);
        llvm::core::LLVMContextDispose(context);
    }

    // Now that we have `out.ll` as am LLVM IR file, we can pass the work onto clang which will
    // compile to assembly and then machine code.
    Command::new("clang").arg(output.to_str().unwrap());
    fs::remove_file("out.ll")?;

    Ok(())
}

#[cfg(test)]
mod tests {

    /*
    Find tests in their respective files
     */
}
