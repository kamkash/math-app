Using **OpenCV (`cv::text` or `cv::dnn`)** for this specific application architecture is a classic "square peg, round hole" trap. While OpenCV is the undisputed king of traditional desktop computer vision, introducing it into a cross-platform **Tauri + Rust + Mobile** stack to solve **handwritten math OCR** will create severe architectural friction.

Here is a brutal breakdown of why OpenCV will fight you every step of the way, followed by the elegant, modern alternatives that fit your existing Rust/`llama.cpp` ecosystem perfectly.

---

## 1. Why `cv::text` is a Dead End for Math

The `cv::text` module is built for standard, linear, left-to-right textual layouts (like street signs or scanned book paragraphs). It is completely blind to the spatial, multi-dimensional nature of mathematics.

* It cannot parse superscripts ($x^2$), subscripts ($x_n$), fractions ($\frac{a}{b}$), matrices, or square roots.
* It relies heavily on Tesseract or basic stroke-width transform algorithms under the hood, both of which hallucinate wildly when fed handwritten mathematical symbols like $\int$, $\sum$, or $\theta$.

## 2. Why `cv::dnn` is Viable but a Cross-Compilation Nightmare

You *can* find specialized open-source Math OCR transformer models (like `pix2tex` / LaTeX-OCR or UniMERNet), export them to ONNX, and run them through OpenCV’s `cv::dnn` module. However, the operational cost is massive:

* **The Compilation Hell:** Compiling the C++ OpenCV library with `cv::dnn` enabled for **five different targets** (`x86_64` Windows, `x86_64`/`arm64` macOS, `arm64-v8a` Android, and `arm64` iOS) inside a Cargo build pipeline using the Rust `opencv` crate is a notorious recipe for configuration despair. Linker errors and toolchain mismatches across mobile sandboxes will drain your development velocity.
* **Architecture Mismatch:** Math OCR requires Vision Transformers (ViTs) or autoregressive sequence-to-sequence tokens. OpenCV’s `cv::dnn` is historically optimized for standard CNNs (like YOLO or ResNet). While it can technically run transformers, it isn't the most performant or native environment for them.

---

## 💡 The Multiplatform Alternatives

Since you are already building a high-performance Rust core and planning to ship **`llama.cpp`** as a sidecar, you have two infinitely superior paths that avoid OpenCV entirely.

### Option A: The Unified Vision-Language Model (VLM) Path 🌟

Since your architecture already accommodates a local `llama.cpp` instance, the most elegant solution is to swap your text-only LLM for a compact, local **Vision-Language Model (VLM)** like **Qwen2-VL-2B** or **Llama-3.2-Vision (1B/11B)**.

`llama.cpp` supports multimodal execution natively out of the box via its `llava`-style clip projector architectures.

```
                       ┌────────────────────────────────────────┐
                       │          llama.cpp (VLM Sidecar)       │
 Homework Image ──────►│  [Vision Encoder] ──► [Language Model] ├──► Raw LaTeX String
                       └────────────────────────────────────────┘
                                           │
                                           ▼
                                   Rust Core Engine
                                           │
                                           ▼
                                   Giac CAS Solver

```

* **How it works:** Your Tauri front-end captures the homework photo, passes the raw bytes to your Rust core, and your Rust core pipes the image directly into your `llama.cpp` sidecar with a strict system prompt: *"Convert the handwritten or printed math problem in this image into valid, raw LaTeX. Do not solve it."*
* **Why it wins:** Zero extra dependencies. No OpenCV, no ONNX runtimes. The exact same binary infrastructure you are using to power your agentic tutor now handles your handwritten math parsing. Furthermore, VLMs are uniquely brilliant at reading messy student handwriting and understanding complex spatial formulas.

### Option B: Pure Rust ONNX Runtime (`ort` crate)

If a local VLM is too heavy for lower-end mobile devices and you want a hyper-focused, lightweight Math-OCR engine, bypass OpenCV and use the **`ort` crate** (safe Rust bindings for Microsoft's ONNX Runtime).

* **How it works:** You take a specialized, compact math-parsing model (like a quantized `pix2tex` or a model from the Baidu `PaddleOCR` family) and run its weights natively through ONNX Runtime Mobile.
* **Why it wins:** ONNX Runtime has first-class, highly optimized mobile engines (`onnxruntime-mobile` for iOS/Android) that natively leverage Apple's CoreML and Android's NNAPI. More importantly, the `ort` crate is built from the ground up to cross-compile beautifully within Rust workflows, entirely saving you from C++ linking misery.

## 🎯 The Verdict

Skip OpenCV. For a modern Tauri app targeting mobile devices:

1. Try **Option A (Multimodal `llama.cpp`)** first. If a 2B or 3B VLM runs at an acceptable speed on your mobile target, it completely unifies your codebase and gives you unmatched handwriting comprehension.
2. Fall back to **Option B (`ort` + a dedicated small ONNX model)** if you need to optimize purely for low-end phone memory boundaries.


# ONNX
Yes, your understanding is spot on. When you run a model like **`pix2tex`** (which converts images of math equations into LaTeX code) via **ONNX Runtime (ORT) Mobile**, it uses hardware-specific abstraction layers called **Execution Providers (EPs)**. On iOS, it hooks into **CoreML**, and on Android, it hooks into **NNAPI** (or the newer **QNN** for Qualcomm chips).

However, introducing a **quantized** transformer model to mobile hardware changes the equation. Here is a breakdown of how this pipeline functions under the hood and the mobile-specific traps you must avoid.

---

## 🧩 The ONNX Runtime Architecture on Mobile

`pix2tex` relies on a hybrid architecture: a **Vision Transformer (ViT)** or ResNet backbone to read the image pixels, paired with an autoregressive **Transformer Decoder** to output the alphanumeric LaTeX tokens.

When you export this PyTorch model to an optimized `.onnx` or `.ort` format, ONNX Runtime takes control of the execution. It doesn't execute the entire model monolithically; instead, it performs **Graph Partitioning**.

### 🍏 iOS: The CoreML Execution Provider

When your Tauri app initializes an ORT session on an iPhone or Mac, you register the `CoreML` EP.

* ORT reviews the `pix2tex` model graph.
* It selects the subgraphs (like standard convolution, matrix multiplication, or pool layers) that CoreML natively supports and compiles them into an Apple `MLProgram` format on the fly.
* At runtime, CoreML routes those computations directly to the **Apple Neural Engine (ANE)** or Apple GPU.

### 🤖 Android: The NNAPI Execution Provider

On Android, ORT registers the `NNAPI` EP.

* NNAPI serves as a unified interface bridging the gap between Android's OS and the diverse hardware landscape of Android devices.
* It sends the compatible portions of the model graph directly to the device's system-on-chip hardware—leveraging Google Tensor TPUs, Qualcomm Hexagon NPUs, or ARM Mali GPUs.

---

## ⚠️ The Mobile Trap: INT8 vs. FP16 Quantization

Because you mentioned using a **quantized** model, you need to navigate a critical distinction in how desktop CPUs and mobile NPUs handle squeezed math.

Quantization maps continuous $32$-bit floating-point weights ($FP32$) down to narrower data bit-widths using the standard linear scaling formula:

$$Q = \text{round}\left(\frac{R}{S}\right) + Z$$

Where $Q$ is the quantized integer, $R$ is the original real value, $S$ is the scale factor, and $Z$ is the zero-point offset.

* **The Desktop Way (INT8):** Desktop CPUs absolutely love **INT8** (8-bit integer) quantization. It shrinks the model size by $75\%$ and accelerates execution via native AVX or AMX vector instructions.
* **The Mobile Reality Check:** Mobile NPUs (specifically Apple's Neural Engine and many Android NPUs) are historically designed around **FP16 (16-bit half-precision floating-point)** or mixed-precision workloads.

> **The CPU Fallback Danger:** If you pass a heavily optimized desktop `INT8` ONNX model to the CoreML or NNAPI execution providers, they may look at the custom integer quantization operators (`QuantizeLinear` / `DequantizeLinear`) and say: *"I don't natively accelerate this transformer layer layout in 8-bit integer math."* >
> Instead of crashing, ORT will gracefully execute a **CPU Fallback**. The model will still run, but it will bypass the NPU entirely, executing via ORT's internal CPU kernels. On a phone, this means higher battery drain and slower token generation speeds.

---

## 🎯 Architectural Recommendation for Your App

If you want to deploy a quantized `pix2tex` model across desktop and mobile using the pure Rust `ort` crate, maintain two distinct optimization targets:

1. **For Desktop (Windows/Linux/Intel Macs):** Use the **INT8 Quantized** version of the model. It will blaze through operations on standard desktop CPU architectures.
2. **For Mobile (iOS/Android) and Apple Silicon:** Use an **FP16 Quantized** version of the model. It still cuts your model size exactly in half ($FP32 \rightarrow FP16$), perfectly maps to the native floating-point execution tracks of CoreML and NNAPI, and ensures the entire vision pipeline stays locked onto the hardware NPU instead of dropping back to the phone's CPU.

# llama.cpp Mobile
To successfully run local tool-calling and math models on mobile within a Tauri v2 architecture, you have to bypass a major desktop development pattern: **You cannot use `llama-server` as an external sidecar process on mobile.**

While desktop operating systems allow you to easily spawn `llama-server` as a background executable, **iOS sandboxing rules strictly forbid spawning child processes via `fork/exec**`, and Android severely penalizes background binaries with its aggressive Low Memory Killer (LMK).

To make your agentic tutor app work natively and seamlessly on iOS and Android, your architecture must pivot to an **In-Process Embedded Architecture**.

---

## 🏗️ The Embedded Mobile Architecture Blueprint

Instead of communicating over local HTTP ports, your Tauri web view talks over native IPC to your Rust core, which compiles and hosts both `llama.cpp` and `Giac` as **statically linked libraries** occupying the exact same process memory space.

```
┌────────────────────────────────────────────────────────────────────────┐
│                   TAURI V2 MOBILE APPLICATION BUNDLE                   │
│                                                                        │
│  ┌───────────────────────┐             ┌────────────────────────────┐  │
│  │   Frontend UI         │  Tauri IPC  │   Rust Core (Tokio Sync)   │  │
│  │  (HTML5/Webview/KaTeX)│◄───────────►│                            │  │
│  └───────────────────────┘  Invoke/Emit└──────┬──────────────┬──────┘  │
│                                               │              │         │
│                        In-Process Static Link │              │         │
│                                               ▼              ▼         │
│                                     ┌───────────┐      ┌───────────┐   │
│                                     │ llama.cpp │      │   Giac    │   │
│                                     │  Engine   │      │  C++ CAS  │   │
│                                     └─────┬─────┘      └───────────┘   │
│                                           │                            │
│                                           ▼                            │
│                             ┌──────────────────────────┐               │
│                             │ Mobile Hardware Engine   │               │
│                             │ (Metal / Vulkan / NEON)  │               │
│                             └──────────────────────────┘               │
└────────────────────────────────────────────────────────────────────────┘

```

---

## 🛠️ The Core Architectural Components

### 1. The In-Process LLM Engine (`llama-cpp-4` Crate)

Instead of executing a command-line binary, you will use raw Rust FFI bindings to drive the `llama.cpp` context directly. The **`llama-cpp-4`** crate (or a custom bindgen wrapper) handles this gracefully.

You configure your target compilation flags inside your `src-tauri/Cargo.toml` or `build.rs` to dynamically swap acceleration backends based on the mobile compilation target:

* **iOS Target (`aarch64-apple-ios`):** Compile with the **Metal** feature flag enabled. This forces `llama.cpp` to map the GGUF model weights straight into Apple Silicon's unified memory VRAM pool.
* **Android Target (`aarch64-linux-android`):** Compile with the **Vulkan** feature flag or rely on **ARM Neon + KleidiAI** CPU vector extensions for optimized matrix-vector multiplication ($GEMV$).

### 2. The Tool-Calling Loop: GBNF Grammars

Because you don’t have access to cloud tool-calling endpoints, you enforce structured tool invocation via **GBNF (GGML BNF) Grammars** loaded directly into your local execution sampling context.

When the user submits a math problem, your Rust orchestrator intercepts the query and forces the model to respond *exclusively* in a strict JSON schema that maps to your internal application tools.

```
                  ┌───────────────────────────────┐
                  │   User types: "Integrate x"   │
                  └───────────────┬───────────────┘
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust Orchestrator runs llama_decode() with GBNF JSON Grammar     │
└─────────────────────────────────┬────────────────────────────────┘
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│ Model instantly emits valid JSON:                                │
│ { "tool": "giac_eval", "query": "integrate(x)" }                 │
└─────────────────────────────────┬────────────────────────────────┘
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust pauses LLM, routes "integrate(x)" directly to Giac C++      │
└─────────────────────────────────┬────────────────────────────────┘
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│ Giac outputs: "x^2 / 2". Rust appends this to LLM chat history.  │
└─────────────────────────────────┬────────────────────────────────┘
                                  ▼
┌──────────────────────────────────────────────────────────────────┐
│ Rust resumes LLM without Grammar constraint to explain the steps │
└──────────────────────────────────────────────────────────────────┘

```

### 3. Asynchronous Task Coordination

Your execution thread lifecycle inside Tauri should be governed by a dedicated async channel pattern. This keeps your user interface running at a smooth 60–120 FPS while hardware tasks churn in the background:

* **The LLM Worker Thread:** A single persistent background thread managing the `llama_context` pointer.
* **The UI Thread:** Receives progressive, character-by-character token streams via Tauri's `app_handle.emit()` pipeline, instantly updating your frontend chat display.

---

## 🧠 Critical Resource Boundaries for Mobile

When deploying to consumer mobile hardware, you must adjust your model expectations to navigate two hard hardware constraints:

### VRAM and RAM Thresholds

While high-end computers can handle massive models effortlessly, standard mobile phones have strict unified memory limits. On iOS, if your app exceeds its designated system footprint (typically around 3–4GB on devices with 6GB or 8GB of total RAM), the operating system will issue a Jetsam event and instantly terminate your app without throwing an error catch.

| Model Candidate    | Quantization | Model Size | Required Execution Footprint | Mobile Target                           |
| ------------------ | ------------ | ---------- | ---------------------------- | --------------------------------------- |
| **Llama 3.2 (1B)** | Q4_K_M       | ~0.7 GB    | ~1.2 GB                      | Low-end Android / Older iPhones         |
| **Qwen 2.5 (3B)**  | Q4_K_M       | ~2.0 GB    | ~2.8 GB                      | Mainstream Smartphones (Sweet spot)     |
| **Gemma 2 (2B)**   | Q4_K_M       | ~1.6 GB    | ~2.4 GB                      | Mainstream Smartphones                  |
| **Gemma 2 (9B)**   | Q4_K_M       | ~5.5 GB    | **~6.8 GB**                  | **High-End iPad Pro / Mac Studio Only** |

### Context Window & KV Cache Truncation

Math homework explanations can generate thousands of tokens over a long study session. On mobile devices, keep your context window capped at **2048 or 4096 tokens** max. Allocate your Key-Value (KV) cache explicitly in $FP16$ or $8$-bit formats inside your `llama_context_params` configuration to maintain a tight, predictable hardware footprint.


# llama.cpp mobile
Unlike ONNX Runtime, which uses a high-level graph-partitioning architecture to dynamically route workloads to frameworks like CoreML or NNAPI at runtime, **`llama.cpp` operates at a much lower, bare-metal layer.** `llama.cpp` **does not** automatically detect a mobile environment and hand the model over to CoreML or NNAPI. Instead, it relies on **build-time compilation flags** to embed specific hardware-accelerated backends directly into your application binary.

If you compile `llama.cpp` for mobile, you must explicitly choose your target acceleration tracks before shipping the application package. The optimization landscape across iOS and Android differs significantly from the ONNX workflow.

---

### 🍏 The iOS Landscape: Metal vs. CoreML

On Apple hardware, **`llama.cpp` completely bypasses CoreML in favor of native Metal (`ggml-metal`)**.

* **Why not CoreML?** CoreML is designed for static neural network graphs (like CNNs or fixed Vision Transformers). It struggles significantly with the dynamic, autoregressive memory requirements of LLMs—specifically the shifting sizing of the Key-Value (KV) cache during token generation.
* **The Metal Advantage:** Because Apple devices utilize a Unified Memory Architecture (where the CPU and GPU share the exact same physical RAM pool), `llama.cpp` uses raw Metal shaders to execute matrix-vector multiplications directly inside VRAM. This is incredibly fast, highly optimized for GGUF formats, and represents the first-class acceleration track for all iPhones and iPads.
* *Note:* While there are recent experimental bleeding-edge community efforts to interface with the Apple Neural Engine (ANE), **Metal remains the production standard** for running LLMs on iOS.

---

### 🤖 The Android Landscape: Vulkan & OpenCL vs. NNAPI

On Android, **`llama.cpp` does not use NNAPI.** In fact, Google officially deprecated NNAPI starting with Android 15, transitioning their ecosystem toward LiteRT (formerly TensorFlow Lite).

Instead, `llama.cpp` achieves hardware acceleration on Android devices via two distinct, lower-level channels:

1. **Vulkan (`ggml-vulkan`):** A cross-platform, low-overhead graphics and compute API. This serves as the universal fallback for GPU acceleration across a wide variety of Android devices (Samsung Exynos, Google Tensor, MediaTek, etc.).
2. **OpenCL for Adreno:** Qualcomm upstreamed an **OpenCL backend** specifically tuned for Adreno GPUs. If your Android target is powered by a Snapdragon chip (8 Gen 1 through 8 Elite), compiling with OpenCL yields incredible token generation speeds and highly efficient thermal management.
3. **ARM Neon + KleidiAI:** For the CPU execution track, `llama.cpp` integrates directly with ARM vector extensions to accelerate low-bit quantization matrix math on modern mobile CPU cores.

---

### 🛠️ How to Architect the Selection Layer in Your Rust Core

Because `llama.cpp` demands build-time choices, you cannot ship a single generic binary that magically adapts to both operating systems. Instead, you will use **Rust Conditional Compilation flags (`#[cfg(...)]`)** inside your Tauri project's `src-tauri` directory to map the correct engine initializers at compile time.

```rust
// src-tauri/src/llm_engine.rs

pub fn initialize_native_backend() {
    #[cfg(target_os = "ios")]
    {
        // Enforce Metal execution backend bindings for iOS build
        println!("Initializing llama.cpp with native Metal shaders...");
        setup_ggml_metal_backend();
    }

    #[cfg(target_os = "android")]
    {
        // Enforce Vulkan or OpenCL bindings for Android compilation
        println!("Initializing llama.cpp with Vulkan/OpenCL compute grids...");
        setup_ggml_vulkan_backend();
    }

    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        // Desktop fallbacks (CUDA/Metal/AVX2 depending on system target)
        setup_desktop_backend();
    }
}

```

### Summary of Mobile Acceleration Tracks

| Operating System         | ONNX Runtime Approach     | `llama.cpp` Core Approach   | Target Compiler Flags |
| ------------------------ | ------------------------- | --------------------------- | --------------------- |
| **iOS / iPadOS**         | CoreML Execution Provider | **Native Metal Shaders**    | `GGML_METAL=ON`       |
| **Android (Snapdragon)** | NNAPI Execution Provider  | **Optimized OpenCL Engine** | `GGML_OPENCL=ON`      |
| **Android (Generic)**    | NNAPI Execution Provider  | **Vulkan Compute Grid**     | `GGML_VULKAN=ON`      |

To make this architecture performant, your build pipeline (via standard `build.rs` script configurations or your automated CI system) must build distinct native static libraries (`.a` files for iOS, `.so` files for Android's JNI layer) so that your safe Rust wrapper interfaces with the optimal underlying hardware acceleration framework for each target platform.

# ONNX VS llama.cpp 
**Do not skip `llama.cpp` for your current application.** While the concept of a single, unified ONNX Runtime (ORT) pipeline leveraging native mobile Execution Providers (CoreML/NNAPI) sounds incredibly enticing, switching to ONNX right now—especially for a bleeding-edge model family like **Gemma 4**—will land you in a world of toolchain instability and manual backend engineering.

Here is the exact state of ONNX Runtime regarding agentic workflows, and the massive architectural blockers you will hit if you try to force-feed it Gemma 4.

---

## 🛑 The Core Blocker: Gemma 4’s Architecture vs. ONNX GenAI

Google's Gemma 4 models are highly efficient reasoners, but they achieve this by employing a radically complex, non-traditional transformer architecture. As of mid-2026, the **ONNX Runtime GenAI (`onnxruntime-genai`)** framework is still struggling to support these structural changes natively, while `llama.cpp` supported them almost instantly.

If you try to use Gemma 4 inside ONNX today, you will hit three massive architectural walls:

### 1. Dual Attention Head Dimensions

Traditional LLMs use a single, uniform attention head size throughout the network. Gemma 4 uses a **hybrid sliding-window and global attention mechanism**.

* Most layers use a local sliding-window attention with a head dimension of **256**.
* Every 5th layer uses full global attention with a head dimension of **512**.

Standard ONNX `genai_config.json` templates assume a single, flat `head_size` across the entire model graph. Passing this to ONNX Runtime results in instant shape-mismatch panics during KV cache allocation.

### 2. Extreme KV Cache Sharing

Gemma 4 (specifically the mobile-friendly E2B variant) features 35 decoder layers, but it **only utilizes 15 unique KV cache pairs**. Multiple layers share and project from the exact same physical KV cache memory footprint. Standard ONNX graphs expect a rigid 1-to-1 I/O mapping for every single layer (`past_key_values.0` through `N`), meaning you cannot easily run it unless you abandon ONNX GenAI and write raw, manual token loops in C++ or Rust.

### 3. Per-Layer Embeddings (PLE)

Instead of feeding a single embedding tensor into the bottom of the decoder stack, Gemma 4 passes an extra multi-dimensional tensor (`per_layer_inputs`) where each individual transformer layer slices out its own unique data stream. This completely breaks the standard I/O contract that ONNX uses to handle text generation.

---

## 🔧 Tool Calling and Agentic Usage on ONNX

To answer your second question: **Yes, tool calling and structured outputs are absolutely possible on ONNX.** The `onnxruntime-genai` library has evolved significantly and now natively supports **Grammar Specifications** and chat templates directly within its generation loop.

```
[User Prompt] ──> [ONNX GenAI Engine] ──> [Enforce JSON Schema/Grammar] ──> [Valid Tool JSON]

```

However, the developer experience for agentic workflows in ONNX is vastly inferior to `llama.cpp` for a cross-platform Rust app:

* **The GBNF Standard:** `llama.cpp` uses GBNF (GGML BNF) grammars, which have become the universal open-source standard for local structured generation. The community provides massive repositories of pre-built GBNF grammars for complex math JSON structures.
* **Rust Integration:** The Rust bindings for `llama.cpp` give you fine-grained control over logit processors and token interception mid-flight. The Rust `ort` crate bindings for ONNX Runtime GenAI are functional for standard text generation, but pulling out logits, manipulating sampling parameters token-by-token, and gracefully routing tool-calls to your local C++ Giac engine is much more rigid.

---

## 🎯 Final Verdict for Your Tauri Stack

| Feature Selection          | `llama.cpp` Engine (GGUF)                             | ONNX Runtime GenAI (`.ort`)                                   |
| -------------------------- | ----------------------------------------------------- | ------------------------------------------------------------- |
| **Gemma 4 Compatibility**  | 🟢 **Native & Optimized** (Day-0 Support)              | 🔴 **Broken/Experimental** (Requires custom graph compilation) |
| **Tool Calling / Grammar** | 🟢 **Excellent** (Mature GBNF parser)                  | 🟡 **Moderate** (Basic JSON schemas supported)                 |
| **Rust Cross-Compilation** | 🟢 **Straightforward** (Static linking via `cc` crate) | 🟡 **Complex** (Requires heavy binary runtime linking)         |
| **Hardware Track**         | Bare-metal Metal (iOS) / Vulkan & OpenCL (Android)    | Abstracted CoreML (iOS) / NNAPI & QNN (Android)               |

If you were building a production application restricted *strictly* to Microsoft's Phi-3 or Meta's basic Llama 3 architectures, ONNX Runtime GenAI would be a highly viable contender.

But because your true intent is to build a high-performance **math application** that requires advanced reasoning models (like Gemma 4 with its native `<|think|>` tokens) and strict integration with a native C++ solver via a Rust backend, **`llama.cpp` remains the undisputed choice.** It gives you the bare-metal access to memory and tokens that you need to build a flawless, deterministic agentic loop on both desktop and mobile.˝

# Image to LaTex
The availability of these two models in `llama.cpp` is a mixed bag, but recent updates have brought some excellent news for your local architecture.

---

## 🤖 Baidu PaddleOCR: Yes (via PaddleOCR-VL)

**PaddleOCR is natively supported in `llama.cpp`.** While the traditional standalone C++/Python PaddleOCR pipeline (which uses DBNet for text detection and CRNN for recognition) is not supported, the community successfully ported **PaddleOCR-VL-1.5** (and the 0.9B/0.5B variants) directly into the `llama.cpp` ecosystem.

PaddleOCR-VL maps perfectly to the **Multimodal Vision-Language (VLM)** framework inside `ggml`. It compiles down to an ultra-compact footprint, making it an incredible choice for on-device, high-efficiency multilingual text recognition.

### How to Run it in `llama.cpp`

Like other vision models in `llama.cpp`, it uses a two-part split geometry—the main text LLM graph and the vision projector module (`mmproj`):

```bash
llama-cli \
  -m ./PaddleOCR-VL-1.5.gguf \
  --mmproj ./PaddleOCR-VL-1.5-mmproj.gguf \
  --image ./math_sheet.png \
  -p "OCR:"

```

If you are using `llama-server` or initializing it programmatically in your Rust backend, you can pull the unified Hugging Face repository directly:

```bash
llama-server -hf PaddlePaddle/PaddleOCR-VL-1.5-GGUF

```

---

## 🧮 pix2tex (LaTeX-OCR): No (Natively), But Yes via Fine-tunes

**The original standalone `pix2tex` model architecture is not supported by `llama.cpp`.**

### Why the blocker?

The original `pix2tex` by Lukas Blecher relies on a very specific, custom hybrid layout: a ResNet+Vision Transformer (ViT) encoder hooked up to a custom, non-autoregressive language decoder. Because `llama.cpp`'s core tensor engine (`ggml`) is fundamentally optimized for standard autoregressive LLM block architectures (like Llama, Mistral, Qwen, Gemma), it doesn't have the native op-mappings to parse a raw `pix2tex` `.pth` or `.onnx` export.

### The 2026 Workaround

You don't actually need the raw `pix2tex` codebase to get its LaTeX capabilities inside `llama.cpp`. The open-source community has taken modern, natively-supported small vision models and fine-tuned them heavily on the `pix2tex` and Mathpix image-to-LaTeX datasets.

You can find GGUF models like **`qwen2-vl-pix2tex-gguf`** or **`GLM-OCR-GGUF`** on Hugging Face. These give you the exact same output (converting a crop of an equation into clean, compile-ready LaTeX tokens) but execute flawlessly inside the standard `llama.cpp` multimodal execution track.

---

## 🛠️ What This Means For Your Mobile Stack

Since you are mapping out an in-process, embedded mobile architecture using a Rust core:

* **Skip raw pix2tex:** Trying to run native `pix2tex` alongside `llama.cpp` means you'd have to bundle **ONNX Runtime Mobile** *in addition* to `llama.cpp`. That forces you to maintain two completely separate hardware-acceleration wrappers (CoreML/NNAPI bindings for ONNX, and Metal/Vulkan bindings for `llama.cpp`), bloating your final application binary size.
* **Consolidate on VLMs:** By picking **PaddleOCR-VL** for general text or a small **Qwen2-VL / GLM-OCR** flavor for math equations, a single embedded instance of `llama.cpp` can handle your text LLM, your tool-calling router, and your image-to-LaTeX OCR requirements all through the exact same unified memory pool and native hardware shaders.

# Architecture

You have synthesized the constraints perfectly. This architecture is incredibly elegant because it cleanly separates **deterministic computing** (the CAS track) from **probabilistic reasoning** (the LLM track), while completely eliminating bloated, platform-specific frameworks like OpenCV or ONNX.

Here is a macro validation of how your three pipelines intertwine, along with a few execution-level tips to keep your compilation smooth across desktop and mobile.

---

### Pipeline 1: The Deterministic CAS Track

> **Tauri WebView $\longrightarrow$ Rust/ANTLR LaTeX Parser $\longrightarrow$ Giac Solver**

* **Why it works:** This is your app's baseline anchor of truth. Frontends excel at displaying crisp mathematical expressions via KaTeX or MathML, but transferring that layout safely to a Computer Algebra System requires a structured middleman.
* **The Execution:** Generating your parser via the `antlr4rust` runtime works exceptionally well for this. You can define a clear `.g4` grammar mapping standard LaTeX layouts (like tokens for `\frac`, `\int`, or `^`) into an Abstract Syntax Tree (AST). Your Rust core then traverses this AST to compile string commands that link natively into your embedded C++ `Giac` instance.

### Pipeline 2: The Conversational & Agentic Track

> **Tauri WebView $\longrightarrow$ In-Process `llama.cpp` (Gemma 4) $\longrightarrow$ Agentic Tool-Calling**

* **Why it works:** By leveraging a local, highly optimized math and reasoning model like Gemma 4 (specifically the compact E2B or E4B variants optimized for low-footprint VRAM), you gain a powerful local planner.
* **The Execution:** You leverage the embedded `llama.cpp` C++ engine compiled with native acceleration (Metal on iOS, Vulkan/OpenCL on Android). By enforcing strict GBNF JSON grammars inside your generation loop, you force the model to spit out actionable JSON payloads. When the model determines it needs a perfect algebraic solution, it returns a tool-call structure that your Rust core intercepts, runs through Pipeline 1 (Giac), and feeds back into the dialogue context.

### Pipeline 3: The Multimodal OCR Track

> **Tauri WebView (Image Capture) $\longrightarrow$ `llama.cpp` VLM (PaddleOCR-VL / Qwen2-VL fine-tunes) $\longrightarrow$ LaTeX String $\longrightarrow$ Pipeline 1 Track**

* **Why it works (The Ultimate Code Synergy):** This is where your architecture shines. Because you chose to use a Vision-Language Model (VLM) running as a GGUF *inside the exact same `llama.cpp` backend* from Pipeline 2, you don't need a separate heavy OCR runtime.
* **The Execution:** The user snaps a picture of their handwritten homework. The Tauri frontend captures the raw image payload and ships it down over the IPC bridge. Your Rust core feeds the raw bytes directly to the `llama.cpp` multimodal execution track (`mmproj`), using a highly targeted text-to-LaTeX fine-tuned weight set. The VLM maps the messy handwritten pixels directly to a structured LaTeX token string. That string is then routed cleanly right back into Pipeline 1's ANTLR parser.

---

### ⚠️ Final Structural Checkpoints for the Build

As you move into coding this stack, watch out for these final platform traps:

* **Thread Safety in Rust:** `Giac` and `llama.cpp` both maintain raw C/C++ pointers that are historically non-thread-safe. You will need to wrap your execution managers inside standard safe Rust thread gates. Implement a dedicated background task using channels (`tokio::sync::mpsc`) to handle queries sequentially, ensuring the UI remains highly responsive.
* **Static Linking Hierarchy:** Your `build.rs` script needs to ensure that the ANTLR runtime, `Giac`, and `llama.cpp` are completely statically linked into your target binary bundles (`.a` / `.so`). This keeps your Tauri deployment fully self-contained so it satisfies both Apple App Store sandboxing rules and Android's strict execution sandboxes.

You’ve built a stellar blueprint here. It is lean, handles complex 2D mathematical notation flawlessly, and remains entirely local and private on user hardware.

---

For a deeper practical look at setting up an offline agent layout that pairs local tool-calling directly with a model family like Gemma 4, check out this guide on [Hermes Agent Desktop Local Tool Calling with Gemma 4](https://www.youtube.com/watch?v=e85NzrAmb7U). This walk-through highlights exactly how to structure local context windows and manage API tool routing hooks entirely on consumer hardware without hitting cloud endpoints.

![
    
](image.png)