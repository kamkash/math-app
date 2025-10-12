use antlr_rust::tree::ParseTreeVisitorCompat; // bring trait into scope
use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use interpreter::latex_gen_interpreter::{eval_symbol_to_f64, LaTeXGenVisitor};
use log::info;
use math_parser::gen_parsers::{latexlexer::LaTeXLexer, latexparser::LaTeXParser};

const TEST_EPSILON: f64 = 1.0e-8;

/// Helper runner for all tests
pub fn run_test(input_lines: Vec<String>, checks: Vec<(&str, f64)>) {
    let final_input = input_lines.join("\n");
    info!(
        "Full input block for algebraic operations:\n{}",
        final_input
    );

    let lexer = LaTeXLexer::new(InputStream::new(final_input.as_str()));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = LaTeXParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let mut visitor = LaTeXGenVisitor::new();
    visitor.visit(parse_tree.as_ref());

    info!(
        "Visitor block expressions after algebraic tests: {:?}",
        visitor.block_expressions
    );

    for (var_name, expected_f64_val) in checks {
        let actual_f64 = eval_symbol_to_f64(&mut visitor, var_name);

        assert!(
            (actual_f64 - expected_f64_val).abs() < TEST_EPSILON,
            "❌ Failed for {}: expected {}, got {}, Δ={}",
            var_name,
            expected_f64_val,
            actual_f64,
            (actual_f64 - expected_f64_val).abs()
        );
        info!("✅ verify {} expected {}, got {}", var_name, expected_f64_val, actual_f64);
    }
}
