use test_log;
mod common_latex_gen;
use crate::common_latex_gen::run_test;

macro_rules! _hide {
    ($($t:tt)*) => {};
}

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
    input_lines.push("a = 10.0".to_string());
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
