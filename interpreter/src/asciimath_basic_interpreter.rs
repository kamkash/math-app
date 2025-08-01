macro_rules! filter_optional_children_texts {
    ($ctx:expr) => {
        $ctx.get_children()
            .filter(|child| {
                let child_text = child.get_text();
                child_text != "(" && child_text != ")"
            })
            .map(|child| child.get_text())
            .collect::<Vec<_>>()
    };
}

use std::collections::VecDeque;
use std::rc::Rc;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};
use antlr_rust::InputStream;
use asciimath2lexer::AsciiMath2Lexer;
use log::info;
use math_core::common::LogicalOperator;
use math_parser::gen_parsers::asciimath2lexer;
use math_parser::gen_parsers::asciimath2parser::{
    AsciiMath2Parser, AsciiMath2ParserContextType, ExplicitKeywordCallContext,
    ExplicitKeywordCallContextAttrs, IdentifierAtomContext, IntegralExpressionContext,
    MultopContext, NumberAtomContext, Power_expressionContext, PowopContext, RelopContext,
    Scripted_op_expressionContext, SumopContext,
};
use math_parser::gen_parsers::asciimath2visitor::AsciiMath2VisitorCompat;
use symengine_rs::basic::Basic;

use crate::{create_function, SymEquation};

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
                let pairs: Vec<(&Rc<Basic>, &Rc<Basic>)> = self.symbol_table.iter().collect();
                let value = Basic::rc_subs(expr, &pairs);
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

    fn visit_mult_div_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Mult_div_expressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 {
            // dbg!(&self.visitor_stack);
            // implicit multiplication add missing multop
            let remove_at = stack_len - len;
            let mut mult_stack: VecDeque<Rc<Basic>> = VecDeque::new();
            for index in 0..len {
                let left_item = self.visitor_stack.remove(remove_at);
                if index % 2 == 0 {
                    mult_stack.push_front(left_item);
                } else {
                    if left_item.is_op() {
                        mult_stack.push_front(left_item);
                    } else {
                        mult_stack.push_front(Rc::new(Basic::mul_op()));
                        mult_stack.push_front(left_item);
                    }
                }
            }
            let mut left: Rc<Basic> = Rc::new(Basic::default());
            let mut right: Rc<Basic>;
            let mut op: Rc<Basic>;
            while mult_stack.len() > 0 {
                if left.is_default() {
                    left = Rc::clone(&mult_stack.pop_back().unwrap());
                    op = Rc::clone(&mult_stack.pop_back().unwrap());
                    right = Rc::clone(&mult_stack.pop_back().unwrap());
                } else {
                    op = Rc::clone(&mult_stack.pop_back().unwrap());
                    right = Rc::clone(&mult_stack.pop_back().unwrap());
                }
                assert!(op.is_op());
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

    fn visit_power_expression(&mut self, ctx: &Power_expressionContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 {
            // dbg!(&self.visitor_stack);
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
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        let rel_op = LogicalOperator::from_str_token(&op_text);
        let basic_op = Rc::new(Basic::logical_op(rel_op.unwrap()));
        self.visitor_stack.push(basic_op);
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

    fn visit_powop(&mut self, ctx: &PowopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "^" || op_text == "**" {
            self.visitor_stack.push(Rc::new(Basic::pow_op()));
        }
        res
    }

    fn visit_scripted_op_expression(
        &mut self,
        ctx: &Scripted_op_expressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        // dbg!(&self.visitor_stack);
        // filter optional children
        let children: Vec<String> = filter_optional_children_texts!(ctx);
        // dbg!(&children);
        if let Some(func) = create_function(children[0].as_str()) {
            // If we have a valid function, we push it onto the stack.
            let arg = self.visitor_stack.pop().unwrap(); // bad assumption: there is always one argument
            let sym_func = func.generate(&[arg]);
            self.visitor_stack.push(sym_func);
        }
        // dbg!(&self.visitor_stack);
        // dbg!(&Basic::is_function(self.visitor_stack.last().unwrap()));
        res
    }

    fn visit_explicitKeywordCall(
        &mut self,
        ctx: &ExplicitKeywordCallContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        // dbg!(&self.visitor_stack);

        // filter optional children
        let children: Vec<String> = filter_optional_children_texts!(ctx);
        // function arguments on top of the stack
        let mut args = Vec::new();
        for i in 1..children.len() {
            let arg_text = ctx.get_child(i).unwrap().get_text();
            if let Some(arg_basic) = self.visitor_stack.pop() {
                args.push(arg_basic);
            } else {
                args.push(Rc::new(Basic::symbol(&arg_text)));
            }
        }
        args.reverse();

        let func_name = if ctx.scripted_op_expression().is_some() {
            let func = self.visitor_stack.pop().unwrap();
            assert!(
                Basic::is_function(&func),
                "Expected a function, found: {:?}",
                func
            );
            func.to_string()
        } else {
            ctx.get_child(0).unwrap().get_text()
        };

        let func_basic = Rc::new(Basic::function(&func_name, &args));
        self.visitor_stack.push(func_basic);
        res
    }

    fn visit_integralExpression(
        &mut self,
        ctx: &IntegralExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_constantAtom(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::ConstantAtomContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let constant_text = ctx.get_text();
        let constant_value = match constant_text.as_str() {
            "pi" => {
                let pi = Rc::new(Basic::pi());
                let pi_val = Rc::new(pi.evalf(Basic::DOUBLE_PRECISION_BITS, true));
                self.symbol_table.insert(Rc::clone(&pi), Rc::clone(&pi_val));
                pi
            }
            "e" => {
                let e = Rc::new(Basic::e());
                let e_val = Rc::new(e.evalf(Basic::DOUBLE_PRECISION_BITS, true));
                self.symbol_table.insert(Rc::clone(&e), Rc::clone(&e_val));
                e
            }
            _ => Basic::symbol(&constant_text).into(),
        };
        self.visitor_stack.push(constant_value);
        res
    }

    // Implement other visitor methods as needed for different node types
    // This is a basic implementation that just traverses the tree
    // You would need to add specific logic for evaluating expressions
}

pub fn evaluate_ascii_math_block(input: &str) -> Result<String, String> {
    info!("evaluate_ascii_math_block: {}", input);
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
