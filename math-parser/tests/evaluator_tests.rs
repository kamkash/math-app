use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use log::info;
use math_parser::gen_calc_parser::{
    calculatorlexer::calculatorLexer, calculatorparser::calculatorParser,
};
use math_parser::symengine_evaluator::SymBasicCalcVisitor;
use std::rc::Rc;
use symengine_rs::basic::Basic;
// use symengine_rs::basic::Basic;
use regex::Regex;

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
fn test_evaluator_motion_constant_acceleration() {
    let _v = Basic::symbol("v");
    let v0 = Basic::symbol("v0");
    let a = Basic::symbol("a");
    let x = Basic::symbol("x");
    let x0 = Basic::symbol("x0");
    let _const_one = Basic::integer(1);
    let const_two = Basic::integer(2);
    let re = Regex::new(r"\*\*2").unwrap();

    let v_sqr = v0.sqr().add(&x.sub(&x0).mul(&const_two).mul(&a));
    let v = v_sqr.sqrt();

    let v_sqr_str = v_sqr.to_string();
    let v_sqr_inp = re.replace_all(&v_sqr_str.as_str(), "^2");
    info!("v^2 = {}", v_sqr_str);
    info!("v^2 input = {}", v_sqr_inp);

    let v_str = v.to_string();
    let v_inp = re.replace_all(&v_str.as_str(), "^2");
    info!("v = {}", v_str);
    info!("v input = {}", v_inp);

    // let mut visitor = SymBasicCalcVisitor::new();
    // let lexer = calculatorLexer::new(InputStream::new(input.as_str()));
    // let token_stream = CommonTokenStream::new(lexer);
    // let mut parser = calculatorParser::new(token_stream);
    // let parse_tree = parser.block().unwrap();
    // let _ = visitor.visit(parse_tree.as_ref());
    // info!("input: {}", input);
    // info!("visitor block result: {:?}", visitor.block_expressions);
    // info!("visitor symbol table: {:?}", visitor.symbol_table);
    // info!("visitor result table: {:?}", visitor.result_table);

    // assert_eq!(visitor.block_expressions.len(), input.split('\n').count());
}
