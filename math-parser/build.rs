use std::env;
use std::error::Error;
use std::process::Command;

fn main() {
    let grammars = vec!["calculator"];
    let additional_args = vec![Some("-visitor"); grammars.len()];
    let antlr_file = "antlr4-4.8-2-SNAPSHOT-complete.jar";

    let path_buf = env::current_dir().unwrap();
    let antlr_path = path_buf.join("tools").join("antlr-libs").join(antlr_file);

    for (grammar, arg) in grammars.into_iter().zip(additional_args) {
        //ignoring error because we do not need to run anything when deploying to crates.io
        let _ = gen_for_grammar(
            grammar,
            antlr_path.to_str().expect("Invalid UTF-8 in antlr_path"),
            arg,
        );
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=/Users/kamran/mathappws/math-app/math-parser/tools/antlr-libs/antlr4-4.8-2-SNAPSHOT-complete.jar");
}

fn gen_for_grammar(
    grammar_file_name: &str,
    antlr_path: &str,
    additional_arg: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let dest_path = Path::new(&out_dir);

    let input = env::current_dir().unwrap().join("tools").join("grammars");
    let file_name = grammar_file_name.to_owned() + ".g4";

    let _ = Command::new("java")
        .current_dir(input)
        .arg("-cp")
        .arg(antlr_path)
        .arg("org.antlr.v4.Tool")
        .arg("-Dlanguage=Rust")
        .arg("-o")
        .arg("../../tests/gen")
        .arg(&file_name)
        .args(additional_arg)
        .spawn()
        .expect("antlr tool failed to start")
        .wait_with_output()?;
    // .unwrap()
    // .stdout;
    // eprintln!("xx{}",String::from_utf8(x).unwrap());

    println!("cargo:rerun-if-changed=tools/grammars/{}", file_name);
    Ok(())
}
