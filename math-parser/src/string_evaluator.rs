#![allow(unused)]
use core::num;
use std::arch::is_aarch64_feature_detected;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use std::result;
use std::str::FromStr;
use std::sync::Arc;

use crate::Relop;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::tree::{ParseTree, TerminalNode};
use antlr_rust::tree::{ParseTreeVisitorCompat, Tree};
use antlr_rust::TidExt;
use antlr_rust::{recognizer, InputStream, Parser};
use log::info;

use crate::gen_calc_parser::calculatorparser::{
    calculatorParserContextType, AtomContext, BlockContext, ConstantContext, CurrencyContext,
    EquationContext, EquationContextAttrs, ExpressionContext, ExpressionContextAttrs, Func_Context,
    FuncnameContext, FunctionDefinitionContext, MultiplyingExpressionContext, PowExpressionContext,
    RelopContext, ScientificContext, SignedAtomContext, VariableContext,
};
use crate::gen_calc_parser::calculatorparser::{
    MultiplyingExpressionContextAttrs, PowExpressionContextAttrs,
};
use crate::gen_calc_parser::calculatorvisitor::calculatorVisitorCompat;
use crate::gen_calc_parser::{
    calculatorlexer::calculatorLexer, calculatorparser::calculatorParser,
};

use antlr_rust::token_stream::TokenStream;
use antlr_rust::{error_listener::ErrorListener, errors::ANTLRError, token::Token};

/// Custom error type for string evaluation errors
#[derive(Debug)]
pub struct StringEvaluationError {
    pub message: String,
    pub line: isize,
    pub column: isize,
}

impl fmt::Display for StringEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Error at line {}:{} - {}",
            self.line, self.column, self.message
        )
    }
}

impl std::error::Error for StringEvaluationError {}

/// Custom error listener for the calculator parser
pub struct StringEvaluatorErrorListener {
    pub errors: Vec<StringEvaluationError>,
}

impl StringEvaluatorErrorListener {
    pub fn new() -> Self {
        StringEvaluatorErrorListener { errors: Vec::new() }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn get_errors(&self) -> &Vec<StringEvaluationError> {
        &self.errors
    }
}

impl<'a, T: Parser<'a>> ErrorListener<'a, T> for StringEvaluatorErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _e: Option<&ANTLRError>,
    ) {
        let _ = _offending_symbol;
        let error = StringEvaluationError {
            message: msg.to_string(),
            line,
            column,
        };
        use std::cell::UnsafeCell;
        let errors = unsafe {
            let cell = (self as *const StringEvaluatorErrorListener
                as *const UnsafeCell<StringEvaluatorErrorListener>)
                .as_ref()
                .unwrap();
            &mut *(*cell).get()
        };
        errors.errors.push(error);
    }
}
/// Function to parse and evaluate a mathematical expression string
pub fn evaluate_string(input: &str) -> Result<String, StringEvaluationError> {
    let input_stream = InputStream::new(input.into());
    let lexer = calculatorLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);

    // Create and add our custom error listener
    let error_listener = Box::new(StringEvaluatorErrorListener::new());
    let error_listener_ref = Box::into_raw(error_listener);
    let boxed_listener = unsafe { Box::from_raw(error_listener_ref) };

    parser.remove_error_listeners();
    parser.add_error_listener(boxed_listener);

    // Parse the input
    let result = parser.block();

    // Get a reference back to check for errors
    let error_listener_ref = unsafe { &*error_listener_ref };

    // Check for errors
    if error_listener_ref.has_errors() {
        // Return the first error
        let first_error = &error_listener_ref.errors[0];
        return Err(StringEvaluationError {
            message: first_error.message.clone(),
            line: first_error.line,
            column: first_error.column,
        });
    }

    // If parsing succeeded, visit the parse tree
    match result {
        Ok(context) => {
            let mut visitor = SymStringVisitor::new();
            let result = visitor.visit(&*context);
            Ok(result)
        }
        Err(e) => {
            // This should not happen as errors should be caught by the error listener
            Err(StringEvaluationError {
                message: format!("Unexpected parsing error: {}", e),
                line: 0,
                column: 0,
            })
        }
    }
}

pub unsafe fn cleanup_error_listener(listener: &mut StringEvaluatorErrorListener) {
    // Convert the leaked reference back to a Box and drop it
    let _ = Box::from_raw(listener as *mut StringEvaluatorErrorListener);
}

pub struct SymStringVisitor {
    pub tmp_result: String,
}

impl SymStringVisitor {
    pub fn new() -> Self {
        SymStringVisitor {
            tmp_result: String::new(),
        }
    }
}

impl ParseTreeVisitorCompat<'_> for SymStringVisitor {
    type Node = calculatorParserContextType;
    type Return = String;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.tmp_result
    }

    fn aggregate_results(&self, _aggregate: Self::Return, _next: Self::Return) -> Self::Return {
        _aggregate + &_next
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        Self::Return::default()
    }
}

impl<'input> calculatorVisitorCompat<'input> for SymStringVisitor {
    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_equation(&mut self, ctx: &EquationContext<'input>) -> Self::Return {
        self.visit_children(ctx);
        let res = ctx.get_text();
        info!("Equation: {}", res);
        res
    }

    fn visit_functionDefinition(
        &mut self,
        ctx: &FunctionDefinitionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx);
        let res = ctx.get_text();
        info!("FunctionDefinition: {}", res);
        res
    }

    fn visit_relational_expression(
        &mut self,
        ctx: &crate::gen_calc_parser::calculatorparser::Relational_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx);
        let res = ctx.get_text();
        info!("RelationalExpression: {}", res);
        res
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx);
        let res = ctx.get_text();
        if let Some(parent) = ctx.get_parent() {
            if parent.is::<BlockContext>() {
                info!("Expression: {}", res);
            }
        }
        res
    }
}
