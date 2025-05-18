use std::rc::Rc;

use crate::gen_parsers::asciimath2lexer::AsciiMath2Lexer;
use crate::gen_parsers::asciimath2parser::{
    AsciiMath2Parser, AsciiMath2ParserContextType, D_by_dContext, Deriv_functionContext,
    DerivativeContext, NoUnaryOperatorContext,
};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};
use antlr_rust::InputStream;
use antlr_rust::TidExt;
use log::info;
use symengine_rs::basic::Basic;

use crate::gen_parsers::asciimath2visitor::AsciiMath2VisitorCompat;
use crate::{Relop, SymEquation};

pub fn evaluate_ascii_math_block(input: &str) -> Result<String, String> {
    let mut visitor = AsciiMathVisitor::new();
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

pub struct AsciiMathVisitor {
    equation_count: u32,
    pub tmp_result: String,
    pub block_expressions: Vec<SymEquation>,
    pub symbol_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
    pub result_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
    visitor_stack: Vec<String>,
}

impl AsciiMathVisitor {
    pub fn new() -> Self {
        AsciiMathVisitor {
            equation_count: 0,
            tmp_result: String::new(),
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

impl<'input> ParseTreeVisitorCompat<'input> for AsciiMathVisitor {
    type Node = AsciiMath2ParserContextType;
    type Return = String;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.tmp_result
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        aggregate + &next
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        Self::Return::default()
    }
}

impl<'input> AsciiMath2VisitorCompat<'input> for AsciiMathVisitor {
    fn visit_block(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::BlockContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        info!("block expressions: {:?}", self.block_expressions);
        self.build_symbol_table();
        self.build_result_table();
        res
    }

    fn visit_expression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::ExpressionContext<'input>,
    ) -> Self::Return {
        self.visitor_stack.clear();
        self.visit_children(ctx)
    }

    fn visit_logical_expression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Logical_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_relation_expression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Relation_expressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        if self.visitor_stack.is_empty() {
            // no one else processed this
            let len = ctx.get_child_count();
            if len == 3 {
                info!("Equation: count {}, {}", len, ctx.get_text());
                let mut left = ctx.get_child(0).unwrap().get_text().to_string();
                let oper = ctx.get_child(1).unwrap().get_text().to_string();
                let mut right = ctx.get_child(2).unwrap().get_text().to_string();
                left = if left.is_empty() {
                    let count = self.equation_count;
                    self.equation_count += 1;
                    format!("__{}__", count).to_string()
                } else {
                    left
                };
                right = if right.is_empty() {
                    let count = self.equation_count;
                    self.equation_count += 1;
                    format!("__{}__", count).to_string()
                } else {
                    right
                };
                let symeq = SymEquation::new(
                    Rc::new(Basic::parse(&left).unwrap()),
                    Rc::new(Basic::parse(&right).unwrap()),
                    Relop::from(oper.as_str()),
                );
                self.block_expressions.push(symeq);
            }
        }
        res
    }

    fn visit_relation_expression_no_rhs(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Relation_expression_no_rhsContext<'input>,
    ) -> Self::Return {
        info!("Relation expression no rhs: {:?}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_add_sub_expression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Add_sub_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_mult_div_implicit_expression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Mult_div_implicit_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_unaryPlusMinus(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::UnaryPlusMinusContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_noUnaryOperator(&mut self, ctx: &NoUnaryOperatorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_differential(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::DifferentialContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_powerSubscriptExpression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::PowerSubscriptExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_subscriptPowerExpression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::SubscriptPowerExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_powerExpression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::PowerExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_subscriptExpression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::SubscriptExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_primeExpression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::PrimeExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_explicitIdentifierCall(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::ExplicitIdentifierCallContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_explicitKeywordCall(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::ExplicitKeywordCallContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_simpleKeywordCall(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::SimpleKeywordCallContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_absExpression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::AbsExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_sqrtFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::SqrtFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_rootFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::RootFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_fracFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::FracFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_textFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::TextFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_integralExpression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::IntegralExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_derivative(&mut self, ctx: &DerivativeContext<'input>) -> Self::Return {
        let len = ctx.get_child_count();
        let res = self.visit_children(ctx);
        for (index, child) in ctx.get_children().enumerate() {
            info!("Child at index {}/{}: {:?}", index, len, child.get_text());
        }
        if len > 1 {
            if let Some(fst_child) = ctx.get_child(0) {
                let is_deriv = fst_child.is::<Deriv_functionContext>();
                let is_dbyd = fst_child.is::<D_by_dContext>();
                info!("First child is driv_function: {:?}", is_deriv);
                info!("First child is d_by_d: {:?}", is_dbyd);
                let exp_str = ctx.get_child(1).unwrap().get_text();
                if is_dbyd {
                    let exp = Basic::parse(&exp_str).unwrap();
                    let deriv = Basic::diff(&exp, &Basic::symbol("x"));
                    info!("Derivative: {:?}", deriv);
                }
            }
        }
        res
    }

    fn visit_partialFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::PartialFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_fractionLeibniz(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::FractionLeibnizContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    // Implement other visitor methods as needed for different node types
    // This is a basic implementation that just traverses the tree
    // You would need to add specific logic for evaluating expressions
}
