//get the bindings for the vSendString function
//generate static library for the hook
//and throw out bindings for the static library

use std::path::PathBuf;

fn main() {
    //give path to the header file to bindgen

    let mut builder = bindgen::Builder::default()
        .use_core()
        .ctypes_prefix("cty")
        .header("hook.h")
        .clang_arg("-I../")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        //.whitelist_function("vSendString");
        .allowlist_function("vSendString");

    builder = builder.clang_arg("--target=riscv64-unknown-none-elf");
    /*if cfg!(target_arch = "riscv64gc-unknown-none-elf") {
        builder = builder.clang_arg("--target=riscv64-unknown-none-elf");
    }*/

    let bindings = builder
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("hook.rs"))
        .expect("Couldn't write bindings!");

    //link with the static library
     println!("cargo:rustc-link-search=../lib/");
    println!("cargo:rustc-link-arg=-Wl,-whole-archive");
    println!("cargo:rustc-link-lib=RTOSDemo");
}