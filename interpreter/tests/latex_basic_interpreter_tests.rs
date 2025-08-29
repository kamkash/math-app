use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use log::info;
use math_parser::gen_parsers::latexlexer::LaTeXLexer;
use math_parser::gen_parsers::latexparser::LaTeXParser;
use symengine_rs::basic::Basic;
use test_log;

// Import the LatexBasicVisitor type from its module
use interpreter::latex_basic_interpreter::LaTeXBasicVisitor;

static EPSILON: f64 = 1e-8;

#[test_log::test]
fn test_latex_basic_simple_eval() {
    let mut visitor = LaTeXBasicVisitor::new();
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
        "Visitor result table after algebraic tests: {:?}",
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
                "Variable {} is not a RealDouble, it's a {:?} type: {}",
                var_name,
                actual_basic_rc.get_type_str(),
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

#[test_log::test]
fn test_latex_basic_power() {
    let mut visitor = LaTeXBasicVisitor::new();
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
            (actual_f64 - expected_f64_val).abs() < EPSILON,
            "Failed for variable: {}. Expected: {}, Got: {}. Difference: {}",
            var_name,
            expected_f64_val,
            actual_f64,
            (actual_f64 - expected_f64_val).abs()
        );
    }
}

// #[test_log::test]
// fn test_latex_basic_sqrt() {
//     let mut visitor = LaTeXBasicVisitor::new();
//     let mut input_lines: Vec<String> = Vec::new();
//     let mut checks: Vec<(&str, f64)> = Vec::new();

//     // Square Root
//     input_lines.push("sqrt_res1 = \\sqrt{100.0}".to_string());
//     checks.push(("sqrt_res1", 100.0f64.sqrt()));
//     input_lines.push("sqrt_res2 = \\sqrt{a * 10.0}".to_string()); // sqrt(10*10) = 10
//     checks.push(("sqrt_res2", (10.0f64 * 10.0f64).sqrt()));
//     input_lines.push("sqrt_res3 = \\sqrt{2.0}".to_string());
//     checks.push(("sqrt_res3", 2.0f64.sqrt()));

//     let final_input = input_lines.join("\n");
//     let lexer = LaTeXLexer::new(InputStream::new(final_input.as_str()));
//     let token_stream = CommonTokenStream::new(lexer);
//     let mut parser = LaTeXParser::new(token_stream);
//     let parse_tree = parser.block().unwrap();
//     visitor.visit(parse_tree.as_ref());

//     for (var_name, expected_f64_val) in checks {
//         info!("Checking: {} = {}", var_name, expected_f64_val);
//         let actual_basic_rc = visitor
//             .result_table
//             .get(&Basic::symbol(var_name))
//             .unwrap_or_else(|| panic!("Variable {} not found in result_table", var_name));

//         let actual_f64 = actual_basic_rc.to_f64().unwrap_or_else(|| {
//             panic!(
//                 "Variable {} is not a RealDouble, it's a {:?} type: {}",
//                 var_name,
//                 actual_basic_rc.get_type_str(),
//                 actual_basic_rc.to_string()
//             )
//         });

//         assert!(
//             (actual_f64 - expected_f64_val).abs() < EPSILON,
//             "Failed for variable: {}. Expected: {}, Got: {}. Difference: {}",
//             var_name,
//             expected_f64_val,
//             actual_f64,
//             (actual_f64 - expected_f64_val).abs()
//         );
//     }
// }

#[test_log::test]
fn test_latex_basic_eval() {
    let mut visitor = LaTeXBasicVisitor::new();
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
    // input_lines.push("sqrt_res1 = sqrt(100.0)".to_string());
    // checks.push(("sqrt_res1", 100.0f64.sqrt()));
    // input_lines.push("sqrt_res2 = sqrt(a * 10.0)".to_string()); // sqrt(10*10) = 10
    // checks.push(("sqrt_res2", (10.0f64 * 10.0f64).sqrt()));
    // input_lines.push("sqrt_res3 = sqrt(2.0)".to_string());
    // checks.push(("sqrt_res3", 2.0f64.sqrt()));

    // // Combined operations / Order of operations
    // input_lines.push("combo_res1 = a + b * 2".to_string()); // 10 + 20.5 * 2 = 10 + 41 = 51
    // checks.push(("combo_res1", 10.0 + 20.5 * 2.0));
    // input_lines.push("combo_res2 = (a + b) * 2".to_string()); // (10 + 20.5) * 2 = 30.5 * 2 = 61
    // checks.push(("combo_res2", (10.0 + 20.5) * 2.0));
    // input_lines.push("combo_res3 = a - b / 2 + 1".to_string()); // 10 - 20.5/2 + 1 = 10 - 10.25 + 1 = 0.75
    // checks.push(("combo_res3", 10.0 - 20.5 / 2.0 + 1.0));
    // input_lines.push("combo_res4 = sqrt(a^2 + 3*b - 1.5)".to_string()); // sqrt(100 + 3*20.5 - 1.5) = sqrt(100 + 61.5 - 1.5) = sqrt(160)
    // checks.push(("combo_res4", (10.0f64.powi(2) + 3.0 * 20.5 - 1.5).sqrt()));

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
        "Visitor result table after algebraic tests: {:?}",
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
                "Variable {} is not a RealDouble, it's a {:?} type: {}",
                var_name,
                actual_basic_rc.get_type_str(),
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
