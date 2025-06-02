use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use interpreter::asciimath_basic_interpreter::AsciiMathBasicVisitor;
use log::info;
use math_parser::gen_parsers::asciimath2lexer::AsciiMath2Lexer;
use math_parser::gen_parsers::asciimath2parser::AsciiMath2Parser;
use symengine_rs::basic::Basic;
use test_log;

#[test_log::test]
fn test_asciimath_basic_visitor_parse_quadratic() {
    // Test for parsing and visiting the string: "y=x^2+2x+5"
    let x = 10.0f64;
    let fx = x.powf(2.0) + 2.0 * x + 5.0;
    let input = "x=10
                 y = x^2 + 2*x + 5";

    // Create a visitor instance
    let mut visitor = AsciiMathBasicVisitor::new();

    // Set up the lexer, token stream, and parser
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);

    // Parse the input and get the parse tree
    let parse_tree = parser.block().unwrap();

    // Visit the parse tree
    visitor.visit(parse_tree.as_ref());

    // Log the input and results
    info!("Input: {}", input);
    info!("Block expressions: {:?}", visitor.block_expressions);
    info!("Symbol table: {:?}", visitor.symbol_table);
    info!("Result table: {:?}", visitor.result_table);
    info!("fx: {}", fx);
}

#[test_log::test]
fn test_asciimath_basic_visitor_eval() {
    // Test for parsing and visiting the string: "y=x^2+2x+5"
    let x = 10.0f64;
    let fx = x.powf(3.0) + x.powf(2.0) / 3.0 - 9.0 * x + 21.0;
    let input = "x=10
                       z = x^3 + x^2 / 3.0 - 9.0 * x + 21.0";

    // Create a visitor instance
    let mut visitor = AsciiMathBasicVisitor::new();

    // Set up the lexer, token stream, and parser
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);

    // Parse the input and get the parse tree
    let parse_tree = parser.block().unwrap();

    // Visit the parse tree
    visitor.visit(parse_tree.as_ref());

    // Log the input and results
    info!("Input: {}", input);
    info!("Block expressions: {:?}", visitor.block_expressions);
    info!("Symbol table: {:?}", visitor.symbol_table);
    info!("Result table: {:?}", visitor.result_table);
    info!("fx: {}", fx);
    let epsilon = 1e-8;
    let actual = visitor
        .result_table
        .get(&Basic::symbol("z"))
        .and_then(|b| b.to_f64())
        .unwrap();
    assert!(
        (actual - fx).abs() < epsilon,
        "actual: {}, expected: {}",
        actual,
        fx
    );
}
