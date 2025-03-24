# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)



## WSL
- export DISPLAY=:0
- npm run tauri dev
- npm run tauri build


## DeepSeek Math
- https://github.com/deepseek-ai/DeepSeek-Math


## llama.cpp
./run.sh -n 2048 "what is 25% of 256.00. IMP: give the output in a valid JSON string"
./run.sh -n 2048 "solve x^2 - 2x + 9. IMP: give the output in a valid JSON string" 

### run.sh
```
#!/bin/bash
MODEL_FILE=/Users/kamran/.lmstudio/models/lmstudio-community/DeepSeek-R1-Distill-Qwen-7B-GGUF/DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf
LLAMA_BUILD_PATH=/Users/kamran/llama.cpp/build/bin
DYLD_LIBRARY_PATH=.:$DYLD_LIBRARY_PATH:$LLAMA_BUILD_PATH ./target/main -m $MODEL_FILE $*
```


