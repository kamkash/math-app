use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use log::info;
use math_parser::symengine_evaluator::SymBasicCalcVisitor;
use math_parser::gen_calc_parser::{
    calculatorlexer::calculatorLexer, calculatorparser::calculatorParser,
};
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
    info!("visitor block result: {:?}", visitor.block_result);
    info!("visitor symbol table: {:?}", visitor.symbol_table);

    assert_eq!(visitor.block_result.len(), input.split('\n').count());
}

#[test]
fn test_evaluator_compound_interest() {

    let input = "p = 100000
                i = 0.1
                n = 5
                compound_interest = p * (1 + i) ^ n ";
    let mut visitor = SymBasicCalcVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let _ = visitor.visit(parse_tree.as_ref());
    info!("input: {}", input);
    info!("visitor block result: {:?}", visitor.block_result);
    info!("visitor symbol table: {:?}", visitor.symbol_table);

    assert_eq!(visitor.block_result.len(), input.split('\n').count());

}