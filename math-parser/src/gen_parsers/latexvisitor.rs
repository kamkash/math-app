#![allow(nonstandard_style)]
// Generated from LaTeX.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::latexparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link LaTeXParser}.
 */
pub trait LaTeXVisitor<'input>: ParseTreeVisitor<'input,LaTeXParserContextType>{
	/**
	 * Visit a parse tree produced by {@link LaTeXParser#block}.
	 * @param ctx the parse tree
	 */
	fn visit_block(&mut self, ctx: &BlockContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#math}.
	 * @param ctx the parse tree
	 */
	fn visit_math(&mut self, ctx: &MathContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#relation}.
	 * @param ctx the parse tree
	 */
	fn visit_relation(&mut self, ctx: &RelationContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#equality}.
	 * @param ctx the parse tree
	 */
	fn visit_equality(&mut self, ctx: &EqualityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_expr(&mut self, ctx: &ExprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#sumop}.
	 * @param ctx the parse tree
	 */
	fn visit_sumop(&mut self, ctx: &SumopContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#multop}.
	 * @param ctx the parse tree
	 */
	fn visit_multop(&mut self, ctx: &MultopContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#relop}.
	 * @param ctx the parse tree
	 */
	fn visit_relop(&mut self, ctx: &RelopContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#powop}.
	 * @param ctx the parse tree
	 */
	fn visit_powop(&mut self, ctx: &PowopContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#additive}.
	 * @param ctx the parse tree
	 */
	fn visit_additive(&mut self, ctx: &AdditiveContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#mp}.
	 * @param ctx the parse tree
	 */
	fn visit_mp(&mut self, ctx: &MpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#mp_nofunc}.
	 * @param ctx the parse tree
	 */
	fn visit_mp_nofunc(&mut self, ctx: &Mp_nofuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#unary}.
	 * @param ctx the parse tree
	 */
	fn visit_unary(&mut self, ctx: &UnaryContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#unary_nofunc}.
	 * @param ctx the parse tree
	 */
	fn visit_unary_nofunc(&mut self, ctx: &Unary_nofuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#postfix}.
	 * @param ctx the parse tree
	 */
	fn visit_postfix(&mut self, ctx: &PostfixContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#postfix_nofunc}.
	 * @param ctx the parse tree
	 */
	fn visit_postfix_nofunc(&mut self, ctx: &Postfix_nofuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#postfix_op}.
	 * @param ctx the parse tree
	 */
	fn visit_postfix_op(&mut self, ctx: &Postfix_opContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#eval_at}.
	 * @param ctx the parse tree
	 */
	fn visit_eval_at(&mut self, ctx: &Eval_atContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#eval_at_sub}.
	 * @param ctx the parse tree
	 */
	fn visit_eval_at_sub(&mut self, ctx: &Eval_at_subContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#eval_at_sup}.
	 * @param ctx the parse tree
	 */
	fn visit_eval_at_sup(&mut self, ctx: &Eval_at_supContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#exp}.
	 * @param ctx the parse tree
	 */
	fn visit_exp(&mut self, ctx: &ExpContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#exp_nofunc}.
	 * @param ctx the parse tree
	 */
	fn visit_exp_nofunc(&mut self, ctx: &Exp_nofuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#comp}.
	 * @param ctx the parse tree
	 */
	fn visit_comp(&mut self, ctx: &CompContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#comp_nofunc}.
	 * @param ctx the parse tree
	 */
	fn visit_comp_nofunc(&mut self, ctx: &Comp_nofuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#group}.
	 * @param ctx the parse tree
	 */
	fn visit_group(&mut self, ctx: &GroupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#abs_group}.
	 * @param ctx the parse tree
	 */
	fn visit_abs_group(&mut self, ctx: &Abs_groupContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#number}.
	 * @param ctx the parse tree
	 */
	fn visit_number(&mut self, ctx: &NumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code atomVariable}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atomVariable(&mut self, ctx: &AtomVariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code atomNumber}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atomNumber(&mut self, ctx: &AtomNumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code atomDifferential}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atomDifferential(&mut self, ctx: &AtomDifferentialContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code atomMathit}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atomMathit(&mut self, ctx: &AtomMathitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code atomFrac}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atomFrac(&mut self, ctx: &AtomFracContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code atomBinom}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atomBinom(&mut self, ctx: &AtomBinomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code atomBra}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atomBra(&mut self, ctx: &AtomBraContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code atomKet}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
	fn visit_atomKet(&mut self, ctx: &AtomKetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#bra}.
	 * @param ctx the parse tree
	 */
	fn visit_bra(&mut self, ctx: &BraContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#ket}.
	 * @param ctx the parse tree
	 */
	fn visit_ket(&mut self, ctx: &KetContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#mathit}.
	 * @param ctx the parse tree
	 */
	fn visit_mathit(&mut self, ctx: &MathitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#mathit_text}.
	 * @param ctx the parse tree
	 */
	fn visit_mathit_text(&mut self, ctx: &Mathit_textContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#frac}.
	 * @param ctx the parse tree
	 */
	fn visit_frac(&mut self, ctx: &FracContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#binom}.
	 * @param ctx the parse tree
	 */
	fn visit_binom(&mut self, ctx: &BinomContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#floor}.
	 * @param ctx the parse tree
	 */
	fn visit_floor(&mut self, ctx: &FloorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#ceil}.
	 * @param ctx the parse tree
	 */
	fn visit_ceil(&mut self, ctx: &CeilContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#func_normal}.
	 * @param ctx the parse tree
	 */
	fn visit_func_normal(&mut self, ctx: &Func_normalContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#func}.
	 * @param ctx the parse tree
	 */
	fn visit_func(&mut self, ctx: &FuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#args}.
	 * @param ctx the parse tree
	 */
	fn visit_args(&mut self, ctx: &ArgsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#limit_sub}.
	 * @param ctx the parse tree
	 */
	fn visit_limit_sub(&mut self, ctx: &Limit_subContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#func_arg}.
	 * @param ctx the parse tree
	 */
	fn visit_func_arg(&mut self, ctx: &Func_argContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#func_arg_noparens}.
	 * @param ctx the parse tree
	 */
	fn visit_func_arg_noparens(&mut self, ctx: &Func_arg_noparensContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#subexpr}.
	 * @param ctx the parse tree
	 */
	fn visit_subexpr(&mut self, ctx: &SubexprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#supexpr}.
	 * @param ctx the parse tree
	 */
	fn visit_supexpr(&mut self, ctx: &SupexprContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#subeq}.
	 * @param ctx the parse tree
	 */
	fn visit_subeq(&mut self, ctx: &SubeqContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#supeq}.
	 * @param ctx the parse tree
	 */
	fn visit_supeq(&mut self, ctx: &SupeqContext<'input>) { self.visit_children(ctx) }

}

pub trait LaTeXVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= LaTeXParserContextType>{
	/**
	 * Visit a parse tree produced by {@link LaTeXParser#block}.
	 * @param ctx the parse tree
	 */
		fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#math}.
	 * @param ctx the parse tree
	 */
		fn visit_math(&mut self, ctx: &MathContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#relation}.
	 * @param ctx the parse tree
	 */
		fn visit_relation(&mut self, ctx: &RelationContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#equality}.
	 * @param ctx the parse tree
	 */
		fn visit_equality(&mut self, ctx: &EqualityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_expr(&mut self, ctx: &ExprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#sumop}.
	 * @param ctx the parse tree
	 */
		fn visit_sumop(&mut self, ctx: &SumopContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#multop}.
	 * @param ctx the parse tree
	 */
		fn visit_multop(&mut self, ctx: &MultopContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#relop}.
	 * @param ctx the parse tree
	 */
		fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#powop}.
	 * @param ctx the parse tree
	 */
		fn visit_powop(&mut self, ctx: &PowopContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#additive}.
	 * @param ctx the parse tree
	 */
		fn visit_additive(&mut self, ctx: &AdditiveContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#mp}.
	 * @param ctx the parse tree
	 */
		fn visit_mp(&mut self, ctx: &MpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#mp_nofunc}.
	 * @param ctx the parse tree
	 */
		fn visit_mp_nofunc(&mut self, ctx: &Mp_nofuncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#unary}.
	 * @param ctx the parse tree
	 */
		fn visit_unary(&mut self, ctx: &UnaryContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#unary_nofunc}.
	 * @param ctx the parse tree
	 */
		fn visit_unary_nofunc(&mut self, ctx: &Unary_nofuncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#postfix}.
	 * @param ctx the parse tree
	 */
		fn visit_postfix(&mut self, ctx: &PostfixContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#postfix_nofunc}.
	 * @param ctx the parse tree
	 */
		fn visit_postfix_nofunc(&mut self, ctx: &Postfix_nofuncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#postfix_op}.
	 * @param ctx the parse tree
	 */
		fn visit_postfix_op(&mut self, ctx: &Postfix_opContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#eval_at}.
	 * @param ctx the parse tree
	 */
		fn visit_eval_at(&mut self, ctx: &Eval_atContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#eval_at_sub}.
	 * @param ctx the parse tree
	 */
		fn visit_eval_at_sub(&mut self, ctx: &Eval_at_subContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#eval_at_sup}.
	 * @param ctx the parse tree
	 */
		fn visit_eval_at_sup(&mut self, ctx: &Eval_at_supContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#exp}.
	 * @param ctx the parse tree
	 */
		fn visit_exp(&mut self, ctx: &ExpContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#exp_nofunc}.
	 * @param ctx the parse tree
	 */
		fn visit_exp_nofunc(&mut self, ctx: &Exp_nofuncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#comp}.
	 * @param ctx the parse tree
	 */
		fn visit_comp(&mut self, ctx: &CompContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#comp_nofunc}.
	 * @param ctx the parse tree
	 */
		fn visit_comp_nofunc(&mut self, ctx: &Comp_nofuncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#group}.
	 * @param ctx the parse tree
	 */
		fn visit_group(&mut self, ctx: &GroupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#abs_group}.
	 * @param ctx the parse tree
	 */
		fn visit_abs_group(&mut self, ctx: &Abs_groupContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#number}.
	 * @param ctx the parse tree
	 */
		fn visit_number(&mut self, ctx: &NumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code atomVariable}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atomVariable(&mut self, ctx: &AtomVariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code atomNumber}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atomNumber(&mut self, ctx: &AtomNumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code atomDifferential}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atomDifferential(&mut self, ctx: &AtomDifferentialContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code atomMathit}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atomMathit(&mut self, ctx: &AtomMathitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code atomFrac}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atomFrac(&mut self, ctx: &AtomFracContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code atomBinom}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atomBinom(&mut self, ctx: &AtomBinomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code atomBra}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atomBra(&mut self, ctx: &AtomBraContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code atomKet}
	 * labeled alternative in {@link LaTeXParser#atom}.
	 * @param ctx the parse tree
	 */
		fn visit_atomKet(&mut self, ctx: &AtomKetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#bra}.
	 * @param ctx the parse tree
	 */
		fn visit_bra(&mut self, ctx: &BraContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#ket}.
	 * @param ctx the parse tree
	 */
		fn visit_ket(&mut self, ctx: &KetContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#mathit}.
	 * @param ctx the parse tree
	 */
		fn visit_mathit(&mut self, ctx: &MathitContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#mathit_text}.
	 * @param ctx the parse tree
	 */
		fn visit_mathit_text(&mut self, ctx: &Mathit_textContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#frac}.
	 * @param ctx the parse tree
	 */
		fn visit_frac(&mut self, ctx: &FracContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#binom}.
	 * @param ctx the parse tree
	 */
		fn visit_binom(&mut self, ctx: &BinomContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#floor}.
	 * @param ctx the parse tree
	 */
		fn visit_floor(&mut self, ctx: &FloorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#ceil}.
	 * @param ctx the parse tree
	 */
		fn visit_ceil(&mut self, ctx: &CeilContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#func_normal}.
	 * @param ctx the parse tree
	 */
		fn visit_func_normal(&mut self, ctx: &Func_normalContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#func}.
	 * @param ctx the parse tree
	 */
		fn visit_func(&mut self, ctx: &FuncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#args}.
	 * @param ctx the parse tree
	 */
		fn visit_args(&mut self, ctx: &ArgsContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#limit_sub}.
	 * @param ctx the parse tree
	 */
		fn visit_limit_sub(&mut self, ctx: &Limit_subContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#func_arg}.
	 * @param ctx the parse tree
	 */
		fn visit_func_arg(&mut self, ctx: &Func_argContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#func_arg_noparens}.
	 * @param ctx the parse tree
	 */
		fn visit_func_arg_noparens(&mut self, ctx: &Func_arg_noparensContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#subexpr}.
	 * @param ctx the parse tree
	 */
		fn visit_subexpr(&mut self, ctx: &SubexprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#supexpr}.
	 * @param ctx the parse tree
	 */
		fn visit_supexpr(&mut self, ctx: &SupexprContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#subeq}.
	 * @param ctx the parse tree
	 */
		fn visit_subeq(&mut self, ctx: &SubeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link LaTeXParser#supeq}.
	 * @param ctx the parse tree
	 */
		fn visit_supeq(&mut self, ctx: &SupeqContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> LaTeXVisitor<'input> for T
where
	T: LaTeXVisitorCompat<'input>
{
	fn visit_block(&mut self, ctx: &BlockContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_block(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_math(&mut self, ctx: &MathContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_math(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relation(&mut self, ctx: &RelationContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_relation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_equality(&mut self, ctx: &EqualityContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_equality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expr(&mut self, ctx: &ExprContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_expr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_sumop(&mut self, ctx: &SumopContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_sumop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multop(&mut self, ctx: &MultopContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_multop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_relop(&mut self, ctx: &RelopContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_relop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_powop(&mut self, ctx: &PowopContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_powop(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_additive(&mut self, ctx: &AdditiveContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_additive(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mp(&mut self, ctx: &MpContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_mp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mp_nofunc(&mut self, ctx: &Mp_nofuncContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_mp_nofunc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unary(&mut self, ctx: &UnaryContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_unary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_unary_nofunc(&mut self, ctx: &Unary_nofuncContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_unary_nofunc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postfix(&mut self, ctx: &PostfixContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_postfix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postfix_nofunc(&mut self, ctx: &Postfix_nofuncContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_postfix_nofunc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_postfix_op(&mut self, ctx: &Postfix_opContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_postfix_op(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eval_at(&mut self, ctx: &Eval_atContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_eval_at(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eval_at_sub(&mut self, ctx: &Eval_at_subContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_eval_at_sub(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_eval_at_sup(&mut self, ctx: &Eval_at_supContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_eval_at_sup(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exp(&mut self, ctx: &ExpContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_exp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_exp_nofunc(&mut self, ctx: &Exp_nofuncContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_exp_nofunc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comp(&mut self, ctx: &CompContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_comp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_comp_nofunc(&mut self, ctx: &Comp_nofuncContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_comp_nofunc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_group(&mut self, ctx: &GroupContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_group(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_abs_group(&mut self, ctx: &Abs_groupContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_abs_group(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_number(&mut self, ctx: &NumberContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomVariable(&mut self, ctx: &AtomVariableContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_atomVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomNumber(&mut self, ctx: &AtomNumberContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_atomNumber(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomDifferential(&mut self, ctx: &AtomDifferentialContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_atomDifferential(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomMathit(&mut self, ctx: &AtomMathitContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_atomMathit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomFrac(&mut self, ctx: &AtomFracContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_atomFrac(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomBinom(&mut self, ctx: &AtomBinomContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_atomBinom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomBra(&mut self, ctx: &AtomBraContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_atomBra(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_atomKet(&mut self, ctx: &AtomKetContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_atomKet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_bra(&mut self, ctx: &BraContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_bra(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ket(&mut self, ctx: &KetContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_ket(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mathit(&mut self, ctx: &MathitContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_mathit(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_mathit_text(&mut self, ctx: &Mathit_textContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_mathit_text(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_frac(&mut self, ctx: &FracContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_frac(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_binom(&mut self, ctx: &BinomContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_binom(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_floor(&mut self, ctx: &FloorContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_floor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_ceil(&mut self, ctx: &CeilContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_ceil(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_func_normal(&mut self, ctx: &Func_normalContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_func_normal(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_func(&mut self, ctx: &FuncContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_func(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_args(&mut self, ctx: &ArgsContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_args(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_limit_sub(&mut self, ctx: &Limit_subContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_limit_sub(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_func_arg(&mut self, ctx: &Func_argContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_func_arg(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_func_arg_noparens(&mut self, ctx: &Func_arg_noparensContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_func_arg_noparens(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subexpr(&mut self, ctx: &SubexprContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_subexpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_supexpr(&mut self, ctx: &SupexprContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_supexpr(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_subeq(&mut self, ctx: &SubeqContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_subeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_supeq(&mut self, ctx: &SupeqContext<'input>){
		let result = <Self as LaTeXVisitorCompat>::visit_supeq(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}