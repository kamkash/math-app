// use log::info;
// use llama_cpp::{ LlamaModel, LlamaParams, SessionParams };
// use llama_cpp::standard_sampler::StandardSampler;

// use llm_chain::executor;
// use llm_chain::{ parameters, prompt };
// use llm_chain::options::*;
// use llm_chain::options;

// pub async fn run_llama_executor() -> Result<(), Box<dyn std::error::Error>> {
//     let opts =
//         options!(
//         Model: ModelRef::from_path(crate::model_path()),
//         ModelType: "llama",
//         MaxContextSize: 512_usize,
//         NThreads: 4_usize,
//         MaxTokens: 0_usize,
//         TopK: 40_i32,
//         TopP: 0.95
//     );

//     let exec = executor!(llama, opts)?;

//     let res = prompt!("You are a helpful assistant", "What is the capital of France?").run(
//         &parameters!(),
//         &exec
//     ).await?;

//     info!("{}", res);

//     Ok(())
// }

// pub fn run_llama_cpp() -> Result<(), Box<dyn std::error::Error>> {
//     info!("run_llama_cpp");
//     // Create a model from anything that implements `AsRef<Path>`:
//     let model = LlamaModel::load_from_file(crate::model_path(), LlamaParams::default()).expect(
//         "Could not load model"
//     );

//     // A `LlamaModel` holds the weights shared across many _sessions_; while your model may be
//     // several gigabytes large, a session is typically a few dozen to a hundred megabytes!
//     let mut ctx = model.create_session(SessionParams::default()).expect("Failed to create session");

//     // You can feed anything that implements `AsRef<[u8]>` into the model's context.
//     ctx.advance_context("This is the story of a man named Stanley.").unwrap();

//     // LLMs are typically used to predict the next word in a sequence. Let's generate some tokens!
//     let max_tokens = 1024;
//     let mut decoded_tokens = 0;

//     // `ctx.start_completing_with` creates a worker thread that generates tokens. When the completion
//     // handle is dropped, tokens stop generating!

//     let completions = ctx.start_completing_with(StandardSampler::default(), 1024)?.into_strings();

//     Ok(for completion in completions {
//         info!("{completion}");
//         // let _ = io::stdout().flush();

//         decoded_tokens += 1;

//         if decoded_tokens > max_tokens {
//             break;
//         }
//     })
// }
