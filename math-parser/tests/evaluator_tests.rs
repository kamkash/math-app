use log::info;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use math_parser::gen_calc_parser::{
    calculatorlexer::calculatorLexer, calculatorparser::calculatorParser,
};
use antlr_rust::tree::ParseTreeVisitorCompat;
use math_parser::calc_evaluator::SymBasicCalcVisitor;
// use symengine_rs::basic::Basic;

#[test]
fn test_evaluator_results() {
    let input = "-1 + 2 = __ans__,
                        p = x + y,
                        i = 1 - x,
                        q = 100000 / w,
                        q1 = 100,000 / w1,
                        z * 7 = t + 3.14";
    let mut visitor = SymBasicCalcVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let _result = visitor.visit(parse_tree.as_ref());
    // info!("Result: {}", result);
    // info!("visitor stack: {:?}", visitor.result_stack);
    // info!("visitor symbol table: {:?}", visitor.symbol_table);
    // info!("Parsed result: {:?}", parse_tree);
    
    info!("input: {}", input);
    info!("visitor block result: {:?}", visitor.block_result);

    // let line_count = input.split(",").count();
    // assert!(line_count * 2 == visitor.result_stack.len());
}
