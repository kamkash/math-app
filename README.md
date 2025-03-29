# Tauri + Vanilla TS

This template should help get you started developing with Tauri in vanilla HTML, CSS and Typescript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)



## WSL
- export DISPLAY=:0
- npm run tauri dev
- npm run tauri build


## DeepSeek Math
- https://github.com/deepseek-ai/DeepSeek-Math


## llama.cpp
./run.sh -n 2048 "what is 25% of 256.00. IMP: give the output in a valid JSON string"
./run.sh -n 2048 "solve x^2 - 2x + 9. IMP: give the output in a valid JSON string" 

### run.sh
```
#!/bin/bash
MODEL_FILE=/Users/kamran/.lmstudio/models/lmstudio-community/DeepSeek-R1-Distill-Qwen-7B-GGUF/DeepSeek-R1-Distill-Qwen-7B-Q4_K_M.gguf
LLAMA_BUILD_PATH=/Users/kamran/llama.cpp/build/bin
DYLD_LIBRARY_PATH=.:$DYLD_LIBRARY_PATH:$LLAMA_BUILD_PATH ./target/main -m $MODEL_FILE $*
```

### Rust llm-chain
https://github.com/sobelio/llm-chain


To embed a LaTeX editor for mathematical symbol editing inside your JavaScript and CSS-based UI, you have several options:

## CodeCogs Equation Editor

The CodeCogs Equation Editor is a popular choice for embedding LaTeX editing capabilities into web applications[1][5].

1. Include the necessary scripts and stylesheets in your HTML:

```html
<head>
  <script src="https://editor.codecogs.com/eqneditor.api.min.js"></script>
  <link rel="stylesheet" href="https://editor.codecogs.com/eqneditor.css"/>
</head>
```

2. Add the required HTML elements:

```html
<div id="equation-editor">
  <div id="toolbar"></div>
  <div id="latexInput" placeholder="Write Equation here..."></div>
  <div id="equation-output">
    <img id="output">
  </div>
</div>
```

3. Initialize the editor with JavaScript:

```javascript
window.onload = function () {
  const textarea = EqEditor.TextArea.link('latexInput')
    .addOutput(new EqEditor.Output('output'));
  EqEditor.Toolbar.link('toolbar').addTextArea(textarea);
};
```

This setup provides a customizable LaTeX editor that can be seamlessly integrated into your UI[5].

## MathJax

While not a full-fledged editor, MathJax is a powerful JavaScript library for rendering mathematical notation in web browsers[2]. You can combine it with a text input to create a LaTeX editing experience:

1. Include MathJax in your HTML:

```html
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js"></script>
```

2. Create an input area and an output div in your HTML:

```html
<textarea id="latex-input"></textarea>
<div id="math-output"></div>
```

3. Use JavaScript to render the LaTeX input:

```javascript
const input = document.getElementById('latex-input');
const output = document.getElementById('math-output');

input.addEventListener('input', () => {
  output.innerHTML = '\\[' + input.value + '\\]';
  MathJax.typeset([output]);
});
```

This approach allows you to create a custom UI around the LaTeX input while leveraging MathJax for high-quality rendering.

Both solutions offer different levels of integration and customization. CodeCogs provides a more complete editing experience out of the box, while the MathJax approach gives you more control over the UI but requires more setup for editing features.

Sources
[1] Equation Editor for online mathematics - create, integrate and ... https://editor.codecogs.com
[2] MathJax | Beautiful math in all browsers. https://www.mathjax.org
[3] About | CodeCogs Equation Editor https://www.codecogs.com/latex/about.php
[4] About the Equation Editor - TCU Online - Texas Christian University https://tcuonline.tcu.edu/kb/html-editor-equation-editor/
[5] 0-quickstart - Equation Editor - CodeCogs https://editor.codecogs.com/docs
[6] 2-API - Equation Editor - CodeCogs https://editor.codecogs.com/docs/2-API.php
[7] 1-about - Equation Editor - CodeCogs https://editor.codecogs.com/support
[8] HostMath - Online LaTeX formula editor and browser-based math ... https://www.hostmath.com
[9] iMathEQ - Mathematics Equation Editor http://www.imatheq.com
[10] WYSIWYG LaTeX editor for maths - TeX - LaTeX Stack Exchange https://tex.stackexchange.com/questions/57068/wysiwyg-latex-editor-for-maths
[11] Is there a javascript LaTeX equation renderer? - Stack Overflow https://stackoverflow.com/questions/3284131/is-there-a-javascript-latex-equation-renderer
[12] Best Latex Writing tool? : r/math - Reddit https://www.reddit.com/r/math/comments/uuiklv/best_latex_writing_tool/
[13] Writing Mathematics for MathJax https://docs.mathjax.org/en/latest/basic/mathematics.html
[14] Interactive tools to make math diagrams in LaTeX - TeX https://tex.stackexchange.com/questions/561504/interactive-tools-to-make-math-diagrams-in-latex
[15] Embedding LaTeX equations into a webpage - TeX https://tex.stackexchange.com/questions/129/embedding-latex-equations-into-a-webpage
[16] Online Latex Equation Editor - Sciweavers https://www.sciweavers.org/free-online-latex-equation-editor
[17] Mathematical expressions - Overleaf, Online LaTeX Editor https://www.overleaf.com/learn/latex/Mathematical_expressions
[18] Is there an LaTeX editor embeddable in a web site? https://softwarerecs.stackexchange.com/questions/52857/is-there-an-latex-editor-embeddable-in-a-web-site
[19] Visual Math Editor :: EquaThEque https://visualmatheditor.equatheque.net
[20] Configuring MathJax — MathJax 3.2 documentation https://docs.mathjax.org/en/latest/options/index.html
[21] Introducing KaTeX (Math) and Embed Support - HYVOR https://hyvor.com/blog/introducing-katex-embed
[22] Loading and Configuring MathJax https://docs.mathjax.org/en/stable/configuration.html
[23] Announcing KaTeX Support: Easily Embed Mathematical Formulas ... https://mistral.bloggrify.com/2024/katex
[24] using code cogs equation editor - YouTube https://www.youtube.com/watch?v=3EvDev4zbtI
[25] The Core Configuration Options — MathJax 2.0 documentation https://docs.mathjax.org/en/stable/options/hub.html
[26] Embedding a JavaScript Library (KaTeX) in Go - hjr265.me https://hjr265.me/blog/embedding-a-javascript-library-katex-in-go/
[27] Codecogs Latex Equation Editor Plugin - WordPress.com https://wordpress.com/plugins/codecogs-latex-equation-editor
[28] MathJax basic tutorial and quick reference - Mathematics Meta https://math.meta.stackexchange.com/questions/5020/mathjax-basic-tutorial-and-quick-reference
[29] KaTeX – The fastest math typesetting library for the web https://katex.org
[30] codecogs like software/ IDE/ LaTeX environment ? - TeX https://tex.stackexchange.com/questions/12062/codecogs-like-software-ide-latex-environment
[31] Getting Started — MathJax 2.0 documentation https://docs.mathjax.org/en/stable/start.html
[32] What is the best way to embed LaTeX in a webpage? - Stack Overflow https://stackoverflow.com/questions/116054/what-is-the-best-way-to-embed-latex-in-a-webpage
[33] Solved: [ARCHIVED] Best way to add LaTeX equations https://community.canvaslms.com/thread/21249-best-way-to-add-latex-equations
[34] How to embed Maths Symbols and Equations in Ajax Editor https://stackoverflow.com/questions/2643319/how-to-embed-maths-symbols-and-equations-in-ajax-editor?rq=3
[35] Insert and edit equations with Equation Editor - Brightspace https://community.d2l.com/brightspace/kb/articles/3393-insert-and-edit-equations-with-equation-editor
[36] Writing mathematical expressions - GitHub Docs https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/writing-mathematical-expressions
[37] Supported Functions - KaTeX https://katex.org/docs/supported.html
[38] Configuring and Loading MathJax https://docs.mathjax.org/en/latest/web/configuration.html
[39] [PDF] katex: Rendering Math to HTML, 'MathML', or R-Documentation Format https://cran.rstudio.com/web/packages/katex/katex.pdf


