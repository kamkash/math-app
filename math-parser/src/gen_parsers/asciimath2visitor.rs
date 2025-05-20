#![allow(nonstandard_style)]
// Generated from AsciiMath2.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::asciimath2parser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link AsciiMath2Parser}.
 */
pub trait AsciiMath2Visitor<'input>: ParseTreeVisitor<'input,AsciiMath2ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#block}.
	 * @param ctx the parse tree
	 */
	fn visit_block(&mut self, ctx: &BlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#logical_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_logical_expression(&mut self, ctx: &Logical_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#relation_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_relation_expression(&mut self, ctx: &Relation_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#relation_expression_no_rhs}.
	 * @param ctx the parse tree
	 */
	fn visit_relation_expression_no_rhs(&mut self, ctx: &Relation_expression_no_rhsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#add_sub_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_add_sub_expression(&mut self, ctx: &Add_sub_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#mult_div_implicit_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_mult_div_implicit_expression(&mut self, ctx: &Mult_div_implicit_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code unaryPlusMinus}
	 * labeled alternative in {@link AsciiMath2Parser#unary_op_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_unaryPlusMinus(&mut self, ctx: &UnaryPlusMinusContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code noUnaryOperator}
	 * labeled alternative in {@link AsciiMath2Parser#unary_op_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_noUnaryOperator(&mut self, ctx: &NoUnaryOperatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#differential}.
	 * @param ctx the parse tree
	 */
	fn visit_differential(&mut self, ctx: &DifferentialContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#integral_body}.
	 * @param ctx the parse tree
	 */
	fn visit_integral_body(&mut self, ctx: &Integral_bodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#integral_upper_limit}.
	 * @param ctx the parse tree
	 */
	fn visit_integral_upper_limit(&mut self, ctx: &Integral_upper_limitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#integral_lower_limit}.
	 * @param ctx the parse tree
	 */
	fn visit_integral_lower_limit(&mut self, ctx: &Integral_lower_limitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code powerSubscriptExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_powerSubscriptExpression(&mut self, ctx: &PowerSubscriptExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code subscriptPowerExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_subscriptPowerExpression(&mut self, ctx: &SubscriptPowerExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code powerExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_powerExpression(&mut self, ctx: &PowerExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code subscriptExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_subscriptExpression(&mut self, ctx: &SubscriptExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code primeExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_primeExpression(&mut self, ctx: &PrimeExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code explicitIdentifierCall}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_explicitIdentifierCall(&mut self, ctx: &ExplicitIdentifierCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parenExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_parenExpression(&mut self, ctx: &ParenExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code braceExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_braceExpression(&mut self, ctx: &BraceExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code absExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_absExpression(&mut self, ctx: &AbsExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code explicitKeywordCall}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_explicitKeywordCall(&mut self, ctx: &ExplicitKeywordCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code simpleKeywordCall}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_simpleKeywordCall(&mut self, ctx: &SimpleKeywordCallContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code sqrtFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_sqrtFunction(&mut self, ctx: &SqrtFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code rootFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_rootFunction(&mut self, ctx: &RootFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code fracFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_fracFunction(&mut self, ctx: &FracFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code textFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_textFunction(&mut self, ctx: &TextFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code integralExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_integralExpression(&mut self, ctx: &IntegralExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code derivativeFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_derivativeFunction(&mut self, ctx: &DerivativeFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code partialFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_partialFunction(&mut self, ctx: &PartialFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code fractionLeibniz}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_fractionLeibniz(&mut self, ctx: &FractionLeibnizContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code limitExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_limitExpression(&mut self, ctx: &LimitExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parenColumnVector}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_parenColumnVector(&mut self, ctx: &ParenColumnVectorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parenMatrix}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_parenMatrix(&mut self, ctx: &ParenMatrixContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code bracketMatrix}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_bracketMatrix(&mut self, ctx: &BracketMatrixContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code angleBracketRowVector}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_angleBracketRowVector(&mut self, ctx: &AngleBracketRowVectorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code matFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_matFunction(&mut self, ctx: &MatFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code detFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_detFunction(&mut self, ctx: &DetFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code transposeFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_transposeFunction(&mut self, ctx: &TransposeFunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code identifierAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_identifierAtom(&mut self, ctx: &IdentifierAtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numberAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_numberAtom(&mut self, ctx: &NumberAtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code numberWithCommasAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_numberWithCommasAtom(&mut self, ctx: &NumberWithCommasAtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code currencyNumberAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_currencyNumberAtom(&mut self, ctx: &CurrencyNumberAtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code greekLetterAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_greekLetterAtom(&mut self, ctx: &GreekLetterAtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code constantAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_constantAtom(&mut self, ctx: &ConstantAtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code stringAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_stringAtom(&mut self, ctx: &StringAtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#paren_element_for_column_vector}.
	 * @param ctx the parse tree
	 */
	fn visit_paren_element_for_column_vector(&mut self, ctx: &Paren_element_for_column_vectorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#arguments}.
	 * @param ctx the parse tree
	 */
	fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#text_argument}.
	 * @param ctx the parse tree
	 */
	fn visit_text_argument(&mut self, ctx: &Text_argumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#wrt_argument}.
	 * @param ctx the parse tree
	 */
	fn visit_wrt_argument(&mut self, ctx: &Wrt_argumentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#matrix_content}.
	 * @param ctx the parse tree
	 */
	fn visit_matrix_content(&mut self, ctx: &Matrix_contentContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#matrix_row}.
	 * @param ctx the parse tree
	 */
	fn visit_matrix_row(&mut self, ctx: &Matrix_rowContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#keyword_func}.
	 * @param ctx the parse tree
	 */
	fn visit_keyword_func(&mut self, ctx: &Keyword_funcContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#simple_keyword_func}.
	 * @param ctx the parse tree
	 */
	fn visit_simple_keyword_func(&mut self, ctx: &Simple_keyword_funcContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#deriv_function}.
	 * @param ctx the parse tree
	 */
	fn visit_deriv_function(&mut self, ctx: &Deriv_functionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#d_by_d}.
	 * @param ctx the parse tree
	 */
	fn visit_d_by_d(&mut self, ctx: &D_by_dContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#derivative}.
	 * @param ctx the parse tree
	 */
	fn visit_derivative(&mut self, ctx: &DerivativeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#partial_derivative}.
	 * @param ctx the parse tree
	 */
	fn visit_partial_derivative(&mut self, ctx: &Partial_derivativeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#function_call}.
	 * @param ctx the parse tree
	 */
	fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#constant_symbol}.
	 * @param ctx the parse tree
	 */
	fn visit_constant_symbol(&mut self, ctx: &Constant_symbolContext<'input>) { self.visit_children(ctx) }

}

pub trait AsciiMath2VisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= AsciiMath2ParserContextType>{
	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#block}.
	 * @param ctx the parse tree
	 */
		fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#logical_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_logical_expression(&mut self, ctx: &Logical_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#relation_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_relation_expression(&mut self, ctx: &Relation_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#relation_expression_no_rhs}.
	 * @param ctx the parse tree
	 */
		fn visit_relation_expression_no_rhs(&mut self, ctx: &Relation_expression_no_rhsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#add_sub_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_add_sub_expression(&mut self, ctx: &Add_sub_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#mult_div_implicit_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_mult_div_implicit_expression(&mut self, ctx: &Mult_div_implicit_expressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code unaryPlusMinus}
	 * labeled alternative in {@link AsciiMath2Parser#unary_op_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_unaryPlusMinus(&mut self, ctx: &UnaryPlusMinusContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code noUnaryOperator}
	 * labeled alternative in {@link AsciiMath2Parser#unary_op_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_noUnaryOperator(&mut self, ctx: &NoUnaryOperatorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#differential}.
	 * @param ctx the parse tree
	 */
		fn visit_differential(&mut self, ctx: &DifferentialContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#integral_body}.
	 * @param ctx the parse tree
	 */
		fn visit_integral_body(&mut self, ctx: &Integral_bodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#integral_upper_limit}.
	 * @param ctx the parse tree
	 */
		fn visit_integral_upper_limit(&mut self, ctx: &Integral_upper_limitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#integral_lower_limit}.
	 * @param ctx the parse tree
	 */
		fn visit_integral_lower_limit(&mut self, ctx: &Integral_lower_limitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code powerSubscriptExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_powerSubscriptExpression(&mut self, ctx: &PowerSubscriptExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code subscriptPowerExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_subscriptPowerExpression(&mut self, ctx: &SubscriptPowerExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code powerExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_powerExpression(&mut self, ctx: &PowerExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code subscriptExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_subscriptExpression(&mut self, ctx: &SubscriptExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code primeExpression}
	 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_primeExpression(&mut self, ctx: &PrimeExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code explicitIdentifierCall}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_explicitIdentifierCall(&mut self, ctx: &ExplicitIdentifierCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parenExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_parenExpression(&mut self, ctx: &ParenExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code braceExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_braceExpression(&mut self, ctx: &BraceExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code absExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_absExpression(&mut self, ctx: &AbsExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code explicitKeywordCall}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_explicitKeywordCall(&mut self, ctx: &ExplicitKeywordCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code simpleKeywordCall}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_simpleKeywordCall(&mut self, ctx: &SimpleKeywordCallContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code sqrtFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_sqrtFunction(&mut self, ctx: &SqrtFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code rootFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_rootFunction(&mut self, ctx: &RootFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code fracFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_fracFunction(&mut self, ctx: &FracFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code textFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_textFunction(&mut self, ctx: &TextFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code integralExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_integralExpression(&mut self, ctx: &IntegralExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code derivativeFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_derivativeFunction(&mut self, ctx: &DerivativeFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code partialFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_partialFunction(&mut self, ctx: &PartialFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code fractionLeibniz}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_fractionLeibniz(&mut self, ctx: &FractionLeibnizContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code limitExpression}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_limitExpression(&mut self, ctx: &LimitExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parenColumnVector}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_parenColumnVector(&mut self, ctx: &ParenColumnVectorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parenMatrix}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_parenMatrix(&mut self, ctx: &ParenMatrixContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code bracketMatrix}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_bracketMatrix(&mut self, ctx: &BracketMatrixContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code angleBracketRowVector}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_angleBracketRowVector(&mut self, ctx: &AngleBracketRowVectorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code matFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_matFunction(&mut self, ctx: &MatFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code detFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_detFunction(&mut self, ctx: &DetFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code transposeFunction}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_transposeFunction(&mut self, ctx: &TransposeFunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code identifierAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_identifierAtom(&mut self, ctx: &IdentifierAtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numberAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_numberAtom(&mut self, ctx: &NumberAtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code numberWithCommasAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_numberWithCommasAtom(&mut self, ctx: &NumberWithCommasAtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code currencyNumberAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_currencyNumberAtom(&mut self, ctx: &CurrencyNumberAtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code greekLetterAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_greekLetterAtom(&mut self, ctx: &GreekLetterAtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code constantAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_constantAtom(&mut self, ctx: &ConstantAtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code stringAtom}
	 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
	 * @param ctx the parse tree
	 */
		fn visit_stringAtom(&mut self, ctx: &StringAtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#paren_element_for_column_vector}.
	 * @param ctx the parse tree
	 */
		fn visit_paren_element_for_column_vector(&mut self, ctx: &Paren_element_for_column_vectorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#arguments}.
	 * @param ctx the parse tree
	 */
		fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#text_argument}.
	 * @param ctx the parse tree
	 */
		fn visit_text_argument(&mut self, ctx: &Text_argumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#wrt_argument}.
	 * @param ctx the parse tree
	 */
		fn visit_wrt_argument(&mut self, ctx: &Wrt_argumentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#matrix_content}.
	 * @param ctx the parse tree
	 */
		fn visit_matrix_content(&mut self, ctx: &Matrix_contentContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#matrix_row}.
	 * @param ctx the parse tree
	 */
		fn visit_matrix_row(&mut self, ctx: &Matrix_rowContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#keyword_func}.
	 * @param ctx the parse tree
	 */
		fn visit_keyword_func(&mut self, ctx: &Keyword_funcContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#simple_keyword_func}.
	 * @param ctx the parse tree
	 */
		fn visit_simple_keyword_func(&mut self, ctx: &Simple_keyword_funcContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#deriv_function}.
	 * @param ctx the parse tree
	 */
		fn visit_deriv_function(&mut self, ctx: &Deriv_functionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#d_by_d}.
	 * @param ctx the parse tree
	 */
		fn visit_d_by_d(&mut self, ctx: &D_by_dContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#derivative}.
	 * @param ctx the parse tree
	 */
		fn visit_derivative(&mut self, ctx: &DerivativeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#partial_derivative}.
	 * @param ctx the parse tree
	 */
		fn visit_partial_derivative(&mut self, ctx: &Partial_derivativeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#function_call}.
	 * @param ctx the parse tree
	 */
		fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link AsciiMath2Parser#constant_symbol}.
	 * @param ctx the parse tree
	 */
		fn visit_constant_symbol(&mut self, ctx: &Constant_symbolContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> AsciiMath2Visitor<'input> for T
where
	T: AsciiMath2VisitorCompat<'input>
{
	fn visit_block(&mut self, ctx: &BlockContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_logical_expression(&mut self, ctx: &Logical_expressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_logical_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relation_expression(&mut self, ctx: &Relation_expressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_relation_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relation_expression_no_rhs(&mut self, ctx: &Relation_expression_no_rhsContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_relation_expression_no_rhs(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_add_sub_expression(&mut self, ctx: &Add_sub_expressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_add_sub_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mult_div_implicit_expression(&mut self, ctx: &Mult_div_implicit_expressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_mult_div_implicit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unaryPlusMinus(&mut self, ctx: &UnaryPlusMinusContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_unaryPlusMinus(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_noUnaryOperator(&mut self, ctx: &NoUnaryOperatorContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_noUnaryOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_differential(&mut self, ctx: &DifferentialContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_differential(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_integral_body(&mut self, ctx: &Integral_bodyContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_integral_body(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_integral_upper_limit(&mut self, ctx: &Integral_upper_limitContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_integral_upper_limit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_integral_lower_limit(&mut self, ctx: &Integral_lower_limitContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_integral_lower_limit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_powerSubscriptExpression(&mut self, ctx: &PowerSubscriptExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_powerSubscriptExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subscriptPowerExpression(&mut self, ctx: &SubscriptPowerExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_subscriptPowerExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_powerExpression(&mut self, ctx: &PowerExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_powerExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subscriptExpression(&mut self, ctx: &SubscriptExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_subscriptExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_primeExpression(&mut self, ctx: &PrimeExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_primeExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_explicitIdentifierCall(&mut self, ctx: &ExplicitIdentifierCallContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_explicitIdentifierCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parenExpression(&mut self, ctx: &ParenExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_parenExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_braceExpression(&mut self, ctx: &BraceExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_braceExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_absExpression(&mut self, ctx: &AbsExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_absExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_explicitKeywordCall(&mut self, ctx: &ExplicitKeywordCallContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_explicitKeywordCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simpleKeywordCall(&mut self, ctx: &SimpleKeywordCallContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_simpleKeywordCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sqrtFunction(&mut self, ctx: &SqrtFunctionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_sqrtFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_rootFunction(&mut self, ctx: &RootFunctionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_rootFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fracFunction(&mut self, ctx: &FracFunctionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_fracFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_textFunction(&mut self, ctx: &TextFunctionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_textFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_integralExpression(&mut self, ctx: &IntegralExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_integralExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_derivativeFunction(&mut self, ctx: &DerivativeFunctionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_derivativeFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partialFunction(&mut self, ctx: &PartialFunctionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_partialFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_fractionLeibniz(&mut self, ctx: &FractionLeibnizContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_fractionLeibniz(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_limitExpression(&mut self, ctx: &LimitExpressionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_limitExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parenColumnVector(&mut self, ctx: &ParenColumnVectorContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_parenColumnVector(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parenMatrix(&mut self, ctx: &ParenMatrixContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_parenMatrix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bracketMatrix(&mut self, ctx: &BracketMatrixContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_bracketMatrix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_angleBracketRowVector(&mut self, ctx: &AngleBracketRowVectorContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_angleBracketRowVector(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_matFunction(&mut self, ctx: &MatFunctionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_matFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_detFunction(&mut self, ctx: &DetFunctionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_detFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_transposeFunction(&mut self, ctx: &TransposeFunctionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_transposeFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_identifierAtom(&mut self, ctx: &IdentifierAtomContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_identifierAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberAtom(&mut self, ctx: &NumberAtomContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_numberAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_numberWithCommasAtom(&mut self, ctx: &NumberWithCommasAtomContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_numberWithCommasAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_currencyNumberAtom(&mut self, ctx: &CurrencyNumberAtomContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_currencyNumberAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_greekLetterAtom(&mut self, ctx: &GreekLetterAtomContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_greekLetterAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constantAtom(&mut self, ctx: &ConstantAtomContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_constantAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_stringAtom(&mut self, ctx: &StringAtomContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_stringAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_paren_element_for_column_vector(&mut self, ctx: &Paren_element_for_column_vectorContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_paren_element_for_column_vector(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_arguments(&mut self, ctx: &ArgumentsContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_arguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_text_argument(&mut self, ctx: &Text_argumentContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_text_argument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_wrt_argument(&mut self, ctx: &Wrt_argumentContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_wrt_argument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_matrix_content(&mut self, ctx: &Matrix_contentContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_matrix_content(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_matrix_row(&mut self, ctx: &Matrix_rowContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_matrix_row(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_keyword_func(&mut self, ctx: &Keyword_funcContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_keyword_func(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_simple_keyword_func(&mut self, ctx: &Simple_keyword_funcContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_simple_keyword_func(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_deriv_function(&mut self, ctx: &Deriv_functionContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_deriv_function(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_d_by_d(&mut self, ctx: &D_by_dContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_d_by_d(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_derivative(&mut self, ctx: &DerivativeContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_derivative(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_partial_derivative(&mut self, ctx: &Partial_derivativeContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_partial_derivative(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function_call(&mut self, ctx: &Function_callContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_function_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constant_symbol(&mut self, ctx: &Constant_symbolContext<'input>){
		let result = <Self as AsciiMath2VisitorCompat>::visit_constant_symbol(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}