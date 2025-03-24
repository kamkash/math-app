import { invoke } from "@tauri-apps/api/core";

declare global {
  interface Window {
    render: any; // You can replace 'any' with a more specific type if known
  }
}


let promptInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;
let contentEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && promptInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: promptInputEl.value,
    });
  }
}

async function llm_generate() {
  if (greetMsgEl && promptInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    let text = await invoke("llm_generate", {
      prompt: promptInputEl.value,
    });
    const options = {
      htmlTags: true,
    };
    console.log(`Answer: ${text}`);
    const html = window.render(text, options);
    if (contentEl) {
      contentEl.outerHTML = html;
    }
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
});

function load_mathpix() {
  let script = document.createElement("script");
  script.src = "/src/assets/mathpix.js";
  document.head.append(script);
  script.onload = function () {
    const isLoaded = window.loadMathJax();
    if (isLoaded) {
      console.log(`Mathpix loaded ${isLoaded}`);
    }
  };
}
