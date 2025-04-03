import { invoke } from "@tauri-apps/api/core";

declare global {
  interface Window {
    render: any; // Mathpix render function
    loadMathJax: any; // MathJax loadMathJax function
  }
}

let promptInputEl: HTMLInputElement | null;
let contentEl: HTMLElement | null;
let rawContentEl: HTMLElement | null;

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
  if (contentEl && promptInputEl) {
    let res = await invoke("add_grammar", {
      grammar: promptInputEl.value,
    });
    contentEl.textContent = `Grammar added: ${res}`;
  }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
async function greet() {
  if (contentEl && promptInputEl) {
    contentEl.textContent = await invoke("greet", {
      name: promptInputEl.value,
    });
  }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
async function llm_generate() {
  if (promptInputEl && contentEl && rawContentEl) {
    let answer: string = await invoke("llm_generate", {
      prompt: promptInputEl.value,
    });
    answer = String.raw`${answer}`;
    const options = {
      htmlTags: true,
    };
    console.log(`${answer}`);
    const html = window.render(answer, options);
    contentEl.innerHTML = html;
    rawContentEl.innerText = answer;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  console.log("dom content loaded");
  load_mathpix();
  promptInputEl = document.querySelector("#prompt-input");
  contentEl = document.querySelector("#content-text");
  rawContentEl = document.querySelector("#raw-content-text");
  document
    .querySelector("#greet-form")
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

function load_mathpix() {
  let script = document.createElement("script");
  script.src = "/src/assets/mathpix.js";
  document.head.append(script);
  script.onload = function () {
    const isLoaded = window.loadMathJax();
    console.log(`Mathpix loaded ${isLoaded}`);
  };
}
