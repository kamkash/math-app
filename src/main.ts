import { invoke } from "@tauri-apps/api/core";
import "mathlive";
import "mathlive/fonts.css";

import {
  // convertAsciiMathToLatex,
  convertLatexToAsciiMath,
  // convertLatexToSpeakableText,
  MathfieldElement,
} from "mathlive";

let promptInputEl: MathfieldElement | null;
let responseOutputEl: MathfieldElement | null;

function load_globals() {
  promptInputEl = document.querySelector("#prompt-input");
  responseOutputEl = document.querySelector("#response-output");
}

async function run_solver(name: string) {
  console.log(`run solver ${name}`);
  if (responseOutputEl && promptInputEl) {
    let res = await invoke("run_solver", {
      input_block: convertLatexToAsciiMath(promptInputEl.value),
    });
    // let latex = processLatexBlock(res as string);
    responseOutputEl.setValue(res as string, { mode: "text" });
  }
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
    let latex: string = processLatexBlock(ans as string);
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
    let latex = processLatexBlock(answer);
    responseOutputEl.value = latex;
  }
}

function processLatexBlock(answer: string) {
  let latex = formatLatexBlock(answer);
  latex = `\\begin{align} ${latex} \\end{align}`;
  return latex;
}

function formatLatexBlock(text: string): string {
  return text
    .replace(/\$\$\s*\\documentclass\{article\}\$\$/gm, "")
    .replace(/\$\$\s*\\usepackage\s*amsmath\s*\$\$/gm, "")
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

  document.querySelector("#solver")?.addEventListener("click", async (e) => {
    e.preventDefault();
    await run_solver("new solver");
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
