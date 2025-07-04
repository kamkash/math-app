// File: build.rs

use std::env;
use std::path::PathBuf;

fn main() {
    // Path to GIAC source (update as needed)
    let giac_src = "/Users/kamran/mathappws/giac";
    let giac_include = format!("{}/src/giac/headers", giac_src);

    let gmp_include = "/opt/homebrew/include"; // or wherever `brew install gmp mpfr` puts headers
    let gmp_lib = "/opt/homebrew/lib";

    // Compile the wrapper
    cc::Build::new()
        .cpp(true)
        .file("giac-wrapper.cpp")
        .flag_if_supported("-std=c++17")
        .flag("-w") // Suppress warnings
        .flag("-mmacosx-version-min=15.5") 
        .include(&giac_include)
        .include(&gmp_include)
        .compile("giacwrapper");

    // Tell cargo to link to GIAC + dependencies
    println!("cargo:rustc-link-search=native={}/build", giac_src);
    println!("cargo:rustc-link-search=native={}", gmp_lib);

    println!("cargo:rustc-link-lib=static=giac"); // assumes libgiac.a already built
    println!("cargo:rustc-link-lib=dylib=mpfr");
    println!("cargo:rustc-link-lib=dylib=gmp");

    // Ensure re-run if wrapper changes
    println!("cargo:rerun-if-changed=giac-wrapper.cpp");
    println!("cargo:rerun-if-changed=giac-wrapper.h");

    #[cfg(target_os = "linux")]
    let bindings = bindgen::Builder::default()
        .header("giac-wrapper.h")
        .clang_arg("-I/usr/include")
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I/usr/include/x86_64-linux-gnu")
        .generate()
        .expect("Unable to generate bindings");

    #[cfg(target_os = "macos")]
    let bindings = bindgen::Builder::default()
        .header("giac-wrapper.h")
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
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");

    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=giac");
    println!("cargo:rustc-link-lib=gmp");
    println!("cargo:rustc-link-lib=mpfr");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");
}
