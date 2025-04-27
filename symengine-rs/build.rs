// File: build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");


#[cfg(target_os = "linux")]
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include")
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I/usr/include/x86_64-linux-gnu")
        .generate()
        .expect("Unable to generate bindings");

#[cfg(target_os = "macos")]
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/include")
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I/opt/homebrew/opt/gmp/include")
        .generate()
        .expect("Unable to generate bindings");


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    
#[cfg(target_os = "macos")]
    println!("cargo:rustc-link-search=native=/opt/homebrew/opt/gmp/lib");

#[cfg(target_os = "linux")]
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");

    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=symengine");
    println!("cargo:rustc-link-lib=gmp");
    
#[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

#[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");
}
