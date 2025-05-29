use lazy_static::lazy_static;
use libloading::{Library, Symbol};
use log::{debug, error, info, warn};
use interpreter::asciimath_basic_string_interpreter;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::{c_char, c_int};
use std::path::PathBuf;
use std::ptr;
use std::str;
use tauri_plugin_log::Target;
use tauri_plugin_log::TargetKind;

const LOG_FILE_NAME: &str = "mathapp";
// pub const MODEL_FILE_NAME: &str = "DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf";
pub const MODEL_FILE_NAME: &str = "gemma-3-4b-it-Q4_K_M.gguf";
const NGL: i32 = 99;
const _SAMPLE_LIST_GRAMMAR: &str = r#"root ::= (\"- \" item)+
                                            item ::= [^\n]+ \"\n\""#;

#[cfg(target_os = "linux")]
lazy_static! {
    static ref GGML_BASE_LIB: (PathBuf, Library) = load_library("libggml-base.so");
    static ref GGML_LIB: (PathBuf, Library) = load_library("libggml.so");
    static ref LLAMA_LIB: (PathBuf, Library) = load_library("libllama.so");
    static ref MATHAPP_LIB: (PathBuf, Library) = load_library("libmathapp.so");
}

#[cfg(target_os = "macos")]
lazy_static! {
    static ref GGML_BASE_LIB: (PathBuf, Library) = load_library("libggml-base.dylib");
    static ref GGML_LIB: (PathBuf, Library) = load_library("libggml.dylib");
    static ref LLAMA_LIB: (PathBuf, Library) = load_library("libllama.dylib");
    static ref MATHAPP_LIB: (PathBuf, Library) = load_library("libmathapp.dylib");
}

fn load_library(lib_name: &str) -> (PathBuf, Library) {
    let lib_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("libs")
        .join(lib_name);
    let library =
        unsafe { Library::new(&lib_path).expect(&format!("Failed to load library: {}", lib_name)) };
    (lib_path, library)
}

pub fn model_path() -> String {
    let model_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets")
        .join("models")
        .join(MODEL_FILE_NAME);
    model_path.to_str().unwrap().to_string()
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    info!("Greeting {}", name);
    warn!("Greeting {}", name);
    error!("Greeting {}", name);

    let res = echo_rust(name).unwrap_or_else(|e| {
        error!("Failed to echo text: {}", e);
        "Failed to echo text".to_string()
    });
    format!("Hello, {}! Wassup? You've been greeted from Rust!", res)
}

#[tauri::command]
fn reset_context(topic: &str) -> () {
    info!("Resetting context");
    let res = rest_context_rust(topic).unwrap_or_else(|e| {
        error!("Failed to reset context: {}", e);
        0
    });
    info!("math app init {}", res);
}

#[tauri::command]
fn reset_model(name: &str) -> () {
    info!("Resetting model {}", name);
    let res = init_math_app_rust(NGL).unwrap_or_else(|e| {
        error!("Failed to reset model: {}", e);
        0
    });
    info!("math app init {}", res);
}

#[tauri::command]
fn llm_generate(prompt: &str) -> String {
    const N_PREDICT: i32 = 4096;
    let res = generate_text_rust(prompt, N_PREDICT).unwrap_or_else(|e| {
        error!("Failed to generate text: {}", e);
        "Failed to generate text".to_string()
    });
    format!("{}", res)
}

#[tauri::command]
fn run_solver(input: &str) -> String {
    let res = run_solver_rust(input).unwrap_or_else(|e| {
        error!("Failed to run solver: {}", e);
        "Failed to run solver".to_string()
    });
    format!("{}", res)
}

#[tauri::command]
fn add_grammar(grammar: &str) -> String {
    let res = add_grammar_rust(grammar).unwrap_or_else(|e| {
        error!("Failed to add grammar: {}", e);
        1
    });
    format!("{}", res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_target_dir: Target = Target::new(TargetKind::LogDir {
        file_name: Some(LOG_FILE_NAME.into()),
    });
    let log_target_stdout: Target = Target::new(TargetKind::Stdout);
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Debug)
                .targets(vec![log_target_dir, log_target_stdout])
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .setup(move |_app| {
            debug!("loading: {:?}", GGML_BASE_LIB.0);
            debug!("loading: {:?}", GGML_LIB.0);
            debug!("loading: {:?}", LLAMA_LIB.0);
            debug!("loading: {:?}", MATHAPP_LIB.0);
            let res = init_math_app_rust(NGL).unwrap_or_else(|e| {
                error!("Failed to initialize mathapp: {}", e);
                0
            });
            info!("math app init {}", res);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            reset_context,
            reset_model,
            llm_generate,
            run_solver,
            greet,
            add_grammar
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//
//////////////////// FFI functions //////////////////////
//
pub fn init_math_app_rust(ngl: i32) -> Result<i32, String> {
    let c_model_path = CString::new(model_path()).map_err(|e| e.to_string())?;
    unsafe {
        let func: Symbol<unsafe extern "C" fn(*const c_char, c_int) -> c_int> = MATHAPP_LIB
            .1
            .get(b"init\0")
            .expect("Failed to load init_math_app");
        let result = func(c_model_path.as_ptr() as *const c_char, ngl);
        if result == 0 {
            return Err("Failed to initialize mathapp".to_string());
        }
        Ok(result)
    }
}

pub fn echo_rust(estr: &str) -> Result<String, String> {
    info!("math-parser echo: {}", math_parser::echo_parser(estr));

    unsafe {
        let func: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> =
            MATHAPP_LIB.1.get(b"echo\0").map_err(|e| e.to_string())?;

        let result_ptr = func(estr.as_ptr() as *const c_char);
        if result_ptr == ptr::null() {
            return Err("Failed to echo".to_string());
        }
        let c_str = CStr::from_ptr(result_ptr);
        c_str
            .to_str()
            .map(|s| s.to_string())
            .map_err(|e| e.to_string())
    }
}

// fixme:: error handling
pub fn run_solver_rust(prompt: &str) -> Result<String, String> {
    let res = asciimath_basic_string_interpreter::evaluate_ascii_math_block(prompt)?;
    Ok(res)
}

pub fn generate_text_rust(prompt: &str, n_predict: i32) -> Result<String, String> {
    let c_prompt = CString::new(prompt).map_err(|e| e.to_string())?;

    unsafe {
        let func: Symbol<unsafe extern "C" fn(*const c_char, c_int) -> *const c_char> = MATHAPP_LIB
            .1
            .get(b"generate_text\0")
            .map_err(|e| e.to_string())?;

        let result_ptr = func(c_prompt.as_ptr(), n_predict);
        if result_ptr == ptr::null() {
            return Err("Failed to generate text".to_string());
        }
        let c_str = CStr::from_ptr(result_ptr);
        c_str
            .to_str()
            .map(|s| s.to_string())
            .map_err(|e| e.to_string())
    }
}

pub fn add_grammar_rust(grammar: &str) -> Result<i32, String> {
    unsafe {
        let func: Symbol<unsafe extern "C" fn(*const c_char) -> c_int> = MATHAPP_LIB
            .1
            .get(b"add_grammar\0")
            .map_err(|e| e.to_string())?;

        let c_grammar = CString::new(grammar).map_err(|e| e.to_string())?;
        let result = func(c_grammar.as_ptr() as *const c_char);
        if result == 0 {
            return Err("Failed to add grammar".to_string());
        }
        Ok(result)
    }
}

pub fn rest_context_rust(topic: &str) -> Result<i32, String> {
    unsafe {
        let func: Symbol<unsafe extern "C" fn(*const c_char) -> c_int> = MATHAPP_LIB
            .1
            .get(b"reset_context\0")
            .map_err(|e| e.to_string())?;

        let result = func(topic.as_ptr() as *const c_char);
        if result == 0 {
            return Err("Failed to reset context".to_string());
        }
        Ok(result)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////
