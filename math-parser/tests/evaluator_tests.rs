use log::info;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use math_parser::gen_calc_parser::{
    calculatorlexer::calculatorLexer, calculatorparser::calculatorParser,
};
use antlr_rust::tree::ParseTreeVisitorCompat;
use math_parser::calc_evaluator::SymBasicCalcVisitor;
// use symengine_rs::basic::Basic;

#[test]
fn test_simple_addition() {
    let input = "-1 + 2 = __ans__";
    let mut visitor = SymBasicCalcVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let result = visitor.visit(parse_tree.as_ref());
    info!("Result: {}", result);
    info!("visitor stack: {:?}", visitor.result_stack);
    info!("visitor block result: {:?}", visitor.block_result);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    // info!("Parsed result: {:?}", parse_tree);
}
