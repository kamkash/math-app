import { invoke } from "@tauri-apps/api/core";
import "mathlive";

import {
  convertAsciiMathToLatex,
  convertLatexToAsciiMath,
  convertLatexToSpeakableText,
  MathfieldElement,
} from "mathlive";

declare global {
  interface Window {
    render: any; // Mathpix render function
    loadMathJax: any; // MathJax loadMathJax function
  }
}

let promptInputEl: MathfieldElement | null;
let responseOutputEl: MathfieldElement | null;

function load_globals() {
  promptInputEl = document.querySelector("#prompt-input");
  responseOutputEl = document.querySelector("#response-output");
}

async function reset_context(topic: string) {
  console.log("reset context");
  await invoke("reset_context", { topic: topic });
}

async function reset_model(name: string) {
  console.log("reset model");
  await invoke("reset_model", { name: name });
}

async function add_grammar() {
  console.log("add grammar");
  if (responseOutputEl && promptInputEl) {
    let res = await invoke("add_grammar", {
      grammar: promptInputEl.value,
    });
    responseOutputEl.value = `Grammar added: ${res}`;
  }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
async function greet() {
  if (responseOutputEl && promptInputEl) {
    let ans = await invoke("greet", {
      name: promptInputEl.value,
    });
    let latex: string = processLatexBlocks(ans as string);
    responseOutputEl.value = latex;
  }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
async function llm_generate() {
  if (promptInputEl && responseOutputEl) {
    let ascii = convertLatexToAsciiMath(promptInputEl.value);
    ascii += " Important: Answer in LaTeX format.";
    let answer: string = await invoke("llm_generate", {
      prompt: ascii,
    });
    let latex = processLatexBlocks(answer);
    latex = `\\begin{align} ${latex} \\end{align}`;
    responseOutputEl.value = latex;
    console.log("using", latex);
  }
}

function processLatexBlocks(text: string): string {
  return text
    .replace(/^```latex\s*$/gm, "\\begin{align}")
    .replace(/^```\s*$/gm, "\\end{align}")
    .replace(/\r?\n|\r$/g, "\\\\")
    .replace(/ +/gm, "~");
}

window.addEventListener("DOMContentLoaded", () => {
  console.log("dom content loaded");
  load_globals();

  document
    .querySelector("#input-form")
    ?.addEventListener("submit", async (e) => {
      e.preventDefault();
      await llm_generate();
    });

  document
    .querySelector("#reset_context")
    ?.addEventListener("click", async (e) => {
      e.preventDefault();
      await reset_context("new topic");
    });

  document
    .querySelector("#reset_model")
    ?.addEventListener("click", async (e) => {
      e.preventDefault();
      await reset_model("new model");
    });

  document
    .querySelector("#add_grammar")
    ?.addEventListener("click", async (e) => {
      e.preventDefault();
      await add_grammar();
    });

  document.querySelector("#greet")?.addEventListener("click", async (e) => {
    e.preventDefault();
    await greet();
  });
});
