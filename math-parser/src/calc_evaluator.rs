#![allow(unused)]
use core::num;
use std::result;

use antlr_rust::tree::ParseTree;
use antlr_rust::tree::{ParseTreeVisitorCompat, Tree};
use log::info;

use crate::gen_calc_parser::calculatorparser::{
    calculatorParserContextType, AtomContext, BlockContext, ConstantContext, CurrencyContext,
    EquationContext, ExpressionContext, Func_Context, FuncnameContext, FunctionDefinitionContext,
    MultiplyingExpressionContext, PowExpressionContext, RelopContext, ScientificContext,
    SignedAtomContext, VariableContext,
};
use crate::gen_calc_parser::calculatorvisitor::calculatorVisitorCompat;
use symengine_rs::basic::Basic;

pub struct SymBasicCalcVisitor {
    pub tmp_result: Basic,
    pub result_stack: Vec<Basic>,
    pub block_result: Vec<Basic>,
    pub symbol_table: std::collections::HashMap<String, Basic>,
}

impl SymBasicCalcVisitor {
    pub fn new() -> Self {
        SymBasicCalcVisitor {
            tmp_result: Basic::default(),
            result_stack: Vec::new(),
            block_result: Vec::new(),
            symbol_table: std::collections::HashMap::new(),
        }
    }
}

impl ParseTreeVisitorCompat<'_> for SymBasicCalcVisitor {
    type Node = calculatorParserContextType;
    type Return = Basic;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.tmp_result
    }

    fn aggregate_results(&self, _aggregate: Self::Return, _next: Self::Return) -> Self::Return {
        _next // add Custom logic for aggregating results
    }
}

impl<'input> calculatorVisitorCompat<'input> for SymBasicCalcVisitor {
    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        // info!("Visiting Block: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_functionDefinition(
        &mut self,
        ctx: &FunctionDefinitionContext<'input>,
    ) -> Self::Return {
        // info!("Visiting FunctionDefinition: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_equation(&mut self, ctx: &EquationContext<'input>) -> Self::Return {
        // info!("Visiting Equation: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx);
        info!("Visiting Expression: {}", ctx.get_text());
        ctx.get_children()
            .for_each(|child| info!("Child: {}", child.get_text()));
        info!("Result stack: {:?}", self.result_stack);
        Basic::default()
    }

    fn visit_multop(
        &mut self,
        ctx: &crate::gen_calc_parser::calculatorparser::MultopContext<'input>,
    ) -> Self::Return {
        Basic::default()
    }

    fn visit_sumop(
        &mut self,
        ctx: &crate::gen_calc_parser::calculatorparser::SumopContext<'input>,
    ) -> Self::Return {
        Basic::default()
    }

    fn visit_multiplyingExpression(
        &mut self,
        ctx: &MultiplyingExpressionContext<'input>,
    ) -> Self::Return {
        // info!("Visiting MultiplyingExpression: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>) -> Self::Return {
        // info!("Visiting PowExpression: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_signedAtom(&mut self, ctx: &SignedAtomContext<'input>) -> Self::Return {
        self.visit_children(ctx);
        let negate = ctx.get_children().any(|child| child.get_text() == "-");
        if negate {
            let signed_basic = self.result_stack.pop().unwrap();
            let result = signed_basic.neg();
            self.result_stack.push(result);
        }
        Basic::default()
    }

    fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
        // info!("Visiting Atom: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_scientific(&mut self, ctx: &ScientificContext<'input>) -> Self::Return {
        self.visit_children(ctx);
        let number: f64 = ctx.get_text().parse().unwrap_or(0.0);
        let result = Basic::real(number);
        info!("Visited Scientific returning: {}", result);
        self.result_stack.push(result);
        Basic::default()
    }

    fn visit_currency(&mut self, ctx: &CurrencyContext<'input>) -> Self::Return {
        self.visit_children(ctx);
        // Extract the text of the currency node
        let currency_text = ctx.get_text();

        // Parse the currency value (assuming it's a number with a currency symbol, e.g., "$100")
        let value: f64 = currency_text
            .trim_start_matches(|c: char| !c.is_numeric()) // Remove non-numeric prefix (e.g., "$")
            .parse()
            .unwrap_or(0.0);

        let result = Basic::real(value);
        self.result_stack.push(result);
        Basic::default()
    }

    fn visit_constant(&mut self, ctx: &ConstantContext<'input>) -> Self::Return {
        // info!("Visiting Constant: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        // info!("Visiting Variable: {}", ctx.get_text());
        self.visit_children(ctx);
        let var_text = ctx.get_text();
        let var_symbol = Basic::symbol(&var_text);
        self.result_stack.push(var_symbol);
        Basic::default()
    }

    fn visit_func_(&mut self, ctx: &Func_Context<'input>) -> Self::Return {
        // info!("Visiting Func: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_funcname(&mut self, ctx: &FuncnameContext<'input>) -> Self::Return {
        // info!("Visiting Funcname: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
        // info!("Visiting Relop: {}", ctx.get_text());
        self.visit_children(ctx)
    }
}
