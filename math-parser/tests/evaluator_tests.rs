use std::rc::Rc;

use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use log::info;
use math_parser::gen_calc_parser::{
    calculatorlexer::calculatorLexer, calculatorparser::calculatorParser,
};
use math_parser::symengine_evaluator::SymBasicCalcVisitor;
use symengine_rs::basic::Basic;
// use symengine_rs::basic::Basic;

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
