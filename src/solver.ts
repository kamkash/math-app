import {invoke} from "@tauri-apps/api/core";
import {MathfieldElement,} from "mathlive";


export function formatLatexBlock(text: string): string {
    return text
        .replace(/\\documentclass{article}/gm, "")
        .replace(/\\usepackage{amsmath}/gm, "")
        .replace(/\$\$\s*\\documentclass\{article\}\$\$/gm, "")
        .replace(/\$\$\s*\\usepackage\s*amsmath\s*\$\$/gm, "")
        .replace(/^```latex\s*$/gm, "\\begin{align}")
        .replace(/^```\s*$/gm, "\\end{align}")
        .replace(/\r?\n|\r$/g, "\\\\")
        .replace(/ +/gm, "~")
        .replace(/^\{(.*)\}$/s, "$1"); // Remove outer braces if they exist
}

export function processLatexBlock(answer: string): string {
    let latex = formatLatexBlock(answer);
    latex = `\\begin{align} ${latex} \\end{align}`;
    return latex;
}

export async function run_solver(
    promptInputEl: MathfieldElement,
    responseOutputEl: MathfieldElement,
) {
    if (responseOutputEl && promptInputEl && promptInputEl.value) {
        let promptLatex = promptInputEl.getValue("latex");
        promptLatex = formatLatexBlock(promptLatex);
        console.log("solver send", promptLatex);
        const res = await invoke("run_solver", {
            input: promptLatex,
        });
        // responseOutputEl.setValue(res as string, {mode: "text"});
        responseOutputEl.setValue(res as string, {mode: "auto"});
    }
}

export async function run_reset_context(topic: string) {
    await invoke("reset_context", {topic: topic});
    // Add any UI feedback specific to this action if needed
    console.log(`Context reset to: ${topic}`);
}

export async function run_reset_model(name: string) {
    await invoke("reset_model", {name: name});
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
        responseOutputEl.value = processLatexBlock(ans as string);
    }
}

export async function run_llm_generate(
    promptInputEl: MathfieldElement,
    responseOutputEl: MathfieldElement
) {
    if (promptInputEl && responseOutputEl && promptInputEl.value) {
        let prompt = promptInputEl.getValue("latex-expanded") ;
        prompt += "  Important: Answer in LaTeX format.";
        console.log("llm_generate send", prompt);
        let answer: string = await invoke("llm_generate", {
            prompt: prompt,
        });
        console.log("llm_generate answer", answer);
        responseOutputEl.setValue(answer as string, {mode: "auto"});
    }
}



