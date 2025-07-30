use antlr_rust::tree::ParseTreeVisitorCompat; // Add this line to bring the trait into scope
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use giac_rs::r#gen::Gen;
use interpreter::asciimath_gen_interpreter::AsciiMathGenVisitor;
use log::info;
use math_parser::gen_parsers::{
    asciimath2lexer::AsciiMath2Lexer, asciimath2parser::AsciiMath2Parser,
};
use test_log;
static EPSILON: f64 = 1e-8;

#[test_log::test]
fn test_asciimath_gen_eval_algebraic_operations() {
    let mut visitor = AsciiMathGenVisitor::new();
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
    input_lines.push("sub_res2 = 15.5 - 5".to_string());
    checks.push(("sub_res2", 15.5 - 5.0));

    // Multiplication
    input_lines.push("mul_res1 = a * b".to_string());
    checks.push(("mul_res1", 10.0 * 20.5));
    input_lines.push("mul_res2 = 3 * 7".to_string());
    checks.push(("mul_res2", 3.0 * 7.0));

    // Division
    input_lines.push("div_res1 = b / a".to_string());
    checks.push(("div_res1", 20.5 / 10.0));
    input_lines.push("div_res2 = 21 / 7".to_string());
    checks.push(("div_res2", 21.0 / 7.0));

    // Power
    input_lines.push("exp_res1 = a ^ 2".to_string());
    checks.push(("exp_res1", 10.0_f64.powf(2.0)));
    input_lines.push("exp_res2 = 2 ^ 3".to_string());
    checks.push(("exp_res2", 2.0_f64.powf(3.0)));

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

    info!("visitor block result: {:?}", visitor.block_expressions);
    // info!("visitor symbol table: {:?}", visitor.symbol_table);
    // info!("visitor result table: {:?}", visitor.result_table);

    for (var_name, expected_f64_val) in checks {
        info!("Checking: {} = {}", var_name, expected_f64_val);
        let actual_gen_rc = visitor
            .result_table
            .get(&Gen::symbol(var_name, &visitor.giac_context).unwrap())
            .unwrap_or_else(|| panic!("Variable {} not found in result_table", var_name));

        let actual_f64 = actual_gen_rc.to_f64().unwrap_or_else(|| {
            panic!(
                "Variable {} is not a RealDouble, it's a type: {}",
                var_name,
                actual_gen_rc.to_string()
            )
        });

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
fn test_asciimath_gen_comp_interest_eval() {
    let p = 100_000.0f64;
    let i = 0.10f64;
    let n = 5.0f64;
    let compound_interest = p * (1.0 + i).powf(n);
    let comp_input = format!(
        "p = {p} 
         interest = {i}
         n = {n}
         compound_interest = p * (1 + interest) ^ n 
         comp2 = p(1 + interest) ^ n
         "
    );

    info!("compound interest input: {}", comp_input);
    let mut visitor = AsciiMathGenVisitor::new();
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
        .get(&Gen::symbol("compound_interest", &visitor.giac_context).unwrap())
        .and_then(|b| b.to_f64())
        .unwrap();
    let comp2_actual = visitor
        .result_table
        .get(&Gen::symbol("comp2", &visitor.giac_context).unwrap())
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
fn test_asciimath_gen_func() {
    let input = "a = 10.0
                         b = 20.5
                         y =  sqrt(a^2 + 3*b - 1.5)";
    let y_actual: f64  = (10.0f64 * 10.0 + 3.0 * 20.5 - 1.5).sqrt();
    let mut visitor = AsciiMathGenVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("visitor block result: {:?}", visitor.block_expressions);
    // info!("visitor symbol table: {:?}", visitor.symbol_table);
    // info!("visitor result table: {:?}", visitor.result_table);
    
    let y_computed = visitor
        .result_table
        .get(&Gen::symbol("y", &visitor.giac_context).unwrap())
        .and_then(|b| b.to_f64())
        .unwrap();
    assert!(
        (y_actual - y_computed).abs() < EPSILON,
        "actual: {}, computed: {}",
        y_actual,
        y_computed
    );
}



#[test_log::test]
fn test_asciimath_gen_builtin_functions() {
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    input_lines.push("ang = 45.0".to_string());
    checks.push(("ang", 45.0f64));
    input_lines.push("w = sin(ang)".to_string());
    checks.push(("w", (45.0f64).sin()));
    input_lines.push("z = sin(10.0)".to_string());
    checks.push(("z", (10.0f64).sin()));
    input_lines.push("z1 = sin(10.0)^2 + cos(10.0)^2".to_string());
    checks.push(("z1", (10.0f64).sin().powf(2.0) + (10.0f64).cos().powf(2.0)));
    input_lines.push("z2 = sqrt(144.0)".to_string());
    checks.push(("z2", (144.0f64).sqrt()));
    input_lines.push("z3 = log(100.0)".to_string());
    checks.push(("z3", (100.0f64).log10()));
    input_lines.push("z4 = pi / 2.0".to_string());
    checks.push(("z4", std::f64::consts::PI / 2.0));
    input_lines.push("z5 = ln(100.0)".to_string());
    checks.push(("z5", (100.0f64).ln()));
    input_lines.push("z6 = ln(ang)".to_string());
    checks.push(("z6", (45.0f64).ln()));

    let final_input = input_lines.join("\n");
    info!(
        "Full input block for algebraic operations:\n {}",
        final_input
    );

    let mut visitor = AsciiMathGenVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!(
        "Visitor block expressions for builtin function tests: {:?}",
        visitor.block_expressions
    );

    for (i, (var_name, expected_f64_val)) in checks.iter().enumerate() {
        info!(
            "Checking: in {}, {} = {}",
            input_lines[i], var_name, expected_f64_val
        );
        let actual_gen_rc = visitor
            .result_table
            .get(&Gen::symbol(var_name, &visitor.giac_context).unwrap())
            .unwrap_or_else(|| panic!("Variable {} not found in result_table", var_name));

        let actual_f64 = actual_gen_rc.to_f64().unwrap_or_else(|| {
            panic!(
                "Variable {} is not a RealDouble, var symbol {}",
                var_name,
                actual_gen_rc.to_string()
            )
        });

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
fn test_asciimath_gen_log_functions() {
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    input_lines.push("ang = 45.0".to_string());
    checks.push(("ang", 45.0f64));
    input_lines.push("z3 = log(100.0)".to_string());
    checks.push(("z3", (100.0f64).log10()));
    input_lines.push("z5 = ln(100.0)".to_string());
    checks.push(("z5", (100.0f64).ln()));
    input_lines.push("z6 = ln(ang)".to_string());
    checks.push(("z6", (45.0f64).ln()));

    let final_input = input_lines.join("\n");
    info!(
        "Full input block for algebraic operations:\n {}",
        final_input
    );

    let mut visitor = AsciiMathGenVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!(
        "Visitor block expressions for builtin function tests: {:?}",
        visitor.block_expressions
    );

    for (i, (var_name, expected_f64_val)) in checks.iter().enumerate() {
        info!(
            "Checking: in {}, {} = {}",
            input_lines[i], var_name, expected_f64_val
        );
        let actual_gen_rc = visitor
            .result_table
            .get(&Gen::symbol(var_name, &visitor.giac_context).unwrap())
            .unwrap_or_else(|| panic!("Variable {} not found in result_table", var_name));

        let actual_f64 = actual_gen_rc.to_f64().unwrap_or_else(|| {
            panic!(
                "Variable {} is not a RealDouble, var symbol {}",
                var_name,
                actual_gen_rc.to_string()
            )
        });

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
fn test_asciimath_basic_subscripted_function() {
    let x = 10.0f64;
    let y = 15.0f64;
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    input_lines.push("x = 10.0".to_string());
    input_lines.push("y = 15.0".to_string());
    input_lines.push("y0 = log(x)".to_string());
    input_lines.push("y1 = ln(x)".to_string());
    input_lines.push("y2 = log _10 (x)".to_string());
    input_lines.push("z = (log)_2(x)".to_string());
    input_lines.push("q = exp(x)".to_string());
    input_lines.push("w = exp(x) - log _2(x)".to_string());
    input_lines.push("u = exp(x) ^ 3 - log _2(x) ^ 2".to_string());
    input_lines.push("v = exp(x/y) ^ 3 - log _2(x*y) ^ 2".to_string());

    checks.push(("x", x));
    checks.push(("y", y));
    checks.push(("y0", x.log10()));
    checks.push(("y1", x.ln()));
    checks.push(("y2", x.log10()));
    checks.push(("z", x.log2()));
    checks.push(("q", x.exp()));
    checks.push(("w", (x.exp() - x.log2())));
    checks.push(("u", (x.exp().powf(3.0) - x.log2().powf(2.0))));
    checks.push(("v", ((x/y).exp().powf(3.0) - (x*y).log2().powf(2.0))));

    let input = input_lines.join("\n");
    let mut visitor = AsciiMathGenVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!("input: {}", input.as_str());
    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);

    for (i, (var_name, expected_f64_val)) in checks.iter().enumerate() {
        info!(
            "Checking: input {}, {} = {}",
            input_lines[i], var_name, expected_f64_val
        );
        let actual_basic_rc = visitor
            .result_table
            .get(&Gen::symbol(var_name, &visitor.giac_context).unwrap())
            .unwrap_or_else(|| panic!("Variable {} not found in result_table", var_name));

        let actual_f64 = actual_basic_rc.to_f64().unwrap_or_else(|| {
            panic!(
                "Variable {} is not a RealDouble, {}",
                var_name,
                actual_basic_rc.to_string()
            )
        });

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
