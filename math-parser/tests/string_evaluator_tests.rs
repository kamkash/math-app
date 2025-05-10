use log::info;
use math_parser::symengine_basic_string_evaluator::{self, SymStringVisitor};
use test_log;

use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use math_parser::gen_parsers::{
    calculatorlexer::calculatorLexer, calculatorparser::calculatorParser,
};

#[test_log::test]
fn test_symstring_eval_equations() {
    let input = "-1 + 2 = __ans__
                        p = x + y
                        i = 1 - x
                        q = 100000 / w
                        q1 = 100,000 / w1
                        q11 != 100,000 / w11
                        z * 7 = t + 3.14";
    let mut visitor = SymStringVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
}

#[test_log::test]
fn test_symstring_eval_eq_func_expr() {
    let x = 10.0f64;
    let input = format!(
        "x = {x} 
        10
        t = sin(x)
         z = x^3 + x^2 / 3 - 9 * x + 21
         f(x) = x^3 + 3*x^2 + 10
         x^3 + x^2 / 3 - 9 * x + 21
         x/3 = "
         
    );
    let mut visitor = SymStringVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
}


#[test_log::test]
fn test_symstring_eval_error_handling() {
    let x = 10.0f64;
    let input = format!(
        "x = {x} 
        10
         z = x^3 + x^2 / 3 - 9 * x + 21
         f(x) = x^3 + 3*x^2 + 10
         x^3 + x^2 / 3 - 9 * x + 21
         x/3 = 
         x(1-x) = 
         "
         
    );
    match symengine_basic_string_evaluator::evaluate_ascii_math_block(input.as_str()) {
        Ok(result) => {
            info!("input: {}", input);
            info!("result: {}", result);
        }
        Err(e) => {
            info!("Error: {}", e);
        }
    }
}

#[test_log::test]
fn test_symstring_basic_eval() {
    let x = 10.0f64;
    let input = format!(
        "x = {x} 
         z = x^3 + x^2 / 3 - 9 * x + 21
         x/3 = 
        10
         "
         
    );
    match symengine_basic_string_evaluator::evaluate_ascii_math_block(input.as_str()) {
        Ok(result) => {
            info!("input: {}", input);
            info!("result: {}", result);
        }
        Err(e) => {
            info!("Error: {}", e);
        }
    }
}