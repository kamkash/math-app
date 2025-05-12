use std::rc::Rc;

use crate::gen_parsers::asciimath2parser::{
    AsciiMath2ParserContextType, BraceExpressionContext, BracketMatrixContext,
    NoUnaryOperatorContext, ParenExpressionContext,
};
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};
use log::info;
use symengine_rs::basic::Basic;

use crate::gen_parsers::asciimath2visitor::AsciiMath2VisitorCompat;
use crate::{Relop, SymEquation};

pub struct AsciiMathVisitor {
    equation_count: u32,
    pub tmp_result: String,
    pub block_expressions: Vec<SymEquation>,
    pub symbol_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
    pub result_table: std::collections::HashMap<Rc<Basic>, Rc<Basic>>,
}

impl AsciiMathVisitor {
    pub fn new() -> Self {
        AsciiMathVisitor {
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
                let value = Basic::rc_subs(expr, self.symbol_table.iter().map(|(k, v)| (k, v)));
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
        let len = ctx.get_child_count();
        
        if len == 3 {
            info!("Equation: count {}, {}", len, ctx.get_text());
            let mut left = ctx.get_child(0).unwrap().get_text().to_string();
            let oper = ctx.get_child(1).unwrap().get_text().to_string();
            let mut right = ctx.get_child(2).unwrap().get_text().to_string();
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
            let symeq = SymEquation::new(
                Rc::new(Basic::parse(&left).unwrap()),                
                Rc::new(Basic::parse(&right).unwrap()),
                Relop::from(oper.as_str()),
            );
            self.block_expressions.push(symeq);
        }
        res
    }

    fn visit_relation_expression_no_rhs(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Relation_expression_no_rhsContext<'input>,
    ) -> Self::Return {
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

    fn visit_appliedDByDxFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::AppliedDByDxFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_appliedDByDxPrefix(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::AppliedDByDxPrefixContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_noUnaryOperator(&mut self, ctx: &NoUnaryOperatorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_d_dx_function(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::D_dx_functionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_d_dx_prefix_operator(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::D_dx_prefix_operatorContext<'input>,
    ) -> Self::Return {
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

    fn visit_parenColumnVector(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::ParenColumnVectorContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_parenMatrix(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::ParenMatrixContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_parenExpression(&mut self, ctx: &ParenExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_bracketMatrix(&mut self, ctx: &BracketMatrixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_angleBracketRowVector(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::AngleBracketRowVectorContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_braceExpression(&mut self, ctx: &BraceExpressionContext<'input>) -> Self::Return {
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

    fn visit_derivativeFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::DerivativeFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
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

    fn visit_limitExpression(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::LimitExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_matFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::MatFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_detFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::DetFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_transposeFunction(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::TransposeFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_identifierAtom(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::IdentifierAtomContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_numberAtom(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::NumberAtomContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_numberWithCommasAtom(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::NumberWithCommasAtomContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_currencyNumberAtom(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::CurrencyNumberAtomContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_greekLetterAtom(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::GreekLetterAtomContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_constantAtom(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::ConstantAtomContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_stringAtom(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::StringAtomContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_paren_element_for_column_vector(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Paren_element_for_column_vectorContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_arguments(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::ArgumentsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_text_argument(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Text_argumentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_wrt_argument(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Wrt_argumentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_matrix_content(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Matrix_contentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_matrix_row(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Matrix_rowContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_constant_symbol(
        &mut self,
        ctx: &crate::gen_parsers::asciimath2parser::Constant_symbolContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    // Implement other visitor methods as needed for different node types
    // This is a basic implementation that just traverses the tree
    // You would need to add specific logic for evaluating expressions
}
