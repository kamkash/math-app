use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use interpreter::asciimath_gen_interpreter::AsciiMathGenVisitor;
use antlr_rust::tree::{ParseTreeVisitorCompat}; // Add this line to bring the trait into scope
use log::info;
use math_parser::gen_parsers::{asciimath2lexer::AsciiMath2Lexer, asciimath2parser::AsciiMath2Parser};
use test_log;

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

    // Exponentiation
    input_lines.push("exp_res1 = a ^ 2".to_string());
    checks.push(("exp_res1", 10.0_f64.powf(2.0)));
    input_lines.push("exp_res2 = 2 ^ 3".to_string());
    checks.push(("exp_res2", 2.0_f64.powf(3.0)));

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
}


#[test_log::test]
fn test_asciimath_gen_eval_division() {
    let mut visitor = AsciiMathGenVisitor::new();
    let mut input_lines: Vec<String> = Vec::new();
    let mut checks: Vec<(&str, f64)> = Vec::new();

    // Division
    input_lines.push("a = 10.0".to_string());
    input_lines.push("b = 20.5".to_string());
    checks.push(("a", 10.0));
    checks.push(("b", 20.5));
    input_lines.push("div_res1 = b / a".to_string());
    checks.push(("div_res1", 20.5 / 10.0));
    input_lines.push("div_res2 = 21 / 7".to_string());
    checks.push(("div_res2", 21.0 / 7.0));

    let final_input = input_lines.join("\n");
    info!("Full input block for division operations:\n{}", final_input);

    let lexer = AsciiMath2Lexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
}
