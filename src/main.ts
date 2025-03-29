import { invoke } from "@tauri-apps/api/core";

declare global {
  interface Window {
    render: any; // Mathpix render function
    loadMathJax: any; // MathJax loadMathJax function
  }
}

let promptInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let contentEl: HTMLElement | null;

async function new_topic() {
  console.log("new topic");
  await invoke("new_topic");
}

async function greet() {
  if (greetMsgEl && promptInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: promptInputEl.value,
    });
  }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
async function llm_generate() {
  if (promptInputEl && contentEl) {
    let answer: String = await invoke("llm_generate", {
      prompt: promptInputEl.value,
    });
    answer = String.raw`${answer}`;
    const options = {
      htmlTags: true,
    };
    console.log(`Answer: ${answer}`);
    const html = window.render(answer, options);
    contentEl.innerHTML = html;
  }
}

window.addEventListener("DOMContentLoaded", () => {
  console.log("dom content loaded");
  load_mathpix();
  promptInputEl = document.querySelector("#prompt-input");
  greetMsgEl = document.querySelector("#greet-msg");
  contentEl = document.querySelector("#content-text");
  document
    .querySelector("#greet-form")
    ?.addEventListener("submit", async (e) => {
      e.preventDefault();
      await llm_generate();
    });

  document
    .querySelector("#new_chat")
    ?.addEventListener("click", async (e) => {
      await new_topic();
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
