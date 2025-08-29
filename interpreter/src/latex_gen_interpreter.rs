use crate::asciimath_gen_interpreter::SymEquationGen;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};
use antlr_rust::InputStream;
use giac_rs::context::Context;
use giac_rs::gen::Gen;
use giac_rs::gen::{GEN_ADD, GEN_DIV, GEN_MUL, GEN_POW, GEN_SUB};
use math_core::common::LogicalOperator;
use std::collections::VecDeque;
use std::rc::Rc;

use log::{error, info};
// Import the generated parser context type
use math_parser::gen_parsers::latexlexer::LaTeXLexer;
use math_parser::gen_parsers::latexparser::{
    AdditiveContext, AtomVariableContext, BlockContext, EqualityContext, ExpContext, ExprContext,
    FuncContext, LaTeXParser, LaTeXParserContextType, MathContext, MpContext, MultopContext,
    NumberContext, PowopContext, RelationContext, RelopContext, SumopContext,
};
use math_parser::gen_parsers::latexvisitor::LaTeXVisitorCompat;

pub struct LaTeXGenVisitor {
    pub visitor_stack: Vec<Rc<Gen>>,
    pub block_expressions: Vec<SymEquationGen>,
    pub giac_context: Rc<Context>,
}

impl LaTeXGenVisitor {
    pub fn new() -> Self {
        let ctx = Rc::new(Context::new());
        Self {
            visitor_stack: Vec::new(),
            block_expressions: Vec::new(),
            giac_context: ctx,
        }
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for LaTeXGenVisitor {
    type Node = LaTeXParserContextType;
    type Return = Rc<Gen>;

    fn temp_result(&mut self) -> &mut Self::Return {
        panic!("Not used");
    }

    fn aggregate_results(&self, _aggregate: Self::Return, next: Self::Return) -> Self::Return {
        next
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        Rc::new(Gen::new("0", &self.giac_context).unwrap())
    }
}

impl<'input> LaTeXVisitorCompat<'input> for LaTeXGenVisitor {
    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        // Optionally build symbol/result tables here if needed
        res
    }

    fn visit_math(&mut self, ctx: &MathContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_relation(&mut self, ctx: &RelationContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len >= 3 && stack_len >= 3 {
            let right = self.visitor_stack.pop().unwrap();
            let op = self.visitor_stack.pop().unwrap();
            let left = self.visitor_stack.pop().unwrap();
            let equation = SymEquationGen::new(left, right, op);
            self.block_expressions.push(equation);
        }
        res
    }

    fn visit_equality(&mut self, ctx: &EqualityContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_expr(&mut self, ctx: &ExprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_additive(&mut self, ctx: &AdditiveContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 && stack_len >= len {
            let remove_at = stack_len - len;
            let mut left = self.visitor_stack.remove(remove_at);
            for _ in 0..(len - 1) / 2 {
                let op = self.visitor_stack.remove(remove_at);
                if op.is_add() {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.add(&right).unwrap());
                } else if op.is_sub() {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.sub(&right).unwrap());
                } else {
                    error!("Invalid operator in additive: {}", op.to_string());
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_mp(&mut self, ctx: &MpContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 && stack_len >= len {
            let remove_at = stack_len - len;
            let mut mult_stack: VecDeque<Rc<Gen>> = VecDeque::new();
            for index in 0..len {
                let left_item = self.visitor_stack.remove(remove_at);
                if index % 2 == 0 {
                    mult_stack.push_front(left_item);
                } else {
                    if left_item.is_mul() || left_item.is_div() {
                        mult_stack.push_front(left_item);
                    } else {
                        mult_stack.push_front(Rc::new(GEN_MUL.clone()));
                        mult_stack.push_front(left_item);
                    }
                }
            }
            let mut left: Rc<Gen> = Rc::new(Gen::new("0", &self.giac_context).unwrap());
            let mut right: Rc<Gen>;
            let mut op: Rc<Gen>;
            while mult_stack.len() > 0 {
                if left.to_string() == "0" {
                    left = Rc::clone(&mult_stack.pop_back().unwrap());
                    op = Rc::clone(&mult_stack.pop_back().unwrap());
                    right = Rc::clone(&mult_stack.pop_back().unwrap());
                } else {
                    op = Rc::clone(&mult_stack.pop_back().unwrap());
                    right = Rc::clone(&mult_stack.pop_back().unwrap());
                }
                if op.is_mul() {
                    left = Rc::new(left.mul(&right).unwrap());
                } else if op.is_div() {
                    left = Rc::new(left.div(&right).unwrap());
                } else {
                    error!("Unexpected operator in mp: {}", op.to_string());
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_exp(&mut self, ctx: &ExpContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 && stack_len >= len {
            let remove_at = stack_len - len;
            let mut left = self.visitor_stack.remove(remove_at);
            for _ in 0..(len - 1) / 2 {
                let op_gen = self.visitor_stack.remove(remove_at);
                if op_gen.is_pow() {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.symb_pow(&right).unwrap());
                } else {
                    info!("Unexpected operator in exp: {}", op_gen.to_string());
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_atomVariable(&mut self, ctx: &AtomVariableContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let var_text = ctx.get_text();
        let var_symbol = Rc::new(Gen::symbol(&var_text, &self.giac_context).unwrap());
        self.visitor_stack.push(var_symbol);
        res
    }

    fn visit_number(&mut self, ctx: &NumberContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let sci_text = ctx.get_text();
        let filtered: String = sci_text
            .chars()
            .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
            .collect();
        let value: f64 = filtered.parse().unwrap_or(0.0);
        let result = Rc::new(Gen::from_f64(value, &self.giac_context).unwrap());
        self.visitor_stack.push(result);
        res
    }

    fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        let rel_op = LogicalOperator::from_str_token(&op_text);
        let gen_op = Gen::logical_op(rel_op.unwrap_or(LogicalOperator::Eq));
        self.visitor_stack.push(Rc::new(gen_op.unwrap()));
        res
    }

    fn visit_sumop(&mut self, ctx: &SumopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "+" {
            self.visitor_stack.push(Rc::new(GEN_ADD.clone()));
        } else {
            self.visitor_stack.push(Rc::new(GEN_SUB.clone()));
        }
        res
    }

    fn visit_multop(&mut self, ctx: &MultopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "*" {
            self.visitor_stack.push(Rc::new(GEN_MUL.clone()));
        } else {
            self.visitor_stack.push(Rc::new(GEN_DIV.clone()));
        }
        res
    }

    fn visit_powop(&mut self, ctx: &PowopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "^" || op_text == "**" {
            self.visitor_stack.push(Rc::new(GEN_POW.clone()));
        }
        res
    }
}
