use std::env;
use std::path::PathBuf;

fn main() {
    let default_root = "/Users/kamran/mathappws/llama.cpp";
    let llama_cpp_root = env::var("LLAMA_CPP_ROOT").unwrap_or_else(|_| default_root.to_string());
    let llama_cpp_include = env::var("LLAMA_CPP_INCLUDE")
        .unwrap_or_else(|_| format!("{}/include", llama_cpp_root));
    let llama_cpp_ggml = format!("{}/ggml/include", llama_cpp_root);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", llama_cpp_include))
        .clang_arg(format!("-I{}", llama_cpp_ggml))
        .clang_arg("-DLLAMA_SHARED")
        .derive_debug(true)
        .derive_default(true)
        .generate()
        .expect("Unable to generate llama bindings");

    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed={}", llama_cpp_include);
    println!("cargo:rerun-if-env-changed=LLAMA_CPP_INCLUDE");
    println!("cargo:rerun-if-env-changed=LLAMA_CPP_ROOT");

    let default_link_path = format!("{}/build-cpu/bin", llama_cpp_root);
    println!("cargo:rustc-link-search=native={}", default_link_path);
    println!("cargo:rustc-link-lib=dylib=llama");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", default_link_path);
}
