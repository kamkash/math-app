use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use interpreter::asciimath_basic_interpreter::AsciiMathBasicVisitor;
use log::info;
use math_parser::gen_parsers::asciimath2lexer::AsciiMath2Lexer;
use math_parser::gen_parsers::asciimath2parser::AsciiMath2Parser;
use symengine_rs::basic::Basic;
use test_log;

static EPSILON: f64 = 1e-8;

#[test_log::test]
fn test_asciimath_basic_visitor_eval() {
    let x = 10.0f64;
    let fz = x.powf(3.0) + x.powf(2.0) / 3.0 - 9.0 * x + 21.0;
    let fy = x.powf(2.0) + 2.0 * x + 5.0;
    let fzz = x.powf(3.0).powf(3.0) + x.powf(2.0) / 3.0 - 9.0 * x + 21.0;
    let input = "x=10
                z = x^3 + x^2 / 3.0 - 9.0 * x + 21.0
                y = x^2 + 2*x + 5
                zz = x^3^3 + x^2 / 3.0 - 9.0 * x + 21.0
                i * n = 
                n > 10
                ";
    let mut visitor = AsciiMathBasicVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!("input: {}", input);
    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);
    info!("fz: {}", fz);
    info!("fy: {}", fy);
    info!("fzz: {}", fzz);
    let z_actual = visitor
        .result_table
        .get(&Basic::symbol("z"))
        .and_then(|b| b.to_f64())
        .unwrap();
    let y_actual = visitor
        .result_table
        .get(&Basic::symbol("y"))
        .and_then(|b| b.to_f64())
        .unwrap();
    let zz_actual = visitor
        .result_table
        .get(&Basic::symbol("zz"))
        .and_then(|b| b.to_f64())
        .unwrap();

    assert!(
        (y_actual - fy).abs() < EPSILON,
        "actual: {}, expected: {}",
        y_actual,
        fy
    );
    assert!(
        (z_actual - fz).abs() < EPSILON,
        "actual: {}, expected: {}",
        z_actual,
        fz
    );
    assert!(
        (zz_actual - fzz).abs() < EPSILON,
        "actual: {}, expected: {}",
        zz_actual,
        fzz
    );
}

#[test_log::test]
fn test_asciimath_basic_comp_interest_eval() {
    let p = 100_000.0f64;
    let i = 0.10f64;
    let n = 5.0f64;
    let compound_interest = p * (1.0 + i).powf(n);
    let comp_input = format!(
        "p = {p} 
         i = {i}
         n = {n}
         compound_interest = p * (1 + i) ^ n 
         "
    );

    info!("compound interest input: {}", comp_input);
    let mut visitor = AsciiMathBasicVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(comp_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);

    let comp_interest_actual = visitor
        .result_table
        .get(&Basic::symbol("compound_interest"))
        .and_then(|b| b.to_f64())
        .unwrap();
    info!("compound interest: {}", comp_interest_actual);
    assert!(
        (comp_interest_actual - compound_interest).abs() < EPSILON,
        "actual: {}, expected: {}",
        comp_interest_actual,
        compound_interest
    );
}

#[test_log::test]
fn test_asciimath_basic_implicit_multiply() {
    let exp_str = "x^3 + 3x^2 - 5x + 2";
    let [x] = Basic::symbols(["x"]);
    let exp = Basic::parse(exp_str).unwrap();
    let inp = format!("{} = 10\ny = {}", x, exp_str);

    let expected = Basic::subs(&exp, vec![(&x, &Basic::real(10.0f64))].into_iter());
    info!("input: {}", inp);
    info!("expected: {}", expected);

    let mut visitor = AsciiMathBasicVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(inp.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    visitor.visit(parse_tree.as_ref());

    info!("visitor block result: {:?}", visitor.block_expressions);
    info!("visitor symbol table: {:?}", visitor.symbol_table);
    info!("visitor result table: {:?}", visitor.result_table);

    let actual = visitor
        .result_table
        .get(&Basic::symbol("y"))
        .and_then(|b| b.to_f64())
        .unwrap();

    assert!(
        (actual - expected.to_f64().unwrap()).abs() < EPSILON,
        "actual: {}, expected: {}",
        actual,
        expected
    );
}
