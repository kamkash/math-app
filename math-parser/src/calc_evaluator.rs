#![allow(unused)]
use core::num;
use std::fmt;
use std::rc::Rc;
use std::result;
use std::sync::Arc;

use antlr_rust::tree::ParseTree;
use antlr_rust::tree::{ParseTreeVisitorCompat, Tree};
use log::info;

use crate::gen_calc_parser::calculatorparser::MultiplyingExpressionContextAttrs;
use crate::gen_calc_parser::calculatorparser::{
    calculatorParserContextType, AtomContext, BlockContext, ConstantContext, CurrencyContext,
    EquationContext, EquationContextAttrs, ExpressionContext, ExpressionContextAttrs, Func_Context,
    FuncnameContext, FunctionDefinitionContext, MultiplyingExpressionContext, PowExpressionContext,
    RelopContext, ScientificContext, SignedAtomContext, VariableContext,
};
use crate::gen_calc_parser::calculatorvisitor::calculatorVisitorCompat;
use symengine_rs::basic::Basic;

pub enum Relop {
    Equal,
    DoubleEqual,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl From<&str> for Relop {
    fn from(s: &str) -> Self {
        match s {
            "=" => Relop::Equal,
            "==" => Relop::DoubleEqual,
            "!=" => Relop::NotEqual,
            "<" => Relop::LessThan,
            ">" => Relop::GreaterThan,
            "<=" => Relop::LessThanOrEqual,
            ">=" => Relop::GreaterThanOrEqual,
            _ => panic!("Unknown relational operator: {}", s),
        }
    }
}

impl fmt::Debug for Relop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Relop::DoubleEqual => "==",
            Relop::Equal => "=",
            Relop::NotEqual => "!=",
            Relop::LessThan => "<",
            Relop::GreaterThan => ">",
            Relop::LessThanOrEqual => "<=",
            Relop::GreaterThanOrEqual => ">=",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Relop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Relop::DoubleEqual => "==",
            Relop::Equal => "=",
            Relop::NotEqual => "!=",
            Relop::LessThan => "<",
            Relop::GreaterThan => ">",
            Relop::LessThanOrEqual => "<=",
            Relop::GreaterThanOrEqual => ">=",
        };
        write!(f, "{}", s)
    }
}

pub struct SymEquation {
    pub left: Basic,
    pub right: Basic,
    pub relop: Relop,
}

impl SymEquation {
    pub fn new(left: Basic, right: Basic, relop: Relop) -> Self {
        SymEquation { left, right, relop }
    }
}

impl std::fmt::Debug for SymEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.left, self.relop, self.right)
    }
}

impl std::fmt::Display for SymEquation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.relop, self.right)
    }
}

pub struct SymBasicCalcVisitor {
    pub tmp_result: Basic,
    pub result_stack: Vec<Basic>,
    pub block_result: Vec<SymEquation>,
    pub symbol_table: std::collections::HashMap<Basic, Basic>,
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
        //info!("Visiting Block: {}", ctx.get_text());
        let res = self.visit_children(ctx);

        // build symbol table
        for sym_eq in &self.block_result {
            info!("Equation: {:?}", sym_eq);
            let left = &sym_eq.left;
            let right = &sym_eq.right;
            let relop = &sym_eq.relop;
            if left.is_symbol() {
                self.symbol_table.insert(left.clone(), right.clone());
            } else if right.is_symbol() {
                self.symbol_table.insert(right.clone(), left.clone());
            }
        }
        res
    }

    fn visit_functionDefinition(
        &mut self,
        ctx: &FunctionDefinitionContext<'input>,
    ) -> Self::Return {
        //info!("Visiting FunctionDefinition: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_equation(&mut self, ctx: &EquationContext<'input>) -> Self::Return {
        //info!("Visiting Equation: {}", ctx.get_text());
        let res = self.visit_children(ctx);
        let len = self.result_stack.len();
        let right = &self.result_stack[len - 1];
        let left = &self.result_stack[len - 2];
        let equation = SymEquation::new(left.clone(), right.clone(), Relop::Equal);
        self.block_result.push(equation);
        res
    }

    fn visit_relational_expression(
        &mut self,
        ctx: &crate::gen_calc_parser::calculatorparser::Relational_expressionContext<'input>,
    ) -> Self::Return {
        //info!("Visiting RelationalExpression: {}", ctx.get_text());
        let res = self.visit_children(ctx);
        let len = self.result_stack.len();
        let right = &self.result_stack[len - 1];
        let left = &self.result_stack[len - 2];
        let relop_text = ctx.get_child(1).unwrap().get_text();
        let relop = Relop::from(relop_text.as_str());
        let equation = SymEquation::new(left.clone(), right.clone(), relop);
        self.block_result.push(equation);
        res
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
        //info!("Visiting Expression: {}", ctx.get_text());
        let res = self.visit_children(ctx);
        let is_add = ctx.PLUS(0).is_some();
        let is_sub = ctx.MINUS(0).is_some();
        if is_add || is_sub {
            let left = self.result_stack.pop().unwrap();
            let right = self.result_stack.pop().unwrap();
            let result = if is_add {
                left.add(&right)
            } else {
                right.sub(&left)
            };
            self.result_stack.push(result);
        }
        res
    }

    fn visit_multiplyingExpression(
        &mut self,
        ctx: &MultiplyingExpressionContext<'input>,
    ) -> Self::Return {
        //info!("Visiting MultiplyingExpression: {}", ctx.get_text());
        let res = self.visit_children(ctx);
        let is_times = ctx.TIMES(0).is_some();
        let is_divide = ctx.DIV(0).is_some();
        if is_times || is_divide {
            let left = self.result_stack.pop().unwrap();
            let right = self.result_stack.pop().unwrap();
            let result = if is_times {
                left.mul(&right)
            } else {
                right.div(&left)
            };
            self.result_stack.push(result);
        }
        res
    }

    fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>) -> Self::Return {
        //info!("Visiting PowExpression: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_signedAtom(&mut self, ctx: &SignedAtomContext<'input>) -> Self::Return {
        //info!("Visiting SignedAtom: {}", ctx.get_text());
        let res = self.visit_children(ctx);
        let negate = ctx.get_children().any(|child| child.get_text() == "-");
        if negate {
            let signed_basic = self.result_stack.pop().unwrap();
            let result = signed_basic.neg();
            self.result_stack.push(result);
        }
        res
    }

    fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
        //info!("Visiting Atom: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_scientific(&mut self, ctx: &ScientificContext<'input>) -> Self::Return {
        let sci_text = ctx.get_text();
        //info!("Visiting Scientific: {}", sci_text);
        let res = self.visit_children(ctx);
        let filtered: String = sci_text
            .chars()
            .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
            .collect();
        let value: f64 = filtered.parse().unwrap_or(0.0);
        let result = Basic::real(value);
        //info!("**** Visited Scientific: {} {}", value, result);
        self.result_stack.push(result);
        res
    }

    fn visit_currency(&mut self, ctx: &CurrencyContext<'input>) -> Self::Return {
        let currency_text = ctx.get_text();
        //info!("Visiting Currency: {}", currency_text);
        let res = self.visit_children(ctx);
        let filtered: String = currency_text
            .chars()
            .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
            .collect();
        let value: f64 = filtered.parse().unwrap_or(0.0);

        let result = Basic::real(value);
        self.result_stack.push(result);
        res
    }

    fn visit_constant(&mut self, ctx: &ConstantContext<'input>) -> Self::Return {
        //info!("Visiting Constant: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
        let var_text = ctx.get_text();
        //info!("Visiting Variable: {}", var_text);
        let res = self.visit_children(ctx);
        let var_symbol = Basic::symbol(&var_text);
        self.result_stack.push(var_symbol);
        res
    }

    fn visit_func_(&mut self, ctx: &Func_Context<'input>) -> Self::Return {
        //info!("Visiting Func: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_funcname(&mut self, ctx: &FuncnameContext<'input>) -> Self::Return {
        //info!("Visiting Funcname: {}", ctx.get_text());
        self.visit_children(ctx)
    }

    fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
        //info!("Visiting Relop: {}", ctx.get_text());
        self.visit_children(ctx)
    }
}
