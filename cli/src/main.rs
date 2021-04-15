use clap::clap_app;
use std::ffi::CString;
use std::process::Command;
use std::{fs, ptr};

fn main() {
    let matches = clap_app!(jargono =>
        (version: "0.1.0")
        (@subcommand compile =>
            (about: "Compiles a single .jo file.")
            (@arg input: +required "Sets the input file to use")
        )
    )
    .get_matches();

    // $ jargono compile myfile.jo
    if let Some(ref matches) = matches.subcommand_matches("compile") {
        let file = matches.value_of("input").unwrap();

        let contents = fs::read_to_string(file).expect(&*format!(
            "Unable to read {}: does it exist and do you have access to it?",
            file
        ));

        let ast = jargono::parser::parse(contents);
        let (builder, module, context) = unsafe { jargono::codegen::codegen(ast) };

        let out_file = CString::new("out.ll").unwrap();

        unsafe {
            llvm::core::LLVMPrintModuleToFile(module, out_file.as_ptr(), ptr::null_mut());

            llvm::core::LLVMDisposeBuilder(builder);
            llvm::core::LLVMDisposeModule(module);
            llvm::core::LLVMContextDispose(context);
        }

        // Now that we have `out.ll` as am LLVM IR file, we can pass the work onto llc and clang to compile to machine code.
        let llc = Command::new("llc").arg("out.ll").output().unwrap();
        let clang = Command::new("clang").arg("out.s").output().unwrap();
    }
}
