use tauri_plugin_log::Target;
use tauri_plugin_log::TargetKind;
use log::{ info, warn, error, debug };
use tauri::Manager;
use libloading::{ Library, Symbol };
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::{ c_char, c_int };
use std::path::PathBuf;
use std::ptr;
use std::str;
use lazy_static::lazy_static;

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
    let library = unsafe {
        Library::new(&lib_path).expect(&format!("Failed to load library: {}", lib_name))
    };
    (lib_path, library)
}

unsafe extern "C" {
    fn generate_text(
        model_path: *const c_char,
        prompt: *const c_char,
        n_predict: c_int,
        ngl: c_int
    ) -> *const c_char;
}

const LOG_FILE_NAME: &str = "mathapp";

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    info!("Greeting {}", name);
    warn!("Greeting {}", name);
    error!("Greeting {}", name);

    let res = echo_rust(name).unwrap_or_else(|e| {
        error!("Failed to generate text: {}", e);
        "Failed to generate text".to_string()
    });
    format!("Hello, {}! Wassup? You've been greeted from Rust!", res)
}

#[tauri::command]
fn generate(prompt: &str) -> String {
    prompt.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_target_dir: Target = Target::new(TargetKind::LogDir {
        file_name: Some(LOG_FILE_NAME.into()),
    });
    let log_target_stdout: Target = Target::new(TargetKind::Stdout);
    tauri::Builder
        ::default()
        .plugin(
            tauri_plugin_log::Builder
                ::default()
                .targets(vec![log_target_dir, log_target_stdout])
                .build()
        )
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            debug!("loading: {:?}", GGML_BASE_LIB.0);
            debug!("loading: {:?}", GGML_LIB.0);
            debug!("loading: {:?}", LLAMA_LIB.0);
            debug!("loading: {:?}", MATHAPP_LIB.0);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, generate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn echo_rust(estr: &str) -> Result<String, String> {
    unsafe {
        let func: Symbol<unsafe extern "C" fn(*const c_char) -> *const c_char> = MATHAPP_LIB.1.get(
            b"echo\0"
        ).map_err(|e| e.to_string())?;

        let result_ptr = func(estr.as_ptr() as *const c_char);
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

pub fn generate_text_rust(
    model_path: &str,
    prompt: &str,
    n_predict: i32,
    ngl: i32
) -> Result<String, String> {
    let c_model_path = CString::new(model_path).map_err(|e| e.to_string())?;
    let c_prompt = CString::new(prompt).map_err(|e| e.to_string())?;

    unsafe {
        let result_ptr = generate_text(c_model_path.as_ptr(), c_prompt.as_ptr(), n_predict, ngl);
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
