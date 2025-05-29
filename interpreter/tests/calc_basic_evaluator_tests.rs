use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use interpreter::calc_basic_interpreter::SymBasicCalcVisitor;
use log::info;
use math_parser::gen_parsers::{
    calculatorlexer::calculatorLexer, calculatorparser::calculatorParser,
};
use std::rc::Rc;
use symengine_rs::basic::Basic;

use test_log;

#[test]
fn test_evaluator_equations() {
    let input = "-1 + 2 = __ans__
                        p = x + y
                        i = 1 - x
                        q = 100000 / w
                        q1 = 100,000 / w1
                        q11 != 100,000 / w11
                        z * 7 = t + 3.14";
    let mut visitor = SymBasicCalcVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let _ = visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);

    assert_eq!(visitor.block_expressions.len(), input.split('\n').count());
}

#[test]
fn test_evaluator_compound_interest() {
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
    let mut visitor = SymBasicCalcVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let _ = visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);

    assert_eq!(visitor.block_expressions.len(), input.split('\n').count());
    assert_eq!(
        visitor
            .result_table
            .get(&Basic::symbol("compound_interest")),
        Some(&Rc::new(Basic::real(compound_interest))),
    );
}

#[test]
fn test_evaluator_polynomial() {
    // $$ z=x^3+3\cdot x^2-9 $$
    // $$ z=x^3+3x^2-9 $$

    let x = 10.0f64;
    let fx = x.powf(3.0) + 3.0 * x.powf(2.0) - 9.0;
    let input = format!(
        "x = {x} 
         z = x^3 + 3*x^2 - 9"
    );
    let mut visitor = SymBasicCalcVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let _ = visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);

    assert_eq!(
        visitor.result_table.get(&Basic::symbol("z")),
        Some(&Rc::new(Basic::real(fx))),
    );
}

#[test_log::test]
fn test_evaluator_polynomial_order_bad_result() {
    // z = x^3 + x^2/3 -9 * x + 21
    let x = 10.0f64;
    let fx = x.powf(3.0) + x.powf(2.0) / 3.0 - x * 9.0 + 21.0;
    let input = format!(
        "x = {x} 
         z = x^3 + x^2 / 3 - 9 * x + 21"
    );
    let mut visitor = SymBasicCalcVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let _ = visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);
    info!("fx: {}", fx);
    info!(
        "result: {}",
        visitor
            .result_table
            .get(&Basic::symbol("z"))
            .unwrap()
            .to_f64()
            .unwrap()
    );
    assert_ne!(
        visitor.result_table.get(&Basic::symbol("z")),
        Some(&Rc::new(Basic::real(fx))),
    );
}

#[test_log::test]
fn test_symengine_eval_error_handling() {
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
    let mut visitor = SymBasicCalcVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);

    let parse_tree = match parser.block() {
        Ok(tree) => {
            info!("Parse no errors");
            tree
        }
        Err(e) => {
            info!("Parse error: {}", e);
            return;
        }
    };
    let _ = visitor.visit(parse_tree.as_ref());
}
