#!/bin/bash
MODEL_FILE=/Users/kamran/.lmstudio/models/lmstudio-community/gemma-3-4b-it-GGUF/gemma-3-4b-it-Q4_K_M.gguf
# MODEL_FILE=/Users/kamran/.lmstudio/models/lmstudio-community/gemma-3-1b-it-GGUF/gemma-3-1b-it-Q4_K_M.gguf
# MODEL_FILE=/Users/kamran/.lmstudio/models/lmstudio-community/gemma-3-27b-it-GGUF/gemma-3-27b-it-Q4_K_M.gguf
# MODEL_FILE=/Users/kamran/mathappws/math-app/src-tauri/assets/models/Qwen2-Math-1.5B-Instruct-GGUF/Qwen2-Math-1.5B-Instruct-Q4_K_M.gguf
# MODEL_FILE=/Users/kamran/.lmstudio/models/lmstudio-community/DeepSeek-R1-Distill-Qwen-7B-GGUF/DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf
# MODEL_FILE=/Users/kamran/.lmstudio/models/lmstudio-community/Phi-4-mini-instruct-GGUF/Phi-4-mini-instruct-Q4_K_M.gguf
# MODEL_FILE=/Users/kamran/.lmstudio/models/lmstudio-community/Mistral-Small-3.1-24B-Instruct-2503-GGUF/Mistral-Small-3.1-24B-Instruct-2503-Q3_K_L.gguf
# MODEL_FILE=/Users/kamran/.lmstudio/models/mradermacher/Anthropic_RLFH_ORDP_40k-GGUF/Anthropic_RLFH_ORDP_40k.Q4_K_S.gguf
# MODEL_FILE=/Users/kamran/.lmstudio/models/QuantFactory/deepseek-math-7b-instruct-GGUF/deepseek-math-7b-instruct.Q4_K_S.gguf
# LLAMA_BUILD_PATH=/Users/kamran/llama.cpp/build/bin
LLAMA_BUILD_PATH=/Users/kamran/mathappws/math-app/src-tauri/assets/libs
# DYLD_LIBRARY_PATH=.:$DYLD_LIBRARY_PATH:$LLAMA_BUILD_PATH ./target/main -m $MODEL_FILE $*
DYLD_LIBRARY_PATH=.:$DYLD_LIBRARY_PATH:$LLAMA_BUILD_PATH ./target/main -n 2048 -m $MODEL_FILE `cat <<EOF
You are a helpful assistant that extracts a block of math LaTex between $$ and $$ from input and outputs only valid JSON describing a function call.
Available function:
solve_math_block(argument: string) -> string

User: Hello assistant. Solve this: $$ \displaylines{x = 10 \\ y = x^2 + 5x + 6 \\ z = x^{2} \\ rat2 = \\frac{2*x}{4*x}} $$
Output JSON only, like:
{"name": "solve_math_block", "arguments": "latex block here"}
EOF
`

