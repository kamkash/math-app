// use std::{env, path::PathBuf};

fn main() {
    tauri_build::build();


    // let home = env::var("HOME").unwrap();
    // let llama_cpp_path = PathBuf::from(&home).join("llama.cpp");
    // let ggml_path = llama_cpp_path.join("ggml").join("include");
    // let llama_h_path = llama_cpp_path.join("include").join("llama.h");

    // // Link with llama.cpp library
    // println!("cargo:rustc-link-search={}", llama_cpp_path.join("build/bin").display());
    // println!("cargo:rustc-link-lib=dylib=llama");

    // // Tell cargo to invalidate the built crate whenever the header changes
    // println!("cargo:rerun-if-changed={}", llama_cpp_path.join("llama.h").display());

    // // Generate bindings
    // let bindings = bindgen::Builder::default()
    //     .clang_arg("-I".to_owned() + ggml_path.to_str().unwrap())
    //     .header(llama_h_path.to_str().unwrap())
    //     .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    //     .generate()
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}