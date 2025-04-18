#![allow(nonstandard_style)]
// Generated from calculator.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::calculatorparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link calculatorParser}.
 */
pub trait calculatorVisitor<'input>: ParseTreeVisitor<'input,calculatorParserContextType>{
	/**
	 * Visit a parse tree produced by {@link calculatorParser#block}.
	 * @param ctx the parse tree
	 */
	fn visit_block(&mut self, ctx: &BlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#functionDefinition}.
	 * @param ctx the parse tree
	 */
	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#equation}.
	 * @param ctx the parse tree
	 */
	fn visit_equation(&mut self, ctx: &EquationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#multiplyingExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplyingExpression(&mut self, ctx: &MultiplyingExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#powExpression}.
	 * @param ctx the parse tree
	 */
	fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#signedAtom}.
	 * @param ctx the parse tree
	 */
	fn visit_signedAtom(&mut self, ctx: &SignedAtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atom(&mut self, ctx: &AtomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#scientific}.
	 * @param ctx the parse tree
	 */
	fn visit_scientific(&mut self, ctx: &ScientificContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#currency}.
	 * @param ctx the parse tree
	 */
	fn visit_currency(&mut self, ctx: &CurrencyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#constant}.
	 * @param ctx the parse tree
	 */
	fn visit_constant(&mut self, ctx: &ConstantContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#func_}.
	 * @param ctx the parse tree
	 */
	fn visit_func_(&mut self, ctx: &Func_Context<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#funcname}.
	 * @param ctx the parse tree
	 */
	fn visit_funcname(&mut self, ctx: &FuncnameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link calculatorParser#relop}.
	 * @param ctx the parse tree
	 */
	fn visit_relop(&mut self, ctx: &RelopContext<'input>) { self.visit_children(ctx) }

}

pub trait calculatorVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= calculatorParserContextType>{
	/**
	 * Visit a parse tree produced by {@link calculatorParser#block}.
	 * @param ctx the parse tree
	 */
		fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#functionDefinition}.
	 * @param ctx the parse tree
	 */
		fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#equation}.
	 * @param ctx the parse tree
	 */
		fn visit_equation(&mut self, ctx: &EquationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#multiplyingExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_multiplyingExpression(&mut self, ctx: &MultiplyingExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#powExpression}.
	 * @param ctx the parse tree
	 */
		fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#signedAtom}.
	 * @param ctx the parse tree
	 */
		fn visit_signedAtom(&mut self, ctx: &SignedAtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#scientific}.
	 * @param ctx the parse tree
	 */
		fn visit_scientific(&mut self, ctx: &ScientificContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#currency}.
	 * @param ctx the parse tree
	 */
		fn visit_currency(&mut self, ctx: &CurrencyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#constant}.
	 * @param ctx the parse tree
	 */
		fn visit_constant(&mut self, ctx: &ConstantContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#func_}.
	 * @param ctx the parse tree
	 */
		fn visit_func_(&mut self, ctx: &Func_Context<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#funcname}.
	 * @param ctx the parse tree
	 */
		fn visit_funcname(&mut self, ctx: &FuncnameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link calculatorParser#relop}.
	 * @param ctx the parse tree
	 */
		fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> calculatorVisitor<'input> for T
where
	T: calculatorVisitorCompat<'input>
{
	fn visit_block(&mut self, ctx: &BlockContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_functionDefinition(&mut self, ctx: &FunctionDefinitionContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_functionDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equation(&mut self, ctx: &EquationContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_equation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_expression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiplyingExpression(&mut self, ctx: &MultiplyingExpressionContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_multiplyingExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_powExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_signedAtom(&mut self, ctx: &SignedAtomContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_signedAtom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atom(&mut self, ctx: &AtomContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_atom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_scientific(&mut self, ctx: &ScientificContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_scientific(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_currency(&mut self, ctx: &CurrencyContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_currency(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_constant(&mut self, ctx: &ConstantContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_constant(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_func_(&mut self, ctx: &Func_Context<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_func_(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_funcname(&mut self, ctx: &FuncnameContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_funcname(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relop(&mut self, ctx: &RelopContext<'input>){
		let result = <Self as calculatorVisitorCompat>::visit_relop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}