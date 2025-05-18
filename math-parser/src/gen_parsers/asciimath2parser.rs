// Generated from AsciiMath2.g4 by ANTLR 4.8
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
use super::asciimath2listener::*;
use super::asciimath2visitor::*;

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

		pub const BUILTIN_KEYWORD_FUNC_NAME:isize=1; 
		pub const INTEGRAL:isize=2; 
		pub const D_LOWERCASE:isize=3; 
		pub const DERIV:isize=4; 
		pub const DBYD:isize=5; 
		pub const PARTIAL:isize=6; 
		pub const LIM:isize=7; 
		pub const SOLVE:isize=8; 
		pub const SIN:isize=9; 
		pub const COS:isize=10; 
		pub const TAN:isize=11; 
		pub const CSC:isize=12; 
		pub const SEC:isize=13; 
		pub const COT:isize=14; 
		pub const ASIN:isize=15; 
		pub const ACOS:isize=16; 
		pub const ATAN:isize=17; 
		pub const ACSC:isize=18; 
		pub const ASEC:isize=19; 
		pub const ACOT:isize=20; 
		pub const SINH:isize=21; 
		pub const COSH:isize=22; 
		pub const TANH:isize=23; 
		pub const CSCH:isize=24; 
		pub const SECH:isize=25; 
		pub const COTH:isize=26; 
		pub const ASINH:isize=27; 
		pub const ACOSH:isize=28; 
		pub const ATANH:isize=29; 
		pub const ACSCH:isize=30; 
		pub const ASECH:isize=31; 
		pub const ACOTH:isize=32; 
		pub const LOG:isize=33; 
		pub const LN:isize=34; 
		pub const EXP:isize=35; 
		pub const FLOOR:isize=36; 
		pub const CEIL:isize=37; 
		pub const ROUND:isize=38; 
		pub const MIN:isize=39; 
		pub const MAX:isize=40; 
		pub const NORM:isize=41; 
		pub const CARD:isize=42; 
		pub const ABS_FUNC:isize=43; 
		pub const SUM:isize=44; 
		pub const PROD:isize=45; 
		pub const VEC:isize=46; 
		pub const SQRT:isize=47; 
		pub const ROOT:isize=48; 
		pub const FRAC:isize=49; 
		pub const TEXT:isize=50; 
		pub const MAT:isize=51; 
		pub const DET:isize=52; 
		pub const TRANSPOSE:isize=53; 
		pub const PI_CONST:isize=54; 
		pub const E_CONST:isize=55; 
		pub const I_CONST:isize=56; 
		pub const INFINITY_CONST:isize=57; 
		pub const GAMMA_CONST:isize=58; 
		pub const PHI_CONST:isize=59; 
		pub const TRUE_CONST:isize=60; 
		pub const FALSE_CONST:isize=61; 
		pub const NAN_CONST:isize=62; 
		pub const PLUS:isize=63; 
		pub const MINUS:isize=64; 
		pub const STAR:isize=65; 
		pub const FSLASH:isize=66; 
		pub const HAT:isize=67; 
		pub const UNDERSCORE:isize=68; 
		pub const PRIME:isize=69; 
		pub const BANG:isize=70; 
		pub const EQ:isize=71; 
		pub const NEQ:isize=72; 
		pub const LT:isize=73; 
		pub const GT:isize=74; 
		pub const LTE:isize=75; 
		pub const GTE:isize=76; 
		pub const TO:isize=77; 
		pub const RARROW:isize=78; 
		pub const LARROW:isize=79; 
		pub const PM:isize=80; 
		pub const TIMES:isize=81; 
		pub const DIV:isize=82; 
		pub const AND:isize=83; 
		pub const OR:isize=84; 
		pub const NOT:isize=85; 
		pub const LPAREN:isize=86; 
		pub const RPAREN:isize=87; 
		pub const LBRACKET:isize=88; 
		pub const RBRACKET:isize=89; 
		pub const LBRACE:isize=90; 
		pub const RBRACE:isize=91; 
		pub const ABS:isize=92; 
		pub const L_ANGLE:isize=93; 
		pub const R_ANGLE:isize=94; 
		pub const COMMA:isize=95; 
		pub const SEMICOLON:isize=96; 
		pub const ALPHA_G:isize=97; 
		pub const BETA_G:isize=98; 
		pub const DELTA_G:isize=99; 
		pub const EPSILON_G:isize=100; 
		pub const ZETA_G:isize=101; 
		pub const ETA_G:isize=102; 
		pub const THETA_G:isize=103; 
		pub const IOTA_G:isize=104; 
		pub const KAPPA_G:isize=105; 
		pub const LAMBDA_G:isize=106; 
		pub const MU_G:isize=107; 
		pub const NU_G:isize=108; 
		pub const XI_G:isize=109; 
		pub const RHO_G:isize=110; 
		pub const SIGMA_G:isize=111; 
		pub const TAU_G:isize=112; 
		pub const UPSILON_G:isize=113; 
		pub const CHI_G:isize=114; 
		pub const PSI_G:isize=115; 
		pub const OMEGA_G:isize=116; 
		pub const GREEK_LETTER:isize=117; 
		pub const IDENTIFIER:isize=118; 
		pub const NUMBER:isize=119; 
		pub const NUMBER_WITH_COMMAS:isize=120; 
		pub const CURRENCY_NUMBER:isize=121; 
		pub const SCIENTIFIC_NUMBER:isize=122; 
		pub const STRING:isize=123; 
		pub const SEPARATOR:isize=124; 
		pub const WS:isize=125;
	pub const RULE_block:usize = 0; 
	pub const RULE_expression:usize = 1; 
	pub const RULE_logical_expression:usize = 2; 
	pub const RULE_relation_expression:usize = 3; 
	pub const RULE_relation_expression_no_rhs:usize = 4; 
	pub const RULE_add_sub_expression:usize = 5; 
	pub const RULE_mult_div_implicit_expression:usize = 6; 
	pub const RULE_unary_op_expression:usize = 7; 
	pub const RULE_differential:usize = 8; 
	pub const RULE_script_op_expression:usize = 9; 
	pub const RULE_primary_expression:usize = 10; 
	pub const RULE_paren_element_for_column_vector:usize = 11; 
	pub const RULE_arguments:usize = 12; 
	pub const RULE_text_argument:usize = 13; 
	pub const RULE_wrt_argument:usize = 14; 
	pub const RULE_matrix_content:usize = 15; 
	pub const RULE_matrix_row:usize = 16; 
	pub const RULE_keyword_func:usize = 17; 
	pub const RULE_simple_keyword_func:usize = 18; 
	pub const RULE_deriv_function:usize = 19; 
	pub const RULE_d_by_d:usize = 20; 
	pub const RULE_derivative:usize = 21; 
	pub const RULE_partial_derivative:usize = 22; 
	pub const RULE_function_call:usize = 23; 
	pub const RULE_constant_symbol:usize = 24;
	pub const ruleNames: [&'static str; 25] =  [
		"block", "expression", "logical_expression", "relation_expression", "relation_expression_no_rhs", 
		"add_sub_expression", "mult_div_implicit_expression", "unary_op_expression", 
		"differential", "script_op_expression", "primary_expression", "paren_element_for_column_vector", 
		"arguments", "text_argument", "wrt_argument", "matrix_content", "matrix_row", 
		"keyword_func", "simple_keyword_func", "deriv_function", "d_by_d", "derivative", 
		"partial_derivative", "function_call", "constant_symbol"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;117] = [
		None, None, None, Some("'d'"), Some("'deriv'"), None, None, Some("'lim'"), 
		Some("'solve'"), Some("'sin'"), Some("'cos'"), Some("'tan'"), Some("'csc'"), 
		Some("'sec'"), Some("'cot'"), None, None, None, None, None, None, Some("'sinh'"), 
		Some("'cosh'"), Some("'tanh'"), Some("'csch'"), Some("'sech'"), Some("'coth'"), 
		None, None, None, None, None, None, Some("'log'"), Some("'ln'"), Some("'exp'"), 
		Some("'floor'"), Some("'ceil'"), Some("'round'"), Some("'min'"), Some("'max'"), 
		Some("'norm'"), Some("'card'"), Some("'abs'"), None, None, Some("'vec'"), 
		None, Some("'root'"), Some("'frac'"), Some("'text'"), Some("'mat'"), Some("'det'"), 
		None, None, Some("'e'"), Some("'i'"), None, None, None, Some("'true'"), 
		Some("'false'"), Some("'NaN'"), Some("'+'"), Some("'-'"), Some("'*'"), 
		Some("'/'"), Some("'^'"), Some("'_'"), Some("'''"), Some("'!'"), Some("'='"), 
		None, Some("'<'"), Some("'>'"), None, None, Some("'to'"), None, None, 
		None, None, None, Some("'and'"), Some("'or'"), Some("'not'"), Some("'('"), 
		Some("')'"), Some("'['"), Some("']'"), Some("'{'"), Some("'}'"), Some("'|'"), 
		None, None, Some("','"), Some("';'"), Some("'alpha'"), Some("'beta'"), 
		Some("'delta'"), Some("'epsilon'"), Some("'zeta'"), Some("'eta'"), Some("'theta'"), 
		Some("'iota'"), Some("'kappa'"), Some("'lambda'"), Some("'mu'"), Some("'nu'"), 
		Some("'xi'"), Some("'rho'"), Some("'sigma'"), Some("'tau'"), Some("'upsilon'"), 
		Some("'chi'"), Some("'psi'"), Some("'omega'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;126]  = [
		None, Some("BUILTIN_KEYWORD_FUNC_NAME"), Some("INTEGRAL"), Some("D_LOWERCASE"), 
		Some("DERIV"), Some("DBYD"), Some("PARTIAL"), Some("LIM"), Some("SOLVE"), 
		Some("SIN"), Some("COS"), Some("TAN"), Some("CSC"), Some("SEC"), Some("COT"), 
		Some("ASIN"), Some("ACOS"), Some("ATAN"), Some("ACSC"), Some("ASEC"), 
		Some("ACOT"), Some("SINH"), Some("COSH"), Some("TANH"), Some("CSCH"), 
		Some("SECH"), Some("COTH"), Some("ASINH"), Some("ACOSH"), Some("ATANH"), 
		Some("ACSCH"), Some("ASECH"), Some("ACOTH"), Some("LOG"), Some("LN"), 
		Some("EXP"), Some("FLOOR"), Some("CEIL"), Some("ROUND"), Some("MIN"), 
		Some("MAX"), Some("NORM"), Some("CARD"), Some("ABS_FUNC"), Some("SUM"), 
		Some("PROD"), Some("VEC"), Some("SQRT"), Some("ROOT"), Some("FRAC"), Some("TEXT"), 
		Some("MAT"), Some("DET"), Some("TRANSPOSE"), Some("PI_CONST"), Some("E_CONST"), 
		Some("I_CONST"), Some("INFINITY_CONST"), Some("GAMMA_CONST"), Some("PHI_CONST"), 
		Some("TRUE_CONST"), Some("FALSE_CONST"), Some("NAN_CONST"), Some("PLUS"), 
		Some("MINUS"), Some("STAR"), Some("FSLASH"), Some("HAT"), Some("UNDERSCORE"), 
		Some("PRIME"), Some("BANG"), Some("EQ"), Some("NEQ"), Some("LT"), Some("GT"), 
		Some("LTE"), Some("GTE"), Some("TO"), Some("RARROW"), Some("LARROW"), 
		Some("PM"), Some("TIMES"), Some("DIV"), Some("AND"), Some("OR"), Some("NOT"), 
		Some("LPAREN"), Some("RPAREN"), Some("LBRACKET"), Some("RBRACKET"), Some("LBRACE"), 
		Some("RBRACE"), Some("ABS"), Some("L_ANGLE"), Some("R_ANGLE"), Some("COMMA"), 
		Some("SEMICOLON"), Some("ALPHA_G"), Some("BETA_G"), Some("DELTA_G"), Some("EPSILON_G"), 
		Some("ZETA_G"), Some("ETA_G"), Some("THETA_G"), Some("IOTA_G"), Some("KAPPA_G"), 
		Some("LAMBDA_G"), Some("MU_G"), Some("NU_G"), Some("XI_G"), Some("RHO_G"), 
		Some("SIGMA_G"), Some("TAU_G"), Some("UPSILON_G"), Some("CHI_G"), Some("PSI_G"), 
		Some("OMEGA_G"), Some("GREEK_LETTER"), Some("IDENTIFIER"), Some("NUMBER"), 
		Some("NUMBER_WITH_COMMAS"), Some("CURRENCY_NUMBER"), Some("SCIENTIFIC_NUMBER"), 
		Some("STRING"), Some("SEPARATOR"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,AsciiMath2ParserExt<'input>, I, AsciiMath2ParserContextType , dyn AsciiMath2Listener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type AsciiMath2TreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, AsciiMath2ParserContextType , dyn AsciiMath2Listener<'input> + 'a>;

/// Parser for AsciiMath2 grammar
pub struct AsciiMath2Parser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
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
				AsciiMath2ParserExt{
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

impl<'input, I> AsciiMath2Parser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> AsciiMath2Parser<'input, I, DefaultErrorStrategy<'input,AsciiMath2ParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for AsciiMath2Parser
pub trait AsciiMath2ParserContext<'input>:
	for<'x> Listenable<dyn AsciiMath2Listener<'input> + 'x > + 
	for<'x> Visitable<dyn AsciiMath2Visitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=AsciiMath2ParserContextType>
{}

antlr_rust::coerce_from!{ 'input : AsciiMath2ParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn AsciiMath2ParserContext<'input> + 'input
where
    T: AsciiMath2Visitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn AsciiMath2Visitor<'input> + 'x))
    }
}

impl<'input> AsciiMath2ParserContext<'input> for TerminalNode<'input,AsciiMath2ParserContextType> {}
impl<'input> AsciiMath2ParserContext<'input> for ErrorNode<'input,AsciiMath2ParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn AsciiMath2ParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn AsciiMath2Listener<'input> + 'input }

pub struct AsciiMath2ParserContextType;
antlr_rust::tid!{AsciiMath2ParserContextType}

impl<'input> ParserNodeType<'input> for AsciiMath2ParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn AsciiMath2ParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct AsciiMath2ParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> AsciiMath2ParserExt<'input>{
}
antlr_rust::tid! { AsciiMath2ParserExt<'a> }

impl<'input> TokenAware<'input> for AsciiMath2ParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for AsciiMath2ParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for AsciiMath2ParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "AsciiMath2.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- block ----------------
pub type BlockContextAll<'input> = BlockContext<'input>;


pub type BlockContext<'input> = BaseParserRuleContext<'input,BlockContextExt<'input>>;

#[derive(Clone)]
pub struct BlockContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for BlockContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for BlockContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_block(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_block(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for BlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_block(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid!{BlockContextExt<'a>}

impl<'input> BlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<BlockContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SEPARATOR in current rule
fn SEPARATOR_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEPARATOR, starting from 0.
/// Returns `None` if number of children corresponding to token SEPARATOR is less or equal than `i`.
fn SEPARATOR(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(SEPARATOR, i)
}

}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
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
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(50);
			recog.expression()?;

			recog.base.set_state(55);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(0,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(51);
					recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(52);
					recog.expression()?;

					}
					} 
				}
				recog.base.set_state(57);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(0,&mut recog.base)?;
			}
			recog.base.set_state(61);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SEPARATOR {
				{
				{
				recog.base.set_state(58);
				recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(63);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(64);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

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
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn logical_expression(&self) -> Option<Rc<Logical_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logical_expression*/
			recog.base.set_state(66);
			recog.logical_expression()?;

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
//------------------- logical_expression ----------------
pub type Logical_expressionContextAll<'input> = Logical_expressionContext<'input>;


pub type Logical_expressionContext<'input> = BaseParserRuleContext<'input,Logical_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Logical_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Logical_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Logical_expressionContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_logical_expression(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_logical_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Logical_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_logical_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Logical_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logical_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logical_expression }
}
antlr_rust::tid!{Logical_expressionContextExt<'a>}

impl<'input> Logical_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Logical_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Logical_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Logical_expressionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Logical_expressionContextExt<'input>>{

fn relation_expression_all(&self) ->  Vec<Rc<Relation_expressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relation_expression(&self, i: usize) -> Option<Rc<Relation_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token AND in current rule
fn AND_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token AND, starting from 0.
/// Returns `None` if number of children corresponding to token AND is less or equal than `i`.
fn AND(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(AND, i)
}
/// Retrieves all `TerminalNode`s corresponding to token OR in current rule
fn OR_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token OR, starting from 0.
/// Returns `None` if number of children corresponding to token OR is less or equal than `i`.
fn OR(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(OR, i)
}

}

impl<'input> Logical_expressionContextAttrs<'input> for Logical_expressionContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logical_expression(&mut self,)
	-> Result<Rc<Logical_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Logical_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_logical_expression);
        let mut _localctx: Rc<Logical_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule relation_expression*/
			recog.base.set_state(68);
			recog.relation_expression()?;

			recog.base.set_state(73);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(69);
					_la = recog.base.input.la(1);
					if { !(_la==AND || _la==OR) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule relation_expression*/
					recog.base.set_state(70);
					recog.relation_expression()?;

					}
					} 
				}
				recog.base.set_state(75);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
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
//------------------- relation_expression ----------------
pub type Relation_expressionContextAll<'input> = Relation_expressionContext<'input>;


pub type Relation_expressionContext<'input> = BaseParserRuleContext<'input,Relation_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Relation_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Relation_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Relation_expressionContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relation_expression(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_relation_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Relation_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_relation_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Relation_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_expression }
}
antlr_rust::tid!{Relation_expressionContextExt<'a>}

impl<'input> Relation_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Relation_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Relation_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Relation_expressionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Relation_expressionContextExt<'input>>{

fn relation_expression_no_rhs(&self) -> Option<Rc<Relation_expression_no_rhsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn add_sub_expression_all(&self) ->  Vec<Rc<Add_sub_expressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn add_sub_expression(&self, i: usize) -> Option<Rc<Add_sub_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
/// Retrieves first TerminalNode corresponding to token NEQ
/// Returns `None` if there is no child corresponding to token NEQ
fn NEQ(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(NEQ, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token LTE
/// Returns `None` if there is no child corresponding to token LTE
fn LTE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(LTE, 0)
}
/// Retrieves first TerminalNode corresponding to token GTE
/// Returns `None` if there is no child corresponding to token GTE
fn GTE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(GTE, 0)
}

}

impl<'input> Relation_expressionContextAttrs<'input> for Relation_expressionContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relation_expression(&mut self,)
	-> Result<Rc<Relation_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Relation_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_relation_expression);
        let mut _localctx: Rc<Relation_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(82);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule relation_expression_no_rhs*/
					recog.base.set_state(76);
					recog.relation_expression_no_rhs()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule add_sub_expression*/
					recog.base.set_state(77);
					recog.add_sub_expression()?;

					recog.base.set_state(80);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(78);
							_la = recog.base.input.la(1);
							if { !(((((_la - 71)) & !0x3f) == 0 && ((1usize << (_la - 71)) & ((1usize << (EQ - 71)) | (1usize << (NEQ - 71)) | (1usize << (LT - 71)) | (1usize << (GT - 71)) | (1usize << (LTE - 71)) | (1usize << (GTE - 71)))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule add_sub_expression*/
							recog.base.set_state(79);
							recog.add_sub_expression()?;

							}
						}

						_ => {}
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
//------------------- relation_expression_no_rhs ----------------
pub type Relation_expression_no_rhsContextAll<'input> = Relation_expression_no_rhsContext<'input>;


pub type Relation_expression_no_rhsContext<'input> = BaseParserRuleContext<'input,Relation_expression_no_rhsContextExt<'input>>;

#[derive(Clone)]
pub struct Relation_expression_no_rhsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Relation_expression_no_rhsContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Relation_expression_no_rhsContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relation_expression_no_rhs(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_relation_expression_no_rhs(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Relation_expression_no_rhsContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_relation_expression_no_rhs(self);
	}
}

impl<'input> CustomRuleContext<'input> for Relation_expression_no_rhsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relation_expression_no_rhs }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relation_expression_no_rhs }
}
antlr_rust::tid!{Relation_expression_no_rhsContextExt<'a>}

impl<'input> Relation_expression_no_rhsContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Relation_expression_no_rhsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Relation_expression_no_rhsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Relation_expression_no_rhsContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Relation_expression_no_rhsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
fn add_sub_expression(&self) -> Option<Rc<Add_sub_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn function_call(&self) -> Option<Rc<Function_callContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Relation_expression_no_rhsContextAttrs<'input> for Relation_expression_no_rhsContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relation_expression_no_rhs(&mut self,)
	-> Result<Rc<Relation_expression_no_rhsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Relation_expression_no_rhsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_relation_expression_no_rhs);
        let mut _localctx: Rc<Relation_expression_no_rhsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(86);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(5,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule add_sub_expression*/
					recog.base.set_state(84);
					recog.add_sub_expression()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule function_call*/
					recog.base.set_state(85);
					recog.function_call()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(88);
			recog.base.match_token(EQ,&mut recog.err_handler)?;

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
//------------------- add_sub_expression ----------------
pub type Add_sub_expressionContextAll<'input> = Add_sub_expressionContext<'input>;


pub type Add_sub_expressionContext<'input> = BaseParserRuleContext<'input,Add_sub_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Add_sub_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Add_sub_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Add_sub_expressionContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_add_sub_expression(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_add_sub_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Add_sub_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_add_sub_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Add_sub_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_add_sub_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_add_sub_expression }
}
antlr_rust::tid!{Add_sub_expressionContextExt<'a>}

impl<'input> Add_sub_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Add_sub_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Add_sub_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Add_sub_expressionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Add_sub_expressionContextExt<'input>>{

fn mult_div_implicit_expression_all(&self) ->  Vec<Rc<Mult_div_implicit_expressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn mult_div_implicit_expression(&self, i: usize) -> Option<Rc<Mult_div_implicit_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token PLUS in current rule
fn PLUS_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PLUS, starting from 0.
/// Returns `None` if number of children corresponding to token PLUS is less or equal than `i`.
fn PLUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(PLUS, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MINUS in current rule
fn MINUS_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MINUS, starting from 0.
/// Returns `None` if number of children corresponding to token MINUS is less or equal than `i`.
fn MINUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(MINUS, i)
}
/// Retrieves all `TerminalNode`s corresponding to token PM in current rule
fn PM_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PM, starting from 0.
/// Returns `None` if number of children corresponding to token PM is less or equal than `i`.
fn PM(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(PM, i)
}

}

impl<'input> Add_sub_expressionContextAttrs<'input> for Add_sub_expressionContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn add_sub_expression(&mut self,)
	-> Result<Rc<Add_sub_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Add_sub_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_add_sub_expression);
        let mut _localctx: Rc<Add_sub_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule mult_div_implicit_expression*/
			recog.base.set_state(90);
			recog.mult_div_implicit_expression()?;

			recog.base.set_state(95);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(91);
					_la = recog.base.input.la(1);
					if { !(((((_la - 63)) & !0x3f) == 0 && ((1usize << (_la - 63)) & ((1usize << (PLUS - 63)) | (1usize << (MINUS - 63)) | (1usize << (PM - 63)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule mult_div_implicit_expression*/
					recog.base.set_state(92);
					recog.mult_div_implicit_expression()?;

					}
					} 
				}
				recog.base.set_state(97);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
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
//------------------- mult_div_implicit_expression ----------------
pub type Mult_div_implicit_expressionContextAll<'input> = Mult_div_implicit_expressionContext<'input>;


pub type Mult_div_implicit_expressionContext<'input> = BaseParserRuleContext<'input,Mult_div_implicit_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Mult_div_implicit_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Mult_div_implicit_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Mult_div_implicit_expressionContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_mult_div_implicit_expression(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_mult_div_implicit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Mult_div_implicit_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_mult_div_implicit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Mult_div_implicit_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mult_div_implicit_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mult_div_implicit_expression }
}
antlr_rust::tid!{Mult_div_implicit_expressionContextExt<'a>}

impl<'input> Mult_div_implicit_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Mult_div_implicit_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Mult_div_implicit_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Mult_div_implicit_expressionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Mult_div_implicit_expressionContextExt<'input>>{

fn unary_op_expression_all(&self) ->  Vec<Rc<Unary_op_expressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn unary_op_expression(&self, i: usize) -> Option<Rc<Unary_op_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token STAR in current rule
fn STAR_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token STAR, starting from 0.
/// Returns `None` if number of children corresponding to token STAR is less or equal than `i`.
fn STAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(STAR, i)
}
/// Retrieves all `TerminalNode`s corresponding to token FSLASH in current rule
fn FSLASH_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token FSLASH, starting from 0.
/// Returns `None` if number of children corresponding to token FSLASH is less or equal than `i`.
fn FSLASH(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(FSLASH, i)
}
/// Retrieves all `TerminalNode`s corresponding to token TIMES in current rule
fn TIMES_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token TIMES, starting from 0.
/// Returns `None` if number of children corresponding to token TIMES is less or equal than `i`.
fn TIMES(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(TIMES, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DIV in current rule
fn DIV_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DIV, starting from 0.
/// Returns `None` if number of children corresponding to token DIV is less or equal than `i`.
fn DIV(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(DIV, i)
}

}

impl<'input> Mult_div_implicit_expressionContextAttrs<'input> for Mult_div_implicit_expressionContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mult_div_implicit_expression(&mut self,)
	-> Result<Rc<Mult_div_implicit_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Mult_div_implicit_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_mult_div_implicit_expression);
        let mut _localctx: Rc<Mult_div_implicit_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule unary_op_expression*/
			recog.base.set_state(98);
			recog.unary_op_expression()?;

			recog.base.set_state(104);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					recog.base.set_state(102);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 STAR | FSLASH | TIMES | DIV 
						=> {
							{
							recog.base.set_state(99);
							_la = recog.base.input.la(1);
							if { !(((((_la - 65)) & !0x3f) == 0 && ((1usize << (_la - 65)) & ((1usize << (STAR - 65)) | (1usize << (FSLASH - 65)) | (1usize << (TIMES - 65)) | (1usize << (DIV - 65)))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule unary_op_expression*/
							recog.base.set_state(100);
							recog.unary_op_expression()?;

							}
						}

					 BUILTIN_KEYWORD_FUNC_NAME | INTEGRAL | D_LOWERCASE | DERIV | DBYD |
					 PARTIAL | LIM | SQRT | ROOT | FRAC | TEXT | MAT | DET | TRANSPOSE |
					 PI_CONST | E_CONST | I_CONST | INFINITY_CONST | GAMMA_CONST | PHI_CONST |
					 TRUE_CONST | FALSE_CONST | NAN_CONST | PLUS | MINUS | HAT | UNDERSCORE |
					 PRIME | LPAREN | LBRACKET | LBRACE | ABS | L_ANGLE | GREEK_LETTER |
					 IDENTIFIER | NUMBER | NUMBER_WITH_COMMAS | CURRENCY_NUMBER | STRING 
						=> {
							{
							/*InvokeRule unary_op_expression*/
							recog.base.set_state(101);
							recog.unary_op_expression()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					} 
				}
				recog.base.set_state(106);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
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
//------------------- unary_op_expression ----------------
#[derive(Debug)]
pub enum Unary_op_expressionContextAll<'input>{
	NoUnaryOperatorContext(NoUnaryOperatorContext<'input>),
	UnaryPlusMinusContext(UnaryPlusMinusContext<'input>),
Error(Unary_op_expressionContext<'input>)
}
antlr_rust::tid!{Unary_op_expressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Unary_op_expressionContextAll<'input>{}

impl<'input> AsciiMath2ParserContext<'input> for Unary_op_expressionContextAll<'input>{}

impl<'input> Deref for Unary_op_expressionContextAll<'input>{
	type Target = dyn Unary_op_expressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use Unary_op_expressionContextAll::*;
		match self{
			NoUnaryOperatorContext(inner) => inner,
			UnaryPlusMinusContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Unary_op_expressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Unary_op_expressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type Unary_op_expressionContext<'input> = BaseParserRuleContext<'input,Unary_op_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Unary_op_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Unary_op_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Unary_op_expressionContext<'input>{
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Unary_op_expressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for Unary_op_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_op_expression }
}
antlr_rust::tid!{Unary_op_expressionContextExt<'a>}

impl<'input> Unary_op_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Unary_op_expressionContextAll<'input>> {
		Rc::new(
		Unary_op_expressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Unary_op_expressionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait Unary_op_expressionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Unary_op_expressionContextExt<'input>>{


}

impl<'input> Unary_op_expressionContextAttrs<'input> for Unary_op_expressionContext<'input>{}

pub type NoUnaryOperatorContext<'input> = BaseParserRuleContext<'input,NoUnaryOperatorContextExt<'input>>;

pub trait NoUnaryOperatorContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn script_op_expression(&self) -> Option<Rc<Script_op_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> NoUnaryOperatorContextAttrs<'input> for NoUnaryOperatorContext<'input>{}

pub struct NoUnaryOperatorContextExt<'input>{
	base:Unary_op_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NoUnaryOperatorContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for NoUnaryOperatorContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for NoUnaryOperatorContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_noUnaryOperator(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_noUnaryOperator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for NoUnaryOperatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_noUnaryOperator(self);
	}
}

impl<'input> CustomRuleContext<'input> for NoUnaryOperatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_op_expression }
}

impl<'input> Borrow<Unary_op_expressionContextExt<'input>> for NoUnaryOperatorContext<'input>{
	fn borrow(&self) -> &Unary_op_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Unary_op_expressionContextExt<'input>> for NoUnaryOperatorContext<'input>{
	fn borrow_mut(&mut self) -> &mut Unary_op_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Unary_op_expressionContextAttrs<'input> for NoUnaryOperatorContext<'input> {}

impl<'input> NoUnaryOperatorContextExt<'input>{
	fn new(ctx: &dyn Unary_op_expressionContextAttrs<'input>) -> Rc<Unary_op_expressionContextAll<'input>>  {
		Rc::new(
			Unary_op_expressionContextAll::NoUnaryOperatorContext(
				BaseParserRuleContext::copy_from(ctx,NoUnaryOperatorContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UnaryPlusMinusContext<'input> = BaseParserRuleContext<'input,UnaryPlusMinusContextExt<'input>>;

pub trait UnaryPlusMinusContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn script_op_expression(&self) -> Option<Rc<Script_op_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token PLUS
	/// Returns `None` if there is no child corresponding to token PLUS
	fn PLUS(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(PLUS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(MINUS, 0)
	}
}

impl<'input> UnaryPlusMinusContextAttrs<'input> for UnaryPlusMinusContext<'input>{}

pub struct UnaryPlusMinusContextExt<'input>{
	base:Unary_op_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{UnaryPlusMinusContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for UnaryPlusMinusContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for UnaryPlusMinusContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unaryPlusMinus(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_unaryPlusMinus(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for UnaryPlusMinusContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_unaryPlusMinus(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryPlusMinusContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_op_expression }
}

impl<'input> Borrow<Unary_op_expressionContextExt<'input>> for UnaryPlusMinusContext<'input>{
	fn borrow(&self) -> &Unary_op_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Unary_op_expressionContextExt<'input>> for UnaryPlusMinusContext<'input>{
	fn borrow_mut(&mut self) -> &mut Unary_op_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Unary_op_expressionContextAttrs<'input> for UnaryPlusMinusContext<'input> {}

impl<'input> UnaryPlusMinusContextExt<'input>{
	fn new(ctx: &dyn Unary_op_expressionContextAttrs<'input>) -> Rc<Unary_op_expressionContextAll<'input>>  {
		Rc::new(
			Unary_op_expressionContextAll::UnaryPlusMinusContext(
				BaseParserRuleContext::copy_from(ctx,UnaryPlusMinusContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unary_op_expression(&mut self,)
	-> Result<Rc<Unary_op_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Unary_op_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_unary_op_expression);
        let mut _localctx: Rc<Unary_op_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(110);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 PLUS | MINUS 
				=> {
					let tmp = UnaryPlusMinusContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(107);
					_la = recog.base.input.la(1);
					if { !(_la==PLUS || _la==MINUS) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule script_op_expression*/
					recog.base.set_state(108);
					recog.script_op_expression()?;

					}
				}

			 BUILTIN_KEYWORD_FUNC_NAME | INTEGRAL | D_LOWERCASE | DERIV | DBYD | PARTIAL |
			 LIM | SQRT | ROOT | FRAC | TEXT | MAT | DET | TRANSPOSE | PI_CONST |
			 E_CONST | I_CONST | INFINITY_CONST | GAMMA_CONST | PHI_CONST | TRUE_CONST |
			 FALSE_CONST | NAN_CONST | HAT | UNDERSCORE | PRIME | LPAREN | LBRACKET |
			 LBRACE | ABS | L_ANGLE | GREEK_LETTER | IDENTIFIER | NUMBER | NUMBER_WITH_COMMAS |
			 CURRENCY_NUMBER | STRING 
				=> {
					let tmp = NoUnaryOperatorContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule script_op_expression*/
					recog.base.set_state(109);
					recog.script_op_expression()?;

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
//------------------- differential ----------------
pub type DifferentialContextAll<'input> = DifferentialContext<'input>;


pub type DifferentialContext<'input> = BaseParserRuleContext<'input,DifferentialContextExt<'input>>;

#[derive(Clone)]
pub struct DifferentialContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for DifferentialContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for DifferentialContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_differential(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_differential(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for DifferentialContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_differential(self);
	}
}

impl<'input> CustomRuleContext<'input> for DifferentialContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_differential }
	//fn type_rule_index() -> usize where Self: Sized { RULE_differential }
}
antlr_rust::tid!{DifferentialContextExt<'a>}

impl<'input> DifferentialContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DifferentialContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DifferentialContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DifferentialContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<DifferentialContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token D_LOWERCASE
/// Returns `None` if there is no child corresponding to token D_LOWERCASE
fn D_LOWERCASE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(D_LOWERCASE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token GREEK_LETTER
/// Returns `None` if there is no child corresponding to token GREEK_LETTER
fn GREEK_LETTER(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(GREEK_LETTER, 0)
}

}

impl<'input> DifferentialContextAttrs<'input> for DifferentialContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn differential(&mut self,)
	-> Result<Rc<DifferentialContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DifferentialContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_differential);
        let mut _localctx: Rc<DifferentialContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(112);
			recog.base.match_token(D_LOWERCASE,&mut recog.err_handler)?;

			recog.base.set_state(113);
			_la = recog.base.input.la(1);
			if { !(_la==GREEK_LETTER || _la==IDENTIFIER) } {
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
//------------------- script_op_expression ----------------
#[derive(Debug)]
pub enum Script_op_expressionContextAll<'input>{
	PowerSubscriptExpressionContext(PowerSubscriptExpressionContext<'input>),
	PowerExpressionContext(PowerExpressionContext<'input>),
	SubscriptExpressionContext(SubscriptExpressionContext<'input>),
	SubscriptPowerExpressionContext(SubscriptPowerExpressionContext<'input>),
	PrimeExpressionContext(PrimeExpressionContext<'input>),
Error(Script_op_expressionContext<'input>)
}
antlr_rust::tid!{Script_op_expressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Script_op_expressionContextAll<'input>{}

impl<'input> AsciiMath2ParserContext<'input> for Script_op_expressionContextAll<'input>{}

impl<'input> Deref for Script_op_expressionContextAll<'input>{
	type Target = dyn Script_op_expressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use Script_op_expressionContextAll::*;
		match self{
			PowerSubscriptExpressionContext(inner) => inner,
			PowerExpressionContext(inner) => inner,
			SubscriptExpressionContext(inner) => inner,
			SubscriptPowerExpressionContext(inner) => inner,
			PrimeExpressionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Script_op_expressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Script_op_expressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type Script_op_expressionContext<'input> = BaseParserRuleContext<'input,Script_op_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Script_op_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Script_op_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Script_op_expressionContext<'input>{
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Script_op_expressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for Script_op_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_script_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_script_op_expression }
}
antlr_rust::tid!{Script_op_expressionContextExt<'a>}

impl<'input> Script_op_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Script_op_expressionContextAll<'input>> {
		Rc::new(
		Script_op_expressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Script_op_expressionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait Script_op_expressionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Script_op_expressionContextExt<'input>>{


}

impl<'input> Script_op_expressionContextAttrs<'input> for Script_op_expressionContext<'input>{}

pub type PowerSubscriptExpressionContext<'input> = BaseParserRuleContext<'input,PowerSubscriptExpressionContextExt<'input>>;

pub trait PowerSubscriptExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn primary_expression_all(&self) ->  Vec<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn primary_expression(&self, i: usize) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token HAT in current rule
	fn HAT_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token HAT, starting from 0.
	/// Returns `None` if number of children corresponding to token HAT is less or equal than `i`.
	fn HAT(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(HAT, i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token UNDERSCORE in current rule
	fn UNDERSCORE_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token UNDERSCORE, starting from 0.
	/// Returns `None` if number of children corresponding to token UNDERSCORE is less or equal than `i`.
	fn UNDERSCORE(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(UNDERSCORE, i)
	}
}

impl<'input> PowerSubscriptExpressionContextAttrs<'input> for PowerSubscriptExpressionContext<'input>{}

pub struct PowerSubscriptExpressionContextExt<'input>{
	base:Script_op_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{PowerSubscriptExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for PowerSubscriptExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for PowerSubscriptExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_powerSubscriptExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_powerSubscriptExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for PowerSubscriptExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_powerSubscriptExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PowerSubscriptExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_script_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_script_op_expression }
}

impl<'input> Borrow<Script_op_expressionContextExt<'input>> for PowerSubscriptExpressionContext<'input>{
	fn borrow(&self) -> &Script_op_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Script_op_expressionContextExt<'input>> for PowerSubscriptExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Script_op_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Script_op_expressionContextAttrs<'input> for PowerSubscriptExpressionContext<'input> {}

impl<'input> PowerSubscriptExpressionContextExt<'input>{
	fn new(ctx: &dyn Script_op_expressionContextAttrs<'input>) -> Rc<Script_op_expressionContextAll<'input>>  {
		Rc::new(
			Script_op_expressionContextAll::PowerSubscriptExpressionContext(
				BaseParserRuleContext::copy_from(ctx,PowerSubscriptExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PowerExpressionContext<'input> = BaseParserRuleContext<'input,PowerExpressionContextExt<'input>>;

pub trait PowerExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token HAT
	/// Returns `None` if there is no child corresponding to token HAT
	fn HAT(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(HAT, 0)
	}
	fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> PowerExpressionContextAttrs<'input> for PowerExpressionContext<'input>{}

pub struct PowerExpressionContextExt<'input>{
	base:Script_op_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{PowerExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for PowerExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for PowerExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_powerExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_powerExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for PowerExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_powerExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PowerExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_script_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_script_op_expression }
}

impl<'input> Borrow<Script_op_expressionContextExt<'input>> for PowerExpressionContext<'input>{
	fn borrow(&self) -> &Script_op_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Script_op_expressionContextExt<'input>> for PowerExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Script_op_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Script_op_expressionContextAttrs<'input> for PowerExpressionContext<'input> {}

impl<'input> PowerExpressionContextExt<'input>{
	fn new(ctx: &dyn Script_op_expressionContextAttrs<'input>) -> Rc<Script_op_expressionContextAll<'input>>  {
		Rc::new(
			Script_op_expressionContextAll::PowerExpressionContext(
				BaseParserRuleContext::copy_from(ctx,PowerExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SubscriptExpressionContext<'input> = BaseParserRuleContext<'input,SubscriptExpressionContextExt<'input>>;

pub trait SubscriptExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token UNDERSCORE
	/// Returns `None` if there is no child corresponding to token UNDERSCORE
	fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(UNDERSCORE, 0)
	}
	fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SubscriptExpressionContextAttrs<'input> for SubscriptExpressionContext<'input>{}

pub struct SubscriptExpressionContextExt<'input>{
	base:Script_op_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SubscriptExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for SubscriptExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for SubscriptExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_subscriptExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_subscriptExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for SubscriptExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_subscriptExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubscriptExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_script_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_script_op_expression }
}

impl<'input> Borrow<Script_op_expressionContextExt<'input>> for SubscriptExpressionContext<'input>{
	fn borrow(&self) -> &Script_op_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Script_op_expressionContextExt<'input>> for SubscriptExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Script_op_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Script_op_expressionContextAttrs<'input> for SubscriptExpressionContext<'input> {}

impl<'input> SubscriptExpressionContextExt<'input>{
	fn new(ctx: &dyn Script_op_expressionContextAttrs<'input>) -> Rc<Script_op_expressionContextAll<'input>>  {
		Rc::new(
			Script_op_expressionContextAll::SubscriptExpressionContext(
				BaseParserRuleContext::copy_from(ctx,SubscriptExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SubscriptPowerExpressionContext<'input> = BaseParserRuleContext<'input,SubscriptPowerExpressionContextExt<'input>>;

pub trait SubscriptPowerExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token UNDERSCORE
	/// Returns `None` if there is no child corresponding to token UNDERSCORE
	fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(UNDERSCORE, 0)
	}
	fn primary_expression_all(&self) ->  Vec<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn primary_expression(&self, i: usize) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token HAT
	/// Returns `None` if there is no child corresponding to token HAT
	fn HAT(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(HAT, 0)
	}
}

impl<'input> SubscriptPowerExpressionContextAttrs<'input> for SubscriptPowerExpressionContext<'input>{}

pub struct SubscriptPowerExpressionContextExt<'input>{
	base:Script_op_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SubscriptPowerExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for SubscriptPowerExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for SubscriptPowerExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_subscriptPowerExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_subscriptPowerExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for SubscriptPowerExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_subscriptPowerExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubscriptPowerExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_script_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_script_op_expression }
}

impl<'input> Borrow<Script_op_expressionContextExt<'input>> for SubscriptPowerExpressionContext<'input>{
	fn borrow(&self) -> &Script_op_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Script_op_expressionContextExt<'input>> for SubscriptPowerExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Script_op_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Script_op_expressionContextAttrs<'input> for SubscriptPowerExpressionContext<'input> {}

impl<'input> SubscriptPowerExpressionContextExt<'input>{
	fn new(ctx: &dyn Script_op_expressionContextAttrs<'input>) -> Rc<Script_op_expressionContextAll<'input>>  {
		Rc::new(
			Script_op_expressionContextAll::SubscriptPowerExpressionContext(
				BaseParserRuleContext::copy_from(ctx,SubscriptPowerExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PrimeExpressionContext<'input> = BaseParserRuleContext<'input,PrimeExpressionContextExt<'input>>;

pub trait PrimeExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token PRIME
	/// Returns `None` if there is no child corresponding to token PRIME
	fn PRIME(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(PRIME, 0)
	}
}

impl<'input> PrimeExpressionContextAttrs<'input> for PrimeExpressionContext<'input>{}

pub struct PrimeExpressionContextExt<'input>{
	base:Script_op_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{PrimeExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for PrimeExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for PrimeExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_primeExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_primeExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for PrimeExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_primeExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PrimeExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_script_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_script_op_expression }
}

impl<'input> Borrow<Script_op_expressionContextExt<'input>> for PrimeExpressionContext<'input>{
	fn borrow(&self) -> &Script_op_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Script_op_expressionContextExt<'input>> for PrimeExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Script_op_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Script_op_expressionContextAttrs<'input> for PrimeExpressionContext<'input> {}

impl<'input> PrimeExpressionContextExt<'input>{
	fn new(ctx: &dyn Script_op_expressionContextAttrs<'input>) -> Rc<Script_op_expressionContextAll<'input>>  {
		Rc::new(
			Script_op_expressionContextAll::PrimeExpressionContext(
				BaseParserRuleContext::copy_from(ctx,PrimeExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn script_op_expression(&mut self,)
	-> Result<Rc<Script_op_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Script_op_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_script_op_expression);
        let mut _localctx: Rc<Script_op_expressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(138);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
				1 =>{
					let tmp = PowerSubscriptExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule primary_expression*/
					recog.base.set_state(115);
					recog.primary_expression()?;

					recog.base.set_state(124);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							{
							recog.base.set_state(116);
							recog.base.match_token(HAT,&mut recog.err_handler)?;

							/*InvokeRule primary_expression*/
							recog.base.set_state(117);
							recog.primary_expression()?;

							recog.base.set_state(120);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
								x if x == 1=>{
									{
									recog.base.set_state(118);
									recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

									/*InvokeRule primary_expression*/
									recog.base.set_state(119);
									recog.primary_expression()?;

									}
								}

								_ => {}
							}
							}
							}
							} 
						}
						recog.base.set_state(126);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
					}
					}
				}
			,
				2 =>{
					let tmp = SubscriptPowerExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					{
					recog.base.set_state(127);
					recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(128);
					recog.primary_expression()?;

					recog.base.set_state(131);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(129);
							recog.base.match_token(HAT,&mut recog.err_handler)?;

							/*InvokeRule primary_expression*/
							recog.base.set_state(130);
							recog.primary_expression()?;

							}
						}

						_ => {}
					}
					}
					}
				}
			,
				3 =>{
					let tmp = PowerExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(133);
					recog.base.match_token(HAT,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(134);
					recog.primary_expression()?;

					}
				}
			,
				4 =>{
					let tmp = SubscriptExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(135);
					recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(136);
					recog.primary_expression()?;

					}
				}
			,
				5 =>{
					let tmp = PrimeExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(137);
					recog.base.match_token(PRIME,&mut recog.err_handler)?;

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
//------------------- primary_expression ----------------
#[derive(Debug)]
pub enum Primary_expressionContextAll<'input>{
	BracketMatrixContext(BracketMatrixContext<'input>),
	ExplicitIdentifierCallContext(ExplicitIdentifierCallContext<'input>),
	BraceExpressionContext(BraceExpressionContext<'input>),
	ParenExpressionContext(ParenExpressionContext<'input>),
	IntegralExpressionContext(IntegralExpressionContext<'input>),
	CurrencyNumberAtomContext(CurrencyNumberAtomContext<'input>),
	RootFunctionContext(RootFunctionContext<'input>),
	AbsExpressionContext(AbsExpressionContext<'input>),
	DerivativeFunctionContext(DerivativeFunctionContext<'input>),
	AngleBracketRowVectorContext(AngleBracketRowVectorContext<'input>),
	ExplicitKeywordCallContext(ExplicitKeywordCallContext<'input>),
	IdentifierAtomContext(IdentifierAtomContext<'input>),
	GreekLetterAtomContext(GreekLetterAtomContext<'input>),
	NumberWithCommasAtomContext(NumberWithCommasAtomContext<'input>),
	ParenColumnVectorContext(ParenColumnVectorContext<'input>),
	NumberAtomContext(NumberAtomContext<'input>),
	LimitExpressionContext(LimitExpressionContext<'input>),
	PartialFunctionContext(PartialFunctionContext<'input>),
	DetFunctionContext(DetFunctionContext<'input>),
	ConstantAtomContext(ConstantAtomContext<'input>),
	FracFunctionContext(FracFunctionContext<'input>),
	TransposeFunctionContext(TransposeFunctionContext<'input>),
	FractionLeibnizContext(FractionLeibnizContext<'input>),
	StringAtomContext(StringAtomContext<'input>),
	TextFunctionContext(TextFunctionContext<'input>),
	SqrtFunctionContext(SqrtFunctionContext<'input>),
	MatFunctionContext(MatFunctionContext<'input>),
	ParenMatrixContext(ParenMatrixContext<'input>),
	SimpleKeywordCallContext(SimpleKeywordCallContext<'input>),
Error(Primary_expressionContext<'input>)
}
antlr_rust::tid!{Primary_expressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Primary_expressionContextAll<'input>{}

impl<'input> AsciiMath2ParserContext<'input> for Primary_expressionContextAll<'input>{}

impl<'input> Deref for Primary_expressionContextAll<'input>{
	type Target = dyn Primary_expressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use Primary_expressionContextAll::*;
		match self{
			BracketMatrixContext(inner) => inner,
			ExplicitIdentifierCallContext(inner) => inner,
			BraceExpressionContext(inner) => inner,
			ParenExpressionContext(inner) => inner,
			IntegralExpressionContext(inner) => inner,
			CurrencyNumberAtomContext(inner) => inner,
			RootFunctionContext(inner) => inner,
			AbsExpressionContext(inner) => inner,
			DerivativeFunctionContext(inner) => inner,
			AngleBracketRowVectorContext(inner) => inner,
			ExplicitKeywordCallContext(inner) => inner,
			IdentifierAtomContext(inner) => inner,
			GreekLetterAtomContext(inner) => inner,
			NumberWithCommasAtomContext(inner) => inner,
			ParenColumnVectorContext(inner) => inner,
			NumberAtomContext(inner) => inner,
			LimitExpressionContext(inner) => inner,
			PartialFunctionContext(inner) => inner,
			DetFunctionContext(inner) => inner,
			ConstantAtomContext(inner) => inner,
			FracFunctionContext(inner) => inner,
			TransposeFunctionContext(inner) => inner,
			FractionLeibnizContext(inner) => inner,
			StringAtomContext(inner) => inner,
			TextFunctionContext(inner) => inner,
			SqrtFunctionContext(inner) => inner,
			MatFunctionContext(inner) => inner,
			ParenMatrixContext(inner) => inner,
			SimpleKeywordCallContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Primary_expressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Primary_expressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type Primary_expressionContext<'input> = BaseParserRuleContext<'input,Primary_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Primary_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Primary_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Primary_expressionContext<'input>{
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Primary_expressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for Primary_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}
antlr_rust::tid!{Primary_expressionContextExt<'a>}

impl<'input> Primary_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Primary_expressionContextAll<'input>> {
		Rc::new(
		Primary_expressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Primary_expressionContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait Primary_expressionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Primary_expressionContextExt<'input>>{


}

impl<'input> Primary_expressionContextAttrs<'input> for Primary_expressionContext<'input>{}

pub type BracketMatrixContext<'input> = BaseParserRuleContext<'input,BracketMatrixContextExt<'input>>;

pub trait BracketMatrixContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LBRACKET
	/// Returns `None` if there is no child corresponding to token LBRACKET
	fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(LBRACKET, 0)
	}
	fn matrix_content(&self) -> Option<Rc<Matrix_contentContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACKET
	/// Returns `None` if there is no child corresponding to token RBRACKET
	fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(RBRACKET, 0)
	}
}

impl<'input> BracketMatrixContextAttrs<'input> for BracketMatrixContext<'input>{}

pub struct BracketMatrixContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BracketMatrixContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for BracketMatrixContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for BracketMatrixContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_bracketMatrix(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_bracketMatrix(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for BracketMatrixContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_bracketMatrix(self);
	}
}

impl<'input> CustomRuleContext<'input> for BracketMatrixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for BracketMatrixContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for BracketMatrixContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for BracketMatrixContext<'input> {}

impl<'input> BracketMatrixContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::BracketMatrixContext(
				BaseParserRuleContext::copy_from(ctx,BracketMatrixContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExplicitIdentifierCallContext<'input> = BaseParserRuleContext<'input,ExplicitIdentifierCallContextExt<'input>>;

pub trait ExplicitIdentifierCallContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn function_call(&self) -> Option<Rc<Function_callContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENTIFIER
	/// Returns `None` if there is no child corresponding to token IDENTIFIER
	fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(IDENTIFIER, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token PRIME in current rule
	fn PRIME_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token PRIME, starting from 0.
	/// Returns `None` if number of children corresponding to token PRIME is less or equal than `i`.
	fn PRIME(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(PRIME, i)
	}
}

impl<'input> ExplicitIdentifierCallContextAttrs<'input> for ExplicitIdentifierCallContext<'input>{}

pub struct ExplicitIdentifierCallContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExplicitIdentifierCallContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for ExplicitIdentifierCallContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for ExplicitIdentifierCallContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_explicitIdentifierCall(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_explicitIdentifierCall(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for ExplicitIdentifierCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_explicitIdentifierCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExplicitIdentifierCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for ExplicitIdentifierCallContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for ExplicitIdentifierCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for ExplicitIdentifierCallContext<'input> {}

impl<'input> ExplicitIdentifierCallContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::ExplicitIdentifierCallContext(
				BaseParserRuleContext::copy_from(ctx,ExplicitIdentifierCallContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BraceExpressionContext<'input> = BaseParserRuleContext<'input,BraceExpressionContextExt<'input>>;

pub trait BraceExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LBRACE
	/// Returns `None` if there is no child corresponding to token LBRACE
	fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(LBRACE, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACE
	/// Returns `None` if there is no child corresponding to token RBRACE
	fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(RBRACE, 0)
	}
}

impl<'input> BraceExpressionContextAttrs<'input> for BraceExpressionContext<'input>{}

pub struct BraceExpressionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BraceExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for BraceExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for BraceExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_braceExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_braceExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for BraceExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_braceExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for BraceExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for BraceExpressionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for BraceExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for BraceExpressionContext<'input> {}

impl<'input> BraceExpressionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::BraceExpressionContext(
				BaseParserRuleContext::copy_from(ctx,BraceExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParenExpressionContext<'input> = BaseParserRuleContext<'input,ParenExpressionContextExt<'input>>;

pub trait ParenExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
}

impl<'input> ParenExpressionContextAttrs<'input> for ParenExpressionContext<'input>{}

pub struct ParenExpressionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParenExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for ParenExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for ParenExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parenExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_parenExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for ParenExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_parenExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParenExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for ParenExpressionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for ParenExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for ParenExpressionContext<'input> {}

impl<'input> ParenExpressionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::ParenExpressionContext(
				BaseParserRuleContext::copy_from(ctx,ParenExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntegralExpressionContext<'input> = BaseParserRuleContext<'input,IntegralExpressionContextExt<'input>>;

pub trait IntegralExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INTEGRAL
	/// Returns `None` if there is no child corresponding to token INTEGRAL
	fn INTEGRAL(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(INTEGRAL, 0)
	}
	fn primary_expression_all(&self) ->  Vec<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn primary_expression(&self, i: usize) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token UNDERSCORE
	/// Returns `None` if there is no child corresponding to token UNDERSCORE
	fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(UNDERSCORE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token HAT
	/// Returns `None` if there is no child corresponding to token HAT
	fn HAT(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(HAT, 0)
	}
	fn differential(&self) -> Option<Rc<DifferentialContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> IntegralExpressionContextAttrs<'input> for IntegralExpressionContext<'input>{}

pub struct IntegralExpressionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IntegralExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for IntegralExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for IntegralExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_integralExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_integralExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for IntegralExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_integralExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntegralExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for IntegralExpressionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for IntegralExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for IntegralExpressionContext<'input> {}

impl<'input> IntegralExpressionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::IntegralExpressionContext(
				BaseParserRuleContext::copy_from(ctx,IntegralExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CurrencyNumberAtomContext<'input> = BaseParserRuleContext<'input,CurrencyNumberAtomContextExt<'input>>;

pub trait CurrencyNumberAtomContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token CURRENCY_NUMBER
	/// Returns `None` if there is no child corresponding to token CURRENCY_NUMBER
	fn CURRENCY_NUMBER(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(CURRENCY_NUMBER, 0)
	}
}

impl<'input> CurrencyNumberAtomContextAttrs<'input> for CurrencyNumberAtomContext<'input>{}

pub struct CurrencyNumberAtomContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CurrencyNumberAtomContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for CurrencyNumberAtomContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for CurrencyNumberAtomContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_currencyNumberAtom(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_currencyNumberAtom(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for CurrencyNumberAtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_currencyNumberAtom(self);
	}
}

impl<'input> CustomRuleContext<'input> for CurrencyNumberAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for CurrencyNumberAtomContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for CurrencyNumberAtomContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for CurrencyNumberAtomContext<'input> {}

impl<'input> CurrencyNumberAtomContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::CurrencyNumberAtomContext(
				BaseParserRuleContext::copy_from(ctx,CurrencyNumberAtomContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type RootFunctionContext<'input> = BaseParserRuleContext<'input,RootFunctionContextExt<'input>>;

pub trait RootFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ROOT
	/// Returns `None` if there is no child corresponding to token ROOT
	fn ROOT(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(ROOT, 0)
	}
	fn primary_expression_all(&self) ->  Vec<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn primary_expression(&self, i: usize) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> RootFunctionContextAttrs<'input> for RootFunctionContext<'input>{}

pub struct RootFunctionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{RootFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for RootFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for RootFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_rootFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_rootFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for RootFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_rootFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for RootFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for RootFunctionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for RootFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for RootFunctionContext<'input> {}

impl<'input> RootFunctionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::RootFunctionContext(
				BaseParserRuleContext::copy_from(ctx,RootFunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AbsExpressionContext<'input> = BaseParserRuleContext<'input,AbsExpressionContextExt<'input>>;

pub trait AbsExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves all `TerminalNode`s corresponding to token ABS in current rule
	fn ABS_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token ABS, starting from 0.
	/// Returns `None` if number of children corresponding to token ABS is less or equal than `i`.
	fn ABS(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(ABS, i)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AbsExpressionContextAttrs<'input> for AbsExpressionContext<'input>{}

pub struct AbsExpressionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AbsExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for AbsExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for AbsExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_absExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_absExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for AbsExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_absExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for AbsExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for AbsExpressionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for AbsExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for AbsExpressionContext<'input> {}

impl<'input> AbsExpressionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::AbsExpressionContext(
				BaseParserRuleContext::copy_from(ctx,AbsExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DerivativeFunctionContext<'input> = BaseParserRuleContext<'input,DerivativeFunctionContextExt<'input>>;

pub trait DerivativeFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn derivative(&self) -> Option<Rc<DerivativeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DerivativeFunctionContextAttrs<'input> for DerivativeFunctionContext<'input>{}

pub struct DerivativeFunctionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DerivativeFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for DerivativeFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for DerivativeFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_derivativeFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_derivativeFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for DerivativeFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_derivativeFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for DerivativeFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for DerivativeFunctionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for DerivativeFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for DerivativeFunctionContext<'input> {}

impl<'input> DerivativeFunctionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::DerivativeFunctionContext(
				BaseParserRuleContext::copy_from(ctx,DerivativeFunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AngleBracketRowVectorContext<'input> = BaseParserRuleContext<'input,AngleBracketRowVectorContextExt<'input>>;

pub trait AngleBracketRowVectorContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token L_ANGLE
	/// Returns `None` if there is no child corresponding to token L_ANGLE
	fn L_ANGLE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(L_ANGLE, 0)
	}
	fn matrix_row(&self) -> Option<Rc<Matrix_rowContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token R_ANGLE
	/// Returns `None` if there is no child corresponding to token R_ANGLE
	fn R_ANGLE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(R_ANGLE, 0)
	}
}

impl<'input> AngleBracketRowVectorContextAttrs<'input> for AngleBracketRowVectorContext<'input>{}

pub struct AngleBracketRowVectorContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AngleBracketRowVectorContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for AngleBracketRowVectorContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for AngleBracketRowVectorContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_angleBracketRowVector(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_angleBracketRowVector(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for AngleBracketRowVectorContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_angleBracketRowVector(self);
	}
}

impl<'input> CustomRuleContext<'input> for AngleBracketRowVectorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for AngleBracketRowVectorContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for AngleBracketRowVectorContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for AngleBracketRowVectorContext<'input> {}

impl<'input> AngleBracketRowVectorContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::AngleBracketRowVectorContext(
				BaseParserRuleContext::copy_from(ctx,AngleBracketRowVectorContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExplicitKeywordCallContext<'input> = BaseParserRuleContext<'input,ExplicitKeywordCallContextExt<'input>>;

pub trait ExplicitKeywordCallContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn keyword_func(&self) -> Option<Rc<Keyword_funcContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExplicitKeywordCallContextAttrs<'input> for ExplicitKeywordCallContext<'input>{}

pub struct ExplicitKeywordCallContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExplicitKeywordCallContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for ExplicitKeywordCallContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for ExplicitKeywordCallContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_explicitKeywordCall(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_explicitKeywordCall(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for ExplicitKeywordCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_explicitKeywordCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExplicitKeywordCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for ExplicitKeywordCallContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for ExplicitKeywordCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for ExplicitKeywordCallContext<'input> {}

impl<'input> ExplicitKeywordCallContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::ExplicitKeywordCallContext(
				BaseParserRuleContext::copy_from(ctx,ExplicitKeywordCallContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdentifierAtomContext<'input> = BaseParserRuleContext<'input,IdentifierAtomContextExt<'input>>;

pub trait IdentifierAtomContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENTIFIER
	/// Returns `None` if there is no child corresponding to token IDENTIFIER
	fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(IDENTIFIER, 0)
	}
}

impl<'input> IdentifierAtomContextAttrs<'input> for IdentifierAtomContext<'input>{}

pub struct IdentifierAtomContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IdentifierAtomContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for IdentifierAtomContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for IdentifierAtomContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_identifierAtom(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_identifierAtom(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for IdentifierAtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_identifierAtom(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentifierAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for IdentifierAtomContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for IdentifierAtomContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for IdentifierAtomContext<'input> {}

impl<'input> IdentifierAtomContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::IdentifierAtomContext(
				BaseParserRuleContext::copy_from(ctx,IdentifierAtomContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type GreekLetterAtomContext<'input> = BaseParserRuleContext<'input,GreekLetterAtomContextExt<'input>>;

pub trait GreekLetterAtomContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token GREEK_LETTER
	/// Returns `None` if there is no child corresponding to token GREEK_LETTER
	fn GREEK_LETTER(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(GREEK_LETTER, 0)
	}
}

impl<'input> GreekLetterAtomContextAttrs<'input> for GreekLetterAtomContext<'input>{}

pub struct GreekLetterAtomContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{GreekLetterAtomContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for GreekLetterAtomContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for GreekLetterAtomContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_greekLetterAtom(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_greekLetterAtom(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for GreekLetterAtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_greekLetterAtom(self);
	}
}

impl<'input> CustomRuleContext<'input> for GreekLetterAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for GreekLetterAtomContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for GreekLetterAtomContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for GreekLetterAtomContext<'input> {}

impl<'input> GreekLetterAtomContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::GreekLetterAtomContext(
				BaseParserRuleContext::copy_from(ctx,GreekLetterAtomContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NumberWithCommasAtomContext<'input> = BaseParserRuleContext<'input,NumberWithCommasAtomContextExt<'input>>;

pub trait NumberWithCommasAtomContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NUMBER_WITH_COMMAS
	/// Returns `None` if there is no child corresponding to token NUMBER_WITH_COMMAS
	fn NUMBER_WITH_COMMAS(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(NUMBER_WITH_COMMAS, 0)
	}
}

impl<'input> NumberWithCommasAtomContextAttrs<'input> for NumberWithCommasAtomContext<'input>{}

pub struct NumberWithCommasAtomContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NumberWithCommasAtomContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for NumberWithCommasAtomContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for NumberWithCommasAtomContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_numberWithCommasAtom(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_numberWithCommasAtom(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for NumberWithCommasAtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_numberWithCommasAtom(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberWithCommasAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for NumberWithCommasAtomContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for NumberWithCommasAtomContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for NumberWithCommasAtomContext<'input> {}

impl<'input> NumberWithCommasAtomContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::NumberWithCommasAtomContext(
				BaseParserRuleContext::copy_from(ctx,NumberWithCommasAtomContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParenColumnVectorContext<'input> = BaseParserRuleContext<'input,ParenColumnVectorContextExt<'input>>;

pub trait ParenColumnVectorContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	fn paren_element_for_column_vector_all(&self) ->  Vec<Rc<Paren_element_for_column_vectorContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn paren_element_for_column_vector(&self, i: usize) -> Option<Rc<Paren_element_for_column_vectorContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> ParenColumnVectorContextAttrs<'input> for ParenColumnVectorContext<'input>{}

pub struct ParenColumnVectorContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParenColumnVectorContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for ParenColumnVectorContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for ParenColumnVectorContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parenColumnVector(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_parenColumnVector(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for ParenColumnVectorContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_parenColumnVector(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParenColumnVectorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for ParenColumnVectorContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for ParenColumnVectorContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for ParenColumnVectorContext<'input> {}

impl<'input> ParenColumnVectorContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::ParenColumnVectorContext(
				BaseParserRuleContext::copy_from(ctx,ParenColumnVectorContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NumberAtomContext<'input> = BaseParserRuleContext<'input,NumberAtomContextExt<'input>>;

pub trait NumberAtomContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NUMBER
	/// Returns `None` if there is no child corresponding to token NUMBER
	fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(NUMBER, 0)
	}
}

impl<'input> NumberAtomContextAttrs<'input> for NumberAtomContext<'input>{}

pub struct NumberAtomContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NumberAtomContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for NumberAtomContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for NumberAtomContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_numberAtom(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_numberAtom(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for NumberAtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_numberAtom(self);
	}
}

impl<'input> CustomRuleContext<'input> for NumberAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for NumberAtomContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for NumberAtomContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for NumberAtomContext<'input> {}

impl<'input> NumberAtomContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::NumberAtomContext(
				BaseParserRuleContext::copy_from(ctx,NumberAtomContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LimitExpressionContext<'input> = BaseParserRuleContext<'input,LimitExpressionContextExt<'input>>;

pub trait LimitExpressionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LIM
	/// Returns `None` if there is no child corresponding to token LIM
	fn LIM(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(LIM, 0)
	}
	/// Retrieves first TerminalNode corresponding to token UNDERSCORE
	/// Returns `None` if there is no child corresponding to token UNDERSCORE
	fn UNDERSCORE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(UNDERSCORE, 0)
	}
	fn primary_expression_all(&self) ->  Vec<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn primary_expression(&self, i: usize) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token TO
	/// Returns `None` if there is no child corresponding to token TO
	fn TO(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(TO, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RARROW
	/// Returns `None` if there is no child corresponding to token RARROW
	fn RARROW(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(RARROW, 0)
	}
}

impl<'input> LimitExpressionContextAttrs<'input> for LimitExpressionContext<'input>{}

pub struct LimitExpressionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LimitExpressionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for LimitExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for LimitExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_limitExpression(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_limitExpression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for LimitExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_limitExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for LimitExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for LimitExpressionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for LimitExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for LimitExpressionContext<'input> {}

impl<'input> LimitExpressionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::LimitExpressionContext(
				BaseParserRuleContext::copy_from(ctx,LimitExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PartialFunctionContext<'input> = BaseParserRuleContext<'input,PartialFunctionContextExt<'input>>;

pub trait PartialFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn partial_derivative(&self) -> Option<Rc<Partial_derivativeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> PartialFunctionContextAttrs<'input> for PartialFunctionContext<'input>{}

pub struct PartialFunctionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{PartialFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for PartialFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for PartialFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_partialFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_partialFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for PartialFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_partialFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for PartialFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for PartialFunctionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for PartialFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for PartialFunctionContext<'input> {}

impl<'input> PartialFunctionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::PartialFunctionContext(
				BaseParserRuleContext::copy_from(ctx,PartialFunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DetFunctionContext<'input> = BaseParserRuleContext<'input,DetFunctionContextExt<'input>>;

pub trait DetFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token DET
	/// Returns `None` if there is no child corresponding to token DET
	fn DET(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(DET, 0)
	}
	fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DetFunctionContextAttrs<'input> for DetFunctionContext<'input>{}

pub struct DetFunctionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DetFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for DetFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for DetFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_detFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_detFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for DetFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_detFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for DetFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for DetFunctionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for DetFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for DetFunctionContext<'input> {}

impl<'input> DetFunctionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::DetFunctionContext(
				BaseParserRuleContext::copy_from(ctx,DetFunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ConstantAtomContext<'input> = BaseParserRuleContext<'input,ConstantAtomContextExt<'input>>;

pub trait ConstantAtomContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn constant_symbol(&self) -> Option<Rc<Constant_symbolContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ConstantAtomContextAttrs<'input> for ConstantAtomContext<'input>{}

pub struct ConstantAtomContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ConstantAtomContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for ConstantAtomContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for ConstantAtomContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_constantAtom(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_constantAtom(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for ConstantAtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_constantAtom(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstantAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for ConstantAtomContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for ConstantAtomContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for ConstantAtomContext<'input> {}

impl<'input> ConstantAtomContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::ConstantAtomContext(
				BaseParserRuleContext::copy_from(ctx,ConstantAtomContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FracFunctionContext<'input> = BaseParserRuleContext<'input,FracFunctionContextExt<'input>>;

pub trait FracFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FRAC
	/// Returns `None` if there is no child corresponding to token FRAC
	fn FRAC(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(FRAC, 0)
	}
	fn primary_expression_all(&self) ->  Vec<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn primary_expression(&self, i: usize) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> FracFunctionContextAttrs<'input> for FracFunctionContext<'input>{}

pub struct FracFunctionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FracFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for FracFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for FracFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fracFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_fracFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for FracFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_fracFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for FracFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for FracFunctionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for FracFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for FracFunctionContext<'input> {}

impl<'input> FracFunctionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::FracFunctionContext(
				BaseParserRuleContext::copy_from(ctx,FracFunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TransposeFunctionContext<'input> = BaseParserRuleContext<'input,TransposeFunctionContextExt<'input>>;

pub trait TransposeFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token TRANSPOSE
	/// Returns `None` if there is no child corresponding to token TRANSPOSE
	fn TRANSPOSE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(TRANSPOSE, 0)
	}
	fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> TransposeFunctionContextAttrs<'input> for TransposeFunctionContext<'input>{}

pub struct TransposeFunctionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TransposeFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for TransposeFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for TransposeFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_transposeFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_transposeFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for TransposeFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_transposeFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for TransposeFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for TransposeFunctionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for TransposeFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for TransposeFunctionContext<'input> {}

impl<'input> TransposeFunctionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::TransposeFunctionContext(
				BaseParserRuleContext::copy_from(ctx,TransposeFunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FractionLeibnizContext<'input> = BaseParserRuleContext<'input,FractionLeibnizContextExt<'input>>;

pub trait FractionLeibnizContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn differential_all(&self) ->  Vec<Rc<DifferentialContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn differential(&self, i: usize) -> Option<Rc<DifferentialContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token FSLASH
	/// Returns `None` if there is no child corresponding to token FSLASH
	fn FSLASH(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(FSLASH, 0)
	}
}

impl<'input> FractionLeibnizContextAttrs<'input> for FractionLeibnizContext<'input>{}

pub struct FractionLeibnizContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FractionLeibnizContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for FractionLeibnizContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for FractionLeibnizContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_fractionLeibniz(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_fractionLeibniz(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for FractionLeibnizContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_fractionLeibniz(self);
	}
}

impl<'input> CustomRuleContext<'input> for FractionLeibnizContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for FractionLeibnizContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for FractionLeibnizContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for FractionLeibnizContext<'input> {}

impl<'input> FractionLeibnizContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::FractionLeibnizContext(
				BaseParserRuleContext::copy_from(ctx,FractionLeibnizContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StringAtomContext<'input> = BaseParserRuleContext<'input,StringAtomContextExt<'input>>;

pub trait StringAtomContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING
	/// Returns `None` if there is no child corresponding to token STRING
	fn STRING(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(STRING, 0)
	}
}

impl<'input> StringAtomContextAttrs<'input> for StringAtomContext<'input>{}

pub struct StringAtomContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StringAtomContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for StringAtomContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for StringAtomContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_stringAtom(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_stringAtom(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for StringAtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_stringAtom(self);
	}
}

impl<'input> CustomRuleContext<'input> for StringAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for StringAtomContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for StringAtomContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for StringAtomContext<'input> {}

impl<'input> StringAtomContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::StringAtomContext(
				BaseParserRuleContext::copy_from(ctx,StringAtomContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TextFunctionContext<'input> = BaseParserRuleContext<'input,TextFunctionContextExt<'input>>;

pub trait TextFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token TEXT
	/// Returns `None` if there is no child corresponding to token TEXT
	fn TEXT(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(TEXT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	fn text_argument(&self) -> Option<Rc<Text_argumentContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
}

impl<'input> TextFunctionContextAttrs<'input> for TextFunctionContext<'input>{}

pub struct TextFunctionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TextFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for TextFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for TextFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_textFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_textFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for TextFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_textFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for TextFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for TextFunctionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for TextFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for TextFunctionContext<'input> {}

impl<'input> TextFunctionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::TextFunctionContext(
				BaseParserRuleContext::copy_from(ctx,TextFunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SqrtFunctionContext<'input> = BaseParserRuleContext<'input,SqrtFunctionContextExt<'input>>;

pub trait SqrtFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token SQRT
	/// Returns `None` if there is no child corresponding to token SQRT
	fn SQRT(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(SQRT, 0)
	}
	fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SqrtFunctionContextAttrs<'input> for SqrtFunctionContext<'input>{}

pub struct SqrtFunctionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SqrtFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for SqrtFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for SqrtFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_sqrtFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_sqrtFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for SqrtFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_sqrtFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for SqrtFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for SqrtFunctionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for SqrtFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for SqrtFunctionContext<'input> {}

impl<'input> SqrtFunctionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::SqrtFunctionContext(
				BaseParserRuleContext::copy_from(ctx,SqrtFunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MatFunctionContext<'input> = BaseParserRuleContext<'input,MatFunctionContextExt<'input>>;

pub trait MatFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MAT
	/// Returns `None` if there is no child corresponding to token MAT
	fn MAT(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(MAT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	fn matrix_content(&self) -> Option<Rc<Matrix_contentContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
}

impl<'input> MatFunctionContextAttrs<'input> for MatFunctionContext<'input>{}

pub struct MatFunctionContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MatFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for MatFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for MatFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_matFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_matFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for MatFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_matFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for MatFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for MatFunctionContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for MatFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for MatFunctionContext<'input> {}

impl<'input> MatFunctionContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::MatFunctionContext(
				BaseParserRuleContext::copy_from(ctx,MatFunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParenMatrixContext<'input> = BaseParserRuleContext<'input,ParenMatrixContextExt<'input>>;

pub trait ParenMatrixContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(LPAREN, 0)
	}
	fn matrix_content(&self) -> Option<Rc<Matrix_contentContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(RPAREN, 0)
	}
}

impl<'input> ParenMatrixContextAttrs<'input> for ParenMatrixContext<'input>{}

pub struct ParenMatrixContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParenMatrixContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for ParenMatrixContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for ParenMatrixContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_parenMatrix(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_parenMatrix(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for ParenMatrixContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_parenMatrix(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParenMatrixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for ParenMatrixContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for ParenMatrixContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for ParenMatrixContext<'input> {}

impl<'input> ParenMatrixContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::ParenMatrixContext(
				BaseParserRuleContext::copy_from(ctx,ParenMatrixContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SimpleKeywordCallContext<'input> = BaseParserRuleContext<'input,SimpleKeywordCallContextExt<'input>>;

pub trait SimpleKeywordCallContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn simple_keyword_func(&self) -> Option<Rc<Simple_keyword_funcContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> SimpleKeywordCallContextAttrs<'input> for SimpleKeywordCallContext<'input>{}

pub struct SimpleKeywordCallContextExt<'input>{
	base:Primary_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SimpleKeywordCallContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for SimpleKeywordCallContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for SimpleKeywordCallContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_simpleKeywordCall(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_simpleKeywordCall(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for SimpleKeywordCallContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_simpleKeywordCall(self);
	}
}

impl<'input> CustomRuleContext<'input> for SimpleKeywordCallContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}

impl<'input> Borrow<Primary_expressionContextExt<'input>> for SimpleKeywordCallContext<'input>{
	fn borrow(&self) -> &Primary_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Primary_expressionContextExt<'input>> for SimpleKeywordCallContext<'input>{
	fn borrow_mut(&mut self) -> &mut Primary_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Primary_expressionContextAttrs<'input> for SimpleKeywordCallContext<'input> {}

impl<'input> SimpleKeywordCallContextExt<'input>{
	fn new(ctx: &dyn Primary_expressionContextAttrs<'input>) -> Rc<Primary_expressionContextAll<'input>>  {
		Rc::new(
			Primary_expressionContextAll::SimpleKeywordCallContext(
				BaseParserRuleContext::copy_from(ctx,SimpleKeywordCallContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primary_expression(&mut self,)
	-> Result<Rc<Primary_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Primary_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_primary_expression);
        let mut _localctx: Rc<Primary_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(247);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(20,&mut recog.base)? {
				1 =>{
					let tmp = ExplicitIdentifierCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule function_call*/
					recog.base.set_state(140);
					recog.function_call()?;

					}
				}
			,
				2 =>{
					let tmp = ParenColumnVectorContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(141);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule paren_element_for_column_vector*/
					recog.base.set_state(142);
					recog.paren_element_for_column_vector()?;

					recog.base.set_state(147);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(143);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule paren_element_for_column_vector*/
						recog.base.set_state(144);
						recog.paren_element_for_column_vector()?;

						}
						}
						recog.base.set_state(149);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(150);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = ParenMatrixContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(152);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule matrix_content*/
					recog.base.set_state(153);
					recog.matrix_content()?;

					recog.base.set_state(154);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					let tmp = ParenExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(156);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(157);
					recog.expression()?;

					recog.base.set_state(158);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					let tmp = BracketMatrixContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(160);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					/*InvokeRule matrix_content*/
					recog.base.set_state(161);
					recog.matrix_content()?;

					recog.base.set_state(162);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					let tmp = AngleBracketRowVectorContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(164);
					recog.base.match_token(L_ANGLE,&mut recog.err_handler)?;

					/*InvokeRule matrix_row*/
					recog.base.set_state(165);
					recog.matrix_row()?;

					recog.base.set_state(166);
					recog.base.match_token(R_ANGLE,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					let tmp = BraceExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(168);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(169);
					recog.expression()?;

					recog.base.set_state(170);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					let tmp = AbsExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(172);
					recog.base.match_token(ABS,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(173);
					recog.expression()?;

					recog.base.set_state(174);
					recog.base.match_token(ABS,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					let tmp = ExplicitIdentifierCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9);
					_localctx = tmp;
					{
					recog.base.set_state(176);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(182);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PRIME {
						{
						recog.base.set_state(178); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						loop {
							{
							{
							recog.base.set_state(177);
							recog.base.match_token(PRIME,&mut recog.err_handler)?;

							}
							}
							recog.base.set_state(180); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if !(_la==PRIME) {break}
						}
						}
					}

					recog.base.set_state(184);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule arguments*/
					recog.base.set_state(185);
					recog.arguments()?;

					recog.base.set_state(186);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				10 =>{
					let tmp = ExplicitKeywordCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10);
					_localctx = tmp;
					{
					/*InvokeRule keyword_func*/
					recog.base.set_state(188);
					recog.keyword_func()?;

					}
				}
			,
				11 =>{
					let tmp = SimpleKeywordCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11);
					_localctx = tmp;
					{
					/*InvokeRule simple_keyword_func*/
					recog.base.set_state(189);
					recog.simple_keyword_func()?;

					}
				}
			,
				12 =>{
					let tmp = SqrtFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12);
					_localctx = tmp;
					{
					recog.base.set_state(190);
					recog.base.match_token(SQRT,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(191);
					recog.primary_expression()?;

					}
				}
			,
				13 =>{
					let tmp = RootFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 13);
					_localctx = tmp;
					{
					recog.base.set_state(192);
					recog.base.match_token(ROOT,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(193);
					recog.primary_expression()?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(194);
					recog.primary_expression()?;

					}
				}
			,
				14 =>{
					let tmp = FracFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 14);
					_localctx = tmp;
					{
					recog.base.set_state(196);
					recog.base.match_token(FRAC,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(197);
					recog.primary_expression()?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(198);
					recog.primary_expression()?;

					}
				}
			,
				15 =>{
					let tmp = TextFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 15);
					_localctx = tmp;
					{
					recog.base.set_state(200);
					recog.base.match_token(TEXT,&mut recog.err_handler)?;

					recog.base.set_state(201);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule text_argument*/
					recog.base.set_state(202);
					recog.text_argument()?;

					recog.base.set_state(203);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				16 =>{
					let tmp = IntegralExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 16);
					_localctx = tmp;
					{
					recog.base.set_state(205);
					recog.base.match_token(INTEGRAL,&mut recog.err_handler)?;

					recog.base.set_state(208);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==UNDERSCORE {
						{
						recog.base.set_state(206);
						recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

						/*InvokeRule primary_expression*/
						recog.base.set_state(207);
						recog.primary_expression()?;

						}
					}

					recog.base.set_state(212);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==HAT {
						{
						recog.base.set_state(210);
						recog.base.match_token(HAT,&mut recog.err_handler)?;

						/*InvokeRule primary_expression*/
						recog.base.set_state(211);
						recog.primary_expression()?;

						}
					}

					/*InvokeRule primary_expression*/
					recog.base.set_state(214);
					recog.primary_expression()?;

					recog.base.set_state(216);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule differential*/
							recog.base.set_state(215);
							recog.differential()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				17 =>{
					let tmp = DerivativeFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 17);
					_localctx = tmp;
					{
					/*InvokeRule derivative*/
					recog.base.set_state(218);
					recog.derivative()?;

					}
				}
			,
				18 =>{
					let tmp = PartialFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 18);
					_localctx = tmp;
					{
					/*InvokeRule partial_derivative*/
					recog.base.set_state(219);
					recog.partial_derivative()?;

					}
				}
			,
				19 =>{
					let tmp = FractionLeibnizContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 19);
					_localctx = tmp;
					{
					/*InvokeRule differential*/
					recog.base.set_state(220);
					recog.differential()?;

					recog.base.set_state(221);
					recog.base.match_token(FSLASH,&mut recog.err_handler)?;

					/*InvokeRule differential*/
					recog.base.set_state(222);
					recog.differential()?;

					}
				}
			,
				20 =>{
					let tmp = LimitExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 20);
					_localctx = tmp;
					{
					recog.base.set_state(224);
					recog.base.match_token(LIM,&mut recog.err_handler)?;

					recog.base.set_state(225);
					recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(226);
					recog.primary_expression()?;

					recog.base.set_state(227);
					_la = recog.base.input.la(1);
					if { !(_la==TO || _la==RARROW) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule primary_expression*/
					recog.base.set_state(228);
					recog.primary_expression()?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(229);
					recog.primary_expression()?;

					}
				}
			,
				21 =>{
					let tmp = MatFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 21);
					_localctx = tmp;
					{
					recog.base.set_state(231);
					recog.base.match_token(MAT,&mut recog.err_handler)?;

					recog.base.set_state(232);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule matrix_content*/
					recog.base.set_state(233);
					recog.matrix_content()?;

					recog.base.set_state(234);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				22 =>{
					let tmp = DetFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 22);
					_localctx = tmp;
					{
					recog.base.set_state(236);
					recog.base.match_token(DET,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(237);
					recog.primary_expression()?;

					}
				}
			,
				23 =>{
					let tmp = TransposeFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 23);
					_localctx = tmp;
					{
					recog.base.set_state(238);
					recog.base.match_token(TRANSPOSE,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(239);
					recog.primary_expression()?;

					}
				}
			,
				24 =>{
					let tmp = IdentifierAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 24);
					_localctx = tmp;
					{
					recog.base.set_state(240);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				25 =>{
					let tmp = NumberAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 25);
					_localctx = tmp;
					{
					recog.base.set_state(241);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}
			,
				26 =>{
					let tmp = NumberWithCommasAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 26);
					_localctx = tmp;
					{
					recog.base.set_state(242);
					recog.base.match_token(NUMBER_WITH_COMMAS,&mut recog.err_handler)?;

					}
				}
			,
				27 =>{
					let tmp = CurrencyNumberAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 27);
					_localctx = tmp;
					{
					recog.base.set_state(243);
					recog.base.match_token(CURRENCY_NUMBER,&mut recog.err_handler)?;

					}
				}
			,
				28 =>{
					let tmp = GreekLetterAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 28);
					_localctx = tmp;
					{
					recog.base.set_state(244);
					recog.base.match_token(GREEK_LETTER,&mut recog.err_handler)?;

					}
				}
			,
				29 =>{
					let tmp = ConstantAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 29);
					_localctx = tmp;
					{
					/*InvokeRule constant_symbol*/
					recog.base.set_state(245);
					recog.constant_symbol()?;

					}
				}
			,
				30 =>{
					let tmp = StringAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 30);
					_localctx = tmp;
					{
					recog.base.set_state(246);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

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
//------------------- paren_element_for_column_vector ----------------
pub type Paren_element_for_column_vectorContextAll<'input> = Paren_element_for_column_vectorContext<'input>;


pub type Paren_element_for_column_vectorContext<'input> = BaseParserRuleContext<'input,Paren_element_for_column_vectorContextExt<'input>>;

#[derive(Clone)]
pub struct Paren_element_for_column_vectorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Paren_element_for_column_vectorContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Paren_element_for_column_vectorContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_paren_element_for_column_vector(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_paren_element_for_column_vector(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Paren_element_for_column_vectorContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_paren_element_for_column_vector(self);
	}
}

impl<'input> CustomRuleContext<'input> for Paren_element_for_column_vectorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paren_element_for_column_vector }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paren_element_for_column_vector }
}
antlr_rust::tid!{Paren_element_for_column_vectorContextExt<'a>}

impl<'input> Paren_element_for_column_vectorContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Paren_element_for_column_vectorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Paren_element_for_column_vectorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Paren_element_for_column_vectorContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Paren_element_for_column_vectorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> Paren_element_for_column_vectorContextAttrs<'input> for Paren_element_for_column_vectorContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paren_element_for_column_vector(&mut self,)
	-> Result<Rc<Paren_element_for_column_vectorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Paren_element_for_column_vectorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_paren_element_for_column_vector);
        let mut _localctx: Rc<Paren_element_for_column_vectorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(249);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(250);
			recog.expression()?;

			recog.base.set_state(251);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- arguments ----------------
pub type ArgumentsContextAll<'input> = ArgumentsContext<'input>;


pub type ArgumentsContext<'input> = BaseParserRuleContext<'input,ArgumentsContextExt<'input>>;

#[derive(Clone)]
pub struct ArgumentsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for ArgumentsContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for ArgumentsContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_arguments(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_arguments(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for ArgumentsContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_arguments(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArgumentsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arguments }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arguments }
}
antlr_rust::tid!{ArgumentsContextExt<'a>}

impl<'input> ArgumentsContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArgumentsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArgumentsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ArgumentsContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<ArgumentsContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> ArgumentsContextAttrs<'input> for ArgumentsContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arguments(&mut self,)
	-> Result<Rc<ArgumentsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArgumentsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_arguments);
        let mut _localctx: Rc<ArgumentsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(253);
			recog.expression()?;

			recog.base.set_state(258);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(254);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(255);
				recog.expression()?;

				}
				}
				recog.base.set_state(260);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
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
//------------------- text_argument ----------------
pub type Text_argumentContextAll<'input> = Text_argumentContext<'input>;


pub type Text_argumentContext<'input> = BaseParserRuleContext<'input,Text_argumentContextExt<'input>>;

#[derive(Clone)]
pub struct Text_argumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Text_argumentContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Text_argumentContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_text_argument(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_text_argument(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Text_argumentContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_text_argument(self);
	}
}

impl<'input> CustomRuleContext<'input> for Text_argumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_text_argument }
	//fn type_rule_index() -> usize where Self: Sized { RULE_text_argument }
}
antlr_rust::tid!{Text_argumentContextExt<'a>}

impl<'input> Text_argumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Text_argumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Text_argumentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Text_argumentContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Text_argumentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRING
/// Returns `None` if there is no child corresponding to token STRING
fn STRING(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(STRING, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Text_argumentContextAttrs<'input> for Text_argumentContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn text_argument(&mut self,)
	-> Result<Rc<Text_argumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Text_argumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_text_argument);
        let mut _localctx: Rc<Text_argumentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(263);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(261);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(262);
					recog.expression()?;

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
//------------------- wrt_argument ----------------
pub type Wrt_argumentContextAll<'input> = Wrt_argumentContext<'input>;


pub type Wrt_argumentContext<'input> = BaseParserRuleContext<'input,Wrt_argumentContextExt<'input>>;

#[derive(Clone)]
pub struct Wrt_argumentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Wrt_argumentContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Wrt_argumentContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_wrt_argument(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_wrt_argument(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Wrt_argumentContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_wrt_argument(self);
	}
}

impl<'input> CustomRuleContext<'input> for Wrt_argumentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_wrt_argument }
	//fn type_rule_index() -> usize where Self: Sized { RULE_wrt_argument }
}
antlr_rust::tid!{Wrt_argumentContextExt<'a>}

impl<'input> Wrt_argumentContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Wrt_argumentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Wrt_argumentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Wrt_argumentContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Wrt_argumentContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMMA
/// Returns `None` if there is no child corresponding to token COMMA
fn COMMA(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Wrt_argumentContextAttrs<'input> for Wrt_argumentContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn wrt_argument(&mut self,)
	-> Result<Rc<Wrt_argumentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Wrt_argumentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_wrt_argument);
        let mut _localctx: Rc<Wrt_argumentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(265);
			recog.base.match_token(COMMA,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(266);
			recog.expression()?;

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
//------------------- matrix_content ----------------
pub type Matrix_contentContextAll<'input> = Matrix_contentContext<'input>;


pub type Matrix_contentContext<'input> = BaseParserRuleContext<'input,Matrix_contentContextExt<'input>>;

#[derive(Clone)]
pub struct Matrix_contentContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Matrix_contentContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Matrix_contentContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_matrix_content(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_matrix_content(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Matrix_contentContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_matrix_content(self);
	}
}

impl<'input> CustomRuleContext<'input> for Matrix_contentContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_matrix_content }
	//fn type_rule_index() -> usize where Self: Sized { RULE_matrix_content }
}
antlr_rust::tid!{Matrix_contentContextExt<'a>}

impl<'input> Matrix_contentContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Matrix_contentContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Matrix_contentContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Matrix_contentContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Matrix_contentContextExt<'input>>{

fn matrix_row_all(&self) ->  Vec<Rc<Matrix_rowContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn matrix_row(&self, i: usize) -> Option<Rc<Matrix_rowContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
/// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, i)
}

}

impl<'input> Matrix_contentContextAttrs<'input> for Matrix_contentContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn matrix_content(&mut self,)
	-> Result<Rc<Matrix_contentContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Matrix_contentContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_matrix_content);
        let mut _localctx: Rc<Matrix_contentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule matrix_row*/
			recog.base.set_state(268);
			recog.matrix_row()?;

			recog.base.set_state(273);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SEMICOLON {
				{
				{
				recog.base.set_state(269);
				recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

				/*InvokeRule matrix_row*/
				recog.base.set_state(270);
				recog.matrix_row()?;

				}
				}
				recog.base.set_state(275);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
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
//------------------- matrix_row ----------------
pub type Matrix_rowContextAll<'input> = Matrix_rowContext<'input>;


pub type Matrix_rowContext<'input> = BaseParserRuleContext<'input,Matrix_rowContextExt<'input>>;

#[derive(Clone)]
pub struct Matrix_rowContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Matrix_rowContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Matrix_rowContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_matrix_row(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_matrix_row(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Matrix_rowContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_matrix_row(self);
	}
}

impl<'input> CustomRuleContext<'input> for Matrix_rowContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_matrix_row }
	//fn type_rule_index() -> usize where Self: Sized { RULE_matrix_row }
}
antlr_rust::tid!{Matrix_rowContextExt<'a>}

impl<'input> Matrix_rowContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Matrix_rowContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Matrix_rowContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Matrix_rowContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Matrix_rowContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Matrix_rowContextAttrs<'input> for Matrix_rowContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn matrix_row(&mut self,)
	-> Result<Rc<Matrix_rowContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Matrix_rowContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_matrix_row);
        let mut _localctx: Rc<Matrix_rowContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(276);
			recog.expression()?;

			recog.base.set_state(281);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(277);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(278);
				recog.expression()?;

				}
				}
				recog.base.set_state(283);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
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
//------------------- keyword_func ----------------
pub type Keyword_funcContextAll<'input> = Keyword_funcContext<'input>;


pub type Keyword_funcContext<'input> = BaseParserRuleContext<'input,Keyword_funcContextExt<'input>>;

#[derive(Clone)]
pub struct Keyword_funcContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Keyword_funcContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Keyword_funcContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_keyword_func(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_keyword_func(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Keyword_funcContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_keyword_func(self);
	}
}

impl<'input> CustomRuleContext<'input> for Keyword_funcContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_keyword_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_keyword_func }
}
antlr_rust::tid!{Keyword_funcContextExt<'a>}

impl<'input> Keyword_funcContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Keyword_funcContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Keyword_funcContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Keyword_funcContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Keyword_funcContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BUILTIN_KEYWORD_FUNC_NAME
/// Returns `None` if there is no child corresponding to token BUILTIN_KEYWORD_FUNC_NAME
fn BUILTIN_KEYWORD_FUNC_NAME(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(BUILTIN_KEYWORD_FUNC_NAME, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> Keyword_funcContextAttrs<'input> for Keyword_funcContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn keyword_func(&mut self,)
	-> Result<Rc<Keyword_funcContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Keyword_funcContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_keyword_func);
        let mut _localctx: Rc<Keyword_funcContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(284);
			recog.base.match_token(BUILTIN_KEYWORD_FUNC_NAME,&mut recog.err_handler)?;

			recog.base.set_state(285);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule arguments*/
			recog.base.set_state(286);
			recog.arguments()?;

			recog.base.set_state(287);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- simple_keyword_func ----------------
pub type Simple_keyword_funcContextAll<'input> = Simple_keyword_funcContext<'input>;


pub type Simple_keyword_funcContext<'input> = BaseParserRuleContext<'input,Simple_keyword_funcContextExt<'input>>;

#[derive(Clone)]
pub struct Simple_keyword_funcContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Simple_keyword_funcContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Simple_keyword_funcContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_simple_keyword_func(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_simple_keyword_func(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Simple_keyword_funcContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_simple_keyword_func(self);
	}
}

impl<'input> CustomRuleContext<'input> for Simple_keyword_funcContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_simple_keyword_func }
	//fn type_rule_index() -> usize where Self: Sized { RULE_simple_keyword_func }
}
antlr_rust::tid!{Simple_keyword_funcContextExt<'a>}

impl<'input> Simple_keyword_funcContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Simple_keyword_funcContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Simple_keyword_funcContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Simple_keyword_funcContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Simple_keyword_funcContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BUILTIN_KEYWORD_FUNC_NAME
/// Returns `None` if there is no child corresponding to token BUILTIN_KEYWORD_FUNC_NAME
fn BUILTIN_KEYWORD_FUNC_NAME(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(BUILTIN_KEYWORD_FUNC_NAME, 0)
}
fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Simple_keyword_funcContextAttrs<'input> for Simple_keyword_funcContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn simple_keyword_func(&mut self,)
	-> Result<Rc<Simple_keyword_funcContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Simple_keyword_funcContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_simple_keyword_func);
        let mut _localctx: Rc<Simple_keyword_funcContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(289);
			recog.base.match_token(BUILTIN_KEYWORD_FUNC_NAME,&mut recog.err_handler)?;

			/*InvokeRule primary_expression*/
			recog.base.set_state(290);
			recog.primary_expression()?;

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
//------------------- deriv_function ----------------
pub type Deriv_functionContextAll<'input> = Deriv_functionContext<'input>;


pub type Deriv_functionContext<'input> = BaseParserRuleContext<'input,Deriv_functionContextExt<'input>>;

#[derive(Clone)]
pub struct Deriv_functionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Deriv_functionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Deriv_functionContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_deriv_function(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_deriv_function(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Deriv_functionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_deriv_function(self);
	}
}

impl<'input> CustomRuleContext<'input> for Deriv_functionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_deriv_function }
	//fn type_rule_index() -> usize where Self: Sized { RULE_deriv_function }
}
antlr_rust::tid!{Deriv_functionContextExt<'a>}

impl<'input> Deriv_functionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Deriv_functionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Deriv_functionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Deriv_functionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Deriv_functionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DERIV
/// Returns `None` if there is no child corresponding to token DERIV
fn DERIV(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(DERIV, 0)
}

}

impl<'input> Deriv_functionContextAttrs<'input> for Deriv_functionContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn deriv_function(&mut self,)
	-> Result<Rc<Deriv_functionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Deriv_functionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_deriv_function);
        let mut _localctx: Rc<Deriv_functionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(292);
			recog.base.match_token(DERIV,&mut recog.err_handler)?;

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
//------------------- d_by_d ----------------
pub type D_by_dContextAll<'input> = D_by_dContext<'input>;


pub type D_by_dContext<'input> = BaseParserRuleContext<'input,D_by_dContextExt<'input>>;

#[derive(Clone)]
pub struct D_by_dContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for D_by_dContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for D_by_dContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_d_by_d(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_d_by_d(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for D_by_dContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_d_by_d(self);
	}
}

impl<'input> CustomRuleContext<'input> for D_by_dContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_d_by_d }
	//fn type_rule_index() -> usize where Self: Sized { RULE_d_by_d }
}
antlr_rust::tid!{D_by_dContextExt<'a>}

impl<'input> D_by_dContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<D_by_dContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,D_by_dContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait D_by_dContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<D_by_dContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DBYD
/// Returns `None` if there is no child corresponding to token DBYD
fn DBYD(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(DBYD, 0)
}

}

impl<'input> D_by_dContextAttrs<'input> for D_by_dContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn d_by_d(&mut self,)
	-> Result<Rc<D_by_dContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = D_by_dContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_d_by_d);
        let mut _localctx: Rc<D_by_dContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(294);
			recog.base.match_token(DBYD,&mut recog.err_handler)?;

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
//------------------- derivative ----------------
pub type DerivativeContextAll<'input> = DerivativeContext<'input>;


pub type DerivativeContext<'input> = BaseParserRuleContext<'input,DerivativeContextExt<'input>>;

#[derive(Clone)]
pub struct DerivativeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for DerivativeContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for DerivativeContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_derivative(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_derivative(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for DerivativeContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_derivative(self);
	}
}

impl<'input> CustomRuleContext<'input> for DerivativeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_derivative }
	//fn type_rule_index() -> usize where Self: Sized { RULE_derivative }
}
antlr_rust::tid!{DerivativeContextExt<'a>}

impl<'input> DerivativeContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DerivativeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DerivativeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DerivativeContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<DerivativeContextExt<'input>>{

fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn deriv_function(&self) -> Option<Rc<Deriv_functionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn d_by_d(&self) -> Option<Rc<D_by_dContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn wrt_argument(&self) -> Option<Rc<Wrt_argumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DerivativeContextAttrs<'input> for DerivativeContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn derivative(&mut self,)
	-> Result<Rc<DerivativeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DerivativeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_derivative);
        let mut _localctx: Rc<DerivativeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(298);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 DERIV 
				=> {
					{
					/*InvokeRule deriv_function*/
					recog.base.set_state(296);
					recog.deriv_function()?;

					}
				}

			 DBYD 
				=> {
					{
					/*InvokeRule d_by_d*/
					recog.base.set_state(297);
					recog.d_by_d()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			/*InvokeRule primary_expression*/
			recog.base.set_state(300);
			recog.primary_expression()?;

			recog.base.set_state(302);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule wrt_argument*/
					recog.base.set_state(301);
					recog.wrt_argument()?;

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
//------------------- partial_derivative ----------------
pub type Partial_derivativeContextAll<'input> = Partial_derivativeContext<'input>;


pub type Partial_derivativeContext<'input> = BaseParserRuleContext<'input,Partial_derivativeContextExt<'input>>;

#[derive(Clone)]
pub struct Partial_derivativeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Partial_derivativeContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Partial_derivativeContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_partial_derivative(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_partial_derivative(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Partial_derivativeContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_partial_derivative(self);
	}
}

impl<'input> CustomRuleContext<'input> for Partial_derivativeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_partial_derivative }
	//fn type_rule_index() -> usize where Self: Sized { RULE_partial_derivative }
}
antlr_rust::tid!{Partial_derivativeContextExt<'a>}

impl<'input> Partial_derivativeContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Partial_derivativeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Partial_derivativeContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Partial_derivativeContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Partial_derivativeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PARTIAL
/// Returns `None` if there is no child corresponding to token PARTIAL
fn PARTIAL(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(PARTIAL, 0)
}
fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn wrt_argument(&self) -> Option<Rc<Wrt_argumentContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Partial_derivativeContextAttrs<'input> for Partial_derivativeContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn partial_derivative(&mut self,)
	-> Result<Rc<Partial_derivativeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Partial_derivativeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_partial_derivative);
        let mut _localctx: Rc<Partial_derivativeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(304);
			recog.base.match_token(PARTIAL,&mut recog.err_handler)?;

			/*InvokeRule primary_expression*/
			recog.base.set_state(305);
			recog.primary_expression()?;

			recog.base.set_state(307);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule wrt_argument*/
					recog.base.set_state(306);
					recog.wrt_argument()?;

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
//------------------- function_call ----------------
pub type Function_callContextAll<'input> = Function_callContext<'input>;


pub type Function_callContext<'input> = BaseParserRuleContext<'input,Function_callContextExt<'input>>;

#[derive(Clone)]
pub struct Function_callContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Function_callContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Function_callContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_function_call(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_function_call(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Function_callContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_function_call(self);
	}
}

impl<'input> CustomRuleContext<'input> for Function_callContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_function_call }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_call }
}
antlr_rust::tid!{Function_callContextExt<'a>}

impl<'input> Function_callContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_callContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_callContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_callContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Function_callContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn arguments(&self) -> Option<Rc<ArgumentsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token PRIME in current rule
fn PRIME_all(&self) -> Vec<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PRIME, starting from 0.
/// Returns `None` if number of children corresponding to token PRIME is less or equal than `i`.
fn PRIME(&self, i: usize) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(PRIME, i)
}

}

impl<'input> Function_callContextAttrs<'input> for Function_callContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_call(&mut self,)
	-> Result<Rc<Function_callContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_callContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_function_call);
        let mut _localctx: Rc<Function_callContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(326);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(30,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(309);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(310);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule arguments*/
					recog.base.set_state(311);
					recog.arguments()?;

					recog.base.set_state(312);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(314);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(320);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PRIME {
						{
						recog.base.set_state(316); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						loop {
							{
							{
							recog.base.set_state(315);
							recog.base.match_token(PRIME,&mut recog.err_handler)?;

							}
							}
							recog.base.set_state(318); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if !(_la==PRIME) {break}
						}
						}
					}

					recog.base.set_state(322);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule arguments*/
					recog.base.set_state(323);
					recog.arguments()?;

					recog.base.set_state(324);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- constant_symbol ----------------
pub type Constant_symbolContextAll<'input> = Constant_symbolContext<'input>;


pub type Constant_symbolContext<'input> = BaseParserRuleContext<'input,Constant_symbolContextExt<'input>>;

#[derive(Clone)]
pub struct Constant_symbolContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for Constant_symbolContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for Constant_symbolContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constant_symbol(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_constant_symbol(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for Constant_symbolContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_constant_symbol(self);
	}
}

impl<'input> CustomRuleContext<'input> for Constant_symbolContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant_symbol }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant_symbol }
}
antlr_rust::tid!{Constant_symbolContextExt<'a>}

impl<'input> Constant_symbolContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Constant_symbolContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Constant_symbolContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Constant_symbolContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<Constant_symbolContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PI_CONST
/// Returns `None` if there is no child corresponding to token PI_CONST
fn PI_CONST(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(PI_CONST, 0)
}
/// Retrieves first TerminalNode corresponding to token E_CONST
/// Returns `None` if there is no child corresponding to token E_CONST
fn E_CONST(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(E_CONST, 0)
}
/// Retrieves first TerminalNode corresponding to token I_CONST
/// Returns `None` if there is no child corresponding to token I_CONST
fn I_CONST(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(I_CONST, 0)
}
/// Retrieves first TerminalNode corresponding to token INFINITY_CONST
/// Returns `None` if there is no child corresponding to token INFINITY_CONST
fn INFINITY_CONST(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(INFINITY_CONST, 0)
}
/// Retrieves first TerminalNode corresponding to token GAMMA_CONST
/// Returns `None` if there is no child corresponding to token GAMMA_CONST
fn GAMMA_CONST(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(GAMMA_CONST, 0)
}
/// Retrieves first TerminalNode corresponding to token TRUE_CONST
/// Returns `None` if there is no child corresponding to token TRUE_CONST
fn TRUE_CONST(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(TRUE_CONST, 0)
}
/// Retrieves first TerminalNode corresponding to token FALSE_CONST
/// Returns `None` if there is no child corresponding to token FALSE_CONST
fn FALSE_CONST(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(FALSE_CONST, 0)
}
/// Retrieves first TerminalNode corresponding to token NAN_CONST
/// Returns `None` if there is no child corresponding to token NAN_CONST
fn NAN_CONST(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(NAN_CONST, 0)
}
/// Retrieves first TerminalNode corresponding to token PHI_CONST
/// Returns `None` if there is no child corresponding to token PHI_CONST
fn PHI_CONST(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(PHI_CONST, 0)
}

}

impl<'input> Constant_symbolContextAttrs<'input> for Constant_symbolContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constant_symbol(&mut self,)
	-> Result<Rc<Constant_symbolContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Constant_symbolContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_constant_symbol);
        let mut _localctx: Rc<Constant_symbolContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(328);
			_la = recog.base.input.la(1);
			if { !(((((_la - 54)) & !0x3f) == 0 && ((1usize << (_la - 54)) & ((1usize << (PI_CONST - 54)) | (1usize << (E_CONST - 54)) | (1usize << (I_CONST - 54)) | (1usize << (INFINITY_CONST - 54)) | (1usize << (GAMMA_CONST - 54)) | (1usize << (PHI_CONST - 54)) | (1usize << (TRUE_CONST - 54)) | (1usize << (FALSE_CONST - 54)) | (1usize << (NAN_CONST - 54)))) != 0)) } {
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
	\x7f\u{14d}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x03\x02\x03\x02\x03\
	\x02\x07\x02\x38\x0a\x02\x0c\x02\x0e\x02\x3b\x0b\x02\x03\x02\x07\x02\x3e\
	\x0a\x02\x0c\x02\x0e\x02\x41\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\
	\x04\x03\x04\x03\x04\x07\x04\x4a\x0a\x04\x0c\x04\x0e\x04\x4d\x0b\x04\x03\
	\x05\x03\x05\x03\x05\x03\x05\x05\x05\x53\x0a\x05\x05\x05\x55\x0a\x05\x03\
	\x06\x03\x06\x05\x06\x59\x0a\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\
	\x07\x07\x60\x0a\x07\x0c\x07\x0e\x07\x63\x0b\x07\x03\x08\x03\x08\x03\x08\
	\x03\x08\x07\x08\x69\x0a\x08\x0c\x08\x0e\x08\x6c\x0b\x08\x03\x09\x03\x09\
	\x03\x09\x05\x09\x71\x0a\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\
	\x0b\x03\x0b\x03\x0b\x05\x0b\x7b\x0a\x0b\x07\x0b\x7d\x0a\x0b\x0c\x0b\x0e\
	\x0b\u{80}\x0b\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{86}\x0a\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x05\x0b\u{8d}\x0a\x0b\x03\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\u{94}\x0a\x0c\x0c\x0c\x0e\x0c\u{97}\
	\x0b\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x06\x0c\u{b5}\x0a\x0c\x0d\x0c\x0e\x0c\u{b6}\x05\x0c\u{b9}\
	\x0a\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{d3}\x0a\
	\x0c\x03\x0c\x03\x0c\x05\x0c\u{d7}\x0a\x0c\x03\x0c\x03\x0c\x05\x0c\u{db}\
	\x0a\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{fa}\x0a\x0c\x03\x0d\x03\x0d\x03\x0d\x03\
	\x0d\x03\x0e\x03\x0e\x03\x0e\x07\x0e\u{103}\x0a\x0e\x0c\x0e\x0e\x0e\u{106}\
	\x0b\x0e\x03\x0f\x03\x0f\x05\x0f\u{10a}\x0a\x0f\x03\x10\x03\x10\x03\x10\
	\x03\x11\x03\x11\x03\x11\x07\x11\u{112}\x0a\x11\x0c\x11\x0e\x11\u{115}\x0b\
	\x11\x03\x12\x03\x12\x03\x12\x07\x12\u{11a}\x0a\x12\x0c\x12\x0e\x12\u{11d}\
	\x0b\x12\x03\x13\x03\x13\x03\x13\x03\x13\x03\x13\x03\x14\x03\x14\x03\x14\
	\x03\x15\x03\x15\x03\x16\x03\x16\x03\x17\x03\x17\x05\x17\u{12d}\x0a\x17\
	\x03\x17\x03\x17\x05\x17\u{131}\x0a\x17\x03\x18\x03\x18\x03\x18\x05\x18\
	\u{136}\x0a\x18\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\x03\x19\
	\x06\x19\u{13f}\x0a\x19\x0d\x19\x0e\x19\u{140}\x05\x19\u{143}\x0a\x19\x03\
	\x19\x03\x19\x03\x19\x03\x19\x05\x19\u{149}\x0a\x19\x03\x1a\x03\x1a\x03\
	\x1a\x02\x02\x1b\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\
	\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x02\x0a\x03\x02\x55\x56\x03\
	\x02\x49\x4e\x04\x02\x41\x42\x52\x52\x04\x02\x43\x44\x53\x54\x03\x02\x41\
	\x42\x03\x02\x77\x78\x03\x02\x4f\x50\x03\x02\x38\x40\x02\u{171}\x02\x34\
	\x03\x02\x02\x02\x04\x44\x03\x02\x02\x02\x06\x46\x03\x02\x02\x02\x08\x54\
	\x03\x02\x02\x02\x0a\x58\x03\x02\x02\x02\x0c\x5c\x03\x02\x02\x02\x0e\x64\
	\x03\x02\x02\x02\x10\x70\x03\x02\x02\x02\x12\x72\x03\x02\x02\x02\x14\u{8c}\
	\x03\x02\x02\x02\x16\u{f9}\x03\x02\x02\x02\x18\u{fb}\x03\x02\x02\x02\x1a\
	\u{ff}\x03\x02\x02\x02\x1c\u{109}\x03\x02\x02\x02\x1e\u{10b}\x03\x02\x02\
	\x02\x20\u{10e}\x03\x02\x02\x02\x22\u{116}\x03\x02\x02\x02\x24\u{11e}\x03\
	\x02\x02\x02\x26\u{123}\x03\x02\x02\x02\x28\u{126}\x03\x02\x02\x02\x2a\u{128}\
	\x03\x02\x02\x02\x2c\u{12c}\x03\x02\x02\x02\x2e\u{132}\x03\x02\x02\x02\x30\
	\u{148}\x03\x02\x02\x02\x32\u{14a}\x03\x02\x02\x02\x34\x39\x05\x04\x03\x02\
	\x35\x36\x07\x7e\x02\x02\x36\x38\x05\x04\x03\x02\x37\x35\x03\x02\x02\x02\
	\x38\x3b\x03\x02\x02\x02\x39\x37\x03\x02\x02\x02\x39\x3a\x03\x02\x02\x02\
	\x3a\x3f\x03\x02\x02\x02\x3b\x39\x03\x02\x02\x02\x3c\x3e\x07\x7e\x02\x02\
	\x3d\x3c\x03\x02\x02\x02\x3e\x41\x03\x02\x02\x02\x3f\x3d\x03\x02\x02\x02\
	\x3f\x40\x03\x02\x02\x02\x40\x42\x03\x02\x02\x02\x41\x3f\x03\x02\x02\x02\
	\x42\x43\x07\x02\x02\x03\x43\x03\x03\x02\x02\x02\x44\x45\x05\x06\x04\x02\
	\x45\x05\x03\x02\x02\x02\x46\x4b\x05\x08\x05\x02\x47\x48\x09\x02\x02\x02\
	\x48\x4a\x05\x08\x05\x02\x49\x47\x03\x02\x02\x02\x4a\x4d\x03\x02\x02\x02\
	\x4b\x49\x03\x02\x02\x02\x4b\x4c\x03\x02\x02\x02\x4c\x07\x03\x02\x02\x02\
	\x4d\x4b\x03\x02\x02\x02\x4e\x55\x05\x0a\x06\x02\x4f\x52\x05\x0c\x07\x02\
	\x50\x51\x09\x03\x02\x02\x51\x53\x05\x0c\x07\x02\x52\x50\x03\x02\x02\x02\
	\x52\x53\x03\x02\x02\x02\x53\x55\x03\x02\x02\x02\x54\x4e\x03\x02\x02\x02\
	\x54\x4f\x03\x02\x02\x02\x55\x09\x03\x02\x02\x02\x56\x59\x05\x0c\x07\x02\
	\x57\x59\x05\x30\x19\x02\x58\x56\x03\x02\x02\x02\x58\x57\x03\x02\x02\x02\
	\x59\x5a\x03\x02\x02\x02\x5a\x5b\x07\x49\x02\x02\x5b\x0b\x03\x02\x02\x02\
	\x5c\x61\x05\x0e\x08\x02\x5d\x5e\x09\x04\x02\x02\x5e\x60\x05\x0e\x08\x02\
	\x5f\x5d\x03\x02\x02\x02\x60\x63\x03\x02\x02\x02\x61\x5f\x03\x02\x02\x02\
	\x61\x62\x03\x02\x02\x02\x62\x0d\x03\x02\x02\x02\x63\x61\x03\x02\x02\x02\
	\x64\x6a\x05\x10\x09\x02\x65\x66\x09\x05\x02\x02\x66\x69\x05\x10\x09\x02\
	\x67\x69\x05\x10\x09\x02\x68\x65\x03\x02\x02\x02\x68\x67\x03\x02\x02\x02\
	\x69\x6c\x03\x02\x02\x02\x6a\x68\x03\x02\x02\x02\x6a\x6b\x03\x02\x02\x02\
	\x6b\x0f\x03\x02\x02\x02\x6c\x6a\x03\x02\x02\x02\x6d\x6e\x09\x06\x02\x02\
	\x6e\x71\x05\x14\x0b\x02\x6f\x71\x05\x14\x0b\x02\x70\x6d\x03\x02\x02\x02\
	\x70\x6f\x03\x02\x02\x02\x71\x11\x03\x02\x02\x02\x72\x73\x07\x05\x02\x02\
	\x73\x74\x09\x07\x02\x02\x74\x13\x03\x02\x02\x02\x75\x7e\x05\x16\x0c\x02\
	\x76\x77\x07\x45\x02\x02\x77\x7a\x05\x16\x0c\x02\x78\x79\x07\x46\x02\x02\
	\x79\x7b\x05\x16\x0c\x02\x7a\x78\x03\x02\x02\x02\x7a\x7b\x03\x02\x02\x02\
	\x7b\x7d\x03\x02\x02\x02\x7c\x76\x03\x02\x02\x02\x7d\u{80}\x03\x02\x02\x02\
	\x7e\x7c\x03\x02\x02\x02\x7e\x7f\x03\x02\x02\x02\x7f\u{8d}\x03\x02\x02\x02\
	\u{80}\x7e\x03\x02\x02\x02\u{81}\u{82}\x07\x46\x02\x02\u{82}\u{85}\x05\x16\
	\x0c\x02\u{83}\u{84}\x07\x45\x02\x02\u{84}\u{86}\x05\x16\x0c\x02\u{85}\u{83}\
	\x03\x02\x02\x02\u{85}\u{86}\x03\x02\x02\x02\u{86}\u{8d}\x03\x02\x02\x02\
	\u{87}\u{88}\x07\x45\x02\x02\u{88}\u{8d}\x05\x16\x0c\x02\u{89}\u{8a}\x07\
	\x46\x02\x02\u{8a}\u{8d}\x05\x16\x0c\x02\u{8b}\u{8d}\x07\x47\x02\x02\u{8c}\
	\x75\x03\x02\x02\x02\u{8c}\u{81}\x03\x02\x02\x02\u{8c}\u{87}\x03\x02\x02\
	\x02\u{8c}\u{89}\x03\x02\x02\x02\u{8c}\u{8b}\x03\x02\x02\x02\u{8d}\x15\x03\
	\x02\x02\x02\u{8e}\u{fa}\x05\x30\x19\x02\u{8f}\u{90}\x07\x58\x02\x02\u{90}\
	\u{95}\x05\x18\x0d\x02\u{91}\u{92}\x07\x61\x02\x02\u{92}\u{94}\x05\x18\x0d\
	\x02\u{93}\u{91}\x03\x02\x02\x02\u{94}\u{97}\x03\x02\x02\x02\u{95}\u{93}\
	\x03\x02\x02\x02\u{95}\u{96}\x03\x02\x02\x02\u{96}\u{98}\x03\x02\x02\x02\
	\u{97}\u{95}\x03\x02\x02\x02\u{98}\u{99}\x07\x59\x02\x02\u{99}\u{fa}\x03\
	\x02\x02\x02\u{9a}\u{9b}\x07\x58\x02\x02\u{9b}\u{9c}\x05\x20\x11\x02\u{9c}\
	\u{9d}\x07\x59\x02\x02\u{9d}\u{fa}\x03\x02\x02\x02\u{9e}\u{9f}\x07\x58\x02\
	\x02\u{9f}\u{a0}\x05\x04\x03\x02\u{a0}\u{a1}\x07\x59\x02\x02\u{a1}\u{fa}\
	\x03\x02\x02\x02\u{a2}\u{a3}\x07\x5a\x02\x02\u{a3}\u{a4}\x05\x20\x11\x02\
	\u{a4}\u{a5}\x07\x5b\x02\x02\u{a5}\u{fa}\x03\x02\x02\x02\u{a6}\u{a7}\x07\
	\x5f\x02\x02\u{a7}\u{a8}\x05\x22\x12\x02\u{a8}\u{a9}\x07\x60\x02\x02\u{a9}\
	\u{fa}\x03\x02\x02\x02\u{aa}\u{ab}\x07\x5c\x02\x02\u{ab}\u{ac}\x05\x04\x03\
	\x02\u{ac}\u{ad}\x07\x5d\x02\x02\u{ad}\u{fa}\x03\x02\x02\x02\u{ae}\u{af}\
	\x07\x5e\x02\x02\u{af}\u{b0}\x05\x04\x03\x02\u{b0}\u{b1}\x07\x5e\x02\x02\
	\u{b1}\u{fa}\x03\x02\x02\x02\u{b2}\u{b8}\x07\x78\x02\x02\u{b3}\u{b5}\x07\
	\x47\x02\x02\u{b4}\u{b3}\x03\x02\x02\x02\u{b5}\u{b6}\x03\x02\x02\x02\u{b6}\
	\u{b4}\x03\x02\x02\x02\u{b6}\u{b7}\x03\x02\x02\x02\u{b7}\u{b9}\x03\x02\x02\
	\x02\u{b8}\u{b4}\x03\x02\x02\x02\u{b8}\u{b9}\x03\x02\x02\x02\u{b9}\u{ba}\
	\x03\x02\x02\x02\u{ba}\u{bb}\x07\x58\x02\x02\u{bb}\u{bc}\x05\x1a\x0e\x02\
	\u{bc}\u{bd}\x07\x59\x02\x02\u{bd}\u{fa}\x03\x02\x02\x02\u{be}\u{fa}\x05\
	\x24\x13\x02\u{bf}\u{fa}\x05\x26\x14\x02\u{c0}\u{c1}\x07\x31\x02\x02\u{c1}\
	\u{fa}\x05\x16\x0c\x02\u{c2}\u{c3}\x07\x32\x02\x02\u{c3}\u{c4}\x05\x16\x0c\
	\x02\u{c4}\u{c5}\x05\x16\x0c\x02\u{c5}\u{fa}\x03\x02\x02\x02\u{c6}\u{c7}\
	\x07\x33\x02\x02\u{c7}\u{c8}\x05\x16\x0c\x02\u{c8}\u{c9}\x05\x16\x0c\x02\
	\u{c9}\u{fa}\x03\x02\x02\x02\u{ca}\u{cb}\x07\x34\x02\x02\u{cb}\u{cc}\x07\
	\x58\x02\x02\u{cc}\u{cd}\x05\x1c\x0f\x02\u{cd}\u{ce}\x07\x59\x02\x02\u{ce}\
	\u{fa}\x03\x02\x02\x02\u{cf}\u{d2}\x07\x04\x02\x02\u{d0}\u{d1}\x07\x46\x02\
	\x02\u{d1}\u{d3}\x05\x16\x0c\x02\u{d2}\u{d0}\x03\x02\x02\x02\u{d2}\u{d3}\
	\x03\x02\x02\x02\u{d3}\u{d6}\x03\x02\x02\x02\u{d4}\u{d5}\x07\x45\x02\x02\
	\u{d5}\u{d7}\x05\x16\x0c\x02\u{d6}\u{d4}\x03\x02\x02\x02\u{d6}\u{d7}\x03\
	\x02\x02\x02\u{d7}\u{d8}\x03\x02\x02\x02\u{d8}\u{da}\x05\x16\x0c\x02\u{d9}\
	\u{db}\x05\x12\x0a\x02\u{da}\u{d9}\x03\x02\x02\x02\u{da}\u{db}\x03\x02\x02\
	\x02\u{db}\u{fa}\x03\x02\x02\x02\u{dc}\u{fa}\x05\x2c\x17\x02\u{dd}\u{fa}\
	\x05\x2e\x18\x02\u{de}\u{df}\x05\x12\x0a\x02\u{df}\u{e0}\x07\x44\x02\x02\
	\u{e0}\u{e1}\x05\x12\x0a\x02\u{e1}\u{fa}\x03\x02\x02\x02\u{e2}\u{e3}\x07\
	\x09\x02\x02\u{e3}\u{e4}\x07\x46\x02\x02\u{e4}\u{e5}\x05\x16\x0c\x02\u{e5}\
	\u{e6}\x09\x08\x02\x02\u{e6}\u{e7}\x05\x16\x0c\x02\u{e7}\u{e8}\x05\x16\x0c\
	\x02\u{e8}\u{fa}\x03\x02\x02\x02\u{e9}\u{ea}\x07\x35\x02\x02\u{ea}\u{eb}\
	\x07\x58\x02\x02\u{eb}\u{ec}\x05\x20\x11\x02\u{ec}\u{ed}\x07\x59\x02\x02\
	\u{ed}\u{fa}\x03\x02\x02\x02\u{ee}\u{ef}\x07\x36\x02\x02\u{ef}\u{fa}\x05\
	\x16\x0c\x02\u{f0}\u{f1}\x07\x37\x02\x02\u{f1}\u{fa}\x05\x16\x0c\x02\u{f2}\
	\u{fa}\x07\x78\x02\x02\u{f3}\u{fa}\x07\x79\x02\x02\u{f4}\u{fa}\x07\x7a\x02\
	\x02\u{f5}\u{fa}\x07\x7b\x02\x02\u{f6}\u{fa}\x07\x77\x02\x02\u{f7}\u{fa}\
	\x05\x32\x1a\x02\u{f8}\u{fa}\x07\x7d\x02\x02\u{f9}\u{8e}\x03\x02\x02\x02\
	\u{f9}\u{8f}\x03\x02\x02\x02\u{f9}\u{9a}\x03\x02\x02\x02\u{f9}\u{9e}\x03\
	\x02\x02\x02\u{f9}\u{a2}\x03\x02\x02\x02\u{f9}\u{a6}\x03\x02\x02\x02\u{f9}\
	\u{aa}\x03\x02\x02\x02\u{f9}\u{ae}\x03\x02\x02\x02\u{f9}\u{b2}\x03\x02\x02\
	\x02\u{f9}\u{be}\x03\x02\x02\x02\u{f9}\u{bf}\x03\x02\x02\x02\u{f9}\u{c0}\
	\x03\x02\x02\x02\u{f9}\u{c2}\x03\x02\x02\x02\u{f9}\u{c6}\x03\x02\x02\x02\
	\u{f9}\u{ca}\x03\x02\x02\x02\u{f9}\u{cf}\x03\x02\x02\x02\u{f9}\u{dc}\x03\
	\x02\x02\x02\u{f9}\u{dd}\x03\x02\x02\x02\u{f9}\u{de}\x03\x02\x02\x02\u{f9}\
	\u{e2}\x03\x02\x02\x02\u{f9}\u{e9}\x03\x02\x02\x02\u{f9}\u{ee}\x03\x02\x02\
	\x02\u{f9}\u{f0}\x03\x02\x02\x02\u{f9}\u{f2}\x03\x02\x02\x02\u{f9}\u{f3}\
	\x03\x02\x02\x02\u{f9}\u{f4}\x03\x02\x02\x02\u{f9}\u{f5}\x03\x02\x02\x02\
	\u{f9}\u{f6}\x03\x02\x02\x02\u{f9}\u{f7}\x03\x02\x02\x02\u{f9}\u{f8}\x03\
	\x02\x02\x02\u{fa}\x17\x03\x02\x02\x02\u{fb}\u{fc}\x07\x58\x02\x02\u{fc}\
	\u{fd}\x05\x04\x03\x02\u{fd}\u{fe}\x07\x59\x02\x02\u{fe}\x19\x03\x02\x02\
	\x02\u{ff}\u{104}\x05\x04\x03\x02\u{100}\u{101}\x07\x61\x02\x02\u{101}\u{103}\
	\x05\x04\x03\x02\u{102}\u{100}\x03\x02\x02\x02\u{103}\u{106}\x03\x02\x02\
	\x02\u{104}\u{102}\x03\x02\x02\x02\u{104}\u{105}\x03\x02\x02\x02\u{105}\
	\x1b\x03\x02\x02\x02\u{106}\u{104}\x03\x02\x02\x02\u{107}\u{10a}\x07\x7d\
	\x02\x02\u{108}\u{10a}\x05\x04\x03\x02\u{109}\u{107}\x03\x02\x02\x02\u{109}\
	\u{108}\x03\x02\x02\x02\u{10a}\x1d\x03\x02\x02\x02\u{10b}\u{10c}\x07\x61\
	\x02\x02\u{10c}\u{10d}\x05\x04\x03\x02\u{10d}\x1f\x03\x02\x02\x02\u{10e}\
	\u{113}\x05\x22\x12\x02\u{10f}\u{110}\x07\x62\x02\x02\u{110}\u{112}\x05\
	\x22\x12\x02\u{111}\u{10f}\x03\x02\x02\x02\u{112}\u{115}\x03\x02\x02\x02\
	\u{113}\u{111}\x03\x02\x02\x02\u{113}\u{114}\x03\x02\x02\x02\u{114}\x21\
	\x03\x02\x02\x02\u{115}\u{113}\x03\x02\x02\x02\u{116}\u{11b}\x05\x04\x03\
	\x02\u{117}\u{118}\x07\x61\x02\x02\u{118}\u{11a}\x05\x04\x03\x02\u{119}\
	\u{117}\x03\x02\x02\x02\u{11a}\u{11d}\x03\x02\x02\x02\u{11b}\u{119}\x03\
	\x02\x02\x02\u{11b}\u{11c}\x03\x02\x02\x02\u{11c}\x23\x03\x02\x02\x02\u{11d}\
	\u{11b}\x03\x02\x02\x02\u{11e}\u{11f}\x07\x03\x02\x02\u{11f}\u{120}\x07\
	\x58\x02\x02\u{120}\u{121}\x05\x1a\x0e\x02\u{121}\u{122}\x07\x59\x02\x02\
	\u{122}\x25\x03\x02\x02\x02\u{123}\u{124}\x07\x03\x02\x02\u{124}\u{125}\
	\x05\x16\x0c\x02\u{125}\x27\x03\x02\x02\x02\u{126}\u{127}\x07\x06\x02\x02\
	\u{127}\x29\x03\x02\x02\x02\u{128}\u{129}\x07\x07\x02\x02\u{129}\x2b\x03\
	\x02\x02\x02\u{12a}\u{12d}\x05\x28\x15\x02\u{12b}\u{12d}\x05\x2a\x16\x02\
	\u{12c}\u{12a}\x03\x02\x02\x02\u{12c}\u{12b}\x03\x02\x02\x02\u{12d}\u{12e}\
	\x03\x02\x02\x02\u{12e}\u{130}\x05\x16\x0c\x02\u{12f}\u{131}\x05\x1e\x10\
	\x02\u{130}\u{12f}\x03\x02\x02\x02\u{130}\u{131}\x03\x02\x02\x02\u{131}\
	\x2d\x03\x02\x02\x02\u{132}\u{133}\x07\x08\x02\x02\u{133}\u{135}\x05\x16\
	\x0c\x02\u{134}\u{136}\x05\x1e\x10\x02\u{135}\u{134}\x03\x02\x02\x02\u{135}\
	\u{136}\x03\x02\x02\x02\u{136}\x2f\x03\x02\x02\x02\u{137}\u{138}\x07\x78\
	\x02\x02\u{138}\u{139}\x07\x58\x02\x02\u{139}\u{13a}\x05\x1a\x0e\x02\u{13a}\
	\u{13b}\x07\x59\x02\x02\u{13b}\u{149}\x03\x02\x02\x02\u{13c}\u{142}\x07\
	\x78\x02\x02\u{13d}\u{13f}\x07\x47\x02\x02\u{13e}\u{13d}\x03\x02\x02\x02\
	\u{13f}\u{140}\x03\x02\x02\x02\u{140}\u{13e}\x03\x02\x02\x02\u{140}\u{141}\
	\x03\x02\x02\x02\u{141}\u{143}\x03\x02\x02\x02\u{142}\u{13e}\x03\x02\x02\
	\x02\u{142}\u{143}\x03\x02\x02\x02\u{143}\u{144}\x03\x02\x02\x02\u{144}\
	\u{145}\x07\x58\x02\x02\u{145}\u{146}\x05\x1a\x0e\x02\u{146}\u{147}\x07\
	\x59\x02\x02\u{147}\u{149}\x03\x02\x02\x02\u{148}\u{137}\x03\x02\x02\x02\
	\u{148}\u{13c}\x03\x02\x02\x02\u{149}\x31\x03\x02\x02\x02\u{14a}\u{14b}\
	\x09\x09\x02\x02\u{14b}\x33\x03\x02\x02\x02\x21\x39\x3f\x4b\x52\x54\x58\
	\x61\x68\x6a\x70\x7a\x7e\u{85}\u{8c}\u{95}\u{b6}\u{b8}\u{d2}\u{d6}\u{da}\
	\u{f9}\u{104}\u{109}\u{113}\u{11b}\u{12c}\u{130}\u{135}\u{140}\u{142}\u{148}";

