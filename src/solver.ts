import { invoke } from "@tauri-apps/api/core";
import {
  convertLatexToAsciiMath,
  convertLatexToMathMl,
  MathfieldElement,
} from "mathlive";

let mainPagePromptInputEl: MathfieldElement | null;
let mainPageResponseOutputEl: MathfieldElement | null;

export function formatLatexBlock(text: string): string {
  return text
    .replace(/\\documentclass{article}/gm, "")
    .replace(/\\usepackage{amsmath}/gm, "")
    .replace(/\$\$\s*\\documentclass\{article\}\$\$/gm, "")
    .replace(/\$\$\s*\\usepackage\s*amsmath\s*\$\$/gm, "")
    .replace(/^```latex\s*$/gm, "\\begin{align}")
    .replace(/^```\s*$/gm, "\\end{align}")
    .replace(/\r?\n|\r$/g, "\\\\")
    .replace(/ +/gm, "~");
}

export function processLatexBlock(answer: string): string {
  let latex = formatLatexBlock(answer);
  latex = `\\begin{align} ${latex} \\end{align}`;
  return latex;
}

export async function run_solver(
  promptInputEl: MathfieldElement,
  responseOutputEl: MathfieldElement,
  _solverName?: string // Added for consistency with potential future use, though currently unused by backend
) {
  if (responseOutputEl && promptInputEl && promptInputEl.value) {
    let prompt = convertLatexToAsciiMath(promptInputEl.value);
    console.log(`MathML: ${convertLatexToMathMl(promptInputEl.value)}`);
    console.log("AsciiMath input", prompt);
    const res = await invoke("run_solver", {
      input: prompt,
    });
    responseOutputEl.setValue(res as string, { mode: "text" });
  }
}

export async function run_reset_context(topic: string) {
  await invoke("reset_context", { topic: topic });
  // Add any UI feedback specific to this action if needed
  console.log(`Context reset to: ${topic}`);
}

export async function run_reset_model(name: string) {
  await invoke("reset_model", { name: name });
  // Add any UI feedback specific to this action if needed
  console.log(`Model reset to: ${name}`);
}

export async function run_add_grammar(
  promptInputEl: MathfieldElement,
  responseOutputEl: MathfieldElement
) {
  if (responseOutputEl && promptInputEl && promptInputEl.value) {
    const res = await invoke("add_grammar", {
      grammar: promptInputEl.value,
    });
    responseOutputEl.value = `Grammar added: ${res}`;
  }
}

export async function run_greet(
  promptInputEl: MathfieldElement,
  responseOutputEl: MathfieldElement
) {
  if (responseOutputEl && promptInputEl && promptInputEl.value) {
    const ans = await invoke("greet", {
      name: promptInputEl.value,
    });
    const latex: string = processLatexBlock(ans as string);
    responseOutputEl.value = latex;
  }
}

export async function run_llm_generate(
  promptInputEl: MathfieldElement,
  responseOutputEl: MathfieldElement
) {
  if (promptInputEl && responseOutputEl && promptInputEl.value) {
    let ascii = convertLatexToAsciiMath(promptInputEl.value);
    console.log("AsciiMath", ascii);
    ascii += " Important: Answer in LaTeX format.";
    const answer: string = await invoke("llm_generate", {
      prompt: ascii,
    });
    console.log("llm_generate", answer);
    const latex = processLatexBlock(answer);
    responseOutputEl.value = latex;
  }
}

// Setup for the main page's elements and event listeners
function loadMainPageGlobals() {
  mainPagePromptInputEl = document.querySelector("#prompt-input");
  mainPageResponseOutputEl = document.querySelector("#response-output");
  if (!mainPagePromptInputEl) {
    console.warn(
      "Main page #prompt-input not found. Ensure it exists in your main HTML."
    );
  }
  if (!mainPageResponseOutputEl) {
    console.warn(
      "Main page #response-output not found. Ensure it exists in your main HTML."
    );
  }
}

// window.addEventListener("DOMContentLoaded", () => {
//   console.log("DOM content loaded (from shared-math-logic)");
//   loadMainPageGlobals();

//   document
//     .querySelector("#input-form")
//     ?.addEventListener("submit", async (e) => {
//       e.preventDefault();
//       if (mainPagePromptInputEl && mainPageResponseOutputEl) {
//         await run_llm_generate(mainPagePromptInputEl, mainPageResponseOutputEl);
//       }
//     });

//   document.querySelector("#solver")?.addEventListener("click", async (e) => {
//     e.preventDefault();
//     if (mainPagePromptInputEl && mainPageResponseOutputEl) {
//       await run_solver(
//         mainPagePromptInputEl,
//         mainPageResponseOutputEl,
//         "new solver from shared"
//       );
//     }
//   });

//   document
//     .querySelector("#reset_context")
//     ?.addEventListener("click", async (e) => {
//       e.preventDefault();
//       await run_reset_context("new topic from shared");
//     });

//   document
//     .querySelector("#reset_model")
//     ?.addEventListener("click", async (e) => {
//       e.preventDefault();
//       await run_reset_model("new model from shared");
//     });

//   document
//     .querySelector("#add_grammar")
//     ?.addEventListener("click", async (e) => {
//       e.preventDefault();
//       if (mainPagePromptInputEl && mainPageResponseOutputEl) {
//         await run_add_grammar(mainPagePromptInputEl, mainPageResponseOutputEl);
//       }
//     });

//   document.querySelector("#greet")?.addEventListener("click", async (e) => {
//     e.preventDefault();
//     if (mainPagePromptInputEl && mainPageResponseOutputEl) {
//       await run_greet(mainPagePromptInputEl, mainPageResponseOutputEl);
//     }
//   });
// });
