#!/bin/bash
MODEL_FILE=/Volumes/ExFAT512/lmstudio_models/lmstudio-community/gemma-4-31B-it-GGUF/gemma-4-31B-it-Q4_K_M.gguf
LLAMA_BUILD_PATH=/Users/kamran/mathappws/math-app/src-tauri/assets/libs
# DYLD_LIBRARY_PATH=.:$DYLD_LIBRARY_PATH:$LLAMA_BUILD_PATH ./target/main -m $MODEL_FILE $*
# DYLD_LIBRARY_PATH=.:$DYLD_LIBRARY_PATH:$LLAMA_BUILD_PATH ./target/main -n 2048 -ngl 99 -m $MODEL_FILE `cat <<EOF
# You are a helpful assistant that extracts a block of math LaTex between $$ and $$ from input and outputs only valid JSON describing a function call.
# Available function:
# solve_math_block(argument: string) -> string

# User: Hello assistant. Solve this: $$ \displaylines{x = 10 \\ y = x^2 + 5x + 6 \\ z = x^{2} \\ rat2 = \\frac{2*x}{4*x}} $$
# Output JSON only, like:
# {"name": "solve_math_block", "arguments": "latex block here"}
# EOF
#`
DYLD_LIBRARY_PATH=.:$DYLD_LIBRARY_PATH:$LLAMA_BUILD_PATH ./target/main -n 2048 -ngl 99 -m $MODEL_FILE `cat <<EOF
Evaluate y and z for the following equations:
x = 10
y = x^2 + 5x + 6
z = x^2
EOF
`

