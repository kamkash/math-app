extern crate libc;

use libc::{c_char, c_int};
use std::ffi::{CStr, CString};
use std::ptr;

extern "C" {
    fn generate_text(model_path: *const c_char, prompt: *const c_char, n_predict: c_int, ngl: c_int) -> *const c_char;
}

pub fn generate_text_rust(model_path: &str, prompt: &str, n_predict: i32, ngl: i32) -> Result<String, String> {
    let c_model_path = CString::new(model_path).map_err(|e| e.to_string())?;
    let c_prompt = CString::new(prompt).map_err(|e| e.to_string())?;

    unsafe {
        let result_ptr = generate_text(c_model_path.as_ptr(), c_prompt.as_ptr(), n_predict, ngl);
        if result_ptr == ptr::null() {
            return Err("Failed to generate text".to_string());
        }
        let c_str = CStr::from_ptr(result_ptr);
        c_str.to_str().map(|s| s.to_string()).map_err(|e| e.to_string())
    }
}
