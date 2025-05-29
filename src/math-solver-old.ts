import html from "./math-solver-old.html?raw"; // Import the HTML template
import { MathfieldElement } from "mathlive";
import { run_llm_generate, run_solver } from "./solver.ts";

class MathSolverOld extends HTMLElement {
  private shadow: ShadowRoot;
  private promptInputEl: MathfieldElement | null = null;
  private responseOutputEl: MathfieldElement | null = null;

  constructor() {
    super();
    this.shadow = this.attachShadow({ mode: "open" });
    this.shadow.innerHTML = html; // Set the shadow DOM's content to the imported HTML
  }

  connectedCallback() {
    // Component is added to the DOM
    // Add event listeners or other setup logic here
    this.loadElements();
    this.setupEventListeners();
  }

  private loadElements() {
    this.promptInputEl = this.shadow.querySelector(
      "#prompt-input"
    ) as MathfieldElement | null;
    this.responseOutputEl = this.shadow.querySelector(
      "#response-output"
    ) as MathfieldElement | null;

    if (!this.promptInputEl) {
      console.error("MathSolver: #prompt-input not found in shadow DOM");
    }
    if (!this.responseOutputEl) {
      console.error("MathSolver: #response-output not found in shadow DOM");
    }
  }

  disconnectedCallback() {
    // Component is removed from the DOM
    // Clean up event listeners or other resources here
  }

  private setupEventListeners() {
    // Example: Get button and add a click listener
    const resetButton = this.shadow.getElementById("reset_model");
    if (resetButton) {
      resetButton.addEventListener("click", () => {
        console.log("Reset Model button clicked in math-solver-old");
        // Add your reset model logic here
      });
    }

    const resetContextButton = this.shadow.getElementById("reset_context");
    if (resetContextButton) {
      resetContextButton.addEventListener("click", () => {
        console.log("Reset Context button clicked in math-solver-old");
      });
    }

    const addGrammarButton = this.shadow.getElementById("add_grammar");
    if (addGrammarButton) {
      addGrammarButton.addEventListener("click", () => {
        console.log("Add Grammar button clicked in math-solver-old");
      });
    }

    const greetButton = this.shadow.getElementById("greet");
    if (greetButton) {
      greetButton.addEventListener("click", () => {
        console.log("Greet button clicked in math-solver-old");
      });
    }

    this.shadow
      .querySelector("#solver")
      ?.addEventListener("click", async (e) => {
        e.preventDefault();
        if (this.promptInputEl && this.responseOutputEl) {
          await run_solver(this.promptInputEl, this.responseOutputEl);
        }
      });

    this.shadow
      .querySelector("#llm-solver")
      ?.addEventListener("click", async (event) => {
        event.preventDefault(); // Prevent default if it's inside a form that shouldn't submit traditionally
        console.log("LLM button clicked in math-solver-old");
        if (this.promptInputEl && this.responseOutputEl) {
          // Add logic to process prompt and update responseOutput.value
          this.responseOutputEl.value = `Response to: ${this.promptInputEl.value}`;
          if (this.promptInputEl && this.responseOutputEl) {
            await run_llm_generate(this.promptInputEl, this.responseOutputEl);
          }
        }
      });
  }

  // Add listeners for other buttons (reset_context, add_grammar, greet, solver)
  // and math-fields (prompt-input, response-output) as needed.
}

// Define the custom element
if (!customElements.get("math-solver-old")) {
  customElements.define("math-solver-old", MathSolverOld);
}

export default MathSolverOld;
