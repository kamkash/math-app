#![allow(nonstandard_style)]
// Generated from calculator.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::calculatorparser::*;

pub trait calculatorListener<'input> : ParseTreeListener<'input,calculatorParserContextType>{
/**
 * Enter a parse tree produced by {@link calculatorParser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#functionDefinition}.
 * @param ctx the parse tree
 */
fn enter_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#functionDefinition}.
 * @param ctx the parse tree
 */
fn exit_functionDefinition(&mut self, _ctx: &FunctionDefinitionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#equation}.
 * @param ctx the parse tree
 */
fn enter_equation(&mut self, _ctx: &EquationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#equation}.
 * @param ctx the parse tree
 */
fn exit_equation(&mut self, _ctx: &EquationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#multiplyingExpression}.
 * @param ctx the parse tree
 */
fn enter_multiplyingExpression(&mut self, _ctx: &MultiplyingExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#multiplyingExpression}.
 * @param ctx the parse tree
 */
fn exit_multiplyingExpression(&mut self, _ctx: &MultiplyingExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#powExpression}.
 * @param ctx the parse tree
 */
fn enter_powExpression(&mut self, _ctx: &PowExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#powExpression}.
 * @param ctx the parse tree
 */
fn exit_powExpression(&mut self, _ctx: &PowExpressionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#signedAtom}.
 * @param ctx the parse tree
 */
fn enter_signedAtom(&mut self, _ctx: &SignedAtomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#signedAtom}.
 * @param ctx the parse tree
 */
fn exit_signedAtom(&mut self, _ctx: &SignedAtomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atom(&mut self, _ctx: &AtomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#scientific}.
 * @param ctx the parse tree
 */
fn enter_scientific(&mut self, _ctx: &ScientificContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#scientific}.
 * @param ctx the parse tree
 */
fn exit_scientific(&mut self, _ctx: &ScientificContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#currency}.
 * @param ctx the parse tree
 */
fn enter_currency(&mut self, _ctx: &CurrencyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#currency}.
 * @param ctx the parse tree
 */
fn exit_currency(&mut self, _ctx: &CurrencyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#constant}.
 * @param ctx the parse tree
 */
fn enter_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#constant}.
 * @param ctx the parse tree
 */
fn exit_constant(&mut self, _ctx: &ConstantContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#variable}.
 * @param ctx the parse tree
 */
fn enter_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#variable}.
 * @param ctx the parse tree
 */
fn exit_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#func_}.
 * @param ctx the parse tree
 */
fn enter_func_(&mut self, _ctx: &Func_Context<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#func_}.
 * @param ctx the parse tree
 */
fn exit_func_(&mut self, _ctx: &Func_Context<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#funcname}.
 * @param ctx the parse tree
 */
fn enter_funcname(&mut self, _ctx: &FuncnameContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#funcname}.
 * @param ctx the parse tree
 */
fn exit_funcname(&mut self, _ctx: &FuncnameContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#relop}.
 * @param ctx the parse tree
 */
fn enter_relop(&mut self, _ctx: &RelopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#relop}.
 * @param ctx the parse tree
 */
fn exit_relop(&mut self, _ctx: &RelopContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#sumop}.
 * @param ctx the parse tree
 */
fn enter_sumop(&mut self, _ctx: &SumopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#sumop}.
 * @param ctx the parse tree
 */
fn exit_sumop(&mut self, _ctx: &SumopContext<'input>) { }
/**
 * Enter a parse tree produced by {@link calculatorParser#multop}.
 * @param ctx the parse tree
 */
fn enter_multop(&mut self, _ctx: &MultopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link calculatorParser#multop}.
 * @param ctx the parse tree
 */
fn exit_multop(&mut self, _ctx: &MultopContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : calculatorListener<'input> }


