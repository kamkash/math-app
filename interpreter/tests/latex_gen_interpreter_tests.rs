use antlr_rust::tree::ParseTreeVisitorCompat; // bring trait into scope
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use giac_rs::r#gen::Gen;
use interpreter::latex_gen_interpreter::LaTeXGenVisitor;
use log::info;
use math_parser::gen_parsers::{latexlexer::LaTeXLexer, latexparser::LaTeXParser};
use test_log;

macro_rules! hide {
    ($($t:tt)*) => {};
}

fn eval_symbol_to_f64(visitor: &mut LaTeXGenVisitor, var_name: &str) -> f64 {
    let actual_sym = visitor
        .result_table
        .get(&Gen::symbol(var_name, &visitor.giac_context).unwrap())
        .unwrap_or_else(|| panic!("Variable {} not found in result_table", var_name));

    // Try direct conversion
    if let Some(v) = actual_sym.to_f64() {
        return v;
    }

    // Try evaluating stored gen
    if let Some(evaled) = actual_sym.eval() {
        if let Some(v) = evaled.to_f64() {
            return v;
        }
    }

    // Try simplifying then evaluating
    if let Some(simp) = actual_sym.simplify() {
        if let Some(evaled) = simp.eval() {
            if let Some(v) = evaled.to_f64() {
                return v;
            }
        }
    }

    // Fallback: assign the expression to the symbol in the context and read it back
    let sym = Gen::symbol(var_name, &visitor.giac_context).unwrap();
    let expr_str = visitor
        .symbol_table
        .get(&sym)
        .map(|g| g.to_string())
        .unwrap_or_else(|| panic!("No expression found for {}", var_name));
    let assign = format!("{} := {}", var_name, expr_str);
    Gen::new(assign.as_str(), &visitor.giac_context).unwrap().eval();
    let read = Gen::new(var_name, &visitor.giac_context).unwrap().eval();
    if let Some(r) = read {
        if let Some(v) = r.to_f64() {
            return v;
        }
    }

    // Some GIAC evaluations produce singleton-list string forms like "[10.0]".
    // Try to parse a numeric value out of a singleton list before failing.
    let s = actual_sym.to_string();
    if s.starts_with('[') && s.ends_with(']') {
        let inner = s.trim_start_matches('[').trim_end_matches(']').trim();
        if let Ok(v) = inner.parse::<f64>() {
            return v;
        }
    }

    panic!("Variable {} did not evaluate to f64, got: {}", var_name, s);
}

static EPSILON: f64 = 1e-8;

#[test_log::test]
fn test_latex_gen_simple_eval() {
    let mut visitor = LaTeXGenVisitor::new();
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    // Variable assignments
    input_lines.push("a = 10.0".to_string());
    checks.push(("a", 10.0));

    let final_input = input_lines.join("\n");
    info!(
        "Full input block for algebraic operations:\n{}",
        final_input
    );

    let lexer = LaTeXLexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = LaTeXParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!(
        "Visitor block expressions after algebraic tests: {:?}",
        visitor.block_expressions
    );

    for (var_name, expected_f64_val) in checks {
        info!("Checking: {} = {}", var_name, expected_f64_val);
        let actual_f64 = eval_symbol_to_f64(&mut visitor, var_name);

        assert!(
            (actual_f64 - expected_f64_val).abs() < EPSILON,
            "Failed for variable: {}. Expected: {}, Got: {}. Difference: {}",
            var_name,
            expected_f64_val,
            actual_f64,
            (actual_f64 - expected_f64_val).abs()
        );
    }
}

#[test_log::test]
fn test_latex_gen_power() {
    let mut visitor = LaTeXGenVisitor::new();
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    // Power tests
    input_lines.push("a = 10.0".to_string());
    input_lines.push("pow_res1 = a ^ 2".to_string());
    checks.push(("pow_res1", 10.0f64.powi(2)));
    input_lines.push("pow_res2 = 2.0 ^ 3".to_string());
    checks.push(("pow_res2", 2.0f64.powi(3)));

    let final_input = input_lines.join("\n");
    let lexer = LaTeXLexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = LaTeXParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    for (var_name, expected_val) in checks {
        let actual_f64 = eval_symbol_to_f64(&mut visitor, var_name);
        info!("Checking: {} = {} actual {}", var_name, expected_val, actual_f64);

        assert!(
            (actual_f64 - expected_val).abs() < EPSILON,
            "Failed for variable: {}. Expected: {}, Got: {}. Difference: {}",
            var_name,
            expected_val,
            actual_f64,
            (actual_f64 - expected_val).abs()
        );
    }
}

#[test_log::test]
fn test_latex_gen_eval() {
    let mut visitor = LaTeXGenVisitor::new();
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
    checks.push(("cbrt_res", 8.0f64.powf(1.0/3.0)));
    input_lines.push("fourth_root_res = \\sqrt[4]{16.0}".to_string()); // 4th root of 16 = 2
    checks.push(("fourth_root_res", 16.0f64.powf(1.0/4.0)));
    
    hide! {
        // Combined operations / Order of operations
        input_lines.push("combo_res1 = a + b * 2".to_string()); // 10 + 20.5 * 2 = 10 + 41 = 51
        checks.push(("combo_res1", 10.0 + 20.5 * 2.0));
        input_lines.push("combo_res2 = (a + b) * 2".to_string()); // (10 + 20.5) * 2 = 30.5 * 2 = 61
        checks.push(("combo_res2", (10.0 + 20.5) * 2.0));
        input_lines.push("combo_res3 = a - b / 2 + 1".to_string()); // 10 - 20.5/2 + 1 = 10 - 10.25 + 1 = 0.75
        checks.push(("combo_res3", 10.0 - 20.5 / 2.0 + 1.0));
        input_lines.push("combo_res4 = sqrt(a^2 + 3*b - 1.5)".to_string()); // sqrt(100 + 3*20.5 - 1.5) = sqrt(100 + 61.5 - 1.5) = sqrt(160)
        checks.push(("combo_res4", (10.0f64.powi(2) + 3.0 * 20.5 - 1.5).sqrt()));
    }

    let final_input = input_lines.join("\n");
    info!(
        "Full input block for algebraic operations:\n{}",
        final_input
    );

    let lexer = LaTeXLexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = LaTeXParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!(
        "Visitor block expressions after algebraic tests: {:?}",
        visitor.block_expressions
    );

    for (var_name, expected_f64_val) in checks {
        info!("Checking: {} = {}", var_name, expected_f64_val);
        let actual_f64 = eval_symbol_to_f64(&mut visitor, var_name);

        assert!(
            (actual_f64 - expected_f64_val).abs() < EPSILON,
            "Failed for variable: {}. Expected: {}, Got: {}. Difference: {}",
            var_name,
            expected_f64_val,
            actual_f64,
            (actual_f64 - expected_f64_val).abs()
        );
    }
}

#[test_log::test]
fn test_latex_gen_implicit_multiply() {
    let mut visitor = LaTeXGenVisitor::new();
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    // Variable assignments
    input_lines.push("a = 10.0".to_string());
    checks.push(("a", 10.0));
    input_lines.push("b = 3.1415926".to_string());
    checks.push(("b", 3.1415926));
    // input_lines.push("y = 2a".to_string());
    // checks.push(("y", 2.0 * 10.0));
    input_lines.push("y_p = 2*a".to_string());
    checks.push(("y_p", 2.0 * 10.0));
    input_lines.push("t = 2*a*b".to_string());
    checks.push(("t", 2.0 * 10.0 * 3.1415926));

    // t=2ab : todo

    let final_input = input_lines.join("\n");
    info!(
        "Full input block for algebraic operations:\n{}",
        final_input
    );

    let lexer = LaTeXLexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = LaTeXParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!(
        "Visitor block expressions after algebraic tests: {:?}",
        visitor.block_expressions
    );

    for (var_name, expected_f64_val) in checks {
        info!("Checking: {} = {}", var_name, expected_f64_val);
        let actual_f64 = eval_symbol_to_f64(&mut visitor, var_name);

        assert!(
            (actual_f64 - expected_f64_val).abs() < EPSILON,
            "Failed for variable: {}. Expected: {}, Got: {}. Difference: {}",
            var_name,
            expected_f64_val,
            actual_f64,
            (actual_f64 - expected_f64_val).abs()
        );
    }
}


#[test_log::test]
fn test_latex_gen_group() {
    let mut visitor = LaTeXGenVisitor::new();
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    input_lines.push("a = 10.0".to_string());
    checks.push(("a", 10.0));
    input_lines.push("b = 2.0*(1.0 + a)".to_string());
    checks.push(("b", 2.0 * (1.0 + 10.0)));

    let final_input = input_lines.join("\n");
    info!(
        "Full input block for algebraic operations:\n{}",
        final_input
    );

    let lexer = LaTeXLexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = LaTeXParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!(
        "Visitor block expressions after algebraic tests: {:?}",
        visitor.block_expressions
    );

    for (var_name, expected_f64_val) in checks {
        info!("Checking: {} = {}", var_name, expected_f64_val);
        let actual_f64 = eval_symbol_to_f64(&mut visitor, var_name);

        assert!(
            (actual_f64 - expected_f64_val).abs() < EPSILON,
            "Failed for variable: {}. Expected: {}, Got: {}. Difference: {}",
            var_name,
            expected_f64_val,
            actual_f64,
            (actual_f64 - expected_f64_val).abs()
        );
    }
}