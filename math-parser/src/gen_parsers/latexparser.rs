// Generated from LaTeX.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::latexlistener::*;
use super::latexvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const THINSPACE:isize=3; 
		pub const MEDSPACE:isize=4; 
		pub const THICKSPACE:isize=5; 
		pub const QUAD:isize=6; 
		pub const QQUAD:isize=7; 
		pub const NEGTHINSPACE:isize=8; 
		pub const NEGMEDSPACE:isize=9; 
		pub const NEGTHICKSPACE:isize=10; 
		pub const CMD_LEFT:isize=11; 
		pub const CMD_RIGHT:isize=12; 
		pub const IGNORE:isize=13; 
		pub const ADD:isize=14; 
		pub const SUB:isize=15; 
		pub const MUL:isize=16; 
		pub const DIV:isize=17; 
		pub const L_PAREN:isize=18; 
		pub const R_PAREN:isize=19; 
		pub const L_BRACE:isize=20; 
		pub const R_BRACE:isize=21; 
		pub const L_BRACE_LITERAL:isize=22; 
		pub const R_BRACE_LITERAL:isize=23; 
		pub const L_BRACKET:isize=24; 
		pub const R_BRACKET:isize=25; 
		pub const BAR:isize=26; 
		pub const R_BAR:isize=27; 
		pub const L_BAR:isize=28; 
		pub const L_ANGLE:isize=29; 
		pub const R_ANGLE:isize=30; 
		pub const FUNC_LIM:isize=31; 
		pub const LIM_APPROACH_SYM:isize=32; 
		pub const FUNC_INT:isize=33; 
		pub const FUNC_SUM:isize=34; 
		pub const FUNC_PROD:isize=35; 
		pub const FUNC_EXP:isize=36; 
		pub const FUNC_LOG:isize=37; 
		pub const FUNC_LG:isize=38; 
		pub const FUNC_LN:isize=39; 
		pub const FUNC_SIN:isize=40; 
		pub const FUNC_COS:isize=41; 
		pub const FUNC_TAN:isize=42; 
		pub const FUNC_CSC:isize=43; 
		pub const FUNC_SEC:isize=44; 
		pub const FUNC_COT:isize=45; 
		pub const FUNC_ARCSIN:isize=46; 
		pub const FUNC_ARCCOS:isize=47; 
		pub const FUNC_ARCTAN:isize=48; 
		pub const FUNC_ARCCSC:isize=49; 
		pub const FUNC_ARCSEC:isize=50; 
		pub const FUNC_ARCCOT:isize=51; 
		pub const FUNC_SINH:isize=52; 
		pub const FUNC_COSH:isize=53; 
		pub const FUNC_TANH:isize=54; 
		pub const FUNC_ARSINH:isize=55; 
		pub const FUNC_ARCOSH:isize=56; 
		pub const FUNC_ARTANH:isize=57; 
		pub const L_FLOOR:isize=58; 
		pub const R_FLOOR:isize=59; 
		pub const L_CEIL:isize=60; 
		pub const R_CEIL:isize=61; 
		pub const FUNC_SQRT:isize=62; 
		pub const FUNC_OVERLINE:isize=63; 
		pub const CMD_TIMES:isize=64; 
		pub const CMD_CDOT:isize=65; 
		pub const CMD_DIV:isize=66; 
		pub const CMD_FRAC:isize=67; 
		pub const CMD_BINOM:isize=68; 
		pub const CMD_DBINOM:isize=69; 
		pub const CMD_TBINOM:isize=70; 
		pub const CMD_MATHIT:isize=71; 
		pub const UNDERSCORE:isize=72; 
		pub const CARET:isize=73; 
		pub const COLON:isize=74; 
		pub const DIFFERENTIAL:isize=75; 
		pub const DIGIT:isize=76; 
		pub const VAR:isize=77; 
		pub const EQUAL:isize=78; 
		pub const NEQ:isize=79; 
		pub const LT:isize=80; 
		pub const LTE:isize=81; 
		pub const LTE_Q:isize=82; 
		pub const LTE_S:isize=83; 
		pub const LATEX_BLOCK:isize=84; 
		pub const GT:isize=85; 
		pub const GTE:isize=86; 
		pub const GTE_Q:isize=87; 
		pub const GTE_S:isize=88; 
		pub const WS:isize=89; 
		pub const BANG:isize=90; 
		pub const LATEX_NEWLINE:isize=91; 
		pub const SINGLE_QUOTES:isize=92; 
		pub const SYMBOL:isize=93; 
		pub const SEPARATOR:isize=94;
	pub const RULE_block:usize = 0; 
	pub const RULE_math:usize = 1; 
	pub const RULE_relation:usize = 2; 
	pub const RULE_equality:usize = 3; 
	pub const RULE_expr:usize = 4; 
	pub const RULE_sumop:usize = 5; 
	pub const RULE_multop:usize = 6; 
	pub const RULE_relop:usize = 7; 
	pub const RULE_powop:usize = 8; 
	pub const RULE_additive:usize = 9; 
	pub const RULE_mp:usize = 10; 
	pub const RULE_mp_nofunc:usize = 11; 
	pub const RULE_unary:usize = 12; 
	pub const RULE_unary_nofunc:usize = 13; 
	pub const RULE_postfix:usize = 14; 
	pub const RULE_postfix_nofunc:usize = 15; 
	pub const RULE_postfix_op:usize = 16; 
	pub const RULE_eval_at:usize = 17; 
	pub const RULE_eval_at_sub:usize = 18; 
	pub const RULE_eval_at_sup:usize = 19; 
	pub const RULE_exp:usize = 20; 
	pub const RULE_exp_nofunc:usize = 21; 
	pub const RULE_comp:usize = 22; 
	pub const RULE_comp_nofunc:usize = 23; 
	pub const RULE_group:usize = 24; 
	pub const RULE_abs_group:usize = 25; 
	pub const RULE_number:usize = 26; 
	pub const RULE_atom:usize = 27; 
	pub const RULE_bra:usize = 28; 
	pub const RULE_ket:usize = 29; 
	pub const RULE_mathit:usize = 30; 
	pub const RULE_mathit_text:usize = 31; 
	pub const RULE_frac:usize = 32; 
	pub const RULE_binom:usize = 33; 
	pub const RULE_floor:usize = 34; 
	pub const RULE_ceil:usize = 35; 
	pub const RULE_var_sym:usize = 36; 
	pub const RULE_func_normal:usize = 37; 
	pub const RULE_func:usize = 38; 
	pub const RULE_args:usize = 39; 
	pub const RULE_limit_sub:usize = 40; 
	pub const RULE_func_arg:usize = 41; 
	pub const RULE_func_arg_noparens:usize = 42; 
	pub const RULE_subexpr:usize = 43; 
	pub const RULE_supexpr:usize = 44; 
	pub const RULE_subeq:usize = 45; 
	pub const RULE_supeq:usize = 46;
	pub const ruleNames: [&'static str; 47] =  [
		"block", "math", "relation", "equality", "expr", "sumop", "multop", "relop", 
		"powop", "additive", "mp", "mp_nofunc", "unary", "unary_nofunc", "postfix", 
		"postfix_nofunc", "postfix_op", "eval_at", "eval_at_sub", "eval_at_sup", 
		"exp", "exp_nofunc", "comp", "comp_nofunc", "group", "abs_group", "number", 
		"atom", "bra", "ket", "mathit", "mathit_text", "frac", "binom", "floor", 
		"ceil", "var_sym", "func_normal", "func", "args", "limit_sub", "func_arg", 
		"func_arg_noparens", "subexpr", "supexpr", "subeq", "supeq"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;92] = [
		None, Some("','"), Some("'.'"), None, None, None, Some("'\\quad'"), Some("'\\qquad'"), 
		None, Some("'\\negmedspace'"), Some("'\\negthickspace'"), Some("'\\left'"), 
		Some("'\\right'"), None, Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), 
		Some("'('"), Some("')'"), Some("'{'"), Some("'}'"), Some("'\\{'"), Some("'\\}'"), 
		Some("'['"), Some("']'"), Some("'|'"), Some("'\\right|'"), Some("'\\left|'"), 
		Some("'\\langle'"), Some("'\\rangle'"), Some("'\\lim'"), None, None, Some("'\\sum'"), 
		Some("'\\prod'"), Some("'\\exp'"), Some("'\\log'"), Some("'\\lg'"), Some("'\\ln'"), 
		Some("'\\sin'"), Some("'\\cos'"), Some("'\\tan'"), Some("'\\csc'"), Some("'\\sec'"), 
		Some("'\\cot'"), Some("'\\arcsin'"), Some("'\\arccos'"), Some("'\\arctan'"), 
		Some("'\\arccsc'"), Some("'\\arcsec'"), Some("'\\arccot'"), Some("'\\sinh'"), 
		Some("'\\cosh'"), Some("'\\tanh'"), Some("'\\arsinh'"), Some("'\\arcosh'"), 
		Some("'\\artanh'"), Some("'\\lfloor'"), Some("'\\rfloor'"), Some("'\\lceil'"), 
		Some("'\\rceil'"), Some("'\\sqrt'"), Some("'\\overline'"), Some("'\\times'"), 
		Some("'\\cdot'"), Some("'\\div'"), None, Some("'\\binom'"), Some("'\\dbinom'"), 
		Some("'\\tbinom'"), Some("'\\mathit'"), Some("'_'"), Some("'^'"), Some("':'"), 
		None, None, None, None, Some("'\\neq'"), Some("'<'"), None, Some("'\\leqq'"), 
		Some("'\\leqslant'"), Some("'\\displaylines'"), Some("'>'"), None, Some("'\\geqq'"), 
		Some("'\\geqslant'"), None, Some("'!'"), Some("'\\\\'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;95]  = [
		None, None, None, Some("THINSPACE"), Some("MEDSPACE"), Some("THICKSPACE"), 
		Some("QUAD"), Some("QQUAD"), Some("NEGTHINSPACE"), Some("NEGMEDSPACE"), 
		Some("NEGTHICKSPACE"), Some("CMD_LEFT"), Some("CMD_RIGHT"), Some("IGNORE"), 
		Some("ADD"), Some("SUB"), Some("MUL"), Some("DIV"), Some("L_PAREN"), Some("R_PAREN"), 
		Some("L_BRACE"), Some("R_BRACE"), Some("L_BRACE_LITERAL"), Some("R_BRACE_LITERAL"), 
		Some("L_BRACKET"), Some("R_BRACKET"), Some("BAR"), Some("R_BAR"), Some("L_BAR"), 
		Some("L_ANGLE"), Some("R_ANGLE"), Some("FUNC_LIM"), Some("LIM_APPROACH_SYM"), 
		Some("FUNC_INT"), Some("FUNC_SUM"), Some("FUNC_PROD"), Some("FUNC_EXP"), 
		Some("FUNC_LOG"), Some("FUNC_LG"), Some("FUNC_LN"), Some("FUNC_SIN"), 
		Some("FUNC_COS"), Some("FUNC_TAN"), Some("FUNC_CSC"), Some("FUNC_SEC"), 
		Some("FUNC_COT"), Some("FUNC_ARCSIN"), Some("FUNC_ARCCOS"), Some("FUNC_ARCTAN"), 
		Some("FUNC_ARCCSC"), Some("FUNC_ARCSEC"), Some("FUNC_ARCCOT"), Some("FUNC_SINH"), 
		Some("FUNC_COSH"), Some("FUNC_TANH"), Some("FUNC_ARSINH"), Some("FUNC_ARCOSH"), 
		Some("FUNC_ARTANH"), Some("L_FLOOR"), Some("R_FLOOR"), Some("L_CEIL"), 
		Some("R_CEIL"), Some("FUNC_SQRT"), Some("FUNC_OVERLINE"), Some("CMD_TIMES"), 
		Some("CMD_CDOT"), Some("CMD_DIV"), Some("CMD_FRAC"), Some("CMD_BINOM"), 
		Some("CMD_DBINOM"), Some("CMD_TBINOM"), Some("CMD_MATHIT"), Some("UNDERSCORE"), 
		Some("CARET"), Some("COLON"), Some("DIFFERENTIAL"), Some("DIGIT"), Some("VAR"), 
		Some("EQUAL"), Some("NEQ"), Some("LT"), Some("LTE"), Some("LTE_Q"), Some("LTE_S"), 
		Some("LATEX_BLOCK"), Some("GT"), Some("GTE"), Some("GTE_Q"), Some("GTE_S"), 
		Some("WS"), Some("BANG"), Some("LATEX_NEWLINE"), Some("SINGLE_QUOTES"), 
		Some("SYMBOL"), Some("SEPARATOR")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,LaTeXParserExt<'input>, I, LaTeXParserContextType , dyn LaTeXListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type LaTeXTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, LaTeXParserContextType , dyn LaTeXListener<'input> + 'a>;

/// Parser for LaTeX grammar
pub struct LaTeXParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				LaTeXParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> LaTeXParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> LaTeXParser<'input, I, DefaultErrorStrategy<'input,LaTeXParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for LaTeXParser
pub trait LaTeXParserContext<'input>:
	for<'x> Listenable<dyn LaTeXListener<'input> + 'x > + 
	for<'x> Visitable<dyn LaTeXVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=LaTeXParserContextType>
{}

antlr_rust::coerce_from!{ 'input : LaTeXParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn LaTeXParserContext<'input> + 'input
where
    T: LaTeXVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn LaTeXVisitor<'input> + 'x))
    }
}

impl<'input> LaTeXParserContext<'input> for TerminalNode<'input,LaTeXParserContextType> {}
impl<'input> LaTeXParserContext<'input> for ErrorNode<'input,LaTeXParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn LaTeXParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn LaTeXListener<'input> + 'input }

pub struct LaTeXParserContextType;
antlr_rust::tid!{LaTeXParserContextType}

impl<'input> ParserNodeType<'input> for LaTeXParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn LaTeXParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct LaTeXParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> LaTeXParserExt<'input>{
}
antlr_rust::tid! { LaTeXParserExt<'a> }

impl<'input> TokenAware<'input> for LaTeXParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for LaTeXParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for LaTeXParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "LaTeX.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn LaTeXParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					2 => LaTeXParser::<'input,I,_>::relation_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					9 => LaTeXParser::<'input,I,_>::additive_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					10 => LaTeXParser::<'input,I,_>::mp_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					11 => LaTeXParser::<'input,I,_>::mp_nofunc_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					20 => LaTeXParser::<'input,I,_>::exp_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					21 => LaTeXParser::<'input,I,_>::exp_nofunc_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> LaTeXParser<'input, I, DefaultErrorStrategy<'input,LaTeXParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn relation_sempred(_localctx: Option<&RelationContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn additive_sempred(_localctx: Option<&AdditiveContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				1=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn mp_sempred(_localctx: Option<&MpContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				2=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
	fn mp_nofunc_sempred(_localctx: Option<&Mp_nofuncContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				3=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn exp_sempred(_localctx: Option<&ExpContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				4=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
	fn exp_nofunc_sempred(_localctx: Option<&Exp_nofuncContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				5=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;


pub type BlockContext<'input> = BaseParserRuleContext<'input,BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for BlockContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for BlockContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_block(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_block(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for BlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_block(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid!{BlockContextExt<'a>}

impl<'input> BlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<BlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn relation_all(&self) ->  Vec<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relation(&self, i: usize) -> Option<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SEPARATOR in current rule
fn SEPARATOR_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEPARATOR, starting from 0.
/// Returns `None` if number of children corresponding to token SEPARATOR is less or equal than `i`.
fn SEPARATOR(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(SEPARATOR, i)
}
/// Retrieves first TerminalNode corresponding to token LATEX_BLOCK
/// Returns `None` if there is no child corresponding to token LATEX_BLOCK
fn LATEX_BLOCK(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(LATEX_BLOCK, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token LATEX_NEWLINE in current rule
fn LATEX_NEWLINE_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LATEX_NEWLINE, starting from 0.
/// Returns `None` if number of children corresponding to token LATEX_NEWLINE is less or equal than `i`.
fn LATEX_NEWLINE(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(LATEX_NEWLINE, i)
}

}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn block(&mut self,)
	-> Result<Rc<BlockContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BlockContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_block);
        let mut _localctx: Rc<BlockContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(141);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ADD | SUB | L_PAREN | L_BRACE | L_BRACE_LITERAL | L_BRACKET | BAR | L_BAR |
			 L_ANGLE | FUNC_LIM | FUNC_INT | FUNC_SUM | FUNC_PROD | FUNC_EXP | FUNC_LOG |
			 FUNC_LG | FUNC_LN | FUNC_SIN | FUNC_COS | FUNC_TAN | FUNC_CSC | FUNC_SEC |
			 FUNC_COT | FUNC_ARCSIN | FUNC_ARCCOS | FUNC_ARCTAN | FUNC_ARCCSC | FUNC_ARCSEC |
			 FUNC_ARCCOT | FUNC_SINH | FUNC_COSH | FUNC_TANH | FUNC_ARSINH | FUNC_ARCOSH |
			 FUNC_ARTANH | L_FLOOR | L_CEIL | FUNC_SQRT | FUNC_OVERLINE | CMD_FRAC |
			 CMD_BINOM | CMD_DBINOM | CMD_TBINOM | CMD_MATHIT | DIFFERENTIAL | DIGIT |
			 VAR | SYMBOL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(96);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule relation*/
							recog.base.set_state(94);
							recog.relation_rec(0)?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule expr*/
							recog.base.set_state(95);
							recog.expr()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(105);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(98);
							recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

							recog.base.set_state(101);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
								1 =>{
									{
									/*InvokeRule relation*/
									recog.base.set_state(99);
									recog.relation_rec(0)?;

									}
								}
							,
								2 =>{
									{
									/*InvokeRule expr*/
									recog.base.set_state(100);
									recog.expr()?;

									}
								}

								_ => {}
							}
							}
							} 
						}
						recog.base.set_state(107);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
					}
					recog.base.set_state(111);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==SEPARATOR {
						{
						{
						recog.base.set_state(108);
						recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(113);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(114);
					recog.base.match_token(EOF,&mut recog.err_handler)?;

					}
				}

			 LATEX_BLOCK 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(116);
					recog.base.match_token(LATEX_BLOCK,&mut recog.err_handler)?;

					recog.base.set_state(117);
					recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

					recog.base.set_state(120);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
						1 =>{
							{
							/*InvokeRule relation*/
							recog.base.set_state(118);
							recog.relation_rec(0)?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule expr*/
							recog.base.set_state(119);
							recog.expr()?;

							}
						}

						_ => {}
					}
					recog.base.set_state(129);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==LATEX_NEWLINE {
						{
						{
						recog.base.set_state(122);
						recog.base.match_token(LATEX_NEWLINE,&mut recog.err_handler)?;

						recog.base.set_state(125);
						recog.err_handler.sync(&mut recog.base)?;
						match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
							1 =>{
								{
								/*InvokeRule relation*/
								recog.base.set_state(123);
								recog.relation_rec(0)?;

								}
							}
						,
							2 =>{
								{
								/*InvokeRule expr*/
								recog.base.set_state(124);
								recog.expr()?;

								}
							}

							_ => {}
						}
						}
						}
						recog.base.set_state(131);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(132);
					recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

					recog.base.set_state(136);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==SEPARATOR {
						{
						{
						recog.base.set_state(133);
						recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

						}
						}
						recog.base.set_state(138);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(139);
					recog.base.match_token(EOF,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- math ----------------
pub type MathContextAll<'input> = MathContext<'input>;


pub type MathContext<'input> = BaseParserRuleContext<'input,MathContextExt<'input>>;

#[derive(Clone)]
pub struct MathContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for MathContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for MathContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_math(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_math(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for MathContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_math(self);
	}
}

impl<'input> CustomRuleContext<'input> for MathContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_math }
	//fn type_rule_index() -> usize where Self: Sized { RULE_math }
}
antlr_rust::tid!{MathContextExt<'a>}

impl<'input> MathContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MathContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MathContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MathContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<MathContextExt<'input>>{

fn relation(&self) -> Option<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MathContextAttrs<'input> for MathContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn math(&mut self,)
	-> Result<Rc<MathContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MathContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_math);
        let mut _localctx: Rc<MathContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule relation*/
			recog.base.set_state(143);
			recog.relation_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- relation ----------------
pub type RelationContextAll<'input> = RelationContext<'input>;


pub type RelationContext<'input> = BaseParserRuleContext<'input,RelationContextExt<'input>>;

#[derive(Clone)]
pub struct RelationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for RelationContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for RelationContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relation(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_relation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for RelationContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_relation(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation }
}
antlr_rust::tid!{RelationContextExt<'a>}

impl<'input> RelationContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelationContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<RelationContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn relation_all(&self) ->  Vec<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relation(&self, i: usize) -> Option<Rc<RelationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn relop(&self) -> Option<Rc<RelopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> RelationContextAttrs<'input> for RelationContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  relation(&mut self,)
	-> Result<Rc<RelationContextAll<'input>>,ANTLRError> {
		self.relation_rec(0)
	}

	fn relation_rec(&mut self, _p: isize)
	-> Result<Rc<RelationContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = RelationContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 4, RULE_relation, _p);
	    let mut _localctx: Rc<RelationContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 4;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule expr*/
			recog.base.set_state(146);
			recog.expr()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(154);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(9,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = RelationContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_relation);
					_localctx = tmp;
					recog.base.set_state(148);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					/*InvokeRule relop*/
					recog.base.set_state(149);
					recog.relop()?;

					/*InvokeRule relation*/
					recog.base.set_state(150);
					recog.relation_rec(3)?;

					}
					} 
				}
				recog.base.set_state(156);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(9,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- equality ----------------
pub type EqualityContextAll<'input> = EqualityContext<'input>;


pub type EqualityContext<'input> = BaseParserRuleContext<'input,EqualityContextExt<'input>>;

#[derive(Clone)]
pub struct EqualityContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for EqualityContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for EqualityContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_equality(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_equality(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for EqualityContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_equality(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqualityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equality }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equality }
}
antlr_rust::tid!{EqualityContextExt<'a>}

impl<'input> EqualityContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EqualityContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqualityContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EqualityContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<EqualityContextExt<'input>>{

fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}

}

impl<'input> EqualityContextAttrs<'input> for EqualityContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equality(&mut self,)
	-> Result<Rc<EqualityContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqualityContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_equality);
        let mut _localctx: Rc<EqualityContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expr*/
			recog.base.set_state(157);
			recog.expr()?;

			recog.base.set_state(158);
			recog.base.match_token(EQUAL,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(159);
			recog.expr()?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expr ----------------
pub type ExprContextAll<'input> = ExprContext<'input>;


pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for ExprContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expr(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_expr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for ExprContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_expr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExprContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<ExprContextExt<'input>>{

fn additive(&self) -> Option<Rc<AdditiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_expr);
        let mut _localctx: Rc<ExprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule additive*/
			recog.base.set_state(161);
			recog.additive_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- sumop ----------------
pub type SumopContextAll<'input> = SumopContext<'input>;


pub type SumopContext<'input> = BaseParserRuleContext<'input,SumopContextExt<'input>>;

#[derive(Clone)]
pub struct SumopContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for SumopContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for SumopContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sumop(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_sumop(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for SumopContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_sumop(self);
	}
}

impl<'input> CustomRuleContext<'input> for SumopContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sumop }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sumop }
}
antlr_rust::tid!{SumopContextExt<'a>}

impl<'input> SumopContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SumopContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SumopContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SumopContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<SumopContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ADD
/// Returns `None` if there is no child corresponding to token ADD
fn ADD(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(ADD, 0)
}
/// Retrieves first TerminalNode corresponding to token SUB
/// Returns `None` if there is no child corresponding to token SUB
fn SUB(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(SUB, 0)
}

}

impl<'input> SumopContextAttrs<'input> for SumopContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sumop(&mut self,)
	-> Result<Rc<SumopContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SumopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_sumop);
        let mut _localctx: Rc<SumopContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(163);
			_la = recog.base.input.la(1);
			if { !(_la==ADD || _la==SUB) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- multop ----------------
pub type MultopContextAll<'input> = MultopContext<'input>;


pub type MultopContext<'input> = BaseParserRuleContext<'input,MultopContextExt<'input>>;

#[derive(Clone)]
pub struct MultopContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for MultopContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for MultopContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_multop(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_multop(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for MultopContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_multop(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultopContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multop }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multop }
}
antlr_rust::tid!{MultopContextExt<'a>}

impl<'input> MultopContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultopContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultopContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultopContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<MultopContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token MUL
/// Returns `None` if there is no child corresponding to token MUL
fn MUL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(MUL, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_TIMES
/// Returns `None` if there is no child corresponding to token CMD_TIMES
fn CMD_TIMES(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CMD_TIMES, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_CDOT
/// Returns `None` if there is no child corresponding to token CMD_CDOT
fn CMD_CDOT(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CMD_CDOT, 0)
}
/// Retrieves first TerminalNode corresponding to token DIV
/// Returns `None` if there is no child corresponding to token DIV
fn DIV(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(DIV, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_DIV
/// Returns `None` if there is no child corresponding to token CMD_DIV
fn CMD_DIV(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CMD_DIV, 0)
}
/// Retrieves first TerminalNode corresponding to token COLON
/// Returns `None` if there is no child corresponding to token COLON
fn COLON(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(COLON, 0)
}

}

impl<'input> MultopContextAttrs<'input> for MultopContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multop(&mut self,)
	-> Result<Rc<MultopContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_multop);
        let mut _localctx: Rc<MultopContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(165);
			_la = recog.base.input.la(1);
			if { !(_la==MUL || _la==DIV || ((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (CMD_TIMES - 64)) | (1usize << (CMD_CDOT - 64)) | (1usize << (CMD_DIV - 64)) | (1usize << (COLON - 64)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- relop ----------------
pub type RelopContextAll<'input> = RelopContext<'input>;


pub type RelopContext<'input> = BaseParserRuleContext<'input,RelopContextExt<'input>>;

#[derive(Clone)]
pub struct RelopContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for RelopContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for RelopContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relop(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_relop(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for RelopContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_relop(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelopContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relop }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relop }
}
antlr_rust::tid!{RelopContextExt<'a>}

impl<'input> RelopContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelopContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelopContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelopContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<RelopContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EQUAL
/// Returns `None` if there is no child corresponding to token EQUAL
fn EQUAL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(EQUAL, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token LTE
/// Returns `None` if there is no child corresponding to token LTE
fn LTE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(LTE, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token GTE
/// Returns `None` if there is no child corresponding to token GTE
fn GTE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(GTE, 0)
}
/// Retrieves first TerminalNode corresponding to token NEQ
/// Returns `None` if there is no child corresponding to token NEQ
fn NEQ(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(NEQ, 0)
}

}

impl<'input> RelopContextAttrs<'input> for RelopContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relop(&mut self,)
	-> Result<Rc<RelopContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_relop);
        let mut _localctx: Rc<RelopContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(167);
			_la = recog.base.input.la(1);
			if { !(((((_la - 78)) & !0x3f) == 0 && ((1usize << (_la - 78)) & ((1usize << (EQUAL - 78)) | (1usize << (NEQ - 78)) | (1usize << (LT - 78)) | (1usize << (LTE - 78)) | (1usize << (GT - 78)) | (1usize << (GTE - 78)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- powop ----------------
pub type PowopContextAll<'input> = PowopContext<'input>;


pub type PowopContext<'input> = BaseParserRuleContext<'input,PowopContextExt<'input>>;

#[derive(Clone)]
pub struct PowopContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for PowopContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for PowopContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_powop(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_powop(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for PowopContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_powop(self);
	}
}

impl<'input> CustomRuleContext<'input> for PowopContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_powop }
	//fn type_rule_index() -> usize where Self: Sized { RULE_powop }
}
antlr_rust::tid!{PowopContextExt<'a>}

impl<'input> PowopContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PowopContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PowopContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PowopContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<PowopContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CARET, 0)
}

}

impl<'input> PowopContextAttrs<'input> for PowopContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn powop(&mut self,)
	-> Result<Rc<PowopContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PowopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_powop);
        let mut _localctx: Rc<PowopContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(169);
			recog.base.match_token(CARET,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- additive ----------------
pub type AdditiveContextAll<'input> = AdditiveContext<'input>;


pub type AdditiveContext<'input> = BaseParserRuleContext<'input,AdditiveContextExt<'input>>;

#[derive(Clone)]
pub struct AdditiveContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for AdditiveContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AdditiveContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_additive(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_additive(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AdditiveContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_additive(self);
	}
}

impl<'input> CustomRuleContext<'input> for AdditiveContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additive }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additive }
}
antlr_rust::tid!{AdditiveContextExt<'a>}

impl<'input> AdditiveContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AdditiveContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AdditiveContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AdditiveContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<AdditiveContextExt<'input>>{

fn mp(&self) -> Option<Rc<MpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn additive(&self) -> Option<Rc<AdditiveContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn sumop(&self) -> Option<Rc<SumopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> AdditiveContextAttrs<'input> for AdditiveContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  additive(&mut self,)
	-> Result<Rc<AdditiveContextAll<'input>>,ANTLRError> {
		self.additive_rec(0)
	}

	fn additive_rec(&mut self, _p: isize)
	-> Result<Rc<AdditiveContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = AdditiveContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 18, RULE_additive, _p);
	    let mut _localctx: Rc<AdditiveContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 18;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule mp*/
			recog.base.set_state(172);
			recog.mp_rec(0)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(180);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = AdditiveContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_additive);
					_localctx = tmp;
					recog.base.set_state(174);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					/*InvokeRule sumop*/
					recog.base.set_state(175);
					recog.sumop()?;

					/*InvokeRule mp*/
					recog.base.set_state(176);
					recog.mp_rec(0)?;

					}
					} 
				}
				recog.base.set_state(182);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- mp ----------------
pub type MpContextAll<'input> = MpContext<'input>;


pub type MpContext<'input> = BaseParserRuleContext<'input,MpContextExt<'input>>;

#[derive(Clone)]
pub struct MpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for MpContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for MpContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mp(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_mp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for MpContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_mp(self);
	}
}

impl<'input> CustomRuleContext<'input> for MpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mp }
}
antlr_rust::tid!{MpContextExt<'a>}

impl<'input> MpContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MpContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<MpContextExt<'input>>{

fn exp(&self) -> Option<Rc<ExpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unary(&self) -> Option<Rc<UnaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mp_all(&self) ->  Vec<Rc<MpContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mp(&self, i: usize) -> Option<Rc<MpContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn multop(&self) -> Option<Rc<MultopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> MpContextAttrs<'input> for MpContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  mp(&mut self,)
	-> Result<Rc<MpContextAll<'input>>,ANTLRError> {
		self.mp_rec(0)
	}

	fn mp_rec(&mut self, _p: isize)
	-> Result<Rc<MpContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = MpContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 20, RULE_mp, _p);
	    let mut _localctx: Rc<MpContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 20;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(186);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule exp*/
					recog.base.set_state(184);
					recog.exp_rec(0)?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule unary*/
					recog.base.set_state(185);
					recog.unary()?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(194);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = MpContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_mp);
					_localctx = tmp;
					recog.base.set_state(188);
					if !({recog.precpred(None, 3)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
					}
					/*InvokeRule multop*/
					recog.base.set_state(189);
					recog.multop()?;

					/*InvokeRule mp*/
					recog.base.set_state(190);
					recog.mp_rec(4)?;

					}
					} 
				}
				recog.base.set_state(196);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- mp_nofunc ----------------
pub type Mp_nofuncContextAll<'input> = Mp_nofuncContext<'input>;


pub type Mp_nofuncContext<'input> = BaseParserRuleContext<'input,Mp_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Mp_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Mp_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Mp_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mp_nofunc(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_mp_nofunc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Mp_nofuncContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_mp_nofunc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mp_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mp_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mp_nofunc }
}
antlr_rust::tid!{Mp_nofuncContextExt<'a>}

impl<'input> Mp_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mp_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mp_nofuncContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mp_nofuncContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Mp_nofuncContextExt<'input>>{

fn unary_nofunc(&self) -> Option<Rc<Unary_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn mp_nofunc_all(&self) ->  Vec<Rc<Mp_nofuncContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mp_nofunc(&self, i: usize) -> Option<Rc<Mp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn multop(&self) -> Option<Rc<MultopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Mp_nofuncContextAttrs<'input> for Mp_nofuncContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  mp_nofunc(&mut self,)
	-> Result<Rc<Mp_nofuncContextAll<'input>>,ANTLRError> {
		self.mp_nofunc_rec(0)
	}

	fn mp_nofunc_rec(&mut self, _p: isize)
	-> Result<Rc<Mp_nofuncContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Mp_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 22, RULE_mp_nofunc, _p);
	    let mut _localctx: Rc<Mp_nofuncContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 22;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule unary_nofunc*/
			recog.base.set_state(198);
			recog.unary_nofunc()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(206);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Mp_nofuncContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_mp_nofunc);
					_localctx = tmp;
					recog.base.set_state(200);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					/*InvokeRule multop*/
					recog.base.set_state(201);
					recog.multop()?;

					/*InvokeRule mp_nofunc*/
					recog.base.set_state(202);
					recog.mp_nofunc_rec(3)?;

					}
					} 
				}
				recog.base.set_state(208);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(13,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- unary ----------------
pub type UnaryContextAll<'input> = UnaryContext<'input>;


pub type UnaryContext<'input> = BaseParserRuleContext<'input,UnaryContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for UnaryContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for UnaryContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unary(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_unary(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for UnaryContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_unary(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary }
}
antlr_rust::tid!{UnaryContextExt<'a>}

impl<'input> UnaryContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait UnaryContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<UnaryContextExt<'input>>{

fn sumop(&self) -> Option<Rc<SumopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unary(&self) -> Option<Rc<UnaryContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postfix_all(&self) ->  Vec<Rc<PostfixContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn postfix(&self, i: usize) -> Option<Rc<PostfixContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> UnaryContextAttrs<'input> for UnaryContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unary(&mut self,)
	-> Result<Rc<UnaryContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_unary);
        let mut _localctx: Rc<UnaryContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(217);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ADD | SUB 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule sumop*/
					recog.base.set_state(209);
					recog.sumop()?;

					/*InvokeRule unary*/
					recog.base.set_state(210);
					recog.unary()?;

					}
				}

			 L_PAREN | L_BRACE | L_BRACE_LITERAL | L_BRACKET | BAR | L_BAR | L_ANGLE |
			 FUNC_LIM | FUNC_INT | FUNC_SUM | FUNC_PROD | FUNC_EXP | FUNC_LOG | FUNC_LG |
			 FUNC_LN | FUNC_SIN | FUNC_COS | FUNC_TAN | FUNC_CSC | FUNC_SEC | FUNC_COT |
			 FUNC_ARCSIN | FUNC_ARCCOS | FUNC_ARCTAN | FUNC_ARCCSC | FUNC_ARCSEC |
			 FUNC_ARCCOT | FUNC_SINH | FUNC_COSH | FUNC_TANH | FUNC_ARSINH | FUNC_ARCOSH |
			 FUNC_ARTANH | L_FLOOR | L_CEIL | FUNC_SQRT | FUNC_OVERLINE | CMD_FRAC |
			 CMD_BINOM | CMD_DBINOM | CMD_TBINOM | CMD_MATHIT | DIFFERENTIAL | DIGIT |
			 VAR | SYMBOL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(213); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							/*InvokeRule postfix*/
							recog.base.set_state(212);
							recog.postfix()?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(215); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unary_nofunc ----------------
pub type Unary_nofuncContextAll<'input> = Unary_nofuncContext<'input>;


pub type Unary_nofuncContext<'input> = BaseParserRuleContext<'input,Unary_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Unary_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Unary_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Unary_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_unary_nofunc(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_unary_nofunc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Unary_nofuncContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_unary_nofunc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Unary_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_nofunc }
}
antlr_rust::tid!{Unary_nofuncContextExt<'a>}

impl<'input> Unary_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Unary_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Unary_nofuncContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Unary_nofuncContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Unary_nofuncContextExt<'input>>{

fn sumop(&self) -> Option<Rc<SumopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unary_nofunc(&self) -> Option<Rc<Unary_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postfix(&self) -> Option<Rc<PostfixContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postfix_nofunc_all(&self) ->  Vec<Rc<Postfix_nofuncContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn postfix_nofunc(&self, i: usize) -> Option<Rc<Postfix_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Unary_nofuncContextAttrs<'input> for Unary_nofuncContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unary_nofunc(&mut self,)
	-> Result<Rc<Unary_nofuncContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Unary_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_unary_nofunc);
        let mut _localctx: Rc<Unary_nofuncContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(229);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ADD | SUB 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule sumop*/
					recog.base.set_state(219);
					recog.sumop()?;

					/*InvokeRule unary_nofunc*/
					recog.base.set_state(220);
					recog.unary_nofunc()?;

					}
				}

			 L_PAREN | L_BRACE | L_BRACE_LITERAL | L_BRACKET | BAR | L_BAR | L_ANGLE |
			 FUNC_LIM | FUNC_INT | FUNC_SUM | FUNC_PROD | FUNC_EXP | FUNC_LOG | FUNC_LG |
			 FUNC_LN | FUNC_SIN | FUNC_COS | FUNC_TAN | FUNC_CSC | FUNC_SEC | FUNC_COT |
			 FUNC_ARCSIN | FUNC_ARCCOS | FUNC_ARCTAN | FUNC_ARCCSC | FUNC_ARCSEC |
			 FUNC_ARCCOT | FUNC_SINH | FUNC_COSH | FUNC_TANH | FUNC_ARSINH | FUNC_ARCOSH |
			 FUNC_ARTANH | L_FLOOR | L_CEIL | FUNC_SQRT | FUNC_OVERLINE | CMD_FRAC |
			 CMD_BINOM | CMD_DBINOM | CMD_TBINOM | CMD_MATHIT | DIFFERENTIAL | DIGIT |
			 VAR | SYMBOL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule postfix*/
					recog.base.set_state(222);
					recog.postfix()?;

					recog.base.set_state(226);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							/*InvokeRule postfix_nofunc*/
							recog.base.set_state(223);
							recog.postfix_nofunc()?;

							}
							} 
						}
						recog.base.set_state(228);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
					}
					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- postfix ----------------
pub type PostfixContextAll<'input> = PostfixContext<'input>;


pub type PostfixContext<'input> = BaseParserRuleContext<'input,PostfixContextExt<'input>>;

#[derive(Clone)]
pub struct PostfixContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for PostfixContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for PostfixContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_postfix(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_postfix(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for PostfixContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_postfix(self);
	}
}

impl<'input> CustomRuleContext<'input> for PostfixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfix }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfix }
}
antlr_rust::tid!{PostfixContextExt<'a>}

impl<'input> PostfixContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PostfixContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PostfixContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PostfixContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<PostfixContextExt<'input>>{

fn exp(&self) -> Option<Rc<ExpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postfix_op_all(&self) ->  Vec<Rc<Postfix_opContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn postfix_op(&self, i: usize) -> Option<Rc<Postfix_opContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> PostfixContextAttrs<'input> for PostfixContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn postfix(&mut self,)
	-> Result<Rc<PostfixContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PostfixContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_postfix);
        let mut _localctx: Rc<PostfixContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule exp*/
			recog.base.set_state(231);
			recog.exp_rec(0)?;

			recog.base.set_state(235);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule postfix_op*/
					recog.base.set_state(232);
					recog.postfix_op()?;

					}
					} 
				}
				recog.base.set_state(237);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- postfix_nofunc ----------------
pub type Postfix_nofuncContextAll<'input> = Postfix_nofuncContext<'input>;


pub type Postfix_nofuncContext<'input> = BaseParserRuleContext<'input,Postfix_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Postfix_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Postfix_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Postfix_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_postfix_nofunc(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_postfix_nofunc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Postfix_nofuncContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_postfix_nofunc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Postfix_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfix_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfix_nofunc }
}
antlr_rust::tid!{Postfix_nofuncContextExt<'a>}

impl<'input> Postfix_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Postfix_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Postfix_nofuncContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Postfix_nofuncContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Postfix_nofuncContextExt<'input>>{

fn exp_nofunc(&self) -> Option<Rc<Exp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postfix_op_all(&self) ->  Vec<Rc<Postfix_opContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn postfix_op(&self, i: usize) -> Option<Rc<Postfix_opContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Postfix_nofuncContextAttrs<'input> for Postfix_nofuncContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn postfix_nofunc(&mut self,)
	-> Result<Rc<Postfix_nofuncContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Postfix_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_postfix_nofunc);
        let mut _localctx: Rc<Postfix_nofuncContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule exp_nofunc*/
			recog.base.set_state(238);
			recog.exp_nofunc_rec(0)?;

			recog.base.set_state(242);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					/*InvokeRule postfix_op*/
					recog.base.set_state(239);
					recog.postfix_op()?;

					}
					} 
				}
				recog.base.set_state(244);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- postfix_op ----------------
pub type Postfix_opContextAll<'input> = Postfix_opContext<'input>;


pub type Postfix_opContext<'input> = BaseParserRuleContext<'input,Postfix_opContextExt<'input>>;

#[derive(Clone)]
pub struct Postfix_opContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Postfix_opContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Postfix_opContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_postfix_op(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_postfix_op(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Postfix_opContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_postfix_op(self);
	}
}

impl<'input> CustomRuleContext<'input> for Postfix_opContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfix_op }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfix_op }
}
antlr_rust::tid!{Postfix_opContextExt<'a>}

impl<'input> Postfix_opContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Postfix_opContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Postfix_opContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Postfix_opContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Postfix_opContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BANG
/// Returns `None` if there is no child corresponding to token BANG
fn BANG(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(BANG, 0)
}
fn eval_at(&self) -> Option<Rc<Eval_atContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Postfix_opContextAttrs<'input> for Postfix_opContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn postfix_op(&mut self,)
	-> Result<Rc<Postfix_opContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Postfix_opContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_postfix_op);
        let mut _localctx: Rc<Postfix_opContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(247);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BANG 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(245);
					recog.base.match_token(BANG,&mut recog.err_handler)?;

					}
				}

			 BAR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule eval_at*/
					recog.base.set_state(246);
					recog.eval_at()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eval_at ----------------
pub type Eval_atContextAll<'input> = Eval_atContext<'input>;


pub type Eval_atContext<'input> = BaseParserRuleContext<'input,Eval_atContextExt<'input>>;

#[derive(Clone)]
pub struct Eval_atContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Eval_atContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Eval_atContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eval_at(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_eval_at(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Eval_atContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_eval_at(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eval_atContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eval_at }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eval_at }
}
antlr_rust::tid!{Eval_atContextExt<'a>}

impl<'input> Eval_atContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eval_atContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eval_atContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eval_atContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Eval_atContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BAR
/// Returns `None` if there is no child corresponding to token BAR
fn BAR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(BAR, 0)
}
fn eval_at_sup(&self) -> Option<Rc<Eval_at_supContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eval_at_sub(&self) -> Option<Rc<Eval_at_subContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Eval_atContextAttrs<'input> for Eval_atContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eval_at(&mut self,)
	-> Result<Rc<Eval_atContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eval_atContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_eval_at);
        let mut _localctx: Rc<Eval_atContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(249);
			recog.base.match_token(BAR,&mut recog.err_handler)?;

			recog.base.set_state(255);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule eval_at_sup*/
					recog.base.set_state(250);
					recog.eval_at_sup()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule eval_at_sub*/
					recog.base.set_state(251);
					recog.eval_at_sub()?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule eval_at_sup*/
					recog.base.set_state(252);
					recog.eval_at_sup()?;

					/*InvokeRule eval_at_sub*/
					recog.base.set_state(253);
					recog.eval_at_sub()?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eval_at_sub ----------------
pub type Eval_at_subContextAll<'input> = Eval_at_subContext<'input>;


pub type Eval_at_subContext<'input> = BaseParserRuleContext<'input,Eval_at_subContextExt<'input>>;

#[derive(Clone)]
pub struct Eval_at_subContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Eval_at_subContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Eval_at_subContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eval_at_sub(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_eval_at_sub(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Eval_at_subContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_eval_at_sub(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eval_at_subContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eval_at_sub }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eval_at_sub }
}
antlr_rust::tid!{Eval_at_subContextExt<'a>}

impl<'input> Eval_at_subContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eval_at_subContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eval_at_subContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eval_at_subContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Eval_at_subContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(UNDERSCORE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn equality(&self) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Eval_at_subContextAttrs<'input> for Eval_at_subContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eval_at_sub(&mut self,)
	-> Result<Rc<Eval_at_subContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eval_at_subContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_eval_at_sub);
        let mut _localctx: Rc<Eval_at_subContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(257);
			recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(258);
			recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(261);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule expr*/
					recog.base.set_state(259);
					recog.expr()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule equality*/
					recog.base.set_state(260);
					recog.equality()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(263);
			recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eval_at_sup ----------------
pub type Eval_at_supContextAll<'input> = Eval_at_supContext<'input>;


pub type Eval_at_supContext<'input> = BaseParserRuleContext<'input,Eval_at_supContextExt<'input>>;

#[derive(Clone)]
pub struct Eval_at_supContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Eval_at_supContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Eval_at_supContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_eval_at_sup(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_eval_at_sup(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Eval_at_supContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_eval_at_sup(self);
	}
}

impl<'input> CustomRuleContext<'input> for Eval_at_supContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eval_at_sup }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eval_at_sup }
}
antlr_rust::tid!{Eval_at_supContextExt<'a>}

impl<'input> Eval_at_supContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Eval_at_supContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Eval_at_supContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Eval_at_supContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Eval_at_supContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CARET, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn equality(&self) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Eval_at_supContextAttrs<'input> for Eval_at_supContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eval_at_sup(&mut self,)
	-> Result<Rc<Eval_at_supContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Eval_at_supContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_eval_at_sup);
        let mut _localctx: Rc<Eval_at_supContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(265);
			recog.base.match_token(CARET,&mut recog.err_handler)?;

			recog.base.set_state(266);
			recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(269);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule expr*/
					recog.base.set_state(267);
					recog.expr()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule equality*/
					recog.base.set_state(268);
					recog.equality()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(271);
			recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- exp ----------------
pub type ExpContextAll<'input> = ExpContext<'input>;


pub type ExpContext<'input> = BaseParserRuleContext<'input,ExpContextExt<'input>>;

#[derive(Clone)]
pub struct ExpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for ExpContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for ExpContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_exp(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_exp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for ExpContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_exp(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exp }
}
antlr_rust::tid!{ExpContextExt<'a>}

impl<'input> ExpContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<ExpContextExt<'input>>{

fn comp(&self) -> Option<Rc<CompContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn exp(&self) -> Option<Rc<ExpContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn powop(&self) -> Option<Rc<PowopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}
fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpContextAttrs<'input> for ExpContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  exp(&mut self,)
	-> Result<Rc<ExpContextAll<'input>>,ANTLRError> {
		self.exp_rec(0)
	}

	fn exp_rec(&mut self, _p: isize)
	-> Result<Rc<ExpContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 40, RULE_exp, _p);
	    let mut _localctx: Rc<ExpContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 40;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule comp*/
			recog.base.set_state(274);
			recog.comp()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(290);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ExpContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_exp);
					_localctx = tmp;
					recog.base.set_state(276);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					/*InvokeRule powop*/
					recog.base.set_state(277);
					recog.powop()?;

					recog.base.set_state(283);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 BAR | L_BAR | L_ANGLE | CMD_FRAC | CMD_BINOM | CMD_DBINOM | CMD_TBINOM |
					 CMD_MATHIT | DIFFERENTIAL | DIGIT | VAR | SYMBOL 
						=> {
							{
							/*InvokeRule atom*/
							recog.base.set_state(278);
							recog.atom()?;

							}
						}

					 L_BRACE 
						=> {
							{
							recog.base.set_state(279);
							recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(280);
							recog.expr()?;

							recog.base.set_state(281);
							recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(286);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(25,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule subexpr*/
							recog.base.set_state(285);
							recog.subexpr()?;

							}
						}

						_ => {}
					}
					}
					} 
				}
				recog.base.set_state(292);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- exp_nofunc ----------------
pub type Exp_nofuncContextAll<'input> = Exp_nofuncContext<'input>;


pub type Exp_nofuncContext<'input> = BaseParserRuleContext<'input,Exp_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Exp_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Exp_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Exp_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_exp_nofunc(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_exp_nofunc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Exp_nofuncContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_exp_nofunc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Exp_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_exp_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_exp_nofunc }
}
antlr_rust::tid!{Exp_nofuncContextExt<'a>}

impl<'input> Exp_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Exp_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Exp_nofuncContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Exp_nofuncContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Exp_nofuncContextExt<'input>>{

fn comp_nofunc(&self) -> Option<Rc<Comp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn exp_nofunc(&self) -> Option<Rc<Exp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn powop(&self) -> Option<Rc<PowopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}
fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Exp_nofuncContextAttrs<'input> for Exp_nofuncContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  exp_nofunc(&mut self,)
	-> Result<Rc<Exp_nofuncContextAll<'input>>,ANTLRError> {
		self.exp_nofunc_rec(0)
	}

	fn exp_nofunc_rec(&mut self, _p: isize)
	-> Result<Rc<Exp_nofuncContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Exp_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 42, RULE_exp_nofunc, _p);
	    let mut _localctx: Rc<Exp_nofuncContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 42;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule comp_nofunc*/
			recog.base.set_state(294);
			recog.comp_nofunc()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(310);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(29,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Exp_nofuncContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_exp_nofunc);
					_localctx = tmp;
					recog.base.set_state(296);
					if !({recog.precpred(None, 2)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
					}
					/*InvokeRule powop*/
					recog.base.set_state(297);
					recog.powop()?;

					recog.base.set_state(303);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 BAR | L_BAR | L_ANGLE | CMD_FRAC | CMD_BINOM | CMD_DBINOM | CMD_TBINOM |
					 CMD_MATHIT | DIFFERENTIAL | DIGIT | VAR | SYMBOL 
						=> {
							{
							/*InvokeRule atom*/
							recog.base.set_state(298);
							recog.atom()?;

							}
						}

					 L_BRACE 
						=> {
							{
							recog.base.set_state(299);
							recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(300);
							recog.expr()?;

							recog.base.set_state(301);
							recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					recog.base.set_state(306);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(28,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule subexpr*/
							recog.base.set_state(305);
							recog.subexpr()?;

							}
						}

						_ => {}
					}
					}
					} 
				}
				recog.base.set_state(312);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(29,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- comp ----------------
pub type CompContextAll<'input> = CompContext<'input>;


pub type CompContext<'input> = BaseParserRuleContext<'input,CompContextExt<'input>>;

#[derive(Clone)]
pub struct CompContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for CompContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for CompContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comp(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_comp(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for CompContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_comp(self);
	}
}

impl<'input> CustomRuleContext<'input> for CompContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comp }
}
antlr_rust::tid!{CompContextExt<'a>}

impl<'input> CompContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CompContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CompContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CompContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<CompContextExt<'input>>{

fn group(&self) -> Option<Rc<GroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn abs_group(&self) -> Option<Rc<Abs_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn func(&self) -> Option<Rc<FuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn floor(&self) -> Option<Rc<FloorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ceil(&self) -> Option<Rc<CeilContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CompContextAttrs<'input> for CompContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comp(&mut self,)
	-> Result<Rc<CompContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CompContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_comp);
        let mut _localctx: Rc<CompContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(319);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(30,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule group*/
					recog.base.set_state(313);
					recog.group()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule abs_group*/
					recog.base.set_state(314);
					recog.abs_group()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule func*/
					recog.base.set_state(315);
					recog.func()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule atom*/
					recog.base.set_state(316);
					recog.atom()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule floor*/
					recog.base.set_state(317);
					recog.floor()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule ceil*/
					recog.base.set_state(318);
					recog.ceil()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- comp_nofunc ----------------
pub type Comp_nofuncContextAll<'input> = Comp_nofuncContext<'input>;


pub type Comp_nofuncContext<'input> = BaseParserRuleContext<'input,Comp_nofuncContextExt<'input>>;

#[derive(Clone)]
pub struct Comp_nofuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Comp_nofuncContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Comp_nofuncContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_comp_nofunc(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_comp_nofunc(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Comp_nofuncContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_comp_nofunc(self);
	}
}

impl<'input> CustomRuleContext<'input> for Comp_nofuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_comp_nofunc }
	//fn type_rule_index() -> usize where Self: Sized { RULE_comp_nofunc }
}
antlr_rust::tid!{Comp_nofuncContextExt<'a>}

impl<'input> Comp_nofuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Comp_nofuncContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Comp_nofuncContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Comp_nofuncContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Comp_nofuncContextExt<'input>>{

fn group(&self) -> Option<Rc<GroupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn abs_group(&self) -> Option<Rc<Abs_groupContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn floor(&self) -> Option<Rc<FloorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ceil(&self) -> Option<Rc<CeilContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Comp_nofuncContextAttrs<'input> for Comp_nofuncContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn comp_nofunc(&mut self,)
	-> Result<Rc<Comp_nofuncContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Comp_nofuncContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_comp_nofunc);
        let mut _localctx: Rc<Comp_nofuncContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(326);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule group*/
					recog.base.set_state(321);
					recog.group()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule abs_group*/
					recog.base.set_state(322);
					recog.abs_group()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule atom*/
					recog.base.set_state(323);
					recog.atom()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule floor*/
					recog.base.set_state(324);
					recog.floor()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule ceil*/
					recog.base.set_state(325);
					recog.ceil()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- group ----------------
pub type GroupContextAll<'input> = GroupContext<'input>;


pub type GroupContext<'input> = BaseParserRuleContext<'input,GroupContextExt<'input>>;

#[derive(Clone)]
pub struct GroupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for GroupContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for GroupContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_group(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_group(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for GroupContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_group(self);
	}
}

impl<'input> CustomRuleContext<'input> for GroupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_group }
}
antlr_rust::tid!{GroupContextExt<'a>}

impl<'input> GroupContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<GroupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,GroupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait GroupContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<GroupContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_PAREN
/// Returns `None` if there is no child corresponding to token L_PAREN
fn L_PAREN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_PAREN, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_PAREN
/// Returns `None` if there is no child corresponding to token R_PAREN
fn R_PAREN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_PAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACKET
/// Returns `None` if there is no child corresponding to token L_BRACKET
fn L_BRACKET(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACKET
/// Returns `None` if there is no child corresponding to token R_BRACKET
fn R_BRACKET(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACKET, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE_LITERAL
/// Returns `None` if there is no child corresponding to token L_BRACE_LITERAL
fn L_BRACE_LITERAL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE_LITERAL, 0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE_LITERAL
/// Returns `None` if there is no child corresponding to token R_BRACE_LITERAL
fn R_BRACE_LITERAL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE_LITERAL, 0)
}

}

impl<'input> GroupContextAttrs<'input> for GroupContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn group(&mut self,)
	-> Result<Rc<GroupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = GroupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_group);
        let mut _localctx: Rc<GroupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(344);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 L_PAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(328);
					recog.base.match_token(L_PAREN,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(329);
					recog.expr()?;

					recog.base.set_state(330);
					recog.base.match_token(R_PAREN,&mut recog.err_handler)?;

					}
				}

			 L_BRACKET 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(332);
					recog.base.match_token(L_BRACKET,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(333);
					recog.expr()?;

					recog.base.set_state(334);
					recog.base.match_token(R_BRACKET,&mut recog.err_handler)?;

					}
				}

			 L_BRACE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(336);
					recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(337);
					recog.expr()?;

					recog.base.set_state(338);
					recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

					}
				}

			 L_BRACE_LITERAL 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(340);
					recog.base.match_token(L_BRACE_LITERAL,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(341);
					recog.expr()?;

					recog.base.set_state(342);
					recog.base.match_token(R_BRACE_LITERAL,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- abs_group ----------------
pub type Abs_groupContextAll<'input> = Abs_groupContext<'input>;


pub type Abs_groupContext<'input> = BaseParserRuleContext<'input,Abs_groupContextExt<'input>>;

#[derive(Clone)]
pub struct Abs_groupContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Abs_groupContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Abs_groupContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_abs_group(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_abs_group(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Abs_groupContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_abs_group(self);
	}
}

impl<'input> CustomRuleContext<'input> for Abs_groupContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_abs_group }
	//fn type_rule_index() -> usize where Self: Sized { RULE_abs_group }
}
antlr_rust::tid!{Abs_groupContextExt<'a>}

impl<'input> Abs_groupContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Abs_groupContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Abs_groupContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Abs_groupContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Abs_groupContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token BAR in current rule
fn BAR_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token BAR, starting from 0.
/// Returns `None` if number of children corresponding to token BAR is less or equal than `i`.
fn BAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(BAR, i)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Abs_groupContextAttrs<'input> for Abs_groupContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn abs_group(&mut self,)
	-> Result<Rc<Abs_groupContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Abs_groupContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_abs_group);
        let mut _localctx: Rc<Abs_groupContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(346);
			recog.base.match_token(BAR,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(347);
			recog.expr()?;

			recog.base.set_state(348);
			recog.base.match_token(BAR,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- number ----------------
pub type NumberContextAll<'input> = NumberContext<'input>;


pub type NumberContext<'input> = BaseParserRuleContext<'input,NumberContextExt<'input>>;

#[derive(Clone)]
pub struct NumberContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for NumberContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for NumberContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_number(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_number(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for NumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_number(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_number }
	//fn type_rule_index() -> usize where Self: Sized { RULE_number }
}
antlr_rust::tid!{NumberContextExt<'a>}

impl<'input> NumberContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<NumberContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NumberContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait NumberContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<NumberContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token DIGIT in current rule
fn DIGIT_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DIGIT, starting from 0.
/// Returns `None` if number of children corresponding to token DIGIT is less or equal than `i`.
fn DIGIT(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(DIGIT, i)
}

}

impl<'input> NumberContextAttrs<'input> for NumberContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn number(&mut self,)
	-> Result<Rc<NumberContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NumberContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_number);
        let mut _localctx: Rc<NumberContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(351); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(350);
					recog.base.match_token(DIGIT,&mut recog.err_handler)?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(353); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(33,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
			}
			recog.base.set_state(361);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(355);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					recog.base.set_state(356);
					recog.base.match_token(DIGIT,&mut recog.err_handler)?;

					recog.base.set_state(357);
					recog.base.match_token(DIGIT,&mut recog.err_handler)?;

					recog.base.set_state(358);
					recog.base.match_token(DIGIT,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(363);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(34,&mut recog.base)?;
			}
			recog.base.set_state(370);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(36,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(364);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					recog.base.set_state(366); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							recog.base.set_state(365);
							recog.base.match_token(DIGIT,&mut recog.err_handler)?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(368); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(35,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- atom ----------------
#[derive(Debug)]
pub enum AtomContextAll<'input>{
	AtomKetContext(AtomKetContext<'input>),
	AtomVariableContext(AtomVariableContext<'input>),
	AtomMathitContext(AtomMathitContext<'input>),
	AtomNumberContext(AtomNumberContext<'input>),
	AtomDifferentialContext(AtomDifferentialContext<'input>),
	AtomBinomContext(AtomBinomContext<'input>),
	AtomBraContext(AtomBraContext<'input>),
	AtomFracContext(AtomFracContext<'input>),
Error(AtomContext<'input>)
}
antlr_rust::tid!{AtomContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for AtomContextAll<'input>{}

impl<'input> LaTeXParserContext<'input> for AtomContextAll<'input>{}

impl<'input> Deref for AtomContextAll<'input>{
	type Target = dyn AtomContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use AtomContextAll::*;
		match self{
			AtomKetContext(inner) => inner,
			AtomVariableContext(inner) => inner,
			AtomMathitContext(inner) => inner,
			AtomNumberContext(inner) => inner,
			AtomDifferentialContext(inner) => inner,
			AtomBinomContext(inner) => inner,
			AtomBraContext(inner) => inner,
			AtomFracContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomContextAll<'input>{
    fn enter(&self, listener: &mut (dyn LaTeXListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn LaTeXListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type AtomContext<'input> = BaseParserRuleContext<'input,AtomContextExt<'input>>;

#[derive(Clone)]
pub struct AtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for AtomContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomContext<'input>{
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomContext<'input>{
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}
antlr_rust::tid!{AtomContextExt<'a>}

impl<'input> AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AtomContextAll<'input>> {
		Rc::new(
		AtomContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait AtomContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<AtomContextExt<'input>>{


}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input>{}

pub type AtomKetContext<'input> = BaseParserRuleContext<'input,AtomKetContextExt<'input>>;

pub trait AtomKetContextAttrs<'input>: LaTeXParserContext<'input>{
	fn ket(&self) -> Option<Rc<KetContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AtomKetContextAttrs<'input> for AtomKetContext<'input>{}

pub struct AtomKetContextExt<'input>{
	base:AtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomKetContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for AtomKetContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomKetContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_atomKet(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_atomKet(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomKetContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_atomKet(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomKetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}

impl<'input> Borrow<AtomContextExt<'input>> for AtomKetContext<'input>{
	fn borrow(&self) -> &AtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomContextExt<'input>> for AtomKetContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomContextExt<'input> { &mut self.base }
}

impl<'input> AtomContextAttrs<'input> for AtomKetContext<'input> {}

impl<'input> AtomKetContextExt<'input>{
	fn new(ctx: &dyn AtomContextAttrs<'input>) -> Rc<AtomContextAll<'input>>  {
		Rc::new(
			AtomContextAll::AtomKetContext(
				BaseParserRuleContext::copy_from(ctx,AtomKetContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomVariableContext<'input> = BaseParserRuleContext<'input,AtomVariableContextExt<'input>>;

pub trait AtomVariableContextAttrs<'input>: LaTeXParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token VAR
	/// Returns `None` if there is no child corresponding to token VAR
	fn VAR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(VAR, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SYMBOL
	/// Returns `None` if there is no child corresponding to token SYMBOL
	fn SYMBOL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(SYMBOL, 0)
	}
	fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SINGLE_QUOTES
	/// Returns `None` if there is no child corresponding to token SINGLE_QUOTES
	fn SINGLE_QUOTES(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(SINGLE_QUOTES, 0)
	}
}

impl<'input> AtomVariableContextAttrs<'input> for AtomVariableContext<'input>{}

pub struct AtomVariableContextExt<'input>{
	base:AtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomVariableContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for AtomVariableContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomVariableContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_atomVariable(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_atomVariable(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomVariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_atomVariable(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomVariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}

impl<'input> Borrow<AtomContextExt<'input>> for AtomVariableContext<'input>{
	fn borrow(&self) -> &AtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomContextExt<'input>> for AtomVariableContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomContextExt<'input> { &mut self.base }
}

impl<'input> AtomContextAttrs<'input> for AtomVariableContext<'input> {}

impl<'input> AtomVariableContextExt<'input>{
	fn new(ctx: &dyn AtomContextAttrs<'input>) -> Rc<AtomContextAll<'input>>  {
		Rc::new(
			AtomContextAll::AtomVariableContext(
				BaseParserRuleContext::copy_from(ctx,AtomVariableContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomMathitContext<'input> = BaseParserRuleContext<'input,AtomMathitContextExt<'input>>;

pub trait AtomMathitContextAttrs<'input>: LaTeXParserContext<'input>{
	fn mathit(&self) -> Option<Rc<MathitContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AtomMathitContextAttrs<'input> for AtomMathitContext<'input>{}

pub struct AtomMathitContextExt<'input>{
	base:AtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomMathitContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for AtomMathitContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomMathitContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_atomMathit(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_atomMathit(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomMathitContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_atomMathit(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomMathitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}

impl<'input> Borrow<AtomContextExt<'input>> for AtomMathitContext<'input>{
	fn borrow(&self) -> &AtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomContextExt<'input>> for AtomMathitContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomContextExt<'input> { &mut self.base }
}

impl<'input> AtomContextAttrs<'input> for AtomMathitContext<'input> {}

impl<'input> AtomMathitContextExt<'input>{
	fn new(ctx: &dyn AtomContextAttrs<'input>) -> Rc<AtomContextAll<'input>>  {
		Rc::new(
			AtomContextAll::AtomMathitContext(
				BaseParserRuleContext::copy_from(ctx,AtomMathitContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomNumberContext<'input> = BaseParserRuleContext<'input,AtomNumberContextExt<'input>>;

pub trait AtomNumberContextAttrs<'input>: LaTeXParserContext<'input>{
	fn number(&self) -> Option<Rc<NumberContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AtomNumberContextAttrs<'input> for AtomNumberContext<'input>{}

pub struct AtomNumberContextExt<'input>{
	base:AtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomNumberContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for AtomNumberContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomNumberContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_atomNumber(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_atomNumber(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomNumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_atomNumber(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomNumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}

impl<'input> Borrow<AtomContextExt<'input>> for AtomNumberContext<'input>{
	fn borrow(&self) -> &AtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomContextExt<'input>> for AtomNumberContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomContextExt<'input> { &mut self.base }
}

impl<'input> AtomContextAttrs<'input> for AtomNumberContext<'input> {}

impl<'input> AtomNumberContextExt<'input>{
	fn new(ctx: &dyn AtomContextAttrs<'input>) -> Rc<AtomContextAll<'input>>  {
		Rc::new(
			AtomContextAll::AtomNumberContext(
				BaseParserRuleContext::copy_from(ctx,AtomNumberContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomDifferentialContext<'input> = BaseParserRuleContext<'input,AtomDifferentialContextExt<'input>>;

pub trait AtomDifferentialContextAttrs<'input>: LaTeXParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token DIFFERENTIAL
	/// Returns `None` if there is no child corresponding to token DIFFERENTIAL
	fn DIFFERENTIAL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(DIFFERENTIAL, 0)
	}
}

impl<'input> AtomDifferentialContextAttrs<'input> for AtomDifferentialContext<'input>{}

pub struct AtomDifferentialContextExt<'input>{
	base:AtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomDifferentialContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for AtomDifferentialContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomDifferentialContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_atomDifferential(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_atomDifferential(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomDifferentialContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_atomDifferential(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomDifferentialContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}

impl<'input> Borrow<AtomContextExt<'input>> for AtomDifferentialContext<'input>{
	fn borrow(&self) -> &AtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomContextExt<'input>> for AtomDifferentialContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomContextExt<'input> { &mut self.base }
}

impl<'input> AtomContextAttrs<'input> for AtomDifferentialContext<'input> {}

impl<'input> AtomDifferentialContextExt<'input>{
	fn new(ctx: &dyn AtomContextAttrs<'input>) -> Rc<AtomContextAll<'input>>  {
		Rc::new(
			AtomContextAll::AtomDifferentialContext(
				BaseParserRuleContext::copy_from(ctx,AtomDifferentialContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomBinomContext<'input> = BaseParserRuleContext<'input,AtomBinomContextExt<'input>>;

pub trait AtomBinomContextAttrs<'input>: LaTeXParserContext<'input>{
	fn binom(&self) -> Option<Rc<BinomContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AtomBinomContextAttrs<'input> for AtomBinomContext<'input>{}

pub struct AtomBinomContextExt<'input>{
	base:AtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomBinomContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for AtomBinomContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomBinomContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_atomBinom(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_atomBinom(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomBinomContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_atomBinom(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomBinomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}

impl<'input> Borrow<AtomContextExt<'input>> for AtomBinomContext<'input>{
	fn borrow(&self) -> &AtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomContextExt<'input>> for AtomBinomContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomContextExt<'input> { &mut self.base }
}

impl<'input> AtomContextAttrs<'input> for AtomBinomContext<'input> {}

impl<'input> AtomBinomContextExt<'input>{
	fn new(ctx: &dyn AtomContextAttrs<'input>) -> Rc<AtomContextAll<'input>>  {
		Rc::new(
			AtomContextAll::AtomBinomContext(
				BaseParserRuleContext::copy_from(ctx,AtomBinomContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomBraContext<'input> = BaseParserRuleContext<'input,AtomBraContextExt<'input>>;

pub trait AtomBraContextAttrs<'input>: LaTeXParserContext<'input>{
	fn bra(&self) -> Option<Rc<BraContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AtomBraContextAttrs<'input> for AtomBraContext<'input>{}

pub struct AtomBraContextExt<'input>{
	base:AtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomBraContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for AtomBraContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomBraContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_atomBra(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_atomBra(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomBraContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_atomBra(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomBraContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}

impl<'input> Borrow<AtomContextExt<'input>> for AtomBraContext<'input>{
	fn borrow(&self) -> &AtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomContextExt<'input>> for AtomBraContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomContextExt<'input> { &mut self.base }
}

impl<'input> AtomContextAttrs<'input> for AtomBraContext<'input> {}

impl<'input> AtomBraContextExt<'input>{
	fn new(ctx: &dyn AtomContextAttrs<'input>) -> Rc<AtomContextAll<'input>>  {
		Rc::new(
			AtomContextAll::AtomBraContext(
				BaseParserRuleContext::copy_from(ctx,AtomBraContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomFracContext<'input> = BaseParserRuleContext<'input,AtomFracContextExt<'input>>;

pub trait AtomFracContextAttrs<'input>: LaTeXParserContext<'input>{
	fn frac(&self) -> Option<Rc<FracContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AtomFracContextAttrs<'input> for AtomFracContext<'input>{}

pub struct AtomFracContextExt<'input>{
	base:AtomContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomFracContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for AtomFracContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomFracContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_atomFrac(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_atomFrac(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomFracContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_atomFrac(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomFracContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}

impl<'input> Borrow<AtomContextExt<'input>> for AtomFracContext<'input>{
	fn borrow(&self) -> &AtomContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomContextExt<'input>> for AtomFracContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomContextExt<'input> { &mut self.base }
}

impl<'input> AtomContextAttrs<'input> for AtomFracContext<'input> {}

impl<'input> AtomFracContextExt<'input>{
	fn new(ctx: &dyn AtomContextAttrs<'input>) -> Rc<AtomContextAll<'input>>  {
		Rc::new(
			AtomContextAll::AtomFracContext(
				BaseParserRuleContext::copy_from(ctx,AtomFracContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atom(&mut self,)
	-> Result<Rc<AtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_atom);
        let mut _localctx: Rc<AtomContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(394);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 VAR | SYMBOL 
				=> {
					let tmp = AtomVariableContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(372);
					_la = recog.base.input.la(1);
					if { !(_la==VAR || _la==SYMBOL) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(385);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(41,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(374);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(37,&mut recog.base)? {
								x if x == 1=>{
									{
									/*InvokeRule subexpr*/
									recog.base.set_state(373);
									recog.subexpr()?;

									}
								}

								_ => {}
							}
							recog.base.set_state(377);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(38,&mut recog.base)? {
								x if x == 1=>{
									{
									recog.base.set_state(376);
									recog.base.match_token(SINGLE_QUOTES,&mut recog.err_handler)?;

									}
								}

								_ => {}
							}
							}
						}
					,
						2 =>{
							{
							recog.base.set_state(380);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(39,&mut recog.base)? {
								x if x == 1=>{
									{
									recog.base.set_state(379);
									recog.base.match_token(SINGLE_QUOTES,&mut recog.err_handler)?;

									}
								}

								_ => {}
							}
							recog.base.set_state(383);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(40,&mut recog.base)? {
								x if x == 1=>{
									{
									/*InvokeRule subexpr*/
									recog.base.set_state(382);
									recog.subexpr()?;

									}
								}

								_ => {}
							}
							}
						}

						_ => {}
					}
					}
				}

			 DIGIT 
				=> {
					let tmp = AtomNumberContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule number*/
					recog.base.set_state(387);
					recog.number()?;

					}
				}

			 DIFFERENTIAL 
				=> {
					let tmp = AtomDifferentialContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(388);
					recog.base.match_token(DIFFERENTIAL,&mut recog.err_handler)?;

					}
				}

			 CMD_MATHIT 
				=> {
					let tmp = AtomMathitContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					/*InvokeRule mathit*/
					recog.base.set_state(389);
					recog.mathit()?;

					}
				}

			 CMD_FRAC 
				=> {
					let tmp = AtomFracContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					/*InvokeRule frac*/
					recog.base.set_state(390);
					recog.frac()?;

					}
				}

			 CMD_BINOM | CMD_DBINOM | CMD_TBINOM 
				=> {
					let tmp = AtomBinomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					/*InvokeRule binom*/
					recog.base.set_state(391);
					recog.binom()?;

					}
				}

			 L_ANGLE 
				=> {
					let tmp = AtomBraContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					/*InvokeRule bra*/
					recog.base.set_state(392);
					recog.bra()?;

					}
				}

			 BAR | L_BAR 
				=> {
					let tmp = AtomKetContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					/*InvokeRule ket*/
					recog.base.set_state(393);
					recog.ket()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- bra ----------------
pub type BraContextAll<'input> = BraContext<'input>;


pub type BraContext<'input> = BaseParserRuleContext<'input,BraContextExt<'input>>;

#[derive(Clone)]
pub struct BraContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for BraContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for BraContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_bra(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_bra(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for BraContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_bra(self);
	}
}

impl<'input> CustomRuleContext<'input> for BraContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_bra }
	//fn type_rule_index() -> usize where Self: Sized { RULE_bra }
}
antlr_rust::tid!{BraContextExt<'a>}

impl<'input> BraContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BraContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BraContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BraContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<BraContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_ANGLE
/// Returns `None` if there is no child corresponding to token L_ANGLE
fn L_ANGLE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_ANGLE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BAR
/// Returns `None` if there is no child corresponding to token R_BAR
fn R_BAR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BAR, 0)
}
/// Retrieves first TerminalNode corresponding to token BAR
/// Returns `None` if there is no child corresponding to token BAR
fn BAR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(BAR, 0)
}

}

impl<'input> BraContextAttrs<'input> for BraContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn bra(&mut self,)
	-> Result<Rc<BraContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BraContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_bra);
        let mut _localctx: Rc<BraContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(396);
			recog.base.match_token(L_ANGLE,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(397);
			recog.expr()?;

			recog.base.set_state(398);
			_la = recog.base.input.la(1);
			if { !(_la==BAR || _la==R_BAR) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ket ----------------
pub type KetContextAll<'input> = KetContext<'input>;


pub type KetContext<'input> = BaseParserRuleContext<'input,KetContextExt<'input>>;

#[derive(Clone)]
pub struct KetContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for KetContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for KetContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ket(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_ket(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for KetContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_ket(self);
	}
}

impl<'input> CustomRuleContext<'input> for KetContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ket }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ket }
}
antlr_rust::tid!{KetContextExt<'a>}

impl<'input> KetContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<KetContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,KetContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait KetContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<KetContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_ANGLE
/// Returns `None` if there is no child corresponding to token R_ANGLE
fn R_ANGLE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_ANGLE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BAR
/// Returns `None` if there is no child corresponding to token L_BAR
fn L_BAR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BAR, 0)
}
/// Retrieves first TerminalNode corresponding to token BAR
/// Returns `None` if there is no child corresponding to token BAR
fn BAR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(BAR, 0)
}

}

impl<'input> KetContextAttrs<'input> for KetContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ket(&mut self,)
	-> Result<Rc<KetContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = KetContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_ket);
        let mut _localctx: Rc<KetContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(400);
			_la = recog.base.input.la(1);
			if { !(_la==BAR || _la==L_BAR) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule expr*/
			recog.base.set_state(401);
			recog.expr()?;

			recog.base.set_state(402);
			recog.base.match_token(R_ANGLE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- mathit ----------------
pub type MathitContextAll<'input> = MathitContext<'input>;


pub type MathitContext<'input> = BaseParserRuleContext<'input,MathitContextExt<'input>>;

#[derive(Clone)]
pub struct MathitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for MathitContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for MathitContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mathit(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_mathit(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for MathitContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_mathit(self);
	}
}

impl<'input> CustomRuleContext<'input> for MathitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mathit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mathit }
}
antlr_rust::tid!{MathitContextExt<'a>}

impl<'input> MathitContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MathitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MathitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MathitContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<MathitContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CMD_MATHIT
/// Returns `None` if there is no child corresponding to token CMD_MATHIT
fn CMD_MATHIT(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CMD_MATHIT, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
fn mathit_text(&self) -> Option<Rc<Mathit_textContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}

}

impl<'input> MathitContextAttrs<'input> for MathitContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mathit(&mut self,)
	-> Result<Rc<MathitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MathitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_mathit);
        let mut _localctx: Rc<MathitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(404);
			recog.base.match_token(CMD_MATHIT,&mut recog.err_handler)?;

			recog.base.set_state(405);
			recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule mathit_text*/
			recog.base.set_state(406);
			recog.mathit_text()?;

			recog.base.set_state(407);
			recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- mathit_text ----------------
pub type Mathit_textContextAll<'input> = Mathit_textContext<'input>;


pub type Mathit_textContext<'input> = BaseParserRuleContext<'input,Mathit_textContextExt<'input>>;

#[derive(Clone)]
pub struct Mathit_textContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Mathit_textContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Mathit_textContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mathit_text(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_mathit_text(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Mathit_textContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_mathit_text(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mathit_textContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mathit_text }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mathit_text }
}
antlr_rust::tid!{Mathit_textContextExt<'a>}

impl<'input> Mathit_textContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mathit_textContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mathit_textContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mathit_textContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Mathit_textContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VAR
/// Returns `None` if there is no child corresponding to token VAR
fn VAR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(VAR, 0)
}

}

impl<'input> Mathit_textContextAttrs<'input> for Mathit_textContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mathit_text(&mut self,)
	-> Result<Rc<Mathit_textContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mathit_textContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_mathit_text);
        let mut _localctx: Rc<Mathit_textContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(410);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==VAR {
				{
				recog.base.set_state(409);
				recog.base.match_token(VAR,&mut recog.err_handler)?;

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- frac ----------------
pub type FracContextAll<'input> = FracContext<'input>;


pub type FracContext<'input> = BaseParserRuleContext<'input,FracContextExt<'input>>;

#[derive(Clone)]
pub struct FracContextExt<'input>{
	pub upperd: Option<TokenType<'input>>,
	pub upper: Option<Rc<ExprContextAll<'input>>>,
	pub lowerd: Option<TokenType<'input>>,
	pub lower: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for FracContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for FracContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_frac(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_frac(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for FracContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_frac(self);
	}
}

impl<'input> CustomRuleContext<'input> for FracContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_frac }
	//fn type_rule_index() -> usize where Self: Sized { RULE_frac }
}
antlr_rust::tid!{FracContextExt<'a>}

impl<'input> FracContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FracContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FracContextExt{
				upperd: None, lowerd: None, 
				upper: None, lower: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait FracContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<FracContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CMD_FRAC
/// Returns `None` if there is no child corresponding to token CMD_FRAC
fn CMD_FRAC(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CMD_FRAC, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token L_BRACE in current rule
fn L_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token L_BRACE is less or equal than `i`.
fn L_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token R_BRACE in current rule
fn R_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token R_BRACE is less or equal than `i`.
fn R_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DIGIT in current rule
fn DIGIT_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DIGIT, starting from 0.
/// Returns `None` if number of children corresponding to token DIGIT is less or equal than `i`.
fn DIGIT(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(DIGIT, i)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> FracContextAttrs<'input> for FracContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn frac(&mut self,)
	-> Result<Rc<FracContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FracContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_frac);
        let mut _localctx: Rc<FracContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(412);
			recog.base.match_token(CMD_FRAC,&mut recog.err_handler)?;

			recog.base.set_state(418);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DIGIT 
				=> {
					{
					recog.base.set_state(413);
					let tmp = recog.base.match_token(DIGIT,&mut recog.err_handler)?;
					 cast_mut::<_,FracContext >(&mut _localctx).upperd = Some(tmp.clone());
					  

					}
				}

			 L_BRACE 
				=> {
					{
					recog.base.set_state(414);
					recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(415);
					let tmp = recog.expr()?;
					 cast_mut::<_,FracContext >(&mut _localctx).upper = Some(tmp.clone());
					  

					recog.base.set_state(416);
					recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			recog.base.set_state(425);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DIGIT 
				=> {
					{
					recog.base.set_state(420);
					let tmp = recog.base.match_token(DIGIT,&mut recog.err_handler)?;
					 cast_mut::<_,FracContext >(&mut _localctx).lowerd = Some(tmp.clone());
					  

					}
				}

			 L_BRACE 
				=> {
					{
					recog.base.set_state(421);
					recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(422);
					let tmp = recog.expr()?;
					 cast_mut::<_,FracContext >(&mut _localctx).lower = Some(tmp.clone());
					  

					recog.base.set_state(423);
					recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- binom ----------------
pub type BinomContextAll<'input> = BinomContext<'input>;


pub type BinomContext<'input> = BaseParserRuleContext<'input,BinomContextExt<'input>>;

#[derive(Clone)]
pub struct BinomContextExt<'input>{
	pub n: Option<Rc<ExprContextAll<'input>>>,
	pub k: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for BinomContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for BinomContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_binom(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_binom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for BinomContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_binom(self);
	}
}

impl<'input> CustomRuleContext<'input> for BinomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binom }
}
antlr_rust::tid!{BinomContextExt<'a>}

impl<'input> BinomContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BinomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BinomContextExt{
				n: None, k: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait BinomContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<BinomContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token L_BRACE in current rule
fn L_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token L_BRACE is less or equal than `i`.
fn L_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, i)
}
/// Retrieves all `TerminalNode`s corresponding to token R_BRACE in current rule
fn R_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token R_BRACE is less or equal than `i`.
fn R_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, i)
}
/// Retrieves first TerminalNode corresponding to token CMD_BINOM
/// Returns `None` if there is no child corresponding to token CMD_BINOM
fn CMD_BINOM(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CMD_BINOM, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_DBINOM
/// Returns `None` if there is no child corresponding to token CMD_DBINOM
fn CMD_DBINOM(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CMD_DBINOM, 0)
}
/// Retrieves first TerminalNode corresponding to token CMD_TBINOM
/// Returns `None` if there is no child corresponding to token CMD_TBINOM
fn CMD_TBINOM(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CMD_TBINOM, 0)
}
fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BinomContextAttrs<'input> for BinomContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn binom(&mut self,)
	-> Result<Rc<BinomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BinomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_binom);
        let mut _localctx: Rc<BinomContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(427);
			_la = recog.base.input.la(1);
			if { !(((((_la - 68)) & !0x3f) == 0 && ((1usize << (_la - 68)) & ((1usize << (CMD_BINOM - 68)) | (1usize << (CMD_DBINOM - 68)) | (1usize << (CMD_TBINOM - 68)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(428);
			recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(429);
			let tmp = recog.expr()?;
			 cast_mut::<_,BinomContext >(&mut _localctx).n = Some(tmp.clone());
			  

			recog.base.set_state(430);
			recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(431);
			recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(432);
			let tmp = recog.expr()?;
			 cast_mut::<_,BinomContext >(&mut _localctx).k = Some(tmp.clone());
			  

			recog.base.set_state(433);
			recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- floor ----------------
pub type FloorContextAll<'input> = FloorContext<'input>;


pub type FloorContext<'input> = BaseParserRuleContext<'input,FloorContextExt<'input>>;

#[derive(Clone)]
pub struct FloorContextExt<'input>{
	pub val: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for FloorContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for FloorContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_floor(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_floor(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for FloorContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_floor(self);
	}
}

impl<'input> CustomRuleContext<'input> for FloorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_floor }
	//fn type_rule_index() -> usize where Self: Sized { RULE_floor }
}
antlr_rust::tid!{FloorContextExt<'a>}

impl<'input> FloorContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FloorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FloorContextExt{
				val: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait FloorContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<FloorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_FLOOR
/// Returns `None` if there is no child corresponding to token L_FLOOR
fn L_FLOOR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_FLOOR, 0)
}
/// Retrieves first TerminalNode corresponding to token R_FLOOR
/// Returns `None` if there is no child corresponding to token R_FLOOR
fn R_FLOOR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_FLOOR, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> FloorContextAttrs<'input> for FloorContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn floor(&mut self,)
	-> Result<Rc<FloorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FloorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_floor);
        let mut _localctx: Rc<FloorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(435);
			recog.base.match_token(L_FLOOR,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(436);
			let tmp = recog.expr()?;
			 cast_mut::<_,FloorContext >(&mut _localctx).val = Some(tmp.clone());
			  

			recog.base.set_state(437);
			recog.base.match_token(R_FLOOR,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ceil ----------------
pub type CeilContextAll<'input> = CeilContext<'input>;


pub type CeilContext<'input> = BaseParserRuleContext<'input,CeilContextExt<'input>>;

#[derive(Clone)]
pub struct CeilContextExt<'input>{
	pub val: Option<Rc<ExprContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for CeilContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for CeilContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ceil(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_ceil(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for CeilContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_ceil(self);
	}
}

impl<'input> CustomRuleContext<'input> for CeilContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ceil }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ceil }
}
antlr_rust::tid!{CeilContextExt<'a>}

impl<'input> CeilContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CeilContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CeilContextExt{
				val: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait CeilContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<CeilContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token L_CEIL
/// Returns `None` if there is no child corresponding to token L_CEIL
fn L_CEIL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_CEIL, 0)
}
/// Retrieves first TerminalNode corresponding to token R_CEIL
/// Returns `None` if there is no child corresponding to token R_CEIL
fn R_CEIL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_CEIL, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> CeilContextAttrs<'input> for CeilContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ceil(&mut self,)
	-> Result<Rc<CeilContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CeilContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_ceil);
        let mut _localctx: Rc<CeilContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(439);
			recog.base.match_token(L_CEIL,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(440);
			let tmp = recog.expr()?;
			 cast_mut::<_,CeilContext >(&mut _localctx).val = Some(tmp.clone());
			  

			recog.base.set_state(441);
			recog.base.match_token(R_CEIL,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- var_sym ----------------
#[derive(Debug)]
pub enum Var_symContextAll<'input>{
	AtomVarSymContext(AtomVarSymContext<'input>),
Error(Var_symContext<'input>)
}
antlr_rust::tid!{Var_symContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Var_symContextAll<'input>{}

impl<'input> LaTeXParserContext<'input> for Var_symContextAll<'input>{}

impl<'input> Deref for Var_symContextAll<'input>{
	type Target = dyn Var_symContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use Var_symContextAll::*;
		match self{
			AtomVarSymContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Var_symContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Var_symContextAll<'input>{
    fn enter(&self, listener: &mut (dyn LaTeXListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn LaTeXListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type Var_symContext<'input> = BaseParserRuleContext<'input,Var_symContextExt<'input>>;

#[derive(Clone)]
pub struct Var_symContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Var_symContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Var_symContext<'input>{
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Var_symContext<'input>{
}

impl<'input> CustomRuleContext<'input> for Var_symContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_var_sym }
	//fn type_rule_index() -> usize where Self: Sized { RULE_var_sym }
}
antlr_rust::tid!{Var_symContextExt<'a>}

impl<'input> Var_symContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Var_symContextAll<'input>> {
		Rc::new(
		Var_symContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Var_symContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait Var_symContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Var_symContextExt<'input>>{


}

impl<'input> Var_symContextAttrs<'input> for Var_symContext<'input>{}

pub type AtomVarSymContext<'input> = BaseParserRuleContext<'input,AtomVarSymContextExt<'input>>;

pub trait AtomVarSymContextAttrs<'input>: LaTeXParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token VAR
	/// Returns `None` if there is no child corresponding to token VAR
	fn VAR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(VAR, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SYMBOL
	/// Returns `None` if there is no child corresponding to token SYMBOL
	fn SYMBOL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(SYMBOL, 0)
	}
}

impl<'input> AtomVarSymContextAttrs<'input> for AtomVarSymContext<'input>{}

pub struct AtomVarSymContextExt<'input>{
	base:Var_symContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomVarSymContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for AtomVarSymContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for AtomVarSymContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_atomVarSym(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_atomVarSym(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for AtomVarSymContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_atomVarSym(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomVarSymContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_var_sym }
	//fn type_rule_index() -> usize where Self: Sized { RULE_var_sym }
}

impl<'input> Borrow<Var_symContextExt<'input>> for AtomVarSymContext<'input>{
	fn borrow(&self) -> &Var_symContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Var_symContextExt<'input>> for AtomVarSymContext<'input>{
	fn borrow_mut(&mut self) -> &mut Var_symContextExt<'input> { &mut self.base }
}

impl<'input> Var_symContextAttrs<'input> for AtomVarSymContext<'input> {}

impl<'input> AtomVarSymContextExt<'input>{
	fn new(ctx: &dyn Var_symContextAttrs<'input>) -> Rc<Var_symContextAll<'input>>  {
		Rc::new(
			Var_symContextAll::AtomVarSymContext(
				BaseParserRuleContext::copy_from(ctx,AtomVarSymContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn var_sym(&mut self,)
	-> Result<Rc<Var_symContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Var_symContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_var_sym);
        let mut _localctx: Rc<Var_symContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let tmp = AtomVarSymContextExt::new(&**_localctx);
			recog.base.enter_outer_alt(Some(tmp.clone()), 1);
			_localctx = tmp;
			{
			recog.base.set_state(443);
			_la = recog.base.input.la(1);
			if { !(_la==VAR || _la==SYMBOL) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- func_normal ----------------
pub type Func_normalContextAll<'input> = Func_normalContext<'input>;


pub type Func_normalContext<'input> = BaseParserRuleContext<'input,Func_normalContextExt<'input>>;

#[derive(Clone)]
pub struct Func_normalContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Func_normalContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Func_normalContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_func_normal(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_func_normal(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Func_normalContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_func_normal(self);
	}
}

impl<'input> CustomRuleContext<'input> for Func_normalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_normal }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_normal }
}
antlr_rust::tid!{Func_normalContextExt<'a>}

impl<'input> Func_normalContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Func_normalContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_normalContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Func_normalContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Func_normalContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FUNC_EXP
/// Returns `None` if there is no child corresponding to token FUNC_EXP
fn FUNC_EXP(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_EXP, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_LOG
/// Returns `None` if there is no child corresponding to token FUNC_LOG
fn FUNC_LOG(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_LOG, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_LG
/// Returns `None` if there is no child corresponding to token FUNC_LG
fn FUNC_LG(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_LG, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_LN
/// Returns `None` if there is no child corresponding to token FUNC_LN
fn FUNC_LN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_LN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_SIN
/// Returns `None` if there is no child corresponding to token FUNC_SIN
fn FUNC_SIN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_SIN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_COS
/// Returns `None` if there is no child corresponding to token FUNC_COS
fn FUNC_COS(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_COS, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_TAN
/// Returns `None` if there is no child corresponding to token FUNC_TAN
fn FUNC_TAN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_TAN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_CSC
/// Returns `None` if there is no child corresponding to token FUNC_CSC
fn FUNC_CSC(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_CSC, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_SEC
/// Returns `None` if there is no child corresponding to token FUNC_SEC
fn FUNC_SEC(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_SEC, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_COT
/// Returns `None` if there is no child corresponding to token FUNC_COT
fn FUNC_COT(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_COT, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCSIN
/// Returns `None` if there is no child corresponding to token FUNC_ARCSIN
fn FUNC_ARCSIN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_ARCSIN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCCOS
/// Returns `None` if there is no child corresponding to token FUNC_ARCCOS
fn FUNC_ARCCOS(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_ARCCOS, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCTAN
/// Returns `None` if there is no child corresponding to token FUNC_ARCTAN
fn FUNC_ARCTAN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_ARCTAN, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCCSC
/// Returns `None` if there is no child corresponding to token FUNC_ARCCSC
fn FUNC_ARCCSC(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_ARCCSC, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCSEC
/// Returns `None` if there is no child corresponding to token FUNC_ARCSEC
fn FUNC_ARCSEC(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_ARCSEC, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCCOT
/// Returns `None` if there is no child corresponding to token FUNC_ARCCOT
fn FUNC_ARCCOT(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_ARCCOT, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_SINH
/// Returns `None` if there is no child corresponding to token FUNC_SINH
fn FUNC_SINH(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_SINH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_COSH
/// Returns `None` if there is no child corresponding to token FUNC_COSH
fn FUNC_COSH(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_COSH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_TANH
/// Returns `None` if there is no child corresponding to token FUNC_TANH
fn FUNC_TANH(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_TANH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARSINH
/// Returns `None` if there is no child corresponding to token FUNC_ARSINH
fn FUNC_ARSINH(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_ARSINH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARCOSH
/// Returns `None` if there is no child corresponding to token FUNC_ARCOSH
fn FUNC_ARCOSH(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_ARCOSH, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ARTANH
/// Returns `None` if there is no child corresponding to token FUNC_ARTANH
fn FUNC_ARTANH(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(FUNC_ARTANH, 0)
}

}

impl<'input> Func_normalContextAttrs<'input> for Func_normalContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn func_normal(&mut self,)
	-> Result<Rc<Func_normalContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_normalContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_func_normal);
        let mut _localctx: Rc<Func_normalContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(445);
			_la = recog.base.input.la(1);
			if { !(((((_la - 36)) & !0x3f) == 0 && ((1usize << (_la - 36)) & ((1usize << (FUNC_EXP - 36)) | (1usize << (FUNC_LOG - 36)) | (1usize << (FUNC_LG - 36)) | (1usize << (FUNC_LN - 36)) | (1usize << (FUNC_SIN - 36)) | (1usize << (FUNC_COS - 36)) | (1usize << (FUNC_TAN - 36)) | (1usize << (FUNC_CSC - 36)) | (1usize << (FUNC_SEC - 36)) | (1usize << (FUNC_COT - 36)) | (1usize << (FUNC_ARCSIN - 36)) | (1usize << (FUNC_ARCCOS - 36)) | (1usize << (FUNC_ARCTAN - 36)) | (1usize << (FUNC_ARCCSC - 36)) | (1usize << (FUNC_ARCSEC - 36)) | (1usize << (FUNC_ARCCOT - 36)) | (1usize << (FUNC_SINH - 36)) | (1usize << (FUNC_COSH - 36)) | (1usize << (FUNC_TANH - 36)) | (1usize << (FUNC_ARSINH - 36)) | (1usize << (FUNC_ARCOSH - 36)) | (1usize << (FUNC_ARTANH - 36)))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- func ----------------
#[derive(Debug)]
pub enum FuncContextAll<'input>{
	Fn_sum_prodContext(Fn_sum_prodContext<'input>),
	Fn_intContext(Fn_intContext<'input>),
	Fn_limitContext(Fn_limitContext<'input>),
	Fn_normalContext(Fn_normalContext<'input>),
	Fc_overlineContext(Fc_overlineContext<'input>),
	Fn_sqrtContext(Fn_sqrtContext<'input>),
	Fn_varContext(Fn_varContext<'input>),
	Fn_anonymContext(Fn_anonymContext<'input>),
Error(FuncContext<'input>)
}
antlr_rust::tid!{FuncContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for FuncContextAll<'input>{}

impl<'input> LaTeXParserContext<'input> for FuncContextAll<'input>{}

impl<'input> Deref for FuncContextAll<'input>{
	type Target = dyn FuncContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use FuncContextAll::*;
		match self{
			Fn_sum_prodContext(inner) => inner,
			Fn_intContext(inner) => inner,
			Fn_limitContext(inner) => inner,
			Fn_normalContext(inner) => inner,
			Fc_overlineContext(inner) => inner,
			Fn_sqrtContext(inner) => inner,
			Fn_varContext(inner) => inner,
			Fn_anonymContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for FuncContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for FuncContextAll<'input>{
    fn enter(&self, listener: &mut (dyn LaTeXListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn LaTeXListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type FuncContext<'input> = BaseParserRuleContext<'input,FuncContextExt<'input>>;

#[derive(Clone)]
pub struct FuncContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for FuncContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for FuncContext<'input>{
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for FuncContext<'input>{
}

impl<'input> CustomRuleContext<'input> for FuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}
antlr_rust::tid!{FuncContextExt<'a>}

impl<'input> FuncContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FuncContextAll<'input>> {
		Rc::new(
		FuncContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait FuncContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<FuncContextExt<'input>>{


}

impl<'input> FuncContextAttrs<'input> for FuncContext<'input>{}

pub type Fn_sum_prodContext<'input> = BaseParserRuleContext<'input,Fn_sum_prodContextExt<'input>>;

pub trait Fn_sum_prodContextAttrs<'input>: LaTeXParserContext<'input>{
	fn mp(&self) -> Option<Rc<MpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token FUNC_SUM
	/// Returns `None` if there is no child corresponding to token FUNC_SUM
	fn FUNC_SUM(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(FUNC_SUM, 0)
	}
	/// Retrieves first TerminalNode corresponding to token FUNC_PROD
	/// Returns `None` if there is no child corresponding to token FUNC_PROD
	fn FUNC_PROD(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(FUNC_PROD, 0)
	}
	fn subeq(&self) -> Option<Rc<SubeqContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn supexpr(&self) -> Option<Rc<SupexprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> Fn_sum_prodContextAttrs<'input> for Fn_sum_prodContext<'input>{}

pub struct Fn_sum_prodContextExt<'input>{
	base:FuncContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{Fn_sum_prodContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for Fn_sum_prodContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Fn_sum_prodContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fn_sum_prod(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_fn_sum_prod(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Fn_sum_prodContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_fn_sum_prod(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fn_sum_prodContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}

impl<'input> Borrow<FuncContextExt<'input>> for Fn_sum_prodContext<'input>{
	fn borrow(&self) -> &FuncContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncContextExt<'input>> for Fn_sum_prodContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncContextExt<'input> { &mut self.base }
}

impl<'input> FuncContextAttrs<'input> for Fn_sum_prodContext<'input> {}

impl<'input> Fn_sum_prodContextExt<'input>{
	fn new(ctx: &dyn FuncContextAttrs<'input>) -> Rc<FuncContextAll<'input>>  {
		Rc::new(
			FuncContextAll::Fn_sum_prodContext(
				BaseParserRuleContext::copy_from(ctx,Fn_sum_prodContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Fn_intContext<'input> = BaseParserRuleContext<'input,Fn_intContextExt<'input>>;

pub trait Fn_intContextAttrs<'input>: LaTeXParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FUNC_INT
	/// Returns `None` if there is no child corresponding to token FUNC_INT
	fn FUNC_INT(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(FUNC_INT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token DIFFERENTIAL
	/// Returns `None` if there is no child corresponding to token DIFFERENTIAL
	fn DIFFERENTIAL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(DIFFERENTIAL, 0)
	}
	fn frac(&self) -> Option<Rc<FracContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn additive(&self) -> Option<Rc<AdditiveContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn supexpr(&self) -> Option<Rc<SupexprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> Fn_intContextAttrs<'input> for Fn_intContext<'input>{}

pub struct Fn_intContextExt<'input>{
	base:FuncContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{Fn_intContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for Fn_intContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Fn_intContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fn_int(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_fn_int(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Fn_intContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_fn_int(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fn_intContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}

impl<'input> Borrow<FuncContextExt<'input>> for Fn_intContext<'input>{
	fn borrow(&self) -> &FuncContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncContextExt<'input>> for Fn_intContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncContextExt<'input> { &mut self.base }
}

impl<'input> FuncContextAttrs<'input> for Fn_intContext<'input> {}

impl<'input> Fn_intContextExt<'input>{
	fn new(ctx: &dyn FuncContextAttrs<'input>) -> Rc<FuncContextAll<'input>>  {
		Rc::new(
			FuncContextAll::Fn_intContext(
				BaseParserRuleContext::copy_from(ctx,Fn_intContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Fn_limitContext<'input> = BaseParserRuleContext<'input,Fn_limitContextExt<'input>>;

pub trait Fn_limitContextAttrs<'input>: LaTeXParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FUNC_LIM
	/// Returns `None` if there is no child corresponding to token FUNC_LIM
	fn FUNC_LIM(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(FUNC_LIM, 0)
	}
	fn limit_sub(&self) -> Option<Rc<Limit_subContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn mp(&self) -> Option<Rc<MpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> Fn_limitContextAttrs<'input> for Fn_limitContext<'input>{}

pub struct Fn_limitContextExt<'input>{
	base:FuncContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{Fn_limitContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for Fn_limitContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Fn_limitContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fn_limit(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_fn_limit(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Fn_limitContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_fn_limit(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fn_limitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}

impl<'input> Borrow<FuncContextExt<'input>> for Fn_limitContext<'input>{
	fn borrow(&self) -> &FuncContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncContextExt<'input>> for Fn_limitContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncContextExt<'input> { &mut self.base }
}

impl<'input> FuncContextAttrs<'input> for Fn_limitContext<'input> {}

impl<'input> Fn_limitContextExt<'input>{
	fn new(ctx: &dyn FuncContextAttrs<'input>) -> Rc<FuncContextAll<'input>>  {
		Rc::new(
			FuncContextAll::Fn_limitContext(
				BaseParserRuleContext::copy_from(ctx,Fn_limitContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Fn_normalContext<'input> = BaseParserRuleContext<'input,Fn_normalContextExt<'input>>;

pub trait Fn_normalContextAttrs<'input>: LaTeXParserContext<'input>{
	fn func_normal(&self) -> Option<Rc<Func_normalContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token L_PAREN
	/// Returns `None` if there is no child corresponding to token L_PAREN
	fn L_PAREN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(L_PAREN, 0)
	}
	fn func_arg(&self) -> Option<Rc<Func_argContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token R_PAREN
	/// Returns `None` if there is no child corresponding to token R_PAREN
	fn R_PAREN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(R_PAREN, 0)
	}
	fn func_arg_noparens(&self) -> Option<Rc<Func_arg_noparensContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn supexpr(&self) -> Option<Rc<SupexprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> Fn_normalContextAttrs<'input> for Fn_normalContext<'input>{}

pub struct Fn_normalContextExt<'input>{
	base:FuncContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{Fn_normalContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for Fn_normalContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Fn_normalContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fn_normal(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_fn_normal(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Fn_normalContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_fn_normal(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fn_normalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}

impl<'input> Borrow<FuncContextExt<'input>> for Fn_normalContext<'input>{
	fn borrow(&self) -> &FuncContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncContextExt<'input>> for Fn_normalContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncContextExt<'input> { &mut self.base }
}

impl<'input> FuncContextAttrs<'input> for Fn_normalContext<'input> {}

impl<'input> Fn_normalContextExt<'input>{
	fn new(ctx: &dyn FuncContextAttrs<'input>) -> Rc<FuncContextAll<'input>>  {
		Rc::new(
			FuncContextAll::Fn_normalContext(
				BaseParserRuleContext::copy_from(ctx,Fn_normalContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Fc_overlineContext<'input> = BaseParserRuleContext<'input,Fc_overlineContextExt<'input>>;

pub trait Fc_overlineContextAttrs<'input>: LaTeXParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FUNC_OVERLINE
	/// Returns `None` if there is no child corresponding to token FUNC_OVERLINE
	fn FUNC_OVERLINE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(FUNC_OVERLINE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token L_BRACE
	/// Returns `None` if there is no child corresponding to token L_BRACE
	fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(L_BRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token R_BRACE
	/// Returns `None` if there is no child corresponding to token R_BRACE
	fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(R_BRACE, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> Fc_overlineContextAttrs<'input> for Fc_overlineContext<'input>{}

pub struct Fc_overlineContextExt<'input>{
	base:FuncContextExt<'input>,
	pub olbase: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{Fc_overlineContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for Fc_overlineContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Fc_overlineContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fc_overline(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_fc_overline(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Fc_overlineContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_fc_overline(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fc_overlineContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}

impl<'input> Borrow<FuncContextExt<'input>> for Fc_overlineContext<'input>{
	fn borrow(&self) -> &FuncContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncContextExt<'input>> for Fc_overlineContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncContextExt<'input> { &mut self.base }
}

impl<'input> FuncContextAttrs<'input> for Fc_overlineContext<'input> {}

impl<'input> Fc_overlineContextExt<'input>{
	fn new(ctx: &dyn FuncContextAttrs<'input>) -> Rc<FuncContextAll<'input>>  {
		Rc::new(
			FuncContextAll::Fc_overlineContext(
				BaseParserRuleContext::copy_from(ctx,Fc_overlineContextExt{
        			olbase:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Fn_sqrtContext<'input> = BaseParserRuleContext<'input,Fn_sqrtContextExt<'input>>;

pub trait Fn_sqrtContextAttrs<'input>: LaTeXParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FUNC_SQRT
	/// Returns `None` if there is no child corresponding to token FUNC_SQRT
	fn FUNC_SQRT(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(FUNC_SQRT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token L_BRACE
	/// Returns `None` if there is no child corresponding to token L_BRACE
	fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(L_BRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token R_BRACE
	/// Returns `None` if there is no child corresponding to token R_BRACE
	fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(R_BRACE, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token L_BRACKET
	/// Returns `None` if there is no child corresponding to token L_BRACKET
	fn L_BRACKET(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(L_BRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token R_BRACKET
	/// Returns `None` if there is no child corresponding to token R_BRACKET
	fn R_BRACKET(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(R_BRACKET, 0)
	}
}

impl<'input> Fn_sqrtContextAttrs<'input> for Fn_sqrtContext<'input>{}

pub struct Fn_sqrtContextExt<'input>{
	base:FuncContextExt<'input>,
	pub root: Option<Rc<ExprContextAll<'input>>>,
	pub sqrbase: Option<Rc<ExprContextAll<'input>>>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{Fn_sqrtContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for Fn_sqrtContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Fn_sqrtContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fn_sqrt(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_fn_sqrt(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Fn_sqrtContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_fn_sqrt(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fn_sqrtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}

impl<'input> Borrow<FuncContextExt<'input>> for Fn_sqrtContext<'input>{
	fn borrow(&self) -> &FuncContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncContextExt<'input>> for Fn_sqrtContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncContextExt<'input> { &mut self.base }
}

impl<'input> FuncContextAttrs<'input> for Fn_sqrtContext<'input> {}

impl<'input> Fn_sqrtContextExt<'input>{
	fn new(ctx: &dyn FuncContextAttrs<'input>) -> Rc<FuncContextAll<'input>>  {
		Rc::new(
			FuncContextAll::Fn_sqrtContext(
				BaseParserRuleContext::copy_from(ctx,Fn_sqrtContextExt{
        			root:None, sqrbase:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Fn_varContext<'input> = BaseParserRuleContext<'input,Fn_varContextExt<'input>>;

pub trait Fn_varContextAttrs<'input>: LaTeXParserContext<'input>{
	fn var_sym(&self) -> Option<Rc<Var_symContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn subexpr(&self) -> Option<Rc<SubexprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SINGLE_QUOTES
	/// Returns `None` if there is no child corresponding to token SINGLE_QUOTES
	fn SINGLE_QUOTES(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(SINGLE_QUOTES, 0)
	}
}

impl<'input> Fn_varContextAttrs<'input> for Fn_varContext<'input>{}

pub struct Fn_varContextExt<'input>{
	base:FuncContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{Fn_varContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for Fn_varContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Fn_varContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fn_var(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_fn_var(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Fn_varContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_fn_var(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fn_varContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}

impl<'input> Borrow<FuncContextExt<'input>> for Fn_varContext<'input>{
	fn borrow(&self) -> &FuncContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncContextExt<'input>> for Fn_varContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncContextExt<'input> { &mut self.base }
}

impl<'input> FuncContextAttrs<'input> for Fn_varContext<'input> {}

impl<'input> Fn_varContextExt<'input>{
	fn new(ctx: &dyn FuncContextAttrs<'input>) -> Rc<FuncContextAll<'input>>  {
		Rc::new(
			FuncContextAll::Fn_varContext(
				BaseParserRuleContext::copy_from(ctx,Fn_varContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type Fn_anonymContext<'input> = BaseParserRuleContext<'input,Fn_anonymContextExt<'input>>;

pub trait Fn_anonymContextAttrs<'input>: LaTeXParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token L_PAREN
	/// Returns `None` if there is no child corresponding to token L_PAREN
	fn L_PAREN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(L_PAREN, 0)
	}
	fn args(&self) -> Option<Rc<ArgsContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token R_PAREN
	/// Returns `None` if there is no child corresponding to token R_PAREN
	fn R_PAREN(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
		self.get_token(R_PAREN, 0)
	}
}

impl<'input> Fn_anonymContextAttrs<'input> for Fn_anonymContext<'input>{}

pub struct Fn_anonymContextExt<'input>{
	base:FuncContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{Fn_anonymContextExt<'a>}

impl<'input> LaTeXParserContext<'input> for Fn_anonymContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Fn_anonymContext<'input>{
	fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fn_anonym(self);
	}
	fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
		listener.exit_fn_anonym(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Fn_anonymContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_fn_anonym(self);
	}
}

impl<'input> CustomRuleContext<'input> for Fn_anonymContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func }
}

impl<'input> Borrow<FuncContextExt<'input>> for Fn_anonymContext<'input>{
	fn borrow(&self) -> &FuncContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<FuncContextExt<'input>> for Fn_anonymContext<'input>{
	fn borrow_mut(&mut self) -> &mut FuncContextExt<'input> { &mut self.base }
}

impl<'input> FuncContextAttrs<'input> for Fn_anonymContext<'input> {}

impl<'input> Fn_anonymContextExt<'input>{
	fn new(ctx: &dyn FuncContextAttrs<'input>) -> Rc<FuncContextAll<'input>>  {
		Rc::new(
			FuncContextAll::Fn_anonymContext(
				BaseParserRuleContext::copy_from(ctx,Fn_anonymContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn func(&mut self,)
	-> Result<Rc<FuncContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_func);
        let mut _localctx: Rc<FuncContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(536);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 FUNC_EXP | FUNC_LOG | FUNC_LG | FUNC_LN | FUNC_SIN | FUNC_COS | FUNC_TAN |
			 FUNC_CSC | FUNC_SEC | FUNC_COT | FUNC_ARCSIN | FUNC_ARCCOS | FUNC_ARCTAN |
			 FUNC_ARCCSC | FUNC_ARCSEC | FUNC_ARCCOT | FUNC_SINH | FUNC_COSH | FUNC_TANH |
			 FUNC_ARSINH | FUNC_ARCOSH | FUNC_ARTANH 
				=> {
					let tmp = Fn_normalContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule func_normal*/
					recog.base.set_state(447);
					recog.func_normal()?;

					recog.base.set_state(460);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(50,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(449);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==UNDERSCORE {
								{
								/*InvokeRule subexpr*/
								recog.base.set_state(448);
								recog.subexpr()?;

								}
							}

							recog.base.set_state(452);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==CARET {
								{
								/*InvokeRule supexpr*/
								recog.base.set_state(451);
								recog.supexpr()?;

								}
							}

							}
						}
					,
						2 =>{
							{
							recog.base.set_state(455);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==CARET {
								{
								/*InvokeRule supexpr*/
								recog.base.set_state(454);
								recog.supexpr()?;

								}
							}

							recog.base.set_state(458);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if _la==UNDERSCORE {
								{
								/*InvokeRule subexpr*/
								recog.base.set_state(457);
								recog.subexpr()?;

								}
							}

							}
						}

						_ => {}
					}
					recog.base.set_state(467);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(51,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(462);
							recog.base.match_token(L_PAREN,&mut recog.err_handler)?;

							/*InvokeRule func_arg*/
							recog.base.set_state(463);
							recog.func_arg()?;

							recog.base.set_state(464);
							recog.base.match_token(R_PAREN,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule func_arg_noparens*/
							recog.base.set_state(466);
							recog.func_arg_noparens()?;

							}
						}

						_ => {}
					}
					}
				}

			 VAR | SYMBOL 
				=> {
					let tmp = Fn_varContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule var_sym*/
					recog.base.set_state(469);
					recog.var_sym()?;

					recog.base.set_state(482);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(56,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(471);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(52,&mut recog.base)? {
								x if x == 1=>{
									{
									/*InvokeRule subexpr*/
									recog.base.set_state(470);
									recog.subexpr()?;

									}
								}

								_ => {}
							}
							recog.base.set_state(474);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(53,&mut recog.base)? {
								x if x == 1=>{
									{
									recog.base.set_state(473);
									recog.base.match_token(SINGLE_QUOTES,&mut recog.err_handler)?;

									}
								}

								_ => {}
							}
							}
						}
					,
						2 =>{
							{
							recog.base.set_state(477);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(54,&mut recog.base)? {
								x if x == 1=>{
									{
									recog.base.set_state(476);
									recog.base.match_token(SINGLE_QUOTES,&mut recog.err_handler)?;

									}
								}

								_ => {}
							}
							recog.base.set_state(480);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(55,&mut recog.base)? {
								x if x == 1=>{
									{
									/*InvokeRule subexpr*/
									recog.base.set_state(479);
									recog.subexpr()?;

									}
								}

								_ => {}
							}
							}
						}

						_ => {}
					}
					}
				}

			 L_PAREN 
				=> {
					let tmp = Fn_anonymContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(484);
					recog.base.match_token(L_PAREN,&mut recog.err_handler)?;

					/*InvokeRule args*/
					recog.base.set_state(485);
					recog.args()?;

					recog.base.set_state(486);
					recog.base.match_token(R_PAREN,&mut recog.err_handler)?;

					}
				}

			 FUNC_INT 
				=> {
					let tmp = Fn_intContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(488);
					recog.base.match_token(FUNC_INT,&mut recog.err_handler)?;

					recog.base.set_state(495);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 UNDERSCORE 
						=> {
					    	{
					    	/*InvokeRule subexpr*/
					    	recog.base.set_state(489);
					    	recog.subexpr()?;

					    	/*InvokeRule supexpr*/
					    	recog.base.set_state(490);
					    	recog.supexpr()?;

					    	}
					    }

					 CARET 
						=> {
					    	{
					    	/*InvokeRule supexpr*/
					    	recog.base.set_state(492);
					    	recog.supexpr()?;

					    	/*InvokeRule subexpr*/
					    	recog.base.set_state(493);
					    	recog.subexpr()?;

					    	}
					    }

					 ADD | SUB | L_PAREN | L_BRACE | L_BRACE_LITERAL | L_BRACKET | BAR |
					 L_BAR | L_ANGLE | FUNC_LIM | FUNC_INT | FUNC_SUM | FUNC_PROD | FUNC_EXP |
					 FUNC_LOG | FUNC_LG | FUNC_LN | FUNC_SIN | FUNC_COS | FUNC_TAN | FUNC_CSC |
					 FUNC_SEC | FUNC_COT | FUNC_ARCSIN | FUNC_ARCCOS | FUNC_ARCTAN | FUNC_ARCCSC |
					 FUNC_ARCSEC | FUNC_ARCCOT | FUNC_SINH | FUNC_COSH | FUNC_TANH | FUNC_ARSINH |
					 FUNC_ARCOSH | FUNC_ARTANH | L_FLOOR | L_CEIL | FUNC_SQRT | FUNC_OVERLINE |
					 CMD_FRAC | CMD_BINOM | CMD_DBINOM | CMD_TBINOM | CMD_MATHIT | DIFFERENTIAL |
					 DIGIT | VAR | SYMBOL 
						=> {
					    }

						_ => {}
					}
					recog.base.set_state(503);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(59,&mut recog.base)? {
						1 =>{
							{
							recog.base.set_state(498);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(58,&mut recog.base)? {
								x if x == 1=>{
									{
									/*InvokeRule additive*/
									recog.base.set_state(497);
									recog.additive_rec(0)?;

									}
								}

								_ => {}
							}
							recog.base.set_state(500);
							recog.base.match_token(DIFFERENTIAL,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*InvokeRule frac*/
							recog.base.set_state(501);
							recog.frac()?;

							}
						}
					,
						3 =>{
							{
							/*InvokeRule additive*/
							recog.base.set_state(502);
							recog.additive_rec(0)?;

							}
						}

						_ => {}
					}
					}
				}

			 FUNC_SQRT 
				=> {
					let tmp = Fn_sqrtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(505);
					recog.base.match_token(FUNC_SQRT,&mut recog.err_handler)?;

					recog.base.set_state(510);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==L_BRACKET {
						{
						recog.base.set_state(506);
						recog.base.match_token(L_BRACKET,&mut recog.err_handler)?;

						/*InvokeRule expr*/
						recog.base.set_state(507);
						let tmp = recog.expr()?;
						if let FuncContextAll::Fn_sqrtContext(ctx) = cast_mut::<_,FuncContextAll >(&mut _localctx){
						ctx.root = Some(tmp.clone()); } else {unreachable!("cant cast");}  

						recog.base.set_state(508);
						recog.base.match_token(R_BRACKET,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(512);
					recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(513);
					let tmp = recog.expr()?;
					if let FuncContextAll::Fn_sqrtContext(ctx) = cast_mut::<_,FuncContextAll >(&mut _localctx){
					ctx.sqrbase = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(514);
					recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

					}
				}

			 FUNC_OVERLINE 
				=> {
					let tmp = Fc_overlineContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(516);
					recog.base.match_token(FUNC_OVERLINE,&mut recog.err_handler)?;

					recog.base.set_state(517);
					recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(518);
					let tmp = recog.expr()?;
					if let FuncContextAll::Fc_overlineContext(ctx) = cast_mut::<_,FuncContextAll >(&mut _localctx){
					ctx.olbase = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(519);
					recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

					}
				}

			 FUNC_SUM | FUNC_PROD 
				=> {
					let tmp = Fn_sum_prodContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(521);
					_la = recog.base.input.la(1);
					if { !(_la==FUNC_SUM || _la==FUNC_PROD) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					recog.base.set_state(528);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 UNDERSCORE 
						=> {
							{
							/*InvokeRule subeq*/
							recog.base.set_state(522);
							recog.subeq()?;

							/*InvokeRule supexpr*/
							recog.base.set_state(523);
							recog.supexpr()?;

							}
						}

					 CARET 
						=> {
							{
							/*InvokeRule supexpr*/
							recog.base.set_state(525);
							recog.supexpr()?;

							/*InvokeRule subeq*/
							recog.base.set_state(526);
							recog.subeq()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					/*InvokeRule mp*/
					recog.base.set_state(530);
					recog.mp_rec(0)?;

					}
				}

			 FUNC_LIM 
				=> {
					let tmp = Fn_limitContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(532);
					recog.base.match_token(FUNC_LIM,&mut recog.err_handler)?;

					/*InvokeRule limit_sub*/
					recog.base.set_state(533);
					recog.limit_sub()?;

					/*InvokeRule mp*/
					recog.base.set_state(534);
					recog.mp_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- args ----------------
pub type ArgsContextAll<'input> = ArgsContext<'input>;


pub type ArgsContext<'input> = BaseParserRuleContext<'input,ArgsContextExt<'input>>;

#[derive(Clone)]
pub struct ArgsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for ArgsContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for ArgsContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_args(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_args(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for ArgsContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_args(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArgsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_args }
	//fn type_rule_index() -> usize where Self: Sized { RULE_args }
}
antlr_rust::tid!{ArgsContextExt<'a>}

impl<'input> ArgsContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArgsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArgsContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<ArgsContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn args(&self) -> Option<Rc<ArgsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ArgsContextAttrs<'input> for ArgsContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn args(&mut self,)
	-> Result<Rc<ArgsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 78, RULE_args);
        let mut _localctx: Rc<ArgsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(543);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(63,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					{
					/*InvokeRule expr*/
					recog.base.set_state(538);
					recog.expr()?;

					recog.base.set_state(539);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule args*/
					recog.base.set_state(540);
					recog.args()?;

					}
					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expr*/
					recog.base.set_state(542);
					recog.expr()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- limit_sub ----------------
pub type Limit_subContextAll<'input> = Limit_subContext<'input>;


pub type Limit_subContext<'input> = BaseParserRuleContext<'input,Limit_subContextExt<'input>>;

#[derive(Clone)]
pub struct Limit_subContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Limit_subContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Limit_subContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_limit_sub(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_limit_sub(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Limit_subContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_limit_sub(self);
	}
}

impl<'input> CustomRuleContext<'input> for Limit_subContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_limit_sub }
	//fn type_rule_index() -> usize where Self: Sized { RULE_limit_sub }
}
antlr_rust::tid!{Limit_subContextExt<'a>}

impl<'input> Limit_subContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Limit_subContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Limit_subContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Limit_subContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Limit_subContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(UNDERSCORE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token L_BRACE in current rule
fn L_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token L_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token L_BRACE is less or equal than `i`.
fn L_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, i)
}
/// Retrieves first TerminalNode corresponding to token LIM_APPROACH_SYM
/// Returns `None` if there is no child corresponding to token LIM_APPROACH_SYM
fn LIM_APPROACH_SYM(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(LIM_APPROACH_SYM, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token R_BRACE in current rule
fn R_BRACE_all(&self) -> Vec<Rc<TerminalNode<'input,LaTeXParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token R_BRACE, starting from 0.
/// Returns `None` if number of children corresponding to token R_BRACE is less or equal than `i`.
fn R_BRACE(&self, i: usize) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, i)
}
/// Retrieves first TerminalNode corresponding to token VAR
/// Returns `None` if there is no child corresponding to token VAR
fn VAR(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(VAR, 0)
}
/// Retrieves first TerminalNode corresponding to token SYMBOL
/// Returns `None` if there is no child corresponding to token SYMBOL
fn SYMBOL(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(SYMBOL, 0)
}
/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CARET, 0)
}
fn sumop(&self) -> Option<Rc<SumopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Limit_subContextAttrs<'input> for Limit_subContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn limit_sub(&mut self,)
	-> Result<Rc<Limit_subContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Limit_subContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 80, RULE_limit_sub);
        let mut _localctx: Rc<Limit_subContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(545);
			recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(546);
			recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

			recog.base.set_state(547);
			_la = recog.base.input.la(1);
			if { !(_la==VAR || _la==SYMBOL) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			recog.base.set_state(548);
			recog.base.match_token(LIM_APPROACH_SYM,&mut recog.err_handler)?;

			/*InvokeRule expr*/
			recog.base.set_state(549);
			recog.expr()?;

			recog.base.set_state(558);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==CARET {
				{
				recog.base.set_state(550);
				recog.base.match_token(CARET,&mut recog.err_handler)?;

				recog.base.set_state(556);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				 L_BRACE 
					=> {
						{
						{
						recog.base.set_state(551);
						recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

						/*InvokeRule sumop*/
						recog.base.set_state(552);
						recog.sumop()?;

						recog.base.set_state(553);
						recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

						}
						}
					}

				 ADD | SUB 
					=> {
						{
						/*InvokeRule sumop*/
						recog.base.set_state(555);
						recog.sumop()?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
			}

			recog.base.set_state(560);
			recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- func_arg ----------------
pub type Func_argContextAll<'input> = Func_argContext<'input>;


pub type Func_argContext<'input> = BaseParserRuleContext<'input,Func_argContextExt<'input>>;

#[derive(Clone)]
pub struct Func_argContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Func_argContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Func_argContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_func_arg(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_func_arg(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Func_argContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_func_arg(self);
	}
}

impl<'input> CustomRuleContext<'input> for Func_argContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_arg }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_arg }
}
antlr_rust::tid!{Func_argContextExt<'a>}

impl<'input> Func_argContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Func_argContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_argContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Func_argContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Func_argContextExt<'input>>{

fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn func_arg(&self) -> Option<Rc<Func_argContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_argContextAttrs<'input> for Func_argContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn func_arg(&mut self,)
	-> Result<Rc<Func_argContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_argContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 82, RULE_func_arg);
        let mut _localctx: Rc<Func_argContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(567);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(66,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expr*/
					recog.base.set_state(562);
					recog.expr()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					{
					/*InvokeRule expr*/
					recog.base.set_state(563);
					recog.expr()?;

					recog.base.set_state(564);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule func_arg*/
					recog.base.set_state(565);
					recog.func_arg()?;

					}
					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- func_arg_noparens ----------------
pub type Func_arg_noparensContextAll<'input> = Func_arg_noparensContext<'input>;


pub type Func_arg_noparensContext<'input> = BaseParserRuleContext<'input,Func_arg_noparensContextExt<'input>>;

#[derive(Clone)]
pub struct Func_arg_noparensContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for Func_arg_noparensContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for Func_arg_noparensContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_func_arg_noparens(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_func_arg_noparens(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for Func_arg_noparensContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_func_arg_noparens(self);
	}
}

impl<'input> CustomRuleContext<'input> for Func_arg_noparensContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_arg_noparens }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_arg_noparens }
}
antlr_rust::tid!{Func_arg_noparensContextExt<'a>}

impl<'input> Func_arg_noparensContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Func_arg_noparensContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_arg_noparensContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Func_arg_noparensContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<Func_arg_noparensContextExt<'input>>{

fn mp_nofunc(&self) -> Option<Rc<Mp_nofuncContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_arg_noparensContextAttrs<'input> for Func_arg_noparensContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn func_arg_noparens(&mut self,)
	-> Result<Rc<Func_arg_noparensContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_arg_noparensContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 84, RULE_func_arg_noparens);
        let mut _localctx: Rc<Func_arg_noparensContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule mp_nofunc*/
			recog.base.set_state(569);
			recog.mp_nofunc_rec(0)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- subexpr ----------------
pub type SubexprContextAll<'input> = SubexprContext<'input>;


pub type SubexprContext<'input> = BaseParserRuleContext<'input,SubexprContextExt<'input>>;

#[derive(Clone)]
pub struct SubexprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for SubexprContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for SubexprContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_subexpr(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_subexpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for SubexprContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_subexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subexpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subexpr }
}
antlr_rust::tid!{SubexprContextExt<'a>}

impl<'input> SubexprContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SubexprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubexprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SubexprContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<SubexprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(UNDERSCORE, 0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}

}

impl<'input> SubexprContextAttrs<'input> for SubexprContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn subexpr(&mut self,)
	-> Result<Rc<SubexprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubexprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 86, RULE_subexpr);
        let mut _localctx: Rc<SubexprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(571);
			recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(577);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BAR | L_BAR | L_ANGLE | CMD_FRAC | CMD_BINOM | CMD_DBINOM | CMD_TBINOM |
			 CMD_MATHIT | DIFFERENTIAL | DIGIT | VAR | SYMBOL 
				=> {
					{
					/*InvokeRule atom*/
					recog.base.set_state(572);
					recog.atom()?;

					}
				}

			 L_BRACE 
				=> {
					{
					recog.base.set_state(573);
					recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(574);
					recog.expr()?;

					recog.base.set_state(575);
					recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- supexpr ----------------
pub type SupexprContextAll<'input> = SupexprContext<'input>;


pub type SupexprContext<'input> = BaseParserRuleContext<'input,SupexprContextExt<'input>>;

#[derive(Clone)]
pub struct SupexprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for SupexprContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for SupexprContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_supexpr(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_supexpr(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for SupexprContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_supexpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for SupexprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_supexpr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_supexpr }
}
antlr_rust::tid!{SupexprContextExt<'a>}

impl<'input> SupexprContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SupexprContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SupexprContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SupexprContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<SupexprContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CARET
/// Returns `None` if there is no child corresponding to token CARET
fn CARET(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(CARET, 0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}

}

impl<'input> SupexprContextAttrs<'input> for SupexprContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn supexpr(&mut self,)
	-> Result<Rc<SupexprContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SupexprContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 88, RULE_supexpr);
        let mut _localctx: Rc<SupexprContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(579);
			recog.base.match_token(CARET,&mut recog.err_handler)?;

			recog.base.set_state(585);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BAR | L_BAR | L_ANGLE | CMD_FRAC | CMD_BINOM | CMD_DBINOM | CMD_TBINOM |
			 CMD_MATHIT | DIFFERENTIAL | DIGIT | VAR | SYMBOL 
				=> {
					{
					/*InvokeRule atom*/
					recog.base.set_state(580);
					recog.atom()?;

					}
				}

			 L_BRACE 
				=> {
					{
					recog.base.set_state(581);
					recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(582);
					recog.expr()?;

					recog.base.set_state(583);
					recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- subeq ----------------
pub type SubeqContextAll<'input> = SubeqContext<'input>;


pub type SubeqContext<'input> = BaseParserRuleContext<'input,SubeqContextExt<'input>>;

#[derive(Clone)]
pub struct SubeqContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for SubeqContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for SubeqContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_subeq(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_subeq(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for SubeqContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_subeq(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubeqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_subeq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_subeq }
}
antlr_rust::tid!{SubeqContextExt<'a>}

impl<'input> SubeqContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SubeqContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SubeqContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SubeqContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<SubeqContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(UNDERSCORE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
fn equality(&self) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}

}

impl<'input> SubeqContextAttrs<'input> for SubeqContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn subeq(&mut self,)
	-> Result<Rc<SubeqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SubeqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 90, RULE_subeq);
        let mut _localctx: Rc<SubeqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(587);
			recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(588);
			recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule equality*/
			recog.base.set_state(589);
			recog.equality()?;

			recog.base.set_state(590);
			recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- supeq ----------------
pub type SupeqContextAll<'input> = SupeqContext<'input>;


pub type SupeqContext<'input> = BaseParserRuleContext<'input,SupeqContextExt<'input>>;

#[derive(Clone)]
pub struct SupeqContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> LaTeXParserContext<'input> for SupeqContext<'input>{}

impl<'input,'a> Listenable<dyn LaTeXListener<'input> + 'a> for SupeqContext<'input>{
		fn enter(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_supeq(self);
		}
		fn exit(&self,listener: &mut (dyn LaTeXListener<'input> + 'a)) {
			listener.exit_supeq(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn LaTeXVisitor<'input> + 'a> for SupeqContext<'input>{
	fn accept(&self,visitor: &mut (dyn LaTeXVisitor<'input> + 'a)) {
		visitor.visit_supeq(self);
	}
}

impl<'input> CustomRuleContext<'input> for SupeqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = LaTeXParserContextType;
	fn get_rule_index(&self) -> usize { RULE_supeq }
	//fn type_rule_index() -> usize where Self: Sized { RULE_supeq }
}
antlr_rust::tid!{SupeqContextExt<'a>}

impl<'input> SupeqContextExt<'input>{
	fn new(parent: Option<Rc<dyn LaTeXParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SupeqContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SupeqContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SupeqContextAttrs<'input>: LaTeXParserContext<'input> + BorrowMut<SupeqContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token UNDERSCORE
/// Returns `None` if there is no child corresponding to token UNDERSCORE
fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(UNDERSCORE, 0)
}
/// Retrieves first TerminalNode corresponding to token L_BRACE
/// Returns `None` if there is no child corresponding to token L_BRACE
fn L_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(L_BRACE, 0)
}
fn equality(&self) -> Option<Rc<EqualityContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token R_BRACE
/// Returns `None` if there is no child corresponding to token R_BRACE
fn R_BRACE(&self) -> Option<Rc<TerminalNode<'input,LaTeXParserContextType>>> where Self:Sized{
	self.get_token(R_BRACE, 0)
}

}

impl<'input> SupeqContextAttrs<'input> for SupeqContext<'input>{}

impl<'input, I, H> LaTeXParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn supeq(&mut self,)
	-> Result<Rc<SupeqContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SupeqContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 92, RULE_supeq);
        let mut _localctx: Rc<SupeqContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(592);
			recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

			recog.base.set_state(593);
			recog.base.match_token(L_BRACE,&mut recog.err_handler)?;

			/*InvokeRule equality*/
			recog.base.set_state(594);
			recog.equality()?;

			recog.base.set_state(595);
			recog.base.match_token(R_BRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x60\u{258}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x04\x29\x09\
	\x29\x04\x2a\x09\x2a\x04\x2b\x09\x2b\x04\x2c\x09\x2c\x04\x2d\x09\x2d\x04\
	\x2e\x09\x2e\x04\x2f\x09\x2f\x04\x30\x09\x30\x03\x02\x03\x02\x05\x02\x63\
	\x0a\x02\x03\x02\x03\x02\x03\x02\x05\x02\x68\x0a\x02\x07\x02\x6a\x0a\x02\
	\x0c\x02\x0e\x02\x6d\x0b\x02\x03\x02\x07\x02\x70\x0a\x02\x0c\x02\x0e\x02\
	\x73\x0b\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x05\x02\x7b\
	\x0a\x02\x03\x02\x03\x02\x03\x02\x05\x02\u{80}\x0a\x02\x07\x02\u{82}\x0a\
	\x02\x0c\x02\x0e\x02\u{85}\x0b\x02\x03\x02\x03\x02\x07\x02\u{89}\x0a\x02\
	\x0c\x02\x0e\x02\u{8c}\x0b\x02\x03\x02\x03\x02\x05\x02\u{90}\x0a\x02\x03\
	\x03\x03\x03\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x03\x04\x07\
	\x04\u{9b}\x0a\x04\x0c\x04\x0e\x04\u{9e}\x0b\x04\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x06\x03\x06\x03\x07\x03\x07\x03\x08\x03\x08\x03\x09\x03\x09\
	\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x07\x0b\u{b5}\x0a\x0b\x0c\x0b\x0e\x0b\u{b8}\x0b\x0b\x03\x0c\x03\x0c\x03\
	\x0c\x05\x0c\u{bd}\x0a\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\u{c3}\
	\x0a\x0c\x0c\x0c\x0e\x0c\u{c6}\x0b\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\
	\x0d\x03\x0d\x03\x0d\x07\x0d\u{cf}\x0a\x0d\x0c\x0d\x0e\x0d\u{d2}\x0b\x0d\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x06\x0e\u{d8}\x0a\x0e\x0d\x0e\x0e\x0e\u{d9}\
	\x05\x0e\u{dc}\x0a\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{e3}\
	\x0a\x0f\x0c\x0f\x0e\x0f\u{e6}\x0b\x0f\x05\x0f\u{e8}\x0a\x0f\x03\x10\x03\
	\x10\x07\x10\u{ec}\x0a\x10\x0c\x10\x0e\x10\u{ef}\x0b\x10\x03\x11\x03\x11\
	\x07\x11\u{f3}\x0a\x11\x0c\x11\x0e\x11\u{f6}\x0b\x11\x03\x12\x03\x12\x05\
	\x12\u{fa}\x0a\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x05\x13\
	\u{102}\x0a\x13\x03\x14\x03\x14\x03\x14\x03\x14\x05\x14\u{108}\x0a\x14\x03\
	\x14\x03\x14\x03\x15\x03\x15\x03\x15\x03\x15\x05\x15\u{110}\x0a\x15\x03\
	\x15\x03\x15\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\
	\x16\x03\x16\x03\x16\x05\x16\u{11e}\x0a\x16\x03\x16\x05\x16\u{121}\x0a\x16\
	\x07\x16\u{123}\x0a\x16\x0c\x16\x0e\x16\u{126}\x0b\x16\x03\x17\x03\x17\x03\
	\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x05\x17\u{132}\
	\x0a\x17\x03\x17\x05\x17\u{135}\x0a\x17\x07\x17\u{137}\x0a\x17\x0c\x17\x0e\
	\x17\u{13a}\x0b\x17\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x03\x18\x05\
	\x18\u{142}\x0a\x18\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x05\x19\u{149}\
	\x0a\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\
	\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x05\x1a\
	\u{15b}\x0a\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1c\x06\x1c\u{162}\x0a\
	\x1c\x0d\x1c\x0e\x1c\u{163}\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x07\x1c\u{16a}\
	\x0a\x1c\x0c\x1c\x0e\x1c\u{16d}\x0b\x1c\x03\x1c\x03\x1c\x06\x1c\u{171}\x0a\
	\x1c\x0d\x1c\x0e\x1c\u{172}\x05\x1c\u{175}\x0a\x1c\x03\x1d\x03\x1d\x05\x1d\
	\u{179}\x0a\x1d\x03\x1d\x05\x1d\u{17c}\x0a\x1d\x03\x1d\x05\x1d\u{17f}\x0a\
	\x1d\x03\x1d\x05\x1d\u{182}\x0a\x1d\x05\x1d\u{184}\x0a\x1d\x03\x1d\x03\x1d\
	\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x05\x1d\u{18d}\x0a\x1d\x03\x1e\
	\x03\x1e\x03\x1e\x03\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x03\x20\x03\x20\
	\x03\x20\x03\x20\x03\x20\x03\x21\x05\x21\u{19d}\x0a\x21\x03\x22\x03\x22\
	\x03\x22\x03\x22\x03\x22\x03\x22\x05\x22\u{1a5}\x0a\x22\x03\x22\x03\x22\
	\x03\x22\x03\x22\x03\x22\x05\x22\u{1ac}\x0a\x22\x03\x23\x03\x23\x03\x23\
	\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x03\x24\x03\x24\x03\x24\x03\x24\
	\x03\x25\x03\x25\x03\x25\x03\x25\x03\x26\x03\x26\x03\x27\x03\x27\x03\x28\
	\x03\x28\x05\x28\u{1c4}\x0a\x28\x03\x28\x05\x28\u{1c7}\x0a\x28\x03\x28\x05\
	\x28\u{1ca}\x0a\x28\x03\x28\x05\x28\u{1cd}\x0a\x28\x05\x28\u{1cf}\x0a\x28\
	\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x05\x28\u{1d6}\x0a\x28\x03\x28\
	\x03\x28\x05\x28\u{1da}\x0a\x28\x03\x28\x05\x28\u{1dd}\x0a\x28\x03\x28\x05\
	\x28\u{1e0}\x0a\x28\x03\x28\x05\x28\u{1e3}\x0a\x28\x05\x28\u{1e5}\x0a\x28\
	\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\
	\x03\x28\x03\x28\x05\x28\u{1f2}\x0a\x28\x03\x28\x05\x28\u{1f5}\x0a\x28\x03\
	\x28\x03\x28\x03\x28\x05\x28\u{1fa}\x0a\x28\x03\x28\x03\x28\x03\x28\x03\
	\x28\x03\x28\x05\x28\u{201}\x0a\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\
	\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\x28\x03\
	\x28\x03\x28\x03\x28\x05\x28\u{213}\x0a\x28\x03\x28\x03\x28\x03\x28\x03\
	\x28\x03\x28\x03\x28\x05\x28\u{21b}\x0a\x28\x03\x29\x03\x29\x03\x29\x03\
	\x29\x03\x29\x05\x29\u{222}\x0a\x29\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\
	\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x03\x2a\x05\x2a\u{22f}\x0a\
	\x2a\x05\x2a\u{231}\x0a\x2a\x03\x2a\x03\x2a\x03\x2b\x03\x2b\x03\x2b\x03\
	\x2b\x03\x2b\x05\x2b\u{23a}\x0a\x2b\x03\x2c\x03\x2c\x03\x2d\x03\x2d\x03\
	\x2d\x03\x2d\x03\x2d\x03\x2d\x05\x2d\u{244}\x0a\x2d\x03\x2e\x03\x2e\x03\
	\x2e\x03\x2e\x03\x2e\x03\x2e\x05\x2e\u{24c}\x0a\x2e\x03\x2f\x03\x2f\x03\
	\x2f\x03\x2f\x03\x2f\x03\x30\x03\x30\x03\x30\x03\x30\x03\x30\x03\x30\x02\
	\x08\x06\x14\x16\x18\x2a\x2c\x31\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\
	\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\
	\x3a\x3c\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x50\x52\x54\x56\x58\x5a\x5c\
	\x5e\x02\x0b\x03\x02\x10\x11\x05\x02\x12\x13\x42\x44\x4c\x4c\x04\x02\x50\
	\x53\x57\x58\x04\x02\x4f\x4f\x5f\x5f\x03\x02\x1c\x1d\x04\x02\x1c\x1c\x1e\
	\x1e\x03\x02\x46\x48\x03\x02\x26\x3b\x03\x02\x24\x25\x02\u{285}\x02\u{8f}\
	\x03\x02\x02\x02\x04\u{91}\x03\x02\x02\x02\x06\u{93}\x03\x02\x02\x02\x08\
	\u{9f}\x03\x02\x02\x02\x0a\u{a3}\x03\x02\x02\x02\x0c\u{a5}\x03\x02\x02\x02\
	\x0e\u{a7}\x03\x02\x02\x02\x10\u{a9}\x03\x02\x02\x02\x12\u{ab}\x03\x02\x02\
	\x02\x14\u{ad}\x03\x02\x02\x02\x16\u{bc}\x03\x02\x02\x02\x18\u{c7}\x03\x02\
	\x02\x02\x1a\u{db}\x03\x02\x02\x02\x1c\u{e7}\x03\x02\x02\x02\x1e\u{e9}\x03\
	\x02\x02\x02\x20\u{f0}\x03\x02\x02\x02\x22\u{f9}\x03\x02\x02\x02\x24\u{fb}\
	\x03\x02\x02\x02\x26\u{103}\x03\x02\x02\x02\x28\u{10b}\x03\x02\x02\x02\x2a\
	\u{113}\x03\x02\x02\x02\x2c\u{127}\x03\x02\x02\x02\x2e\u{141}\x03\x02\x02\
	\x02\x30\u{148}\x03\x02\x02\x02\x32\u{15a}\x03\x02\x02\x02\x34\u{15c}\x03\
	\x02\x02\x02\x36\u{161}\x03\x02\x02\x02\x38\u{18c}\x03\x02\x02\x02\x3a\u{18e}\
	\x03\x02\x02\x02\x3c\u{192}\x03\x02\x02\x02\x3e\u{196}\x03\x02\x02\x02\x40\
	\u{19c}\x03\x02\x02\x02\x42\u{19e}\x03\x02\x02\x02\x44\u{1ad}\x03\x02\x02\
	\x02\x46\u{1b5}\x03\x02\x02\x02\x48\u{1b9}\x03\x02\x02\x02\x4a\u{1bd}\x03\
	\x02\x02\x02\x4c\u{1bf}\x03\x02\x02\x02\x4e\u{21a}\x03\x02\x02\x02\x50\u{221}\
	\x03\x02\x02\x02\x52\u{223}\x03\x02\x02\x02\x54\u{239}\x03\x02\x02\x02\x56\
	\u{23b}\x03\x02\x02\x02\x58\u{23d}\x03\x02\x02\x02\x5a\u{245}\x03\x02\x02\
	\x02\x5c\u{24d}\x03\x02\x02\x02\x5e\u{252}\x03\x02\x02\x02\x60\x63\x05\x06\
	\x04\x02\x61\x63\x05\x0a\x06\x02\x62\x60\x03\x02\x02\x02\x62\x61\x03\x02\
	\x02\x02\x63\x6b\x03\x02\x02\x02\x64\x67\x07\x60\x02\x02\x65\x68\x05\x06\
	\x04\x02\x66\x68\x05\x0a\x06\x02\x67\x65\x03\x02\x02\x02\x67\x66\x03\x02\
	\x02\x02\x68\x6a\x03\x02\x02\x02\x69\x64\x03\x02\x02\x02\x6a\x6d\x03\x02\
	\x02\x02\x6b\x69\x03\x02\x02\x02\x6b\x6c\x03\x02\x02\x02\x6c\x71\x03\x02\
	\x02\x02\x6d\x6b\x03\x02\x02\x02\x6e\x70\x07\x60\x02\x02\x6f\x6e\x03\x02\
	\x02\x02\x70\x73\x03\x02\x02\x02\x71\x6f\x03\x02\x02\x02\x71\x72\x03\x02\
	\x02\x02\x72\x74\x03\x02\x02\x02\x73\x71\x03\x02\x02\x02\x74\x75\x07\x02\
	\x02\x03\x75\u{90}\x03\x02\x02\x02\x76\x77\x07\x56\x02\x02\x77\x7a\x07\x16\
	\x02\x02\x78\x7b\x05\x06\x04\x02\x79\x7b\x05\x0a\x06\x02\x7a\x78\x03\x02\
	\x02\x02\x7a\x79\x03\x02\x02\x02\x7b\u{83}\x03\x02\x02\x02\x7c\x7f\x07\x5d\
	\x02\x02\x7d\u{80}\x05\x06\x04\x02\x7e\u{80}\x05\x0a\x06\x02\x7f\x7d\x03\
	\x02\x02\x02\x7f\x7e\x03\x02\x02\x02\u{80}\u{82}\x03\x02\x02\x02\u{81}\x7c\
	\x03\x02\x02\x02\u{82}\u{85}\x03\x02\x02\x02\u{83}\u{81}\x03\x02\x02\x02\
	\u{83}\u{84}\x03\x02\x02\x02\u{84}\u{86}\x03\x02\x02\x02\u{85}\u{83}\x03\
	\x02\x02\x02\u{86}\u{8a}\x07\x17\x02\x02\u{87}\u{89}\x07\x60\x02\x02\u{88}\
	\u{87}\x03\x02\x02\x02\u{89}\u{8c}\x03\x02\x02\x02\u{8a}\u{88}\x03\x02\x02\
	\x02\u{8a}\u{8b}\x03\x02\x02\x02\u{8b}\u{8d}\x03\x02\x02\x02\u{8c}\u{8a}\
	\x03\x02\x02\x02\u{8d}\u{8e}\x07\x02\x02\x03\u{8e}\u{90}\x03\x02\x02\x02\
	\u{8f}\x62\x03\x02\x02\x02\u{8f}\x76\x03\x02\x02\x02\u{90}\x03\x03\x02\x02\
	\x02\u{91}\u{92}\x05\x06\x04\x02\u{92}\x05\x03\x02\x02\x02\u{93}\u{94}\x08\
	\x04\x01\x02\u{94}\u{95}\x05\x0a\x06\x02\u{95}\u{9c}\x03\x02\x02\x02\u{96}\
	\u{97}\x0c\x04\x02\x02\u{97}\u{98}\x05\x10\x09\x02\u{98}\u{99}\x05\x06\x04\
	\x05\u{99}\u{9b}\x03\x02\x02\x02\u{9a}\u{96}\x03\x02\x02\x02\u{9b}\u{9e}\
	\x03\x02\x02\x02\u{9c}\u{9a}\x03\x02\x02\x02\u{9c}\u{9d}\x03\x02\x02\x02\
	\u{9d}\x07\x03\x02\x02\x02\u{9e}\u{9c}\x03\x02\x02\x02\u{9f}\u{a0}\x05\x0a\
	\x06\x02\u{a0}\u{a1}\x07\x50\x02\x02\u{a1}\u{a2}\x05\x0a\x06\x02\u{a2}\x09\
	\x03\x02\x02\x02\u{a3}\u{a4}\x05\x14\x0b\x02\u{a4}\x0b\x03\x02\x02\x02\u{a5}\
	\u{a6}\x09\x02\x02\x02\u{a6}\x0d\x03\x02\x02\x02\u{a7}\u{a8}\x09\x03\x02\
	\x02\u{a8}\x0f\x03\x02\x02\x02\u{a9}\u{aa}\x09\x04\x02\x02\u{aa}\x11\x03\
	\x02\x02\x02\u{ab}\u{ac}\x07\x4b\x02\x02\u{ac}\x13\x03\x02\x02\x02\u{ad}\
	\u{ae}\x08\x0b\x01\x02\u{ae}\u{af}\x05\x16\x0c\x02\u{af}\u{b6}\x03\x02\x02\
	\x02\u{b0}\u{b1}\x0c\x04\x02\x02\u{b1}\u{b2}\x05\x0c\x07\x02\u{b2}\u{b3}\
	\x05\x16\x0c\x02\u{b3}\u{b5}\x03\x02\x02\x02\u{b4}\u{b0}\x03\x02\x02\x02\
	\u{b5}\u{b8}\x03\x02\x02\x02\u{b6}\u{b4}\x03\x02\x02\x02\u{b6}\u{b7}\x03\
	\x02\x02\x02\u{b7}\x15\x03\x02\x02\x02\u{b8}\u{b6}\x03\x02\x02\x02\u{b9}\
	\u{ba}\x08\x0c\x01\x02\u{ba}\u{bd}\x05\x2a\x16\x02\u{bb}\u{bd}\x05\x1a\x0e\
	\x02\u{bc}\u{b9}\x03\x02\x02\x02\u{bc}\u{bb}\x03\x02\x02\x02\u{bd}\u{c4}\
	\x03\x02\x02\x02\u{be}\u{bf}\x0c\x05\x02\x02\u{bf}\u{c0}\x05\x0e\x08\x02\
	\u{c0}\u{c1}\x05\x16\x0c\x06\u{c1}\u{c3}\x03\x02\x02\x02\u{c2}\u{be}\x03\
	\x02\x02\x02\u{c3}\u{c6}\x03\x02\x02\x02\u{c4}\u{c2}\x03\x02\x02\x02\u{c4}\
	\u{c5}\x03\x02\x02\x02\u{c5}\x17\x03\x02\x02\x02\u{c6}\u{c4}\x03\x02\x02\
	\x02\u{c7}\u{c8}\x08\x0d\x01\x02\u{c8}\u{c9}\x05\x1c\x0f\x02\u{c9}\u{d0}\
	\x03\x02\x02\x02\u{ca}\u{cb}\x0c\x04\x02\x02\u{cb}\u{cc}\x05\x0e\x08\x02\
	\u{cc}\u{cd}\x05\x18\x0d\x05\u{cd}\u{cf}\x03\x02\x02\x02\u{ce}\u{ca}\x03\
	\x02\x02\x02\u{cf}\u{d2}\x03\x02\x02\x02\u{d0}\u{ce}\x03\x02\x02\x02\u{d0}\
	\u{d1}\x03\x02\x02\x02\u{d1}\x19\x03\x02\x02\x02\u{d2}\u{d0}\x03\x02\x02\
	\x02\u{d3}\u{d4}\x05\x0c\x07\x02\u{d4}\u{d5}\x05\x1a\x0e\x02\u{d5}\u{dc}\
	\x03\x02\x02\x02\u{d6}\u{d8}\x05\x1e\x10\x02\u{d7}\u{d6}\x03\x02\x02\x02\
	\u{d8}\u{d9}\x03\x02\x02\x02\u{d9}\u{d7}\x03\x02\x02\x02\u{d9}\u{da}\x03\
	\x02\x02\x02\u{da}\u{dc}\x03\x02\x02\x02\u{db}\u{d3}\x03\x02\x02\x02\u{db}\
	\u{d7}\x03\x02\x02\x02\u{dc}\x1b\x03\x02\x02\x02\u{dd}\u{de}\x05\x0c\x07\
	\x02\u{de}\u{df}\x05\x1c\x0f\x02\u{df}\u{e8}\x03\x02\x02\x02\u{e0}\u{e4}\
	\x05\x1e\x10\x02\u{e1}\u{e3}\x05\x20\x11\x02\u{e2}\u{e1}\x03\x02\x02\x02\
	\u{e3}\u{e6}\x03\x02\x02\x02\u{e4}\u{e2}\x03\x02\x02\x02\u{e4}\u{e5}\x03\
	\x02\x02\x02\u{e5}\u{e8}\x03\x02\x02\x02\u{e6}\u{e4}\x03\x02\x02\x02\u{e7}\
	\u{dd}\x03\x02\x02\x02\u{e7}\u{e0}\x03\x02\x02\x02\u{e8}\x1d\x03\x02\x02\
	\x02\u{e9}\u{ed}\x05\x2a\x16\x02\u{ea}\u{ec}\x05\x22\x12\x02\u{eb}\u{ea}\
	\x03\x02\x02\x02\u{ec}\u{ef}\x03\x02\x02\x02\u{ed}\u{eb}\x03\x02\x02\x02\
	\u{ed}\u{ee}\x03\x02\x02\x02\u{ee}\x1f\x03\x02\x02\x02\u{ef}\u{ed}\x03\x02\
	\x02\x02\u{f0}\u{f4}\x05\x2c\x17\x02\u{f1}\u{f3}\x05\x22\x12\x02\u{f2}\u{f1}\
	\x03\x02\x02\x02\u{f3}\u{f6}\x03\x02\x02\x02\u{f4}\u{f2}\x03\x02\x02\x02\
	\u{f4}\u{f5}\x03\x02\x02\x02\u{f5}\x21\x03\x02\x02\x02\u{f6}\u{f4}\x03\x02\
	\x02\x02\u{f7}\u{fa}\x07\x5c\x02\x02\u{f8}\u{fa}\x05\x24\x13\x02\u{f9}\u{f7}\
	\x03\x02\x02\x02\u{f9}\u{f8}\x03\x02\x02\x02\u{fa}\x23\x03\x02\x02\x02\u{fb}\
	\u{101}\x07\x1c\x02\x02\u{fc}\u{102}\x05\x28\x15\x02\u{fd}\u{102}\x05\x26\
	\x14\x02\u{fe}\u{ff}\x05\x28\x15\x02\u{ff}\u{100}\x05\x26\x14\x02\u{100}\
	\u{102}\x03\x02\x02\x02\u{101}\u{fc}\x03\x02\x02\x02\u{101}\u{fd}\x03\x02\
	\x02\x02\u{101}\u{fe}\x03\x02\x02\x02\u{102}\x25\x03\x02\x02\x02\u{103}\
	\u{104}\x07\x4a\x02\x02\u{104}\u{107}\x07\x16\x02\x02\u{105}\u{108}\x05\
	\x0a\x06\x02\u{106}\u{108}\x05\x08\x05\x02\u{107}\u{105}\x03\x02\x02\x02\
	\u{107}\u{106}\x03\x02\x02\x02\u{108}\u{109}\x03\x02\x02\x02\u{109}\u{10a}\
	\x07\x17\x02\x02\u{10a}\x27\x03\x02\x02\x02\u{10b}\u{10c}\x07\x4b\x02\x02\
	\u{10c}\u{10f}\x07\x16\x02\x02\u{10d}\u{110}\x05\x0a\x06\x02\u{10e}\u{110}\
	\x05\x08\x05\x02\u{10f}\u{10d}\x03\x02\x02\x02\u{10f}\u{10e}\x03\x02\x02\
	\x02\u{110}\u{111}\x03\x02\x02\x02\u{111}\u{112}\x07\x17\x02\x02\u{112}\
	\x29\x03\x02\x02\x02\u{113}\u{114}\x08\x16\x01\x02\u{114}\u{115}\x05\x2e\
	\x18\x02\u{115}\u{124}\x03\x02\x02\x02\u{116}\u{117}\x0c\x04\x02\x02\u{117}\
	\u{11d}\x05\x12\x0a\x02\u{118}\u{11e}\x05\x38\x1d\x02\u{119}\u{11a}\x07\
	\x16\x02\x02\u{11a}\u{11b}\x05\x0a\x06\x02\u{11b}\u{11c}\x07\x17\x02\x02\
	\u{11c}\u{11e}\x03\x02\x02\x02\u{11d}\u{118}\x03\x02\x02\x02\u{11d}\u{119}\
	\x03\x02\x02\x02\u{11e}\u{120}\x03\x02\x02\x02\u{11f}\u{121}\x05\x58\x2d\
	\x02\u{120}\u{11f}\x03\x02\x02\x02\u{120}\u{121}\x03\x02\x02\x02\u{121}\
	\u{123}\x03\x02\x02\x02\u{122}\u{116}\x03\x02\x02\x02\u{123}\u{126}\x03\
	\x02\x02\x02\u{124}\u{122}\x03\x02\x02\x02\u{124}\u{125}\x03\x02\x02\x02\
	\u{125}\x2b\x03\x02\x02\x02\u{126}\u{124}\x03\x02\x02\x02\u{127}\u{128}\
	\x08\x17\x01\x02\u{128}\u{129}\x05\x30\x19\x02\u{129}\u{138}\x03\x02\x02\
	\x02\u{12a}\u{12b}\x0c\x04\x02\x02\u{12b}\u{131}\x05\x12\x0a\x02\u{12c}\
	\u{132}\x05\x38\x1d\x02\u{12d}\u{12e}\x07\x16\x02\x02\u{12e}\u{12f}\x05\
	\x0a\x06\x02\u{12f}\u{130}\x07\x17\x02\x02\u{130}\u{132}\x03\x02\x02\x02\
	\u{131}\u{12c}\x03\x02\x02\x02\u{131}\u{12d}\x03\x02\x02\x02\u{132}\u{134}\
	\x03\x02\x02\x02\u{133}\u{135}\x05\x58\x2d\x02\u{134}\u{133}\x03\x02\x02\
	\x02\u{134}\u{135}\x03\x02\x02\x02\u{135}\u{137}\x03\x02\x02\x02\u{136}\
	\u{12a}\x03\x02\x02\x02\u{137}\u{13a}\x03\x02\x02\x02\u{138}\u{136}\x03\
	\x02\x02\x02\u{138}\u{139}\x03\x02\x02\x02\u{139}\x2d\x03\x02\x02\x02\u{13a}\
	\u{138}\x03\x02\x02\x02\u{13b}\u{142}\x05\x32\x1a\x02\u{13c}\u{142}\x05\
	\x34\x1b\x02\u{13d}\u{142}\x05\x4e\x28\x02\u{13e}\u{142}\x05\x38\x1d\x02\
	\u{13f}\u{142}\x05\x46\x24\x02\u{140}\u{142}\x05\x48\x25\x02\u{141}\u{13b}\
	\x03\x02\x02\x02\u{141}\u{13c}\x03\x02\x02\x02\u{141}\u{13d}\x03\x02\x02\
	\x02\u{141}\u{13e}\x03\x02\x02\x02\u{141}\u{13f}\x03\x02\x02\x02\u{141}\
	\u{140}\x03\x02\x02\x02\u{142}\x2f\x03\x02\x02\x02\u{143}\u{149}\x05\x32\
	\x1a\x02\u{144}\u{149}\x05\x34\x1b\x02\u{145}\u{149}\x05\x38\x1d\x02\u{146}\
	\u{149}\x05\x46\x24\x02\u{147}\u{149}\x05\x48\x25\x02\u{148}\u{143}\x03\
	\x02\x02\x02\u{148}\u{144}\x03\x02\x02\x02\u{148}\u{145}\x03\x02\x02\x02\
	\u{148}\u{146}\x03\x02\x02\x02\u{148}\u{147}\x03\x02\x02\x02\u{149}\x31\
	\x03\x02\x02\x02\u{14a}\u{14b}\x07\x14\x02\x02\u{14b}\u{14c}\x05\x0a\x06\
	\x02\u{14c}\u{14d}\x07\x15\x02\x02\u{14d}\u{15b}\x03\x02\x02\x02\u{14e}\
	\u{14f}\x07\x1a\x02\x02\u{14f}\u{150}\x05\x0a\x06\x02\u{150}\u{151}\x07\
	\x1b\x02\x02\u{151}\u{15b}\x03\x02\x02\x02\u{152}\u{153}\x07\x16\x02\x02\
	\u{153}\u{154}\x05\x0a\x06\x02\u{154}\u{155}\x07\x17\x02\x02\u{155}\u{15b}\
	\x03\x02\x02\x02\u{156}\u{157}\x07\x18\x02\x02\u{157}\u{158}\x05\x0a\x06\
	\x02\u{158}\u{159}\x07\x19\x02\x02\u{159}\u{15b}\x03\x02\x02\x02\u{15a}\
	\u{14a}\x03\x02\x02\x02\u{15a}\u{14e}\x03\x02\x02\x02\u{15a}\u{152}\x03\
	\x02\x02\x02\u{15a}\u{156}\x03\x02\x02\x02\u{15b}\x33\x03\x02\x02\x02\u{15c}\
	\u{15d}\x07\x1c\x02\x02\u{15d}\u{15e}\x05\x0a\x06\x02\u{15e}\u{15f}\x07\
	\x1c\x02\x02\u{15f}\x35\x03\x02\x02\x02\u{160}\u{162}\x07\x4e\x02\x02\u{161}\
	\u{160}\x03\x02\x02\x02\u{162}\u{163}\x03\x02\x02\x02\u{163}\u{161}\x03\
	\x02\x02\x02\u{163}\u{164}\x03\x02\x02\x02\u{164}\u{16b}\x03\x02\x02\x02\
	\u{165}\u{166}\x07\x03\x02\x02\u{166}\u{167}\x07\x4e\x02\x02\u{167}\u{168}\
	\x07\x4e\x02\x02\u{168}\u{16a}\x07\x4e\x02\x02\u{169}\u{165}\x03\x02\x02\
	\x02\u{16a}\u{16d}\x03\x02\x02\x02\u{16b}\u{169}\x03\x02\x02\x02\u{16b}\
	\u{16c}\x03\x02\x02\x02\u{16c}\u{174}\x03\x02\x02\x02\u{16d}\u{16b}\x03\
	\x02\x02\x02\u{16e}\u{170}\x07\x04\x02\x02\u{16f}\u{171}\x07\x4e\x02\x02\
	\u{170}\u{16f}\x03\x02\x02\x02\u{171}\u{172}\x03\x02\x02\x02\u{172}\u{170}\
	\x03\x02\x02\x02\u{172}\u{173}\x03\x02\x02\x02\u{173}\u{175}\x03\x02\x02\
	\x02\u{174}\u{16e}\x03\x02\x02\x02\u{174}\u{175}\x03\x02\x02\x02\u{175}\
	\x37\x03\x02\x02\x02\u{176}\u{183}\x09\x05\x02\x02\u{177}\u{179}\x05\x58\
	\x2d\x02\u{178}\u{177}\x03\x02\x02\x02\u{178}\u{179}\x03\x02\x02\x02\u{179}\
	\u{17b}\x03\x02\x02\x02\u{17a}\u{17c}\x07\x5e\x02\x02\u{17b}\u{17a}\x03\
	\x02\x02\x02\u{17b}\u{17c}\x03\x02\x02\x02\u{17c}\u{184}\x03\x02\x02\x02\
	\u{17d}\u{17f}\x07\x5e\x02\x02\u{17e}\u{17d}\x03\x02\x02\x02\u{17e}\u{17f}\
	\x03\x02\x02\x02\u{17f}\u{181}\x03\x02\x02\x02\u{180}\u{182}\x05\x58\x2d\
	\x02\u{181}\u{180}\x03\x02\x02\x02\u{181}\u{182}\x03\x02\x02\x02\u{182}\
	\u{184}\x03\x02\x02\x02\u{183}\u{178}\x03\x02\x02\x02\u{183}\u{17e}\x03\
	\x02\x02\x02\u{184}\u{18d}\x03\x02\x02\x02\u{185}\u{18d}\x05\x36\x1c\x02\
	\u{186}\u{18d}\x07\x4d\x02\x02\u{187}\u{18d}\x05\x3e\x20\x02\u{188}\u{18d}\
	\x05\x42\x22\x02\u{189}\u{18d}\x05\x44\x23\x02\u{18a}\u{18d}\x05\x3a\x1e\
	\x02\u{18b}\u{18d}\x05\x3c\x1f\x02\u{18c}\u{176}\x03\x02\x02\x02\u{18c}\
	\u{185}\x03\x02\x02\x02\u{18c}\u{186}\x03\x02\x02\x02\u{18c}\u{187}\x03\
	\x02\x02\x02\u{18c}\u{188}\x03\x02\x02\x02\u{18c}\u{189}\x03\x02\x02\x02\
	\u{18c}\u{18a}\x03\x02\x02\x02\u{18c}\u{18b}\x03\x02\x02\x02\u{18d}\x39\
	\x03\x02\x02\x02\u{18e}\u{18f}\x07\x1f\x02\x02\u{18f}\u{190}\x05\x0a\x06\
	\x02\u{190}\u{191}\x09\x06\x02\x02\u{191}\x3b\x03\x02\x02\x02\u{192}\u{193}\
	\x09\x07\x02\x02\u{193}\u{194}\x05\x0a\x06\x02\u{194}\u{195}\x07\x20\x02\
	\x02\u{195}\x3d\x03\x02\x02\x02\u{196}\u{197}\x07\x49\x02\x02\u{197}\u{198}\
	\x07\x16\x02\x02\u{198}\u{199}\x05\x40\x21\x02\u{199}\u{19a}\x07\x17\x02\
	\x02\u{19a}\x3f\x03\x02\x02\x02\u{19b}\u{19d}\x07\x4f\x02\x02\u{19c}\u{19b}\
	\x03\x02\x02\x02\u{19c}\u{19d}\x03\x02\x02\x02\u{19d}\x41\x03\x02\x02\x02\
	\u{19e}\u{1a4}\x07\x45\x02\x02\u{19f}\u{1a5}\x07\x4e\x02\x02\u{1a0}\u{1a1}\
	\x07\x16\x02\x02\u{1a1}\u{1a2}\x05\x0a\x06\x02\u{1a2}\u{1a3}\x07\x17\x02\
	\x02\u{1a3}\u{1a5}\x03\x02\x02\x02\u{1a4}\u{19f}\x03\x02\x02\x02\u{1a4}\
	\u{1a0}\x03\x02\x02\x02\u{1a5}\u{1ab}\x03\x02\x02\x02\u{1a6}\u{1ac}\x07\
	\x4e\x02\x02\u{1a7}\u{1a8}\x07\x16\x02\x02\u{1a8}\u{1a9}\x05\x0a\x06\x02\
	\u{1a9}\u{1aa}\x07\x17\x02\x02\u{1aa}\u{1ac}\x03\x02\x02\x02\u{1ab}\u{1a6}\
	\x03\x02\x02\x02\u{1ab}\u{1a7}\x03\x02\x02\x02\u{1ac}\x43\x03\x02\x02\x02\
	\u{1ad}\u{1ae}\x09\x08\x02\x02\u{1ae}\u{1af}\x07\x16\x02\x02\u{1af}\u{1b0}\
	\x05\x0a\x06\x02\u{1b0}\u{1b1}\x07\x17\x02\x02\u{1b1}\u{1b2}\x07\x16\x02\
	\x02\u{1b2}\u{1b3}\x05\x0a\x06\x02\u{1b3}\u{1b4}\x07\x17\x02\x02\u{1b4}\
	\x45\x03\x02\x02\x02\u{1b5}\u{1b6}\x07\x3c\x02\x02\u{1b6}\u{1b7}\x05\x0a\
	\x06\x02\u{1b7}\u{1b8}\x07\x3d\x02\x02\u{1b8}\x47\x03\x02\x02\x02\u{1b9}\
	\u{1ba}\x07\x3e\x02\x02\u{1ba}\u{1bb}\x05\x0a\x06\x02\u{1bb}\u{1bc}\x07\
	\x3f\x02\x02\u{1bc}\x49\x03\x02\x02\x02\u{1bd}\u{1be}\x09\x05\x02\x02\u{1be}\
	\x4b\x03\x02\x02\x02\u{1bf}\u{1c0}\x09\x09\x02\x02\u{1c0}\x4d\x03\x02\x02\
	\x02\u{1c1}\u{1ce}\x05\x4c\x27\x02\u{1c2}\u{1c4}\x05\x58\x2d\x02\u{1c3}\
	\u{1c2}\x03\x02\x02\x02\u{1c3}\u{1c4}\x03\x02\x02\x02\u{1c4}\u{1c6}\x03\
	\x02\x02\x02\u{1c5}\u{1c7}\x05\x5a\x2e\x02\u{1c6}\u{1c5}\x03\x02\x02\x02\
	\u{1c6}\u{1c7}\x03\x02\x02\x02\u{1c7}\u{1cf}\x03\x02\x02\x02\u{1c8}\u{1ca}\
	\x05\x5a\x2e\x02\u{1c9}\u{1c8}\x03\x02\x02\x02\u{1c9}\u{1ca}\x03\x02\x02\
	\x02\u{1ca}\u{1cc}\x03\x02\x02\x02\u{1cb}\u{1cd}\x05\x58\x2d\x02\u{1cc}\
	\u{1cb}\x03\x02\x02\x02\u{1cc}\u{1cd}\x03\x02\x02\x02\u{1cd}\u{1cf}\x03\
	\x02\x02\x02\u{1ce}\u{1c3}\x03\x02\x02\x02\u{1ce}\u{1c9}\x03\x02\x02\x02\
	\u{1cf}\u{1d5}\x03\x02\x02\x02\u{1d0}\u{1d1}\x07\x14\x02\x02\u{1d1}\u{1d2}\
	\x05\x54\x2b\x02\u{1d2}\u{1d3}\x07\x15\x02\x02\u{1d3}\u{1d6}\x03\x02\x02\
	\x02\u{1d4}\u{1d6}\x05\x56\x2c\x02\u{1d5}\u{1d0}\x03\x02\x02\x02\u{1d5}\
	\u{1d4}\x03\x02\x02\x02\u{1d6}\u{21b}\x03\x02\x02\x02\u{1d7}\u{1e4}\x05\
	\x4a\x26\x02\u{1d8}\u{1da}\x05\x58\x2d\x02\u{1d9}\u{1d8}\x03\x02\x02\x02\
	\u{1d9}\u{1da}\x03\x02\x02\x02\u{1da}\u{1dc}\x03\x02\x02\x02\u{1db}\u{1dd}\
	\x07\x5e\x02\x02\u{1dc}\u{1db}\x03\x02\x02\x02\u{1dc}\u{1dd}\x03\x02\x02\
	\x02\u{1dd}\u{1e5}\x03\x02\x02\x02\u{1de}\u{1e0}\x07\x5e\x02\x02\u{1df}\
	\u{1de}\x03\x02\x02\x02\u{1df}\u{1e0}\x03\x02\x02\x02\u{1e0}\u{1e2}\x03\
	\x02\x02\x02\u{1e1}\u{1e3}\x05\x58\x2d\x02\u{1e2}\u{1e1}\x03\x02\x02\x02\
	\u{1e2}\u{1e3}\x03\x02\x02\x02\u{1e3}\u{1e5}\x03\x02\x02\x02\u{1e4}\u{1d9}\
	\x03\x02\x02\x02\u{1e4}\u{1df}\x03\x02\x02\x02\u{1e5}\u{21b}\x03\x02\x02\
	\x02\u{1e6}\u{1e7}\x07\x14\x02\x02\u{1e7}\u{1e8}\x05\x50\x29\x02\u{1e8}\
	\u{1e9}\x07\x15\x02\x02\u{1e9}\u{21b}\x03\x02\x02\x02\u{1ea}\u{1f1}\x07\
	\x23\x02\x02\u{1eb}\u{1ec}\x05\x58\x2d\x02\u{1ec}\u{1ed}\x05\x5a\x2e\x02\
	\u{1ed}\u{1f2}\x03\x02\x02\x02\u{1ee}\u{1ef}\x05\x5a\x2e\x02\u{1ef}\u{1f0}\
	\x05\x58\x2d\x02\u{1f0}\u{1f2}\x03\x02\x02\x02\u{1f1}\u{1eb}\x03\x02\x02\
	\x02\u{1f1}\u{1ee}\x03\x02\x02\x02\u{1f1}\u{1f2}\x03\x02\x02\x02\u{1f2}\
	\u{1f9}\x03\x02\x02\x02\u{1f3}\u{1f5}\x05\x14\x0b\x02\u{1f4}\u{1f3}\x03\
	\x02\x02\x02\u{1f4}\u{1f5}\x03\x02\x02\x02\u{1f5}\u{1f6}\x03\x02\x02\x02\
	\u{1f6}\u{1fa}\x07\x4d\x02\x02\u{1f7}\u{1fa}\x05\x42\x22\x02\u{1f8}\u{1fa}\
	\x05\x14\x0b\x02\u{1f9}\u{1f4}\x03\x02\x02\x02\u{1f9}\u{1f7}\x03\x02\x02\
	\x02\u{1f9}\u{1f8}\x03\x02\x02\x02\u{1fa}\u{21b}\x03\x02\x02\x02\u{1fb}\
	\u{200}\x07\x40\x02\x02\u{1fc}\u{1fd}\x07\x1a\x02\x02\u{1fd}\u{1fe}\x05\
	\x0a\x06\x02\u{1fe}\u{1ff}\x07\x1b\x02\x02\u{1ff}\u{201}\x03\x02\x02\x02\
	\u{200}\u{1fc}\x03\x02\x02\x02\u{200}\u{201}\x03\x02\x02\x02\u{201}\u{202}\
	\x03\x02\x02\x02\u{202}\u{203}\x07\x16\x02\x02\u{203}\u{204}\x05\x0a\x06\
	\x02\u{204}\u{205}\x07\x17\x02\x02\u{205}\u{21b}\x03\x02\x02\x02\u{206}\
	\u{207}\x07\x41\x02\x02\u{207}\u{208}\x07\x16\x02\x02\u{208}\u{209}\x05\
	\x0a\x06\x02\u{209}\u{20a}\x07\x17\x02\x02\u{20a}\u{21b}\x03\x02\x02\x02\
	\u{20b}\u{212}\x09\x0a\x02\x02\u{20c}\u{20d}\x05\x5c\x2f\x02\u{20d}\u{20e}\
	\x05\x5a\x2e\x02\u{20e}\u{213}\x03\x02\x02\x02\u{20f}\u{210}\x05\x5a\x2e\
	\x02\u{210}\u{211}\x05\x5c\x2f\x02\u{211}\u{213}\x03\x02\x02\x02\u{212}\
	\u{20c}\x03\x02\x02\x02\u{212}\u{20f}\x03\x02\x02\x02\u{213}\u{214}\x03\
	\x02\x02\x02\u{214}\u{215}\x05\x16\x0c\x02\u{215}\u{21b}\x03\x02\x02\x02\
	\u{216}\u{217}\x07\x21\x02\x02\u{217}\u{218}\x05\x52\x2a\x02\u{218}\u{219}\
	\x05\x16\x0c\x02\u{219}\u{21b}\x03\x02\x02\x02\u{21a}\u{1c1}\x03\x02\x02\
	\x02\u{21a}\u{1d7}\x03\x02\x02\x02\u{21a}\u{1e6}\x03\x02\x02\x02\u{21a}\
	\u{1ea}\x03\x02\x02\x02\u{21a}\u{1fb}\x03\x02\x02\x02\u{21a}\u{206}\x03\
	\x02\x02\x02\u{21a}\u{20b}\x03\x02\x02\x02\u{21a}\u{216}\x03\x02\x02\x02\
	\u{21b}\x4f\x03\x02\x02\x02\u{21c}\u{21d}\x05\x0a\x06\x02\u{21d}\u{21e}\
	\x07\x03\x02\x02\u{21e}\u{21f}\x05\x50\x29\x02\u{21f}\u{222}\x03\x02\x02\
	\x02\u{220}\u{222}\x05\x0a\x06\x02\u{221}\u{21c}\x03\x02\x02\x02\u{221}\
	\u{220}\x03\x02\x02\x02\u{222}\x51\x03\x02\x02\x02\u{223}\u{224}\x07\x4a\
	\x02\x02\u{224}\u{225}\x07\x16\x02\x02\u{225}\u{226}\x09\x05\x02\x02\u{226}\
	\u{227}\x07\x22\x02\x02\u{227}\u{230}\x05\x0a\x06\x02\u{228}\u{22e}\x07\
	\x4b\x02\x02\u{229}\u{22a}\x07\x16\x02\x02\u{22a}\u{22b}\x05\x0c\x07\x02\
	\u{22b}\u{22c}\x07\x17\x02\x02\u{22c}\u{22f}\x03\x02\x02\x02\u{22d}\u{22f}\
	\x05\x0c\x07\x02\u{22e}\u{229}\x03\x02\x02\x02\u{22e}\u{22d}\x03\x02\x02\
	\x02\u{22f}\u{231}\x03\x02\x02\x02\u{230}\u{228}\x03\x02\x02\x02\u{230}\
	\u{231}\x03\x02\x02\x02\u{231}\u{232}\x03\x02\x02\x02\u{232}\u{233}\x07\
	\x17\x02\x02\u{233}\x53\x03\x02\x02\x02\u{234}\u{23a}\x05\x0a\x06\x02\u{235}\
	\u{236}\x05\x0a\x06\x02\u{236}\u{237}\x07\x03\x02\x02\u{237}\u{238}\x05\
	\x54\x2b\x02\u{238}\u{23a}\x03\x02\x02\x02\u{239}\u{234}\x03\x02\x02\x02\
	\u{239}\u{235}\x03\x02\x02\x02\u{23a}\x55\x03\x02\x02\x02\u{23b}\u{23c}\
	\x05\x18\x0d\x02\u{23c}\x57\x03\x02\x02\x02\u{23d}\u{243}\x07\x4a\x02\x02\
	\u{23e}\u{244}\x05\x38\x1d\x02\u{23f}\u{240}\x07\x16\x02\x02\u{240}\u{241}\
	\x05\x0a\x06\x02\u{241}\u{242}\x07\x17\x02\x02\u{242}\u{244}\x03\x02\x02\
	\x02\u{243}\u{23e}\x03\x02\x02\x02\u{243}\u{23f}\x03\x02\x02\x02\u{244}\
	\x59\x03\x02\x02\x02\u{245}\u{24b}\x07\x4b\x02\x02\u{246}\u{24c}\x05\x38\
	\x1d\x02\u{247}\u{248}\x07\x16\x02\x02\u{248}\u{249}\x05\x0a\x06\x02\u{249}\
	\u{24a}\x07\x17\x02\x02\u{24a}\u{24c}\x03\x02\x02\x02\u{24b}\u{246}\x03\
	\x02\x02\x02\u{24b}\u{247}\x03\x02\x02\x02\u{24c}\x5b\x03\x02\x02\x02\u{24d}\
	\u{24e}\x07\x4a\x02\x02\u{24e}\u{24f}\x07\x16\x02\x02\u{24f}\u{250}\x05\
	\x08\x05\x02\u{250}\u{251}\x07\x17\x02\x02\u{251}\x5d\x03\x02\x02\x02\u{252}\
	\u{253}\x07\x4a\x02\x02\u{253}\u{254}\x07\x16\x02\x02\u{254}\u{255}\x05\
	\x08\x05\x02\u{255}\u{256}\x07\x17\x02\x02\u{256}\x5f\x03\x02\x02\x02\x47\
	\x62\x67\x6b\x71\x7a\x7f\u{83}\u{8a}\u{8f}\u{9c}\u{b6}\u{bc}\u{c4}\u{d0}\
	\u{d9}\u{db}\u{e4}\u{e7}\u{ed}\u{f4}\u{f9}\u{101}\u{107}\u{10f}\u{11d}\u{120}\
	\u{124}\u{131}\u{134}\u{138}\u{141}\u{148}\u{15a}\u{163}\u{16b}\u{172}\u{174}\
	\u{178}\u{17b}\u{17e}\u{181}\u{183}\u{18c}\u{19c}\u{1a4}\u{1ab}\u{1c3}\u{1c6}\
	\u{1c9}\u{1cc}\u{1ce}\u{1d5}\u{1d9}\u{1dc}\u{1df}\u{1e2}\u{1e4}\u{1f1}\u{1f4}\
	\u{1f9}\u{200}\u{212}\u{21a}\u{221}\u{22e}\u{230}\u{239}\u{243}\u{24b}";

