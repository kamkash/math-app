use test_log;
mod common_latex_gen;
use crate::common_latex_gen::run_test;

#[test_log::test]
fn test_latex_gen_simple_eval() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    // Variable assignments
    input_lines.push("a = 10.0".to_string());
    checks.push(("a", 10.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_latex_gen_power() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    // Power tests
    input_lines.push("a = 10.0".to_string());
    input_lines.push("pow_res1 = a ^ 2".to_string());
    checks.push(("pow_res1", 10.0f64.powi(2)));
    input_lines.push("pow_res2 = 2.0 ^ 3".to_string());
    checks.push(("pow_res2", 2.0f64.powi(3)));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_latex_gen_eval() {
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    // Variable assignments
    input_lines.push("a = 10.0".to_string());
    input_lines.push("b = 20.5".to_string());
    checks.push(("a", 10.0));
    checks.push(("b", 20.5));

    // Addition
    input_lines.push("add_res1 = a + b".to_string());
    checks.push(("add_res1", 10.0 + 20.5));
    input_lines.push("add_res2 = 5 + 15.5".to_string());
    checks.push(("add_res2", 5.0 + 15.5));

    // Subtraction
    input_lines.push("sub_res1 = b - a".to_string());
    checks.push(("sub_res1", 20.5 - 10.0));
    input_lines.push("sub_res2 = 100 - 25.25".to_string());
    checks.push(("sub_res2", 100.0 - 25.25));

    // Multiplication
    input_lines.push("mul_res1 = a * b".to_string());
    checks.push(("mul_res1", 10.0 * 20.5));
    input_lines.push("mul_res2 = 2.5 * 4".to_string());
    checks.push(("mul_res2", 2.5 * 4.0));
    input_lines.push("mul_res3 = 3 * a".to_string());
    checks.push(("mul_res3", 3.0 * 10.0));

    // Division
    input_lines.push("div_res1 = b / a".to_string());
    checks.push(("div_res1", 20.5 / 10.0));
    input_lines.push("div_res2 = 10.0 / 4.0".to_string());
    checks.push(("div_res2", 10.0 / 4.0));
    input_lines.push("div_res3 = a / 2.0".to_string());
    checks.push(("div_res3", 10.0 / 2.0));

    // Power tests
    input_lines.push("pow_res1 = a ^ 2".to_string());
    checks.push(("pow_res1", 10.0f64.powi(2)));
    input_lines.push("pow_res2 = 2.0 ^ 3".to_string());
    checks.push(("pow_res2", 2.0f64.powi(3)));

    // Square Root
    input_lines.push("sqrt_res1 = \\sqrt{100.0}".to_string());
    checks.push(("sqrt_res1", 100.0f64.sqrt()));
    input_lines.push("sqrt_res2 = \\sqrt{a * 10.0}".to_string()); // sqrt(10*10) = 10
    checks.push(("sqrt_res2", (10.0f64 * 10.0f64).sqrt()));
    input_lines.push("sqrt_res3 = \\sqrt{2.0}".to_string());
    checks.push(("sqrt_res3", 2.0f64.sqrt()));

    // Nth Root (with root parameter)
    input_lines.push("cbrt_res = \\sqrt[3]{8.0}".to_string()); // cube root of 8 = 2
    checks.push(("cbrt_res", 8.0f64.powf(1.0 / 3.0)));
    input_lines.push("fourth_root_res = \\sqrt[4]{16.0}".to_string()); // 4th root of 16 = 2
    checks.push(("fourth_root_res", 16.0f64.powf(1.0 / 4.0)));

    // Combined operations / Order of operations
    input_lines.push("combo_res1 = a + b * 2".to_string()); // 10 + 20.5 * 2 = 10 + 41 = 51
    checks.push(("combo_res1", 10.0 + 20.5 * 2.0));
    input_lines.push("combo_res2 = (a + b) * 2".to_string()); // (10 + 20.5) * 2 = 30.5 * 2 = 61
    checks.push(("combo_res2", (10.0 + 20.5) * 2.0));
    input_lines.push("combo_res3 = a - b / 2 + 1".to_string()); // 10 - 20.5/2 + 1 = 10 - 10.25 + 1 = 0.75
    checks.push(("combo_res3", 10.0 - 20.5 / 2.0 + 1.0));
    input_lines.push("combo_res4 = \\sqrt{a^2 + 3*b - 1.5}".to_string()); // sqrt(100 + 3*20.5 - 1.5) = sqrt(100 + 61.5 - 1.5) = sqrt(160)
    checks.push(("combo_res4", (10.0f64.powi(2) + 3.0 * 20.5 - 1.5).sqrt()));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_latex_gen_group() {
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    input_lines.push("a = 10.0".to_string());
    checks.push(("a", 10.0));
    input_lines.push("b = 2.0*(1.0 + a)".to_string());
    checks.push(("b", 2.0 * (1.0 + 10.0)));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_atom_text_ignored() {
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    input_lines.push(r"\text{solve the following:}".to_string());
    input_lines.push("x = 2".to_string());
    checks.push(("x", 2.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_inline_dollar() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    input_lines.push(r"$a = 10$".to_string());
    checks.push(("a", 10.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_inline_paren() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    input_lines.push(r"\(b = 20.5\)".to_string());
    checks.push(("b", 20.5));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_block_bracket() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    input_lines.push(r"\[ x = 5 \]".to_string());
    checks.push(("x", 5.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_block_equation() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    input_lines.push(r"\begin{equation} y = 3 \end{equation}".to_string());
    checks.push(("y", 3.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_block_equation_star() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    input_lines.push(r"\begin{equation*} z = 7 \end{equation*}".to_string());
    checks.push(("z", 7.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_block_equation_with_label() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    input_lines.push(r"\begin{equation} E = 4 \label{eq:energy} \end{equation}".to_string());
    checks.push(("E", 4.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_align() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    input_lines.push(
        r"\begin{align}
          f = 12 \\
          g &= 24
        \end{align}".to_string()
    );
    checks.push(("f", 12.0));
    checks.push(("g", 24.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_align_star() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    input_lines.push(
        r"\begin{align*}
          h &= 100 \\
          k = 200
        \end{align*}".to_string()
    );
    checks.push(("h", 100.0));
    checks.push(("k", 200.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_block_bracket_multiline() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();
    input_lines.push(
        r"\[
          p = 50 \\
          q &= 150
        \]".to_string()
    );
    checks.push(("p", 50.0));
    checks.push(("q", 150.0));
    run_test(input_lines, checks);
}

#[test_log::test]
fn test_latex_gen_custom_commands() {
    let mut input_lines = Vec::new();
    let mut checks = Vec::new();

    // 1. Solve: x - 10 (implies = 0) => x = 10
    // GIAC solve(x-10=0) usually returns [10.0]. 
    // LaTeXGenVisitor handles singleton lists in eval_symbol_to_f64.
    input_lines.push(r"sol_res = \solve{x - 10}".to_string());
    checks.push(("sol_res", 10.0));

    // 2. Factor: x^2 - 1 => (x-1)(x+1). 
    // We evaluate by setting x = 5. (5-1)*(5+1) = 24.
    input_lines.push("x = 5.0".to_string());
    input_lines.push(r"fac_res = \factor{x^2 - 1}".to_string());
    checks.push(("fac_res", 24.0));

    // 3. Diff: d/dx(x^3) = 3*x^2. With x=5, 3*25 = 75.
    input_lines.push(r"diff_res = \diff{x^3}{x}".to_string());
    checks.push(("diff_res", 75.0));

    // 4. Integrate: int(x, x, 0, 2) = [x^2/2]_0^2 = 2.
    // Note: GIAC integrate(expr, var, lower, upper)
    input_lines.push(r"int_res = \integrate{x}{x}{0}{2}".to_string());
    checks.push(("int_res", 2.0));

    // 5. Indefinite Integrate: int(x, x) = x^2/2. With x=5, 12.5.
    input_lines.push(r"int_indef = \integrate{x}{x}".to_string());
    checks.push(("int_indef", 12.5));

    run_test(input_lines, checks);
}
