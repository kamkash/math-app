use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use interpreter::asciimath_basic_interpreter::AsciiMathBasicVisitor;
use log::info;
use math_parser::gen_parsers::asciimath2lexer::AsciiMath2Lexer;
use math_parser::gen_parsers::asciimath2parser::AsciiMath2Parser;
use symengine_rs::basic::Basic;
use test_log;

static EPSILON: f64 = 1e-8;

#[test_log::test]
fn test_asciimath_basic_visitor_eval() {
    let x = 10.0f64;
    let fz = x.powf(3.0) + x.powf(2.0) / 3.0 - 9.0 * x + 21.0;
    let fy = x.powf(2.0) + 2.0 * x + 5.0;
    let fzz = x.powf(3.0).powf(3.0) + x.powf(2.0) / 3.0 - 9.0 * x + 21.0;
    let input = "x=10
                z = x^3 + x^2 / 3.0 - 9.0 * x + 21.0
                y = x^2 + 2*x + 5
                zz = x^3^3 + x^2 / 3.0 - 9.0 * x + 21.0
                ";
    let mut visitor = AsciiMathBasicVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!("input: {}", input);
    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);
    info!("fz: {}", fz);
    info!("fy: {}", fy);
    info!("fzz: {}", fzz);
    let z_actual = visitor
        .result_table
        .get(&Basic::symbol("z"))
        .and_then(|b| b.to_f64())
        .unwrap();
    let y_actual = visitor
        .result_table
        .get(&Basic::symbol("y"))
        .and_then(|b| b.to_f64())
        .unwrap();
    let zz_actual = visitor
        .result_table
        .get(&Basic::symbol("zz"))
        .and_then(|b| b.to_f64())
        .unwrap();

    assert!(
        (y_actual - fy).abs() < EPSILON,
        "actual: {}, expected: {}",
        y_actual,
        fy
    );
    assert!(
        (z_actual - fz).abs() < EPSILON,
        "actual: {}, expected: {}",
        z_actual,
        fz
    );
    assert!(
        (zz_actual - fzz).abs() < EPSILON,
        "actual: {}, expected: {}",
        zz_actual,
        fzz
    );
}

#[test_log::test]
fn test_asciimath_basic_comp_interest_eval() {
    let p = 100_000.0f64;
    let i = 0.10f64;
    let n = 5.0f64;
    let compound_interest = p * (1.0 + i).powf(n);
    let comp_input = format!(
        "p = {p} 
         i = {i}
         n = {n}
         compound_interest = p * (1 + i) ^ n 
         comp2 = p(1 + i) ^ n
         "
    );

    info!("compound interest input: {}", comp_input);
    let mut visitor = AsciiMathBasicVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(comp_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);

    let comp_interest_actual = visitor
        .result_table
        .get(&Basic::symbol("compound_interest"))
        .and_then(|b| b.to_f64())
        .unwrap();
    let comp2_actual = visitor
        .result_table
        .get(&Basic::symbol("comp2"))
        .and_then(|b| b.to_f64())
        .unwrap();
    info!("compound interest: {}", comp_interest_actual);
    assert!(
        (comp_interest_actual - compound_interest).abs() < EPSILON,
        "actual: {}, expected: {}",
        comp_interest_actual,
        compound_interest
    );
    assert!(
        (comp2_actual - compound_interest).abs() < EPSILON,
        "comp2_actual: {}, expected: {}",
        comp2_actual,
        compound_interest
    );
}

#[test_log::test]
fn test_asciimath_basic_func() {
    let input = "a = 10.0
                         b = 20.5
                         y =  sqrt(a^2 + 3*b - 1.5)";
    let mut visitor = AsciiMathBasicVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);
}

#[test_log::test]
fn test_asciimath_basic_implicit_multiply() {
    let exp_str = "x^3 + 3x^2 - 5x + 2";
    let [x] = Basic::symbols(["x"]);
    let exp = Basic::parse(exp_str).unwrap();
    let inp = format!("{} = 10\ny = {}", x, exp_str);

    let expected = Basic::subs(&exp, vec![(&x, &Basic::real(10.0f64))].into_iter());
    info!("input: {}", inp);
    info!("expected: {}", expected);

    let mut visitor = AsciiMathBasicVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(inp.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);

    let actual = visitor
        .result_table
        .get(&Basic::symbol("y"))
        .and_then(|b| b.to_f64())
        .unwrap();

    assert!(
        (actual - expected.to_f64().unwrap()).abs() < EPSILON,
        "actual: {}, expected: {}",
        actual,
        expected
    );
}

#[test_log::test]
fn test_asciimath_basic_eval_algebraic_operations() {
    let mut visitor = AsciiMathBasicVisitor::new();
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
    // input_lines.push("pow_res1 = a ^ 2".to_string());
    // checks.push(("pow_res1", 10.0f64.powi(2)));
    // input_lines.push("pow_res2 = 2.0 ^ 3".to_string());
    // checks.push(("pow_res2", 2.0f64.powi(3)));
    // input_lines.push("pow_res3 = pow(a, 3)".to_string());
    // checks.push(("pow_res3", 10.0f64.powi(3)));
    // input_lines.push("pow_res4 = pow(16, 0.5)".to_string());
    // checks.push(("pow_res4", 16.0f64.powf(0.5)));

    // // Square Root
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
fn test_asciimath_basic_explicit_functions() {
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    input_lines.push("z = sin(10.0)".to_string());
    checks.push(("z", (10.0f64).sin()));
    input_lines.push("z1 = sin(10.0)^2 + cos(10.0)^2".to_string());
    checks.push(("z1", (10.0f64).sin().powf(2.0) + (10.0f64).cos().powf(2.0)));
    input_lines.push("z2 = sqrt(144.0)".to_string());
    checks.push(("z2", (144.0f64).sqrt()));
    input_lines.push("z3 = log(100.0)".to_string());
    checks.push(("z3", (100.0f64).ln()));

    let final_input = input_lines.join("\n");
    info!(
        "Full input block for algebraic operations:\n{}",
        final_input
    );

    let mut visitor = AsciiMathBasicVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    let tolerance = 1e-9; // Adjusted tolerance for f64 comparisons
                          // for (var_name, expected_f64_val) in checks {
    for (i, (var_name, expected_f64_val)) in checks.iter().enumerate() {
        info!(
            "Checking: in {}, {} = {}",
            input_lines[i], var_name, expected_f64_val
        );
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
