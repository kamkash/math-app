use std::path::Path;

use llama_cpp_rs::generate_text;
use log::info;
use test_log;

#[test_log::test]
fn generate_text_returns_error_for_missing_model() {
    let model_path = Path::new("/tmp/nonexistent-llama-model.gguf");
    let result = generate_text(model_path, "Hello from llama-cpp-rs", None, None);

    assert!(result.is_err(), "generate_text should return an error when the model path is invalid");
}

#[test_log::test]
fn generate_text_returns_text_for_valid_mode() {
    let model_path = Path::new("/Volumes/ExFAT512/lmstudio_models/lmstudio-community/gemma-4-31B-it-GGUF/gemma-4-31B-it-Q4_K_M.gguf");
    // In a real test, you would set up a valid model file at the specified path before running this test
    let result = generate_text(model_path, "Hello from llama-cpp-rs. solve x = 12 * 12", Some(2048), None);

    assert!(result.is_ok(), "generate_text should return Ok when the model path is valid");
    let generated_text = result.unwrap();
    info!("response: {}", generated_text);
    assert!(!generated_text.is_empty(), "Generated text should not be empty");
    assert!(generated_text.contains("144"), "Generated text should contain the correct answer to the math problem");
}

#[test_log::test]
fn generate_text_returns_latex_text_for_valid_mode() {
    let model_path = Path::new("/Volumes/ExFAT512/lmstudio_models/lmstudio-community/gemma-4-31B-it-GGUF/gemma-4-31B-it-Q4_K_M.gguf");
    // In a real test, you would set up a valid model file at the specified path before running this test
    let result = generate_text(model_path, "solve \\int x^3dx. Answer in Latex format", Some(2048), None);

    assert!(result.is_ok(), "generate_text should return Ok when the model path is valid");
    let generated_text = result.unwrap();
    info!("response: {}", generated_text);
    assert!(!generated_text.is_empty(), "Generated text should not be empty");
    assert!(generated_text.contains("frac{x^4}{4}"), "Generated text should contain the correct answer to the math problem");
}
