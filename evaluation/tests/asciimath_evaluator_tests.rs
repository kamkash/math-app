use std::rc::Rc;

use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use asciimath_evaluator::AsciiMathVisitor;
use evaluation::asciimath_evaluator;
use log::info;
use math_parser::gen_parsers::asciimath2lexer::AsciiMath2Lexer;
use math_parser::gen_parsers::asciimath2parser::AsciiMath2Parser;
use symengine_rs::basic::Basic;
use test_log;

#[test_log::test]
fn test_asciimath_eval_expressions() {
    let input = "-1 + 2 = __ans__
                        p = x + y
                        i = 1 - x
                        q = 100000 / w
                        q1 = 100000 / w1
                        q11 != 100000 / w11
                        z * 7 = t + 3.14";
    let mut visitor = AsciiMathVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
}

#[test_log::test]
fn test_asciimath_eval_expressions_parentheses() {
    let input = "p = 100000.00
                        i = 10.00
                        f=(p)/(i)";
    let mut visitor = AsciiMathVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
    assert_eq!(
        visitor.result_table.get(&Basic::symbol("f")),
        Some(&Rc::new(Basic::real(100_000.00f64 / 10.00f64))),
    );
}

#[test_log::test]
fn test_asciimath_compound_interest() {
    let p = 100_000.0f64;
    let i = 0.10f64;
    let n = 5.0f64;
    let compound_interest = p * (1.0 + i).powf(n);
    let input = format!(
        "p = {p} 
         i = {i}
         n = {n}
         compound_interest = p * (1 + i) ^ n "
    );
    let mut visitor = AsciiMathVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);
    assert_eq!(
        visitor
            .result_table
            .get(&Basic::symbol("compound_interest")),
        Some(&Rc::new(Basic::real(compound_interest))),
    );
}

#[test_log::test]
fn test_asciimath_eval_expressions_no_rhs() {
    let input = "-1 + 2 = __ans__
                        x = 10
                        y = 3.14159
                        w = x + y
                        w1 = 1 - x
                        w11 = 1 / (1 - x)^2
                        q = 100000 / w
                        q1 = 100000 / w1
                        q11 != 100000 / w11
                        z * 7 = t + 3.14
                        t = (x)/(y)
                        d=(x)/(y)
                        ";
    let mut visitor = AsciiMathVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
}

#[test_log::test]
fn test_asciimath_eval_trig_expressions() {
    let x: f64 = 33.0;
    let y: f64 = x.sin().powi(2) + x.cos().powi(2);
    let yp: f64 = (2.0 * x).sin().powi(2) + x.powi(2).cos().powi(2);

    let input = format!(
        "x = {:?}    
         y = sin(x)^2 + cos(x)^2
         yp = sin(2 * x)^2 + cos(x^2)^2",
        x
    );
    let mut visitor = AsciiMathVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
    assert_eq!(
        visitor.result_table.get(&Basic::symbol("y")),
        Some(&Rc::new(Basic::real(y))),
    );
    assert_eq!(
        visitor.result_table.get(&Basic::symbol("yp")),
        Some(&Rc::new(Basic::real(yp))),
    );
}

#[test_log::test]
fn test_asciimath_eval_individual_trig_functions() {
    let mut visitor = AsciiMathVisitor::new();
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    let pi = std::f64::consts::PI;

    // --- Standard Trigonometric Functions ---
    // sin
    let x_sin = pi / 6.0;
    input_lines.push(format!("x_sin_val = {}", x_sin));
    input_lines.push("res_sin = sin(x_sin_val)".to_string());
    checks.push(("res_sin", x_sin.sin()));

    // cos
    let x_cos = pi / 3.0;
    input_lines.push(format!("x_cos_val = {}", x_cos));
    input_lines.push("res_cos = cos(x_cos_val)".to_string());
    checks.push(("res_cos", x_cos.cos()));

    // tan
    let x_tan = pi / 4.0;
    input_lines.push(format!("x_tan_val = {}", x_tan));
    input_lines.push("res_tan = tan(x_tan_val)".to_string());
    checks.push(("res_tan", x_tan.tan()));

    // csc
    let x_csc = pi / 6.0;
    input_lines.push(format!("x_csc_val = {}", x_csc));
    input_lines.push("res_csc = csc(x_csc_val)".to_string());
    // checks.push(("res_csc", 1.0 / x_csc.sin()));

    // sec
    let x_sec = pi / 3.0;
    input_lines.push(format!("x_sec_val = {}", x_sec));
    input_lines.push("res_sec = sec(x_sec_val)".to_string());
    checks.push(("res_sec", 1.0 / x_sec.cos()));

    // cot
    let x_cot = pi / 4.0;
    input_lines.push(format!("x_cot_val = {}", x_cot));
    input_lines.push("res_cot = cot(x_cot_val)".to_string());
    // checks.push(("res_cot", 1.0 / x_cot.tan()));

    // --- Inverse Trigonometric Functions ---
    // asin
    let x_asin = 0.5f64;
    input_lines.push(format!("x_asin_val = {}", x_asin));
    input_lines.push("res_asin = asin(x_asin_val)".to_string());
    checks.push(("res_asin", x_asin.asin()));

    // acos
    let x_acos = 0.5f64;
    input_lines.push(format!("x_acos_val = {}", x_acos));
    input_lines.push("res_acos = acos(x_acos_val)".to_string());
    checks.push(("res_acos", x_acos.acos()));

    // atan
    let x_atan = 1.0f64;
    input_lines.push(format!("x_atan_val = {}", x_atan));
    input_lines.push("res_atan = atan(x_atan_val)".to_string());
    // checks.push(("res_atan", x_atan.atan()));

    // acsc
    let x_acsc = 2.0f64;
    input_lines.push(format!("x_acsc_val = {}", x_acsc));
    input_lines.push("res_acsc = acsc(x_acsc_val)".to_string());
    // checks.push(("res_acsc", (1.0 / x_acsc).asin()));

    // asec
    let x_asec = 2.0f64;
    input_lines.push(format!("x_asec_val = {}", x_asec));
    input_lines.push("res_asec = asec(x_asec_val)".to_string());
    // checks.push(("res_asec", (1.0 / x_asec).acos()));

    // acot
    let x_acot = 1.0f64; // For x > 0, acot(x) = atan(1/x)
    input_lines.push(format!("x_acot_val = {}", x_acot));
    input_lines.push("res_acot = acot(x_acot_val)".to_string());
    // checks.push(("res_acot", (1.0 / x_acot).atan()));

    // --- Hyperbolic Trigonometric Functions ---
    // sinh
    let x_sinh = 1.5f64;
    input_lines.push(format!("x_sinh_val = {}", x_sinh));
    input_lines.push("res_sinh = sinh(x_sinh_val)".to_string());
    checks.push(("res_sinh", x_sinh.sinh()));

    // cosh
    let x_cosh = 1.5f64;
    input_lines.push(format!("x_cosh_val = {}", x_cosh));
    input_lines.push("res_cosh = cosh(x_cosh_val)".to_string());
    checks.push(("res_cosh", x_cosh.cosh()));

    // tanh
    let x_tanh = 1.5f64;
    input_lines.push(format!("x_tanh_val = {}", x_tanh));
    input_lines.push("res_tanh = tanh(x_tanh_val)".to_string());
    checks.push(("res_tanh", x_tanh.tanh()));

    // csch
    let x_csch = 1.0f64; // csch(0) is undefined
    input_lines.push(format!("x_csch_val = {}", x_csch));
    input_lines.push("res_csch = csch(x_csch_val)".to_string());
    // checks.push(("res_csch", 1.0 / x_csch.sinh()));

    // sech
    let x_sech = 0.5f64;
    input_lines.push(format!("x_sech_val = {}", x_sech));
    input_lines.push("res_sech = sech(x_sech_val)".to_string());
    // checks.push(("res_sech", 1.0 / x_sech.cosh()));

    // coth
    let x_coth = 1.0f64; // coth(0) is undefined
    input_lines.push(format!("x_coth_val = {}", x_coth));
    input_lines.push("res_coth = coth(x_coth_val)".to_string());
    // checks.push(("res_coth", 1.0 / x_coth.tanh()));

    // --- Inverse Hyperbolic Trigonometric Functions ---
    // asinh
    let x_asinh = 2.5f64;
    input_lines.push(format!("x_asinh_val = {}", x_asinh));
    input_lines.push("res_asinh = asinh(x_asinh_val)".to_string());
    checks.push(("res_asinh", x_asinh.asinh()));

    // acosh
    let x_acosh = 2.5f64; // Domain: x >= 1
    input_lines.push(format!("x_acosh_val = {}", x_acosh));
    input_lines.push("res_acosh = acosh(x_acosh_val)".to_string());
    checks.push(("res_acosh", x_acosh.acosh()));

    // atanh
    let x_atanh = 0.5f64; // Domain: -1 < x < 1
    input_lines.push(format!("x_atanh_val = {}", x_atanh));
    input_lines.push("res_atanh = atanh(x_atanh_val)".to_string());
    checks.push(("res_atanh", x_atanh.atanh()));

    // acsch
    let x_acsch = 1.5f64; // Domain: x != 0
    input_lines.push(format!("x_acsch_val = {}", x_acsch));
    input_lines.push("res_acsch = acsch(x_acsch_val)".to_string());
    checks.push(("res_acsch", (1.0 / x_acsch).asinh()));

    // asech
    let x_asech = 0.5f64; // Domain: 0 < x <= 1
    input_lines.push(format!("x_asech_val = {}", x_asech));
    input_lines.push("res_asech = asech(x_asech_val)".to_string());
    checks.push(("res_asech", (1.0 / x_asech).acosh()));

    // acoth
    let x_acoth = 2.0f64; // Domain: |x| > 1
    input_lines.push(format!("x_acoth_val = {}", x_acoth));
    input_lines.push("res_acoth = acoth(x_acoth_val)".to_string());
    // checks.push(("res_acoth", (1.0 / x_acoth).atanh()));

    let final_input = input_lines.join("\n");
    info!("Full input block for trig functions:\n{}", final_input);

    let lexer = AsciiMath2Lexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!(
        "Visitor result table after trig tests: {:?}",
        visitor.result_table
    );

    for (var_name, expected_f64_val) in checks {
        info!("Checking: {} = {}", var_name, expected_f64_val);
        let actual_basic_rc = visitor
            .result_table
            .get(&Basic::symbol(var_name))
            .unwrap_or_else(|| panic!("Variable {} not found in result_table", var_name));

        let actual_f64 = actual_basic_rc.to_f64().unwrap_or_else(|| {
            panic!(
                "Variable {} is not a RealDouble, it's a {:?}",
                var_name,
                actual_basic_rc.get_type()
            )
        });

        let tolerance = 1e-13;
        assert!(
            (actual_f64 - expected_f64_val).abs() < tolerance,
            "Failed for variable: {}. Expected: {}, Got: {}. Difference: {}",
            var_name,
            expected_f64_val,
            actual_f64,
            (actual_f64 - expected_f64_val).abs()
        );
    }
}

#[test_log::test]
fn test_asciimath_eval_algebraic_operations() {
    let mut visitor = AsciiMathVisitor::new();
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

    // Power
    input_lines.push("pow_res1 = a ^ 2".to_string());
    checks.push(("pow_res1", 10.0f64.powi(2)));
    input_lines.push("pow_res2 = 2.0 ^ 3".to_string());
    checks.push(("pow_res2", 2.0f64.powi(3)));
    input_lines.push("pow_res3 = pow(a, 3)".to_string());
    checks.push(("pow_res3", 10.0f64.powi(3)));
    input_lines.push("pow_res4 = pow(16, 0.5)".to_string());
    checks.push(("pow_res4", 16.0f64.powf(0.5)));

    // Square Root
    input_lines.push("sqrt_res1 = sqrt(100.0)".to_string());
    checks.push(("sqrt_res1", 100.0f64.sqrt()));
    input_lines.push("sqrt_res2 = sqrt(a * 10.0)".to_string()); // sqrt(10*10) = 10
    checks.push(("sqrt_res2", (10.0f64 * 10.0f64).sqrt()));
    input_lines.push("sqrt_res3 = sqrt(2.0)".to_string());
    checks.push(("sqrt_res3", 2.0f64.sqrt()));

    // Combined operations / Order of operations
    input_lines.push("combo_res1 = a + b * 2".to_string()); // 10 + 20.5 * 2 = 10 + 41 = 51
    checks.push(("combo_res1", 10.0 + 20.5 * 2.0));
    input_lines.push("combo_res2 = (a + b) * 2".to_string()); // (10 + 20.5) * 2 = 30.5 * 2 = 61
    checks.push(("combo_res2", (10.0 + 20.5) * 2.0));
    input_lines.push("combo_res3 = a - b / 2 + 1".to_string()); // 10 - 20.5/2 + 1 = 10 - 10.25 + 1 = 0.75
    checks.push(("combo_res3", 10.0 - 20.5 / 2.0 + 1.0));
    input_lines.push("combo_res4 = sqrt(a^2 + 3*b - 1.5)".to_string()); // sqrt(100 + 3*20.5 - 1.5) = sqrt(100 + 61.5 - 1.5) = sqrt(160)
    checks.push(("combo_res4", (10.0f64.powi(2) + 3.0 * 20.5 - 1.5).sqrt()));

    let final_input = input_lines.join("\n");
    info!(
        "Full input block for algebraic operations:\n{}",
        final_input
    );

    let lexer = AsciiMath2Lexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!(
        "Visitor result table after algebraic tests: {:?}",
        visitor.result_table
    );

    let tolerance = 1e-9; // Adjusted tolerance for f64 comparisons

    for (var_name, expected_f64_val) in checks {
        info!("Checking: {} = {}", var_name, expected_f64_val);
        let actual_basic_rc = visitor
            .result_table
            .get(&Basic::symbol(var_name))
            .unwrap_or_else(|| panic!("Variable {} not found in result_table", var_name));

        let actual_f64 = actual_basic_rc.to_f64().unwrap_or_else(|| {
            panic!(
                "Variable {} is not a RealDouble, it's a {:?} type: {}",
                var_name,
                actual_basic_rc.get_type_str(),
                actual_basic_rc.to_string()
            )
        });

        assert!(
            (actual_f64 - expected_f64_val).abs() < tolerance,
            "Failed for variable: {}. Expected: {}, Got: {}. Difference: {}",
            var_name,
            expected_f64_val,
            actual_f64,
            (actual_f64 - expected_f64_val).abs()
        );
    }
}

#[test_log::test]
fn test_asciimath_eval_derivatives() {
    let input = "d/dx(x^3+3x^2-10x+4)V
                (d)/(d x)(sin(x)- 1/3 * x ^ 3) =
                (d)/(dx)(e^(-(t)/(x))-k * x)
                deriv(x^2+3x-10,x)=
                deriv(yx^2+3yx-10,y)=";

    let mut visitor = AsciiMathVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
}

#[test_log::test]
fn test_asciimath_integrals() {
    let input = "int _ 0 ^ 1 (x^3 + 3 * x^2 - 10) dx
                               int _ -oo ^ +oo (sin(t)^3 + cos(t) - t) dt
                               int _ -oo ^ oo (sin(t)^3 + cos(t) - t) dt
                               int _ -1 ^ 1 (sin(t)^3 + cos(t) - t) dt
                               int _ -1 ^ +1 (sin(t)^3 + cos(t) - t) dt";

    let mut visitor = AsciiMathVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
}
