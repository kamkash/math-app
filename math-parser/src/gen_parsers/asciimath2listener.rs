#![allow(nonstandard_style)]
// Generated from AsciiMath2.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::asciimath2parser::*;

pub trait AsciiMath2Listener<'input> : ParseTreeListener<'input,AsciiMath2ParserContextType>{
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#logical_expression}.
 * @param ctx the parse tree
 */
fn enter_logical_expression(&mut self, _ctx: &Logical_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#logical_expression}.
 * @param ctx the parse tree
 */
fn exit_logical_expression(&mut self, _ctx: &Logical_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#relation_expression}.
 * @param ctx the parse tree
 */
fn enter_relation_expression(&mut self, _ctx: &Relation_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#relation_expression}.
 * @param ctx the parse tree
 */
fn exit_relation_expression(&mut self, _ctx: &Relation_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#relation_expression_no_rhs}.
 * @param ctx the parse tree
 */
fn enter_relation_expression_no_rhs(&mut self, _ctx: &Relation_expression_no_rhsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#relation_expression_no_rhs}.
 * @param ctx the parse tree
 */
fn exit_relation_expression_no_rhs(&mut self, _ctx: &Relation_expression_no_rhsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#add_sub_expression}.
 * @param ctx the parse tree
 */
fn enter_add_sub_expression(&mut self, _ctx: &Add_sub_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#add_sub_expression}.
 * @param ctx the parse tree
 */
fn exit_add_sub_expression(&mut self, _ctx: &Add_sub_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#mult_div_implicit_expression}.
 * @param ctx the parse tree
 */
fn enter_mult_div_implicit_expression(&mut self, _ctx: &Mult_div_implicit_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#mult_div_implicit_expression}.
 * @param ctx the parse tree
 */
fn exit_mult_div_implicit_expression(&mut self, _ctx: &Mult_div_implicit_expressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code unaryPlusMinus}
 * labeled alternative in {@link AsciiMath2Parser#unary_op_expression}.
 * @param ctx the parse tree
 */
fn enter_unaryPlusMinus(&mut self, _ctx: &UnaryPlusMinusContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code unaryPlusMinus}
 * labeled alternative in {@link AsciiMath2Parser#unary_op_expression}.
 * @param ctx the parse tree
 */
fn exit_unaryPlusMinus(&mut self, _ctx: &UnaryPlusMinusContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code noUnaryOperator}
 * labeled alternative in {@link AsciiMath2Parser#unary_op_expression}.
 * @param ctx the parse tree
 */
fn enter_noUnaryOperator(&mut self, _ctx: &NoUnaryOperatorContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code noUnaryOperator}
 * labeled alternative in {@link AsciiMath2Parser#unary_op_expression}.
 * @param ctx the parse tree
 */
fn exit_noUnaryOperator(&mut self, _ctx: &NoUnaryOperatorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#differential}.
 * @param ctx the parse tree
 */
fn enter_differential(&mut self, _ctx: &DifferentialContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#differential}.
 * @param ctx the parse tree
 */
fn exit_differential(&mut self, _ctx: &DifferentialContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code powerSubscriptExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn enter_powerSubscriptExpression(&mut self, _ctx: &PowerSubscriptExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code powerSubscriptExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn exit_powerSubscriptExpression(&mut self, _ctx: &PowerSubscriptExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code subscriptPowerExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn enter_subscriptPowerExpression(&mut self, _ctx: &SubscriptPowerExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code subscriptPowerExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn exit_subscriptPowerExpression(&mut self, _ctx: &SubscriptPowerExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code powerExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn enter_powerExpression(&mut self, _ctx: &PowerExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code powerExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn exit_powerExpression(&mut self, _ctx: &PowerExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code subscriptExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn enter_subscriptExpression(&mut self, _ctx: &SubscriptExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code subscriptExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn exit_subscriptExpression(&mut self, _ctx: &SubscriptExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code primeExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn enter_primeExpression(&mut self, _ctx: &PrimeExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code primeExpression}
 * labeled alternative in {@link AsciiMath2Parser#script_op_expression}.
 * @param ctx the parse tree
 */
fn exit_primeExpression(&mut self, _ctx: &PrimeExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code explicitIdentifierCall}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_explicitIdentifierCall(&mut self, _ctx: &ExplicitIdentifierCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code explicitIdentifierCall}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_explicitIdentifierCall(&mut self, _ctx: &ExplicitIdentifierCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parenColumnVector}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_parenColumnVector(&mut self, _ctx: &ParenColumnVectorContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parenColumnVector}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_parenColumnVector(&mut self, _ctx: &ParenColumnVectorContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parenMatrix}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_parenMatrix(&mut self, _ctx: &ParenMatrixContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parenMatrix}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_parenMatrix(&mut self, _ctx: &ParenMatrixContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code parenExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_parenExpression(&mut self, _ctx: &ParenExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code parenExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_parenExpression(&mut self, _ctx: &ParenExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code bracketMatrix}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_bracketMatrix(&mut self, _ctx: &BracketMatrixContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code bracketMatrix}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_bracketMatrix(&mut self, _ctx: &BracketMatrixContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code angleBracketRowVector}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_angleBracketRowVector(&mut self, _ctx: &AngleBracketRowVectorContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code angleBracketRowVector}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_angleBracketRowVector(&mut self, _ctx: &AngleBracketRowVectorContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code braceExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_braceExpression(&mut self, _ctx: &BraceExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code braceExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_braceExpression(&mut self, _ctx: &BraceExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code absExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_absExpression(&mut self, _ctx: &AbsExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code absExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_absExpression(&mut self, _ctx: &AbsExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code explicitKeywordCall}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_explicitKeywordCall(&mut self, _ctx: &ExplicitKeywordCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code explicitKeywordCall}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_explicitKeywordCall(&mut self, _ctx: &ExplicitKeywordCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code simpleKeywordCall}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_simpleKeywordCall(&mut self, _ctx: &SimpleKeywordCallContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code simpleKeywordCall}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_simpleKeywordCall(&mut self, _ctx: &SimpleKeywordCallContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code sqrtFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_sqrtFunction(&mut self, _ctx: &SqrtFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code sqrtFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_sqrtFunction(&mut self, _ctx: &SqrtFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code rootFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_rootFunction(&mut self, _ctx: &RootFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code rootFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_rootFunction(&mut self, _ctx: &RootFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fracFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_fracFunction(&mut self, _ctx: &FracFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fracFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_fracFunction(&mut self, _ctx: &FracFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code textFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_textFunction(&mut self, _ctx: &TextFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code textFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_textFunction(&mut self, _ctx: &TextFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code integralExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_integralExpression(&mut self, _ctx: &IntegralExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code integralExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_integralExpression(&mut self, _ctx: &IntegralExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code derivativeFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_derivativeFunction(&mut self, _ctx: &DerivativeFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code derivativeFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_derivativeFunction(&mut self, _ctx: &DerivativeFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code partialFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_partialFunction(&mut self, _ctx: &PartialFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code partialFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_partialFunction(&mut self, _ctx: &PartialFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fractionLeibniz}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_fractionLeibniz(&mut self, _ctx: &FractionLeibnizContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fractionLeibniz}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_fractionLeibniz(&mut self, _ctx: &FractionLeibnizContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code limitExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_limitExpression(&mut self, _ctx: &LimitExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code limitExpression}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_limitExpression(&mut self, _ctx: &LimitExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code matFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_matFunction(&mut self, _ctx: &MatFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code matFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_matFunction(&mut self, _ctx: &MatFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code detFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_detFunction(&mut self, _ctx: &DetFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code detFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_detFunction(&mut self, _ctx: &DetFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code transposeFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_transposeFunction(&mut self, _ctx: &TransposeFunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code transposeFunction}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_transposeFunction(&mut self, _ctx: &TransposeFunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code identifierAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_identifierAtom(&mut self, _ctx: &IdentifierAtomContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code identifierAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_identifierAtom(&mut self, _ctx: &IdentifierAtomContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numberAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_numberAtom(&mut self, _ctx: &NumberAtomContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numberAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_numberAtom(&mut self, _ctx: &NumberAtomContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code numberWithCommasAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_numberWithCommasAtom(&mut self, _ctx: &NumberWithCommasAtomContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code numberWithCommasAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_numberWithCommasAtom(&mut self, _ctx: &NumberWithCommasAtomContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code currencyNumberAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_currencyNumberAtom(&mut self, _ctx: &CurrencyNumberAtomContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code currencyNumberAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_currencyNumberAtom(&mut self, _ctx: &CurrencyNumberAtomContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code greekLetterAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_greekLetterAtom(&mut self, _ctx: &GreekLetterAtomContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code greekLetterAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_greekLetterAtom(&mut self, _ctx: &GreekLetterAtomContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code constantAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_constantAtom(&mut self, _ctx: &ConstantAtomContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code constantAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_constantAtom(&mut self, _ctx: &ConstantAtomContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code stringAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_stringAtom(&mut self, _ctx: &StringAtomContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code stringAtom}
 * labeled alternative in {@link AsciiMath2Parser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_stringAtom(&mut self, _ctx: &StringAtomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#paren_element_for_column_vector}.
 * @param ctx the parse tree
 */
fn enter_paren_element_for_column_vector(&mut self, _ctx: &Paren_element_for_column_vectorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#paren_element_for_column_vector}.
 * @param ctx the parse tree
 */
fn exit_paren_element_for_column_vector(&mut self, _ctx: &Paren_element_for_column_vectorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#arguments}.
 * @param ctx the parse tree
 */
fn enter_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#arguments}.
 * @param ctx the parse tree
 */
fn exit_arguments(&mut self, _ctx: &ArgumentsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#text_argument}.
 * @param ctx the parse tree
 */
fn enter_text_argument(&mut self, _ctx: &Text_argumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#text_argument}.
 * @param ctx the parse tree
 */
fn exit_text_argument(&mut self, _ctx: &Text_argumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#wrt_argument}.
 * @param ctx the parse tree
 */
fn enter_wrt_argument(&mut self, _ctx: &Wrt_argumentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#wrt_argument}.
 * @param ctx the parse tree
 */
fn exit_wrt_argument(&mut self, _ctx: &Wrt_argumentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#matrix_content}.
 * @param ctx the parse tree
 */
fn enter_matrix_content(&mut self, _ctx: &Matrix_contentContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#matrix_content}.
 * @param ctx the parse tree
 */
fn exit_matrix_content(&mut self, _ctx: &Matrix_contentContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#matrix_row}.
 * @param ctx the parse tree
 */
fn enter_matrix_row(&mut self, _ctx: &Matrix_rowContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#matrix_row}.
 * @param ctx the parse tree
 */
fn exit_matrix_row(&mut self, _ctx: &Matrix_rowContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#keyword_func}.
 * @param ctx the parse tree
 */
fn enter_keyword_func(&mut self, _ctx: &Keyword_funcContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#keyword_func}.
 * @param ctx the parse tree
 */
fn exit_keyword_func(&mut self, _ctx: &Keyword_funcContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#simple_keyword_func}.
 * @param ctx the parse tree
 */
fn enter_simple_keyword_func(&mut self, _ctx: &Simple_keyword_funcContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#simple_keyword_func}.
 * @param ctx the parse tree
 */
fn exit_simple_keyword_func(&mut self, _ctx: &Simple_keyword_funcContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#deriv_function}.
 * @param ctx the parse tree
 */
fn enter_deriv_function(&mut self, _ctx: &Deriv_functionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#deriv_function}.
 * @param ctx the parse tree
 */
fn exit_deriv_function(&mut self, _ctx: &Deriv_functionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#d_by_d}.
 * @param ctx the parse tree
 */
fn enter_d_by_d(&mut self, _ctx: &D_by_dContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#d_by_d}.
 * @param ctx the parse tree
 */
fn exit_d_by_d(&mut self, _ctx: &D_by_dContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#derivative}.
 * @param ctx the parse tree
 */
fn enter_derivative(&mut self, _ctx: &DerivativeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#derivative}.
 * @param ctx the parse tree
 */
fn exit_derivative(&mut self, _ctx: &DerivativeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#partial_derivative}.
 * @param ctx the parse tree
 */
fn enter_partial_derivative(&mut self, _ctx: &Partial_derivativeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#partial_derivative}.
 * @param ctx the parse tree
 */
fn exit_partial_derivative(&mut self, _ctx: &Partial_derivativeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#function_call}.
 * @param ctx the parse tree
 */
fn enter_function_call(&mut self, _ctx: &Function_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#function_call}.
 * @param ctx the parse tree
 */
fn exit_function_call(&mut self, _ctx: &Function_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link AsciiMath2Parser#constant_symbol}.
 * @param ctx the parse tree
 */
fn enter_constant_symbol(&mut self, _ctx: &Constant_symbolContext<'input>) { }
/**
 * Exit a parse tree produced by {@link AsciiMath2Parser#constant_symbol}.
 * @param ctx the parse tree
 */
fn exit_constant_symbol(&mut self, _ctx: &Constant_symbolContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : AsciiMath2Listener<'input> }


