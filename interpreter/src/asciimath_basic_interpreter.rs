use std::rc::Rc;

use crate::{Relop, SymEquation};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};
use antlr_rust::InputStream;
use asciimath2lexer::AsciiMath2Lexer;
use log::info;
use math_parser::gen_parsers::asciimath2lexer;
use math_parser::gen_parsers::asciimath2parser::{
    AsciiMath2Parser, AsciiMath2ParserContextType, IdentifierAtomContext, MultopContext,
    NumberAtomContext, Power_expressionContext, PowopContext, RelopContext, SumopContext,
};
use math_parser::gen_parsers::asciimath2visitor::AsciiMath2VisitorCompat;
use symengine_rs::basic::Basic;

pub struct AsciiMathBasicVisitor {
    pub tmp_result: Rc<Basic>,
    pub block_expressions: Vec<SymEquation>,
    pub symbol_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
    pub result_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
    pub visitor_stack: Vec<Rc<Basic>>,
}

impl AsciiMathBasicVisitor {
    pub fn new() -> Self {
        AsciiMathBasicVisitor {
            tmp_result: Rc::new(Basic::default()),
            block_expressions: Vec::new(),
            symbol_table: std::collections::HashMap::new(),
            result_table: std::collections::HashMap::new(),
            visitor_stack: Vec::new(),
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
                let value = Basic::rc_subs(expr, self.symbol_table.iter());
                (Rc::clone(sym), Rc::new(value))
            })
            .collect();
        info!("result table: {:?}", self.result_table);
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for AsciiMathBasicVisitor {
    type Node = AsciiMath2ParserContextType;
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

impl<'input> AsciiMath2VisitorCompat<'input> for AsciiMathBasicVisitor {
    fn visit_block(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::BlockContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        self.build_symbol_table();
        self.build_result_table();
        res
    }

    fn visit_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::ExpressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        // dbg!(&self.visitor_stack);
        let len = self.visitor_stack.len();
        if len == 2 {
            let right = self.visitor_stack.pop().unwrap();
            let left = self.visitor_stack.pop().unwrap();
            let equation = SymEquation::new(Rc::clone(&left), Rc::clone(&right), Relop::Equal);
            self.block_expressions.push(equation);
        }
        self.visitor_stack.clear();
        res
    }

    fn visit_logical_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Logical_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_relation_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Relation_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_relation_expression_no_rhs(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Relation_expression_no_rhsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_add_sub_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Add_sub_expressionContext<'input>,
    ) -> Self::Return {
        // dbg!(ctx.get_text());
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 {
            // dbg!(&self.visitor_stack);
            let remove_at = stack_len - len;
            let mut left = self.visitor_stack.remove(remove_at);
            for _ in 0..(len - 1) / 2 {
                let op = self.visitor_stack.remove(remove_at);
                assert!(op.is_op());
                if op.is_add_op() {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.add(&right));
                } else {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.sub(&right));
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_mult_div_implicit_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Mult_div_implicit_expressionContext<
            'input,
        >,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 {
            // dbg!(&self.visitor_stack);
            let remove_at = stack_len - len;
            let mut left = self.visitor_stack.remove(remove_at);
            for _ in 0..(len - 1) / 2 {
                let op = self.visitor_stack.remove(remove_at);
                assert!(op.is_op());
                if op.is_mul_op() {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.mul(&right));
                } else {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.div(&right));
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_power_expression(&mut self, ctx: &Power_expressionContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 {
            dbg!(&self.visitor_stack);
            let remove_at = stack_len - len;
            let mut left = self.visitor_stack.remove(remove_at);
            for _ in 0..(len - 1) / 2 {
                let op = self.visitor_stack.remove(remove_at);
                assert!(op.is_pow_op());
                let right = self.visitor_stack.remove(remove_at);
                left = Rc::new(left.pow(&right));
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_identifierAtom(&mut self, ctx: &IdentifierAtomContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        assert_eq!(len, 1);
        let var_text = ctx.get_text();
        let var_symbol = Rc::new(Basic::symbol(&var_text));
        self.visitor_stack.push(var_symbol);
        res
    }

    fn visit_numberAtom(&mut self, ctx: &NumberAtomContext<'input>) -> Self::Return {
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

    fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
        self.visit_children(ctx)
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

    fn visit_powop(&mut self, ctx: &PowopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "^" || op_text == "**" {
            self.visitor_stack.push(Rc::new(Basic::pow_op()));
        }
        res
    }

    // Implement other visitor methods as needed for different node types
    // This is a basic implementation that just traverses the tree
    // You would need to add specific logic for evaluating expressions
}

pub fn evaluate_ascii_math_block(input: &str) -> Result<String, String> {
    let mut visitor = AsciiMathBasicVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let result = parser.block();
    // If parsing succeeded, visit the parse tree
    match result {
        Ok(context) => {
            let _ = visitor.visit(&*context);
            let result = format!("{:?}", visitor.result_table);
            Ok(result)
        }
        Err(e) => Err(format!("parser error {}", e).to_string()),
    }
}
