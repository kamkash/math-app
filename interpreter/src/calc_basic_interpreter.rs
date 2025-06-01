#![allow(unused)]
use core::num;
use std::arch::is_aarch64_feature_detected;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use std::result;
use std::str::FromStr;
use std::sync::Arc;

use crate::{Relop, SymEquation};

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::tree::{ParseTreeVisitorCompat, Tree};
use antlr_rust::InputStream;
use antlr_rust::TidExt;
use log::info;

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
use symengine_rs::basic::Basic;

pub struct CalcBasicVisitor {
    pub tmp_result: Rc<Basic>,
    pub visitor_stack: Vec<Rc<Basic>>,
    pub block_expressions: Vec<SymEquation>,
    pub symbol_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
    pub result_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
}

impl CalcBasicVisitor {
    pub fn new() -> Self {
        CalcBasicVisitor {
            tmp_result: Rc::new(Basic::default()),
            visitor_stack: Vec::new(),
            block_expressions: Vec::new(),
            symbol_table: std::collections::HashMap::new(),
            result_table: std::collections::HashMap::new(),
        }
    }

    fn build_symbol_table(&mut self) {
        // build symbol table
        for sym_eq in &self.block_expressions {
            let left = &sym_eq.left;
            let right = &sym_eq.right;
            let relop = &sym_eq.relop;
            if left.is_symbol() {
                self.symbol_table.insert(Rc::clone(left), Rc::clone(right));
            } else if right.is_symbol() {
                self.symbol_table.insert(Rc::clone(right), Rc::clone(left));
            }
        }
    }

    fn build_result_table(&mut self) {
        for (sym, expr) in &self.symbol_table {
            let value = Basic::rc_subs(expr, self.symbol_table.iter().map(|(k, v)| (k, v)));
            self.result_table.insert(Rc::clone(sym), Rc::new(value));
        }
    }
}

impl ParseTreeVisitorCompat<'_> for CalcBasicVisitor {
    type Node = calculatorParserContextType;
    type Return = Rc<Basic>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.tmp_result
    }

    fn aggregate_results(&self, _aggregate: Self::Return, _next: Self::Return) -> Self::Return {
        _next // add Custom logic for aggregating results
    }
}

impl<'input> calculatorVisitorCompat<'input> for CalcBasicVisitor {
    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        self.build_symbol_table();
        self.build_result_table();
        res
    }

    fn visit_functionDefinition(
        &mut self,
        ctx: &FunctionDefinitionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_equation(&mut self, ctx: &EquationContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        assert!(self.visitor_stack.len() >= 2);
        let right = self.visitor_stack.pop().unwrap();
        let left = self.visitor_stack.pop().unwrap();
        let equation = SymEquation::new(Rc::clone(&left), Rc::clone(&right), Relop::Equal);
        self.block_expressions.push(equation);
        res
    }

    fn visit_relational_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::calculatorparser::Relational_expressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = self.visitor_stack.len();
        let left = Rc::clone(&self.visitor_stack[len - 1]);
        let right = Rc::clone(&self.visitor_stack[len - 2]);
        let relop_text = ctx.get_child(1).unwrap().get_text();
        let relop = Relop::from(relop_text.as_str());
        let equation = SymEquation::new(left, right, relop);
        self.block_expressions.push(equation);
        res
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        if len > 1 {
            let mut right = self.visitor_stack.pop().unwrap();
            for c in 0..(len - 1) / 2 {
                let op = self.visitor_stack.pop().unwrap();
                assert!(op.is_integer());
                if op.is_positive() {
                    let left = self.visitor_stack.pop().unwrap();
                    right = Rc::new(left.add(&right));
                } else {
                    let left = self.visitor_stack.pop().unwrap();
                    right = Rc::new(left.sub(&right));
                }
            }
            self.visitor_stack.push(right);
        }
        res
    }

    fn visit_multiplyingExpression(
        &mut self,
        ctx: &MultiplyingExpressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        if len > 1 {
            let mut right = self.visitor_stack.pop().unwrap();
            for c in 0..(len - 1) / 2 {
                let op = self.visitor_stack.pop().unwrap();
                assert!(op.is_integer());
                if op.is_positive() {
                    let left = self.visitor_stack.pop().unwrap();
                    right = Rc::new(left.mul(&right));
                } else {
                    let left = self.visitor_stack.pop().unwrap();
                    right = Rc::new(left.div(&right));
                }
            }
            self.visitor_stack.push(right);
        }
        res
    }

    fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        if len > 1 {
            let mut right = self.visitor_stack.pop().unwrap();
            for c in 0..(len - 1) / 2 {
                let left = self.visitor_stack.pop().unwrap();
                right = Rc::new(left.pow(&right));
            }
            self.visitor_stack.push(right);
        }
        res
    }

    fn visit_signedAtom(&mut self, ctx: &SignedAtomContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let negate = ctx.get_children().any(|child| child.get_text() == "-");
        if negate {
            let signed_basic = self.visitor_stack.pop().unwrap();
            let result = Rc::new(signed_basic.neg());
            self.visitor_stack.push(result);
        }
        res
    }

    fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_scientific(&mut self, ctx: &ScientificContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let sci_text = ctx.get_text();
        let filtered: String = sci_text
            .chars()
            .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
            .collect();
        let value: f64 = filtered.parse().unwrap_or(0.0);
        let result = Rc::new(Basic::real(value));
        self.visitor_stack.push(result);
        res
    }

    fn visit_currency(&mut self, ctx: &CurrencyContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let currency_text = ctx.get_text();
        let filtered: String = currency_text
            .chars()
            .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
            .collect();
        let value: f64 = filtered.parse().unwrap_or(0.0);

        let result = Rc::new(Basic::real(value));
        self.visitor_stack.push(result);
        res
    }

    fn visit_constant(&mut self, ctx: &ConstantContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let var_text = ctx.get_text();
        let var_symbol = Rc::new(Basic::symbol(&var_text));
        self.visitor_stack.push(var_symbol);
        res
    }

    fn visit_func_(&mut self, ctx: &Func_Context<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let func_name = ctx.get_child(0).unwrap().get_text();
        let func_args = ctx
            .get_children()
            .map(|child| {
                let arg_text = child.get_text();
                let arg_value: f64 = arg_text.parse().unwrap_or(0.0);
                Rc::new(Basic::real(arg_value))
            })
            .collect::<Vec<_>>();
        let func_result = match func_name.as_str() {
            "sin" => Basic::sin(&func_args[0]),
            "cos" => Basic::cos(&func_args[0]),
            "tan" => Basic::tan(&func_args[0]),
            "asin" => Basic::asin(&func_args[0]),
            "acos" => Basic::acos(&func_args[0]),
            "atan" => Basic::atan(&func_args[0]),
            "exp" => Basic::exp(&func_args[0]),
            "log" => Basic::log(&func_args[0]),
            _ => panic!("Unknown function: {}", func_name),
        };
        self.visitor_stack.push(Rc::new(func_result));
        res
    }

    fn visit_funcname(&mut self, ctx: &FuncnameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_multop(
        &mut self,
        ctx: &math_parser::gen_parsers::calculatorparser::MultopContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "*" {
            self.visitor_stack.push(Rc::new(Basic::integer(1000)));
        } else {
            self.visitor_stack.push(Rc::new(Basic::integer(-1000)));
        }
        res
    }

    fn visit_sumop(
        &mut self,
        ctx: &math_parser::gen_parsers::calculatorparser::SumopContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "+" {
            self.visitor_stack.push(Rc::new(Basic::integer(1000)));
        } else {
            self.visitor_stack.push(Rc::new(Basic::integer(-1000)));
        }
        res
    }
}

pub fn evaluate_ascii_math_block(input: &str) -> Result<String, String> {
    info!("evaluate_ascii_math {}", input);
    let mut visitor = CalcBasicVisitor::new();
    let lexer = calculatorLexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let _ = visitor.visit(parse_tree.as_ref());
    let result = format!("{:?}", visitor.result_table);
    Ok(result)
}