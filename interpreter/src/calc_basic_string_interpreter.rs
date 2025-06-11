#![allow(unused)]
use core::num;
use std::arch::is_aarch64_feature_detected;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use std::result;
use std::str::FromStr;
use std::sync::Arc;

use crate::SymEquation;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::errors::ANTLRError;
use antlr_rust::token_factory::TokenFactory;
use antlr_rust::tree::{ParseTree, TerminalNode};
use antlr_rust::tree::{ParseTreeVisitorCompat, Tree};
use antlr_rust::TidExt;
use antlr_rust::{recognizer, InputStream, Parser};
use log::info;
use symengine_rs::basic::{Basic, LogicalOperator};

use math_parser::gen_parsers::calculatorparser::{
    calculatorParserContextType, AtomContext, BlockContext, ConstantContext, CurrencyContext,
    EquationContext, EquationContextAttrs, ExpressionContext, ExpressionContextAttrs, Func_Context,
    FuncnameContext, FunctionDefinitionContext, MultiplyingExpressionContext, PowExpressionContext,
    RelopContext, ScientificContext, SignedAtomContext, VariableContext,
};
use math_parser::gen_parsers::calculatorparser::{
    MultiplyingExpressionContextAttrs, PowExpressionContextAttrs,
};
use math_parser::gen_parsers::calculatorvisitor::calculatorVisitorCompat;
use math_parser::gen_parsers::{
    calculatorlexer::calculatorLexer, calculatorparser::calculatorParser,
};

pub fn evaluate_ascii_math_block(input: &str) -> Result<String, String> {
    let input_stream = InputStream::new(input.into());
    let lexer = calculatorLexer::new(input_stream);
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    // Parse the input
    let result = parser.block();
    // If parsing succeeded, visit the parse tree
    match result {
        Ok(context) => {
            let mut visitor = CalcStringVisitor::new();
            let _ = visitor.visit(&*context);
            let result = format!("{:?}", visitor.result_table);
            Ok(result)
        }
        Err(e) => Err(format!("parser error {}", e).to_string()),
    }
}
pub struct CalcStringVisitor {
    equation_count: u32,
    pub tmp_result: String,
    pub block_expressions: Vec<SymEquation>,
    pub symbol_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
    pub result_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
}

impl CalcStringVisitor {
    pub fn new() -> Self {
        CalcStringVisitor {
            equation_count: 0,
            tmp_result: String::new(),
            block_expressions: Vec::new(),
            symbol_table: std::collections::HashMap::new(),
            result_table: std::collections::HashMap::new(),
        }
    }

    fn build_symbol_table(&mut self) {
        self.symbol_table
            .extend(self.block_expressions.iter().filter_map(|sym_eq| {
                if sym_eq.left.is_symbol() {
                    Some((Rc::clone(&sym_eq.left), Rc::clone(&sym_eq.right)))
                } else if sym_eq.right.is_symbol() {
                    Some((Rc::clone(&sym_eq.right), Rc::clone(&sym_eq.left)))
                } else {
                    None
                }
            }));
        info!("symbol table: {:?}", self.symbol_table);
    }

    fn build_result_table(&mut self) {
        self.result_table = self
            .symbol_table
            .iter()
            .map(|(sym, expr)| {
                let value = Basic::rc_subs(expr, &self.symbol_table.iter().collect::<Vec<_>>());
                (Rc::clone(sym), Rc::new(value))
            })
            .collect();
        info!("result table: {:?}", self.result_table);
    }
}

impl ParseTreeVisitorCompat<'_> for CalcStringVisitor {
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

impl<'input> calculatorVisitorCompat<'input> for CalcStringVisitor {
    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        info!("block expressions: {:?}", self.block_expressions);
        self.build_symbol_table();
        self.build_result_table();
        res
    }

    fn visit_equation(&mut self, ctx: &EquationContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        info!("Equation: count {}, {}", len, ctx.get_text());
        assert!(len == 3);
        let mut left = ctx.get_child(0).unwrap().get_text().to_string();
        let oper = ctx.get_child(1).unwrap().get_text().to_string();
        let mut right = ctx.get_child(2).unwrap().get_text().to_string();
        assert!(oper == "=");
        left = if left.is_empty() {
            let count = self.equation_count;
            self.equation_count += 1;
            format!("_{}", count).to_string()
        } else {
            left
        };
        right = if right.is_empty() {
            let count = self.equation_count;
            self.equation_count += 1;
            format!("_{}", count).to_string()
        } else {
            right
        };
        let log_oper = LogicalOperator::from_str_token(&oper);
        let symeq = SymEquation::new(
            Rc::new(Basic::parse(&left).unwrap()),
            Rc::new(Basic::parse(&right).unwrap()),
            Rc::new(Basic::logical_op(log_oper.unwrap())),
        );
        self.block_expressions.push(symeq);
        res
    }

    fn visit_relational_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::calculatorparser::Relational_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
        if let Some(parent) = ctx.get_parent() {
            if parent.is::<BlockContext>() {
                info!("Root Expression '{}'", ctx.get_text());
            }
        }
        self.visit_children(ctx)
    }
}
