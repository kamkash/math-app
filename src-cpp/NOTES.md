# dylib loading

```bash
$ otool -l libllama.dylib
```

The crash is not because `src-tauri/assets/libs/libllama.dylib` is missing. It is there, and your log shows your Rust code loads it first.

The failure is macOS `dyld` resolving `libmathapp.dylib`’s own dependencies:

```text
libmathapp.dylib depends on @rpath/libllama.dylib
dyld tried target/debug, deps, rustup lib, ~/lib, /usr/local/lib, /usr/lib
dyld did not try src-tauri/assets/libs
```

From `otool`, `libmathapp.dylib` contains:

```text
@rpath/libllama.dylib
@rpath/libggml.dylib
@rpath/libggml-base.dylib
```

But it has no `LC_RPATH` pointing at `src-tauri/assets/libs`. Loading `libllama.dylib` by absolute path in Rust does not add that directory to `@rpath`.

There is also a naming mismatch: your bundled `libllama.dylib` identifies itself as:

```text
@rpath/libllama.0.dylib
```

while `libmathapp.dylib` asks for:

```text
@rpath/libllama.dylib
```

So even preloading may not satisfy dyld’s dependency lookup reliably.

The clean fix is to make the bundled dylibs self-relative. Since they all live in the same folder, patch or build them so dependencies use `@loader_path`:

```bash
install_name_tool -id @loader_path/libmathapp.dylib src-tauri/assets/libs/libmathapp.dylib
install_name_tool -change @rpath/libllama.dylib @loader_path/libllama.dylib src-tauri/assets/libs/libmathapp.dylib
install_name_tool -change @rpath/libggml.dylib @loader_path/libggml.dylib src-tauri/assets/libs/libmathapp.dylib
install_name_tool -change @rpath/libggml-base.dylib @loader_path/libggml-base.dylib src-tauri/assets/libs/libmathapp.dylib
```

You likely also need to patch `libllama.dylib`’s ggml dependencies, because it asks for versioned names like `@rpath/libggml.0.dylib` while your files are named `libggml.dylib`, `libggml-cpu.dylib`, etc.

In short: your Rust path is correct, but the embedded Mach-O dependency paths inside the dylibs are not relocatable to `assets/libs`. Use `@loader_path` or add matching rpaths/install names during the CMake/build step.



The short answer is **yes, it is generally best practice to remove the `build/` folder**, especially if you are switching between different hardware backends or if you’ve recently updated the source code.

While build systems like CMake are designed to be "incremental" (meaning they only recompile what has changed), they aren’t always perfect. Here is why and when you should clear it out:

### 1. When you MUST remove it
You should definitely delete the folder (or run `rm -rf build`) if you:
* **Change Backends:** If you previously built for **CPU-only** and now want to build with **CUDA**, **Metal**, or **Vulkan**. Residual configuration files can cause the build to fail or result in a binary that doesn't actually use your GPU.
* **Pull a Major Update:** `llama.cpp` moves fast. If the project structure or dependencies change significantly after a `git pull`, the old cache might point to files that no longer exist.
* **Encounter Weird Errors:** If you see "undefined reference" or "missing symbol" errors during compilation, a stale cache is usually the culprit.

---

### 2. When you can skip it
If you are just making small tweaks to the code or recompiling the same version with the same settings, you can leave it. CMake will:
* Only compile the specific files you modified.
* Save you several minutes of "clean" compile time.

---

### 3. The "Pro" Way (Out-of-Source Builds)
Instead of deleting and recreating, many developers use different folders for different configurations to avoid constant deleting:

```bash
# For CUDA build
cmake -B build-cuda -DGGML_CUDA=ON
cmake --build build-cuda --config Release
cmake --build build-cuda --config Release -j 8

# For CPU build
cmake -B build-cpu
cmake --build build-cpu --config Release
cmake --build build-cpu --config Release -j 8
```

### Summary Recommendation
If in doubt, **nuke it.** A clean build takes a bit longer but guarantees that your binary is using the exact flags and optimizations you intended. If you are using the standard `make` command instead of CMake, simply run `make clean` before running `make` again.

Are you trying to enable a specific GPU acceleration (like CUDA or Metal) for this build?