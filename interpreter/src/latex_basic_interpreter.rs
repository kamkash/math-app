use std::collections::HashMap;
use std::rc::Rc;

use crate::SymEquation;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};
use antlr_rust::InputStream;
use log::info;
use math_core::common::LogicalOperator;
use math_parser::gen_parsers::latexlexer::LaTeXLexer;
use math_parser::gen_parsers::latexparser::{
    AdditiveContext, AtomNumberContext, AtomVariableContext, BlockContext, LaTeXParser,
    LaTeXParserContextType, MpContext, MultopContext, PowopContext, RelationContext, RelopContext,
    SumopContext,
};
use math_parser::gen_parsers::latexvisitor::LaTeXVisitorCompat;
use symengine_rs::basic::Basic;

pub struct LaTeXBasicVisitor {
    pub tmp_result: Rc<Basic>,
    pub visitor_stack: Vec<Rc<Basic>>,
    pub block_expressions: Vec<SymEquation>,
    pub symbol_table: HashMap<Rc<Basic>, Rc<Basic>>,
    pub result_table: HashMap<Rc<Basic>, Rc<Basic>>,
}

impl LaTeXBasicVisitor {
    pub fn new() -> Self {
        LaTeXBasicVisitor {
            tmp_result: Rc::new(Basic::default()),
            visitor_stack: Vec::new(),
            block_expressions: Vec::new(),
            symbol_table: HashMap::new(),
            result_table: HashMap::new(),
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
                let pairs: Vec<(&Rc<Basic>, &Rc<Basic>)> = self.symbol_table.iter().collect();
                let value = Basic::rc_subs(expr, &pairs);
                (Rc::clone(sym), Rc::new(value))
            })
            .collect();
        info!("result table: {:?}", self.result_table);
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for LaTeXBasicVisitor {
    type Node = LaTeXParserContextType;
    type Return = Rc<Basic>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.tmp_result
    }

    fn aggregate_results(&self, _aggregate: Self::Return, next: Self::Return) -> Self::Return {
        next
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        Self::Return::default()
    }
}

impl<'input> LaTeXVisitorCompat<'input> for LaTeXBasicVisitor {
    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        self.build_symbol_table();
        self.build_result_table();
        res
    }

    fn visit_relation(&mut self, ctx: &RelationContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = self.visitor_stack.len();
        if len >= 3 {
            let right = self.visitor_stack.pop().unwrap();
            let op = self.visitor_stack.pop().unwrap();
            let left = self.visitor_stack.pop().unwrap();
            let equation = SymEquation::new(Rc::clone(&left), Rc::clone(&right), op);
            self.block_expressions.push(equation);
        }
        res
    }

    fn visit_additive(&mut self, ctx: &AdditiveContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        if len > 1 {
            let mut left = self.visitor_stack.remove(self.visitor_stack.len() - len);
            for i in 1..len {
                let op = self
                    .visitor_stack
                    .remove(self.visitor_stack.len() - len + i);
                let right = self
                    .visitor_stack
                    .remove(self.visitor_stack.len() - len + i);
                if op.is_add_op() {
                    left = Rc::new(left.add(&right));
                } else {
                    left = Rc::new(left.sub(&right));
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_mp(&mut self, ctx: &MpContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        if len > 1 {
            let mut left = self.visitor_stack.remove(self.visitor_stack.len() - len);
            for i in 1..len {
                let op = self
                    .visitor_stack
                    .remove(self.visitor_stack.len() - len + i);
                let right = self
                    .visitor_stack
                    .remove(self.visitor_stack.len() - len + i);
                if op.is_mul_op() {
                    left = Rc::new(left.mul(&right));
                } else {
                    left = Rc::new(left.div(&right));
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_powop(&mut self, ctx: &PowopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "^" {
            self.visitor_stack.push(Rc::new(Basic::pow_op()));
        }
        res
    }

    fn visit_sumop(&mut self, ctx: &SumopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "+" {
            self.visitor_stack.push(Rc::new(Basic::add_op()));
        } else {
            self.visitor_stack.push(Rc::new(Basic::sub_op()));
        }
        res
    }

    fn visit_multop(&mut self, ctx: &MultopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "*" {
            self.visitor_stack.push(Rc::new(Basic::mul_op()));
        } else {
            self.visitor_stack.push(Rc::new(Basic::div_op()));
        }
        res
    }

    fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        let rel_op = LogicalOperator::from_str_token(&op_text);
        let basic_op = Rc::new(Basic::logical_op(rel_op.unwrap()));
        self.visitor_stack.push(basic_op);
        res
    }

    fn visit_atomVariable(&mut self, ctx: &AtomVariableContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let text = ctx.get_text();
        if let Ok(val) = text.parse::<f64>() {
            let result = Rc::new(Basic::real(val));
            self.visitor_stack.push(result);
        } else {
            let symbol = Rc::new(Basic::symbol(&text));
            self.visitor_stack.push(symbol);
        }
        res
    }

    fn visit_atomNumber(&mut self, ctx: &AtomNumberContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let text = ctx.get_text();
        let value: f64 = text.parse().unwrap_or(0.0);
        let result = Rc::new(Basic::real(value));
        self.visitor_stack.push(result);
        res
    }
}

pub fn evaluate_latex_block(input: &str) -> Result<String, String> {
    info!("evaluate_latex_block: {}", input);
    let mut visitor = LaTeXBasicVisitor::new();
    let lexer = LaTeXLexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = LaTeXParser::new(token_stream);
    let result = parser.block();
    match result {
        Ok(context) => {
            let _ = visitor.visit(&*context);
            let result = format!("{:?}", visitor.visitor_stack);
            Ok(result)
        }
        Err(e) => Err(format!("parser error {}", e).to_string()),
    }
}
