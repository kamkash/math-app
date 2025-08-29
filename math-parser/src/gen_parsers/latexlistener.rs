#![allow(nonstandard_style)]
// Generated from LaTeX.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::latexparser::*;

pub trait LaTeXListener<'input> : ParseTreeListener<'input,LaTeXParserContextType>{
/**
 * Enter a parse tree produced by {@link LaTeXParser#block}.
 * @param ctx the parse tree
 */
fn enter_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#block}.
 * @param ctx the parse tree
 */
fn exit_block(&mut self, _ctx: &BlockContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#math}.
 * @param ctx the parse tree
 */
fn enter_math(&mut self, _ctx: &MathContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#math}.
 * @param ctx the parse tree
 */
fn exit_math(&mut self, _ctx: &MathContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#relation}.
 * @param ctx the parse tree
 */
fn enter_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#relation}.
 * @param ctx the parse tree
 */
fn exit_relation(&mut self, _ctx: &RelationContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#equality}.
 * @param ctx the parse tree
 */
fn enter_equality(&mut self, _ctx: &EqualityContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#equality}.
 * @param ctx the parse tree
 */
fn exit_equality(&mut self, _ctx: &EqualityContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#sumop}.
 * @param ctx the parse tree
 */
fn enter_sumop(&mut self, _ctx: &SumopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#sumop}.
 * @param ctx the parse tree
 */
fn exit_sumop(&mut self, _ctx: &SumopContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#multop}.
 * @param ctx the parse tree
 */
fn enter_multop(&mut self, _ctx: &MultopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#multop}.
 * @param ctx the parse tree
 */
fn exit_multop(&mut self, _ctx: &MultopContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#relop}.
 * @param ctx the parse tree
 */
fn enter_relop(&mut self, _ctx: &RelopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#relop}.
 * @param ctx the parse tree
 */
fn exit_relop(&mut self, _ctx: &RelopContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#powop}.
 * @param ctx the parse tree
 */
fn enter_powop(&mut self, _ctx: &PowopContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#powop}.
 * @param ctx the parse tree
 */
fn exit_powop(&mut self, _ctx: &PowopContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#additive}.
 * @param ctx the parse tree
 */
fn enter_additive(&mut self, _ctx: &AdditiveContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#additive}.
 * @param ctx the parse tree
 */
fn exit_additive(&mut self, _ctx: &AdditiveContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#mp}.
 * @param ctx the parse tree
 */
fn enter_mp(&mut self, _ctx: &MpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#mp}.
 * @param ctx the parse tree
 */
fn exit_mp(&mut self, _ctx: &MpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#mp_nofunc}.
 * @param ctx the parse tree
 */
fn enter_mp_nofunc(&mut self, _ctx: &Mp_nofuncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#mp_nofunc}.
 * @param ctx the parse tree
 */
fn exit_mp_nofunc(&mut self, _ctx: &Mp_nofuncContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#unary}.
 * @param ctx the parse tree
 */
fn enter_unary(&mut self, _ctx: &UnaryContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#unary}.
 * @param ctx the parse tree
 */
fn exit_unary(&mut self, _ctx: &UnaryContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#unary_nofunc}.
 * @param ctx the parse tree
 */
fn enter_unary_nofunc(&mut self, _ctx: &Unary_nofuncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#unary_nofunc}.
 * @param ctx the parse tree
 */
fn exit_unary_nofunc(&mut self, _ctx: &Unary_nofuncContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#postfix}.
 * @param ctx the parse tree
 */
fn enter_postfix(&mut self, _ctx: &PostfixContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#postfix}.
 * @param ctx the parse tree
 */
fn exit_postfix(&mut self, _ctx: &PostfixContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#postfix_nofunc}.
 * @param ctx the parse tree
 */
fn enter_postfix_nofunc(&mut self, _ctx: &Postfix_nofuncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#postfix_nofunc}.
 * @param ctx the parse tree
 */
fn exit_postfix_nofunc(&mut self, _ctx: &Postfix_nofuncContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#postfix_op}.
 * @param ctx the parse tree
 */
fn enter_postfix_op(&mut self, _ctx: &Postfix_opContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#postfix_op}.
 * @param ctx the parse tree
 */
fn exit_postfix_op(&mut self, _ctx: &Postfix_opContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#eval_at}.
 * @param ctx the parse tree
 */
fn enter_eval_at(&mut self, _ctx: &Eval_atContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#eval_at}.
 * @param ctx the parse tree
 */
fn exit_eval_at(&mut self, _ctx: &Eval_atContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#eval_at_sub}.
 * @param ctx the parse tree
 */
fn enter_eval_at_sub(&mut self, _ctx: &Eval_at_subContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#eval_at_sub}.
 * @param ctx the parse tree
 */
fn exit_eval_at_sub(&mut self, _ctx: &Eval_at_subContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#eval_at_sup}.
 * @param ctx the parse tree
 */
fn enter_eval_at_sup(&mut self, _ctx: &Eval_at_supContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#eval_at_sup}.
 * @param ctx the parse tree
 */
fn exit_eval_at_sup(&mut self, _ctx: &Eval_at_supContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#exp}.
 * @param ctx the parse tree
 */
fn enter_exp(&mut self, _ctx: &ExpContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#exp}.
 * @param ctx the parse tree
 */
fn exit_exp(&mut self, _ctx: &ExpContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#exp_nofunc}.
 * @param ctx the parse tree
 */
fn enter_exp_nofunc(&mut self, _ctx: &Exp_nofuncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#exp_nofunc}.
 * @param ctx the parse tree
 */
fn exit_exp_nofunc(&mut self, _ctx: &Exp_nofuncContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#comp}.
 * @param ctx the parse tree
 */
fn enter_comp(&mut self, _ctx: &CompContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#comp}.
 * @param ctx the parse tree
 */
fn exit_comp(&mut self, _ctx: &CompContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#comp_nofunc}.
 * @param ctx the parse tree
 */
fn enter_comp_nofunc(&mut self, _ctx: &Comp_nofuncContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#comp_nofunc}.
 * @param ctx the parse tree
 */
fn exit_comp_nofunc(&mut self, _ctx: &Comp_nofuncContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#group}.
 * @param ctx the parse tree
 */
fn enter_group(&mut self, _ctx: &GroupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#group}.
 * @param ctx the parse tree
 */
fn exit_group(&mut self, _ctx: &GroupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#abs_group}.
 * @param ctx the parse tree
 */
fn enter_abs_group(&mut self, _ctx: &Abs_groupContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#abs_group}.
 * @param ctx the parse tree
 */
fn exit_abs_group(&mut self, _ctx: &Abs_groupContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#number}.
 * @param ctx the parse tree
 */
fn enter_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#number}.
 * @param ctx the parse tree
 */
fn exit_number(&mut self, _ctx: &NumberContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code atomVariable}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atomVariable(&mut self, _ctx: &AtomVariableContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code atomVariable}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atomVariable(&mut self, _ctx: &AtomVariableContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code atomNumber}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atomNumber(&mut self, _ctx: &AtomNumberContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code atomNumber}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atomNumber(&mut self, _ctx: &AtomNumberContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code atomDifferential}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atomDifferential(&mut self, _ctx: &AtomDifferentialContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code atomDifferential}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atomDifferential(&mut self, _ctx: &AtomDifferentialContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code atomMathit}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atomMathit(&mut self, _ctx: &AtomMathitContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code atomMathit}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atomMathit(&mut self, _ctx: &AtomMathitContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code atomFrac}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atomFrac(&mut self, _ctx: &AtomFracContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code atomFrac}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atomFrac(&mut self, _ctx: &AtomFracContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code atomBinom}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atomBinom(&mut self, _ctx: &AtomBinomContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code atomBinom}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atomBinom(&mut self, _ctx: &AtomBinomContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code atomBra}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atomBra(&mut self, _ctx: &AtomBraContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code atomBra}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atomBra(&mut self, _ctx: &AtomBraContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code atomKet}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn enter_atomKet(&mut self, _ctx: &AtomKetContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code atomKet}
 * labeled alternative in {@link LaTeXParser#atom}.
 * @param ctx the parse tree
 */
fn exit_atomKet(&mut self, _ctx: &AtomKetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#bra}.
 * @param ctx the parse tree
 */
fn enter_bra(&mut self, _ctx: &BraContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#bra}.
 * @param ctx the parse tree
 */
fn exit_bra(&mut self, _ctx: &BraContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#ket}.
 * @param ctx the parse tree
 */
fn enter_ket(&mut self, _ctx: &KetContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#ket}.
 * @param ctx the parse tree
 */
fn exit_ket(&mut self, _ctx: &KetContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#mathit}.
 * @param ctx the parse tree
 */
fn enter_mathit(&mut self, _ctx: &MathitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#mathit}.
 * @param ctx the parse tree
 */
fn exit_mathit(&mut self, _ctx: &MathitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#mathit_text}.
 * @param ctx the parse tree
 */
fn enter_mathit_text(&mut self, _ctx: &Mathit_textContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#mathit_text}.
 * @param ctx the parse tree
 */
fn exit_mathit_text(&mut self, _ctx: &Mathit_textContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#frac}.
 * @param ctx the parse tree
 */
fn enter_frac(&mut self, _ctx: &FracContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#frac}.
 * @param ctx the parse tree
 */
fn exit_frac(&mut self, _ctx: &FracContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#binom}.
 * @param ctx the parse tree
 */
fn enter_binom(&mut self, _ctx: &BinomContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#binom}.
 * @param ctx the parse tree
 */
fn exit_binom(&mut self, _ctx: &BinomContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#floor}.
 * @param ctx the parse tree
 */
fn enter_floor(&mut self, _ctx: &FloorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#floor}.
 * @param ctx the parse tree
 */
fn exit_floor(&mut self, _ctx: &FloorContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#ceil}.
 * @param ctx the parse tree
 */
fn enter_ceil(&mut self, _ctx: &CeilContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#ceil}.
 * @param ctx the parse tree
 */
fn exit_ceil(&mut self, _ctx: &CeilContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#func_normal}.
 * @param ctx the parse tree
 */
fn enter_func_normal(&mut self, _ctx: &Func_normalContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#func_normal}.
 * @param ctx the parse tree
 */
fn exit_func_normal(&mut self, _ctx: &Func_normalContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fn_normal}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn enter_fn_normal(&mut self, _ctx: &Fn_normalContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fn_normal}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn exit_fn_normal(&mut self, _ctx: &Fn_normalContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fn_symbol}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn enter_fn_symbol(&mut self, _ctx: &Fn_symbolContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fn_symbol}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn exit_fn_symbol(&mut self, _ctx: &Fn_symbolContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fn_int}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn enter_fn_int(&mut self, _ctx: &Fn_intContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fn_int}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn exit_fn_int(&mut self, _ctx: &Fn_intContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fn_sqrt}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn enter_fn_sqrt(&mut self, _ctx: &Fn_sqrtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fn_sqrt}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn exit_fn_sqrt(&mut self, _ctx: &Fn_sqrtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fn_overline}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn enter_fn_overline(&mut self, _ctx: &Fn_overlineContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fn_overline}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn exit_fn_overline(&mut self, _ctx: &Fn_overlineContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fn_sum}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn enter_fn_sum(&mut self, _ctx: &Fn_sumContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fn_sum}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn exit_fn_sum(&mut self, _ctx: &Fn_sumContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code fn_limit}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn enter_fn_limit(&mut self, _ctx: &Fn_limitContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code fn_limit}
 * labeled alternative in {@link LaTeXParser#func}.
 * @param ctx the parse tree
 */
fn exit_fn_limit(&mut self, _ctx: &Fn_limitContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#args}.
 * @param ctx the parse tree
 */
fn enter_args(&mut self, _ctx: &ArgsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#args}.
 * @param ctx the parse tree
 */
fn exit_args(&mut self, _ctx: &ArgsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#limit_sub}.
 * @param ctx the parse tree
 */
fn enter_limit_sub(&mut self, _ctx: &Limit_subContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#limit_sub}.
 * @param ctx the parse tree
 */
fn exit_limit_sub(&mut self, _ctx: &Limit_subContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#func_arg}.
 * @param ctx the parse tree
 */
fn enter_func_arg(&mut self, _ctx: &Func_argContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#func_arg}.
 * @param ctx the parse tree
 */
fn exit_func_arg(&mut self, _ctx: &Func_argContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#func_arg_noparens}.
 * @param ctx the parse tree
 */
fn enter_func_arg_noparens(&mut self, _ctx: &Func_arg_noparensContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#func_arg_noparens}.
 * @param ctx the parse tree
 */
fn exit_func_arg_noparens(&mut self, _ctx: &Func_arg_noparensContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#subexpr}.
 * @param ctx the parse tree
 */
fn enter_subexpr(&mut self, _ctx: &SubexprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#subexpr}.
 * @param ctx the parse tree
 */
fn exit_subexpr(&mut self, _ctx: &SubexprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#supexpr}.
 * @param ctx the parse tree
 */
fn enter_supexpr(&mut self, _ctx: &SupexprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#supexpr}.
 * @param ctx the parse tree
 */
fn exit_supexpr(&mut self, _ctx: &SupexprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#subeq}.
 * @param ctx the parse tree
 */
fn enter_subeq(&mut self, _ctx: &SubeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#subeq}.
 * @param ctx the parse tree
 */
fn exit_subeq(&mut self, _ctx: &SubeqContext<'input>) { }
/**
 * Enter a parse tree produced by {@link LaTeXParser#supeq}.
 * @param ctx the parse tree
 */
fn enter_supeq(&mut self, _ctx: &SupeqContext<'input>) { }
/**
 * Exit a parse tree produced by {@link LaTeXParser#supeq}.
 * @param ctx the parse tree
 */
fn exit_supeq(&mut self, _ctx: &SupeqContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : LaTeXListener<'input> }


