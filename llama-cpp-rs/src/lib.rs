#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code)]

use std::ffi::{CString};
use std::path::Path;

pub mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

/// A small sample wrapper that demonstrates calling llama.cpp through the generated bindings.
pub fn generate_text(model_path: &Path, prompt: &str, n_predict: Option<i32>, n_gpu_layers: Option<i32>) -> Result<String, String> {
    let model_path_c = CString::new(model_path.to_string_lossy().as_ref())
        .map_err(|err| format!("model path conversion failed: {err}"))?;
    let prompt_c = CString::new(prompt).map_err(|err| format!("prompt conversion failed: {err}"))?;

    let n_predict = n_predict.unwrap_or(32);
    let n_gpu_layers = n_gpu_layers.unwrap_or(99);

    unsafe {
        let mut model_params = sys::llama_model_default_params();
        model_params.n_gpu_layers = n_gpu_layers;

        let model = sys::llama_model_load_from_file(model_path_c.as_ptr(), model_params);
        if model.is_null() {
            return Err("llama_model_load_from_file returned null".into());
        }

        let vocab = sys::llama_model_get_vocab(model);
        if vocab.is_null() {
            sys::llama_model_free(model);
            return Err("llama_model_get_vocab returned null".into());
        }

        // Optimization: Single-pass tokenization. 
        // Most prompts result in fewer tokens than characters.
        let mut prompt_tokens = vec![0 as sys::llama_token; prompt.len() + 4];
        let mut n_prompt = sys::llama_tokenize(
            vocab,
            prompt_c.as_ptr(),
            prompt.len() as i32,
            prompt_tokens.as_mut_ptr(),
            prompt_tokens.len() as i32,
            true,
            true,
        );

        if n_prompt < 0 {
            // If buffer was too small, n_prompt is the negative of required size
            let required_size = -n_prompt;
            prompt_tokens.resize(required_size as usize, 0);
            n_prompt = sys::llama_tokenize(vocab, prompt_c.as_ptr(), prompt.len() as i32, prompt_tokens.as_mut_ptr(), required_size, true, true);
        }

        let mut ctx_params = sys::llama_context_default_params();
        ctx_params.n_ctx = (n_prompt + n_predict) as u32; // Total context size is prompt tokens + predicted tokens
        ctx_params.n_batch = n_prompt as u32; // Process the entire prompt in one batch for efficiency

        let ctx = sys::llama_init_from_model(model, ctx_params);
        if ctx.is_null() {
            sys::llama_model_free(model);
            return Err("llama_init_from_model returned null".into());
        }

        let sampler = sys::llama_sampler_chain_init(sys::llama_sampler_chain_default_params());
        if sampler.is_null() {
            sys::llama_free(ctx);
            sys::llama_model_free(model);
            return Err("llama_sampler_chain_init returned null".into());
        }

        let top_k = sys::llama_sampler_init_top_k(40);
        let top_p = sys::llama_sampler_init_top_p(0.95, 1);
        let temp = sys::llama_sampler_init_temp(0.8);
        let dist = sys::llama_sampler_init_dist(sys::LLAMA_DEFAULT_SEED);

        if top_k.is_null() || top_p.is_null() || temp.is_null() || dist.is_null() {
            sys::llama_sampler_free(sampler);
            sys::llama_free(ctx);
            sys::llama_model_free(model);
            return Err("failed to initialize sampler chain".into());
        }

        sys::llama_sampler_chain_add(sampler, top_k);
        sys::llama_sampler_chain_add(sampler, top_p);
        sys::llama_sampler_chain_add(sampler, temp);
        sys::llama_sampler_chain_add(sampler, dist);
        sys::llama_sampler_chain_add(sampler, sys::llama_sampler_init_greedy());

        // Optimization: Pre-allocate string capacity to avoid reallocations during generation
        let mut result_text = String::with_capacity((n_predict * 4) as usize);
        let mut batch = sys::llama_batch_get_one(prompt_tokens.as_mut_ptr(), n_prompt);

        // Optimization: Use a stack-allocated buffer for pieces instead of a heap-allocated Vec
        let mut piece_buf = [0i8; 128];

        for _ in 0..n_predict {
            let decode_result = sys::llama_decode(ctx, batch);
            if decode_result != 0 {
                break;
            }

            let mut token_id = sys::llama_sampler_sample(sampler, ctx, -1);
            if sys::llama_vocab_is_eog(vocab, token_id) {
                break;
            }

            let piece_len = sys::llama_token_to_piece(
                vocab,
                token_id,
                piece_buf.as_mut_ptr(),
                piece_buf.len() as i32,
                0,
                true,
            );

            if piece_len > 0 {
                // Convert i8 buffer slice to u8 slice for UTF-8 conversion
                let piece_slice = std::slice::from_raw_parts(piece_buf.as_ptr() as *const u8, piece_len as usize);
                result_text.push_str(&String::from_utf8_lossy(piece_slice));
            }

            batch = sys::llama_batch_get_one(&mut token_id, 1);
        }

        sys::llama_sampler_free(sampler);
        sys::llama_free(ctx);
        sys::llama_model_free(model);

        Ok(result_text)
    }
}
