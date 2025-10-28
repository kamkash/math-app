// src/libs/math-solver.ts
import mathSolverTemplate from "./math-solver.html?raw";
import { MathfieldElement, renderMathInDocument } from "mathlive";
import {
  run_llm_generate,
  run_greet,
  run_add_grammar,
  run_reset_model,
  run_reset_context,
  run_solver,
} from "./solver.ts";

window.addEventListener("DOMContentLoaded", () => {
  console.log("MathSolver: DOMContentLoaded");
  renderMathInDocument();
});
class MathSolver extends HTMLElement {
  private shadow: ShadowRoot;
  private promptInputEl: MathfieldElement | null = null;
  private responseOutputEl: MathfieldElement | null = null;

  constructor() {
    super();
    this.shadow = this.attachShadow({ mode: "open" });
    this.shadow.innerHTML = mathSolverTemplate;
  }

  connectedCallback() {
    renderMathInDocument();
    this.loadElements();
    this.attachEventListeners();
    if (this.promptInputEl) {
        this.promptInputEl.smartMode = true;
    //   this.promptInputEl.setValue(
        
    //     `            x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a} 
    //     \\displaylines{
    //         \\text{The second taxicab number is} \\\\
    //         \\(1729 = 10^3 + 9^3 = 12^3 + 1^3\\) \\\\
    //         \\text{Solve the following equations:} \\\\
    //         x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a} \\\\
    //         e^{i\\pi} + 1 = 0 \\\\
    //         \\int_0^\\infty e^{-x^2}\\,dx = \\frac{\\sqrt{\\pi}}{2} \\\\
    //         pow\\_res = a^{2} \\\\
    //         exp1 = e^{1}
    //     }`,
    //     { mode: "math" }
    //   );
    }
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

  private attachEventListeners() {
    this.shadow
      .querySelector("#llm-solver")
      ?.addEventListener("click", async (e) => {
        e.preventDefault();
        if (this.promptInputEl && this.responseOutputEl) {
          await run_llm_generate(this.promptInputEl, this.responseOutputEl);
        }
      });

    this.shadow
      .querySelector("#math-solver")
      ?.addEventListener("click", async (e) => {
        e.preventDefault();
        if (this.promptInputEl && this.responseOutputEl) {
          await run_solver(this.promptInputEl, this.responseOutputEl);
        }
      });

    this.shadow
      .querySelector("#reset_context")
      ?.addEventListener("click", async (e) => {
        e.preventDefault();
        // Topic can be configurable via an attribute or property if needed
        await run_reset_context("new topic from math-solver");
      });

    this.shadow
      .querySelector("#reset_model")
      ?.addEventListener("click", async (e) => {
        e.preventDefault();
        // Model name can be configurable
        await run_reset_model("new model from math-solver");
      });

    this.shadow
      .querySelector("#add_grammar")
      ?.addEventListener("click", async (e) => {
        e.preventDefault();
        if (this.promptInputEl && this.responseOutputEl) {
          await run_add_grammar(this.promptInputEl, this.responseOutputEl);
        }
      });

    this.shadow
      .querySelector("#greet")
      ?.addEventListener("click", async (e) => {
        e.preventDefault();
        if (this.promptInputEl && this.responseOutputEl) {
          await run_greet(this.promptInputEl, this.responseOutputEl);
        }
      });
  }
}

customElements.define("math-solver", MathSolver);
