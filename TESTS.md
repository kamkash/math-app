# Tests

```bash
cargo test -p interpreter --test latex_gen_interpreter_tests -- --test-threads=1
cargo test --package math-parser -lib -- --nocapture
```

giac-tauri

```bash
MACOSX_DEPLOYMENT_TARGET=15.5 cargo  build
MACOSX_DEPLOYMENT_TARGET=15.5 cargo  test
```
