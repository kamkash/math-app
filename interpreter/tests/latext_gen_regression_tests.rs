mod common_latex_gen;

use crate::common_latex_gen::run_test;
use test_log;

//
// 1. Arithmetic
//
#[test_log::test]
fn test_arithmetic_basic() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();

    input_lines.push("a = 10".to_string());
    input_lines.push("b = 20.5".to_string());
    input_lines.push("add\\_res = a + b".to_string()); // LaTeX sum
    checks.push(("add_res", 30.5));

    input_lines.push("mul\\_res = a \\times b".to_string()); // LaTeX ×
    checks.push(("mul_res", 205.0));

    input_lines.push("div\\_res = \\frac{b}{a}".to_string()); // LaTeX fraction
    checks.push(("div_res", 2.05));

    input_lines.push("pow\\_res = a^{2}".to_string()); // LaTeX exponent
    checks.push(("pow_res", 100.0));
    input_lines.push("p = b^{2}^{3}".to_string()); // LaTeX exponent
    checks.push(("p", (20.5f64.powf(2.0)).powf(3.0)));

    run_test(input_lines, checks);
}

//
// 2. Polynomials
//
#[test_log::test]
fn test_polynomials() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();

    input_lines.push("x = 2".to_string());

    input_lines.push("poly1 = (x+1)^{3}".to_string()); // (2+1)^3 = 27
    checks.push(("poly1", 27.0));

    input_lines.push("poly2 = x^{2} - 1".to_string()); // 2^2 - 1 = 3
    checks.push(("poly2", 3.0));

    run_test(input_lines, checks);
}

//
// 3. Rational expressions
//
#[test_log::test]
fn test_rational_expressions() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();

    input_lines.push("x = 3".to_string());

    input_lines.push("rat1 = \\frac{x^{2} - 1}{x - 1}".to_string()); // (9-1)/2 = 4
    checks.push(("rat1", 4.0));

    input_lines.push("rat2 = \\frac{2*x}{4*x}".to_string()); // simplifies to 0.5
    checks.push(("rat2", 0.5));

    run_test(input_lines, checks);
}

//
// 4. Exponentials and Logs
//
#[test_log::test]
fn test_exponentials_and_logs() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();

    input_lines.push("exp1 = e^1".to_string());
    checks.push(("exp1", std::f64::consts::E));

    input_lines.push("exp2 = e^{1}".to_string());
    checks.push(("exp2", std::f64::consts::E));

    input_lines.push("exp3 = e^{3}".to_string());
    checks.push(("exp3", std::f64::consts::E.powf(3.0)));

    input_lines.push("l1 = \\ln(2.0)".to_string()); // ln(2.0) = 0.693147
    checks.push(("l1", 2.0_f64.ln()));

    input_lines.push("l2 = \\ln{(e^{2})}".to_string()); // ln(e^2) = 2
    checks.push(("l2", 2.0));

    input_lines.push("l10 = \\log(100)".to_string()); // log10(100) = 2
    checks.push(("l10", 100.0_f64.log10()));

    // input_lines.push("l20 = \\log_{2}{8}".to_string()); // log base 2 of 8 = 3
    // checks.push(("l20", 3.0));

    run_test(input_lines, checks);
}

//
// 5. Trigonometry
//
#[test_log::test]
fn test_trigonometry() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();

    input_lines.push("x = 10".to_string());
    checks.push(("x", 10.0));
    
    input_lines.push("t1 = \\sin{\\frac{\\pi}{2}}".to_string());
    checks.push(("t1", std::f64::consts::FRAC_PI_2.sin()));

    input_lines.push("t2 = \\cos{0}".to_string());
    checks.push(("t2", 0.0f64.cos()));

    input_lines.push("t3 = \\tan{\\frac{\\pi}{4}}".to_string());
    checks.push(("t3", std::f64::consts::FRAC_PI_4.tan()));

    input_lines.push("t4 = \\sin^{2}{\\frac{\\pi}{6}} + \\cos^{2}{\\frac{\\pi}{6}}".to_string());
    checks.push(("t4", 1.0));
    
    input_lines.push("t5 = \\sin^{x+2}{\\frac{\\pi}{6}}".to_string());
    checks.push(("t5", (std::f64::consts::FRAC_PI_6.sin()).powf(12.0)));

    run_test(input_lines, checks);
}

//
// 6. Hyperbolic
//
#[test_log::test]
fn test_hyperbolic() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();

    input_lines.push("x = 10".to_string());
    checks.push(("x", 10.0));
    
    input_lines.push("h1 = \\sinh{0}".to_string());
    checks.push(("h1", 0.0));

    input_lines.push("h2 = \\cosh{0}".to_string());
    checks.push(("h2", 1.0));

    input_lines.push("h3 = \\tanh{10}".to_string());
    checks.push(("h3", (10.0f64).tanh()));

    input_lines.push("t5 = \\sinh^{x+2}{\\frac{\\pi}{6}}".to_string());
    checks.push(("t5", (std::f64::consts::FRAC_PI_6.sinh()).powf(12.0)));

    run_test(input_lines, checks);
}

//
// 7. Variable Context & Substitution
//
#[test_log::test]
fn test_variable_context() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();

    input_lines.push("a = 10".to_string());
    input_lines.push("b = 20.5".to_string());

    input_lines.push("combo1 = a^{2} + 3*b - 1.5".to_string()); // 100 + 61.5 - 1.5 = 160
    checks.push(("combo1", 160.0));

    input_lines.push("combo2 = \\sqrt{a^{2} + 3*b - 1.5}".to_string()); // sqrt(160)
    checks.push(("combo2", (160.0f64).sqrt()));

    run_test(input_lines, checks);
}
