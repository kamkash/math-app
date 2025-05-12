use std::rc::Rc;

use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use log::info;
use math_parser::gen_parsers::asciimath2lexer::AsciiMath2Lexer;
use math_parser::asciimath_evaluator::AsciiMathVisitor;
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
                        t = 
                        ";
    let mut visitor = AsciiMathVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
}