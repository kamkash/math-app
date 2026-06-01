# Notes

The project uses a GIAC FFI which is not fully thread-safe. For deterministic
test runs, when tests exercise the GIAC FFI, run the test suite single-threaded.

    ```bash
    MACOSX_DEPLOYMENT_TARGET=15.5 cargo  build
    MACOSX_DEPLOYMENT_TARGET=15.5 cargo  test
    cargo test -- --test-threads=1
    RUST_TEST_THREADS=1 cargo test --workspace
    RUST_LOG=debug cargo test -- --nocapture
    ```
