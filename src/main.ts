import "mathlive";
import "mathlive/fonts.css";

import "./solver.ts"; // This import will execute the DOMContentLoaded listener within it.
import "./math-solver"
// import "./math-solver-old"

console.log("Main.ts loaded. Shared logic and math-solver component should be initializing.");

// main.ts is now primarily for importing modules.
// The shared-math-logic.ts module handles the main page's DOMContentLoaded setup.
// The math-solver.ts module defines the custom element.
