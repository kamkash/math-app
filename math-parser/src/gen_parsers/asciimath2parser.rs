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
		pub const SIN:isize=8; 
		pub const COS:isize=9; 
		pub const TAN:isize=10; 
		pub const CSC:isize=11; 
		pub const SEC:isize=12; 
		pub const COT:isize=13; 
		pub const ASIN:isize=14; 
		pub const ACOS:isize=15; 
		pub const ATAN:isize=16; 
		pub const ACSC:isize=17; 
		pub const ASEC:isize=18; 
		pub const ACOT:isize=19; 
		pub const SINH:isize=20; 
		pub const COSH:isize=21; 
		pub const TANH:isize=22; 
		pub const CSCH:isize=23; 
		pub const SECH:isize=24; 
		pub const COTH:isize=25; 
		pub const ASINH:isize=26; 
		pub const ACOSH:isize=27; 
		pub const ATANH:isize=28; 
		pub const ACSCH:isize=29; 
		pub const ASECH:isize=30; 
		pub const ACOTH:isize=31; 
		pub const LOG:isize=32; 
		pub const LN:isize=33; 
		pub const EXP:isize=34; 
		pub const FLOOR:isize=35; 
		pub const CEIL:isize=36; 
		pub const ROUND:isize=37; 
		pub const MIN:isize=38; 
		pub const MAX:isize=39; 
		pub const NORM:isize=40; 
		pub const CARD:isize=41; 
		pub const ABS_FUNC:isize=42; 
		pub const SUM:isize=43; 
		pub const PROD:isize=44; 
		pub const VEC:isize=45; 
		pub const SQRT:isize=46; 
		pub const ROOT:isize=47; 
		pub const FRAC:isize=48; 
		pub const TEXT:isize=49; 
		pub const MAT:isize=50; 
		pub const DET:isize=51; 
		pub const TRANSPOSE:isize=52; 
		pub const PI_CONST:isize=53; 
		pub const E_CONST:isize=54; 
		pub const I_CONST:isize=55; 
		pub const INFINITY_CONST:isize=56; 
		pub const GAMMA_CONST:isize=57; 
		pub const PHI_CONST:isize=58; 
		pub const TRUE_CONST:isize=59; 
		pub const FALSE_CONST:isize=60; 
		pub const NAN_CONST:isize=61; 
		pub const PLUS:isize=62; 
		pub const MINUS:isize=63; 
		pub const STAR:isize=64; 
		pub const FSLASH:isize=65; 
		pub const HAT:isize=66; 
		pub const UNDERSCORE:isize=67; 
		pub const PRIME:isize=68; 
		pub const BANG:isize=69; 
		pub const EQ:isize=70; 
		pub const NEQ:isize=71; 
		pub const LT:isize=72; 
		pub const GT:isize=73; 
		pub const LTE:isize=74; 
		pub const GTE:isize=75; 
		pub const TO:isize=76; 
		pub const RARROW:isize=77; 
		pub const LARROW:isize=78; 
		pub const PM:isize=79; 
		pub const TIMES:isize=80; 
		pub const DIV:isize=81; 
		pub const AND:isize=82; 
		pub const OR:isize=83; 
		pub const NOT:isize=84; 
		pub const LPAREN:isize=85; 
		pub const RPAREN:isize=86; 
		pub const LBRACKET:isize=87; 
		pub const RBRACKET:isize=88; 
		pub const LBRACE:isize=89; 
		pub const RBRACE:isize=90; 
		pub const ABS:isize=91; 
		pub const L_ANGLE:isize=92; 
		pub const R_ANGLE:isize=93; 
		pub const COMMA:isize=94; 
		pub const SEMICOLON:isize=95; 
		pub const ALPHA_G:isize=96; 
		pub const BETA_G:isize=97; 
		pub const DELTA_G:isize=98; 
		pub const EPSILON_G:isize=99; 
		pub const ZETA_G:isize=100; 
		pub const ETA_G:isize=101; 
		pub const THETA_G:isize=102; 
		pub const IOTA_G:isize=103; 
		pub const KAPPA_G:isize=104; 
		pub const LAMBDA_G:isize=105; 
		pub const MU_G:isize=106; 
		pub const NU_G:isize=107; 
		pub const XI_G:isize=108; 
		pub const RHO_G:isize=109; 
		pub const SIGMA_G:isize=110; 
		pub const TAU_G:isize=111; 
		pub const UPSILON_G:isize=112; 
		pub const CHI_G:isize=113; 
		pub const PSI_G:isize=114; 
		pub const OMEGA_G:isize=115; 
		pub const GREEK_LETTER:isize=116; 
		pub const IDENTIFIER:isize=117; 
		pub const NUMBER:isize=118; 
		pub const NUMBER_WITH_COMMAS:isize=119; 
		pub const CURRENCY_NUMBER:isize=120; 
		pub const SCIENTIFIC_NUMBER:isize=121; 
		pub const STRING:isize=122; 
		pub const SEPARATOR:isize=123; 
		pub const WS:isize=124;
	pub const RULE_block:usize = 0; 
	pub const RULE_expression:usize = 1; 
	pub const RULE_logical_expression:usize = 2; 
	pub const RULE_relation_expression:usize = 3; 
	pub const RULE_relation_expression_no_rhs:usize = 4; 
	pub const RULE_add_sub_expression:usize = 5; 
	pub const RULE_mult_div_implicit_expression:usize = 6; 
	pub const RULE_unary_op_expression:usize = 7; 
	pub const RULE_d_dx_function:usize = 8; 
	pub const RULE_d_dx_prefix_operator:usize = 9; 
	pub const RULE_differential:usize = 10; 
	pub const RULE_script_op_expression:usize = 11; 
	pub const RULE_primary_expression:usize = 12; 
	pub const RULE_paren_element_for_column_vector:usize = 13; 
	pub const RULE_arguments:usize = 14; 
	pub const RULE_text_argument:usize = 15; 
	pub const RULE_wrt_argument:usize = 16; 
	pub const RULE_matrix_content:usize = 17; 
	pub const RULE_matrix_row:usize = 18; 
	pub const RULE_constant_symbol:usize = 19;
	pub const ruleNames: [&'static str; 20] =  [
		"block", "expression", "logical_expression", "relation_expression", "relation_expression_no_rhs", 
		"add_sub_expression", "mult_div_implicit_expression", "unary_op_expression", 
		"d_dx_function", "d_dx_prefix_operator", "differential", "script_op_expression", 
		"primary_expression", "paren_element_for_column_vector", "arguments", 
		"text_argument", "wrt_argument", "matrix_content", "matrix_row", "constant_symbol"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;116] = [
		None, None, None, Some("'d'"), None, None, None, Some("'lim'"), Some("'sin'"), 
		Some("'cos'"), Some("'tan'"), Some("'csc'"), Some("'sec'"), Some("'cot'"), 
		None, None, None, None, None, None, Some("'sinh'"), Some("'cosh'"), Some("'tanh'"), 
		Some("'csch'"), Some("'sech'"), Some("'coth'"), None, None, None, None, 
		None, None, Some("'log'"), Some("'ln'"), Some("'exp'"), Some("'floor'"), 
		Some("'ceil'"), Some("'round'"), Some("'min'"), Some("'max'"), Some("'norm'"), 
		Some("'card'"), Some("'abs'"), None, None, Some("'vec'"), None, Some("'root'"), 
		Some("'frac'"), Some("'text'"), Some("'mat'"), Some("'det'"), None, None, 
		Some("'e'"), Some("'i'"), None, None, None, Some("'true'"), Some("'false'"), 
		Some("'NaN'"), Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), Some("'^'"), 
		Some("'_'"), Some("'''"), Some("'!'"), Some("'='"), None, Some("'<'"), 
		Some("'>'"), None, None, Some("'to'"), None, None, None, None, None, Some("'and'"), 
		Some("'or'"), Some("'not'"), Some("'('"), Some("')'"), Some("'['"), Some("']'"), 
		Some("'{'"), Some("'}'"), Some("'|'"), None, None, Some("','"), Some("';'"), 
		Some("'alpha'"), Some("'beta'"), Some("'delta'"), Some("'epsilon'"), Some("'zeta'"), 
		Some("'eta'"), Some("'theta'"), Some("'iota'"), Some("'kappa'"), Some("'lambda'"), 
		Some("'mu'"), Some("'nu'"), Some("'xi'"), Some("'rho'"), Some("'sigma'"), 
		Some("'tau'"), Some("'upsilon'"), Some("'chi'"), Some("'psi'"), Some("'omega'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;125]  = [
		None, Some("BUILTIN_KEYWORD_FUNC_NAME"), Some("INTEGRAL"), Some("D_LOWERCASE"), 
		Some("DERIV"), Some("DBYD"), Some("PARTIAL"), Some("LIM"), Some("SIN"), 
		Some("COS"), Some("TAN"), Some("CSC"), Some("SEC"), Some("COT"), Some("ASIN"), 
		Some("ACOS"), Some("ATAN"), Some("ACSC"), Some("ASEC"), Some("ACOT"), 
		Some("SINH"), Some("COSH"), Some("TANH"), Some("CSCH"), Some("SECH"), 
		Some("COTH"), Some("ASINH"), Some("ACOSH"), Some("ATANH"), Some("ACSCH"), 
		Some("ASECH"), Some("ACOTH"), Some("LOG"), Some("LN"), Some("EXP"), Some("FLOOR"), 
		Some("CEIL"), Some("ROUND"), Some("MIN"), Some("MAX"), Some("NORM"), Some("CARD"), 
		Some("ABS_FUNC"), Some("SUM"), Some("PROD"), Some("VEC"), Some("SQRT"), 
		Some("ROOT"), Some("FRAC"), Some("TEXT"), Some("MAT"), Some("DET"), Some("TRANSPOSE"), 
		Some("PI_CONST"), Some("E_CONST"), Some("I_CONST"), Some("INFINITY_CONST"), 
		Some("GAMMA_CONST"), Some("PHI_CONST"), Some("TRUE_CONST"), Some("FALSE_CONST"), 
		Some("NAN_CONST"), Some("PLUS"), Some("MINUS"), Some("STAR"), Some("FSLASH"), 
		Some("HAT"), Some("UNDERSCORE"), Some("PRIME"), Some("BANG"), Some("EQ"), 
		Some("NEQ"), Some("LT"), Some("GT"), Some("LTE"), Some("GTE"), Some("TO"), 
		Some("RARROW"), Some("LARROW"), Some("PM"), Some("TIMES"), Some("DIV"), 
		Some("AND"), Some("OR"), Some("NOT"), Some("LPAREN"), Some("RPAREN"), 
		Some("LBRACKET"), Some("RBRACKET"), Some("LBRACE"), Some("RBRACE"), Some("ABS"), 
		Some("L_ANGLE"), Some("R_ANGLE"), Some("COMMA"), Some("SEMICOLON"), Some("ALPHA_G"), 
		Some("BETA_G"), Some("DELTA_G"), Some("EPSILON_G"), Some("ZETA_G"), Some("ETA_G"), 
		Some("THETA_G"), Some("IOTA_G"), Some("KAPPA_G"), Some("LAMBDA_G"), Some("MU_G"), 
		Some("NU_G"), Some("XI_G"), Some("RHO_G"), Some("SIGMA_G"), Some("TAU_G"), 
		Some("UPSILON_G"), Some("CHI_G"), Some("PSI_G"), Some("OMEGA_G"), Some("GREEK_LETTER"), 
		Some("IDENTIFIER"), Some("NUMBER"), Some("NUMBER_WITH_COMMAS"), Some("CURRENCY_NUMBER"), 
		Some("SCIENTIFIC_NUMBER"), Some("STRING"), Some("SEPARATOR"), Some("WS")
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
			recog.base.set_state(40);
			recog.expression()?;

			recog.base.set_state(45);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(0,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(41);
					recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(42);
					recog.expression()?;

					}
					} 
				}
				recog.base.set_state(47);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(0,&mut recog.base)?;
			}
			recog.base.set_state(51);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SEPARATOR {
				{
				{
				recog.base.set_state(48);
				recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(53);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(54);
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
			recog.base.set_state(56);
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
			recog.base.set_state(58);
			recog.relation_expression()?;

			recog.base.set_state(63);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(2,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(59);
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
					recog.base.set_state(60);
					recog.relation_expression()?;

					}
					} 
				}
				recog.base.set_state(65);
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

			recog.base.set_state(72);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(4,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule relation_expression_no_rhs*/
					recog.base.set_state(66);
					recog.relation_expression_no_rhs()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule add_sub_expression*/
					recog.base.set_state(67);
					recog.add_sub_expression()?;

					recog.base.set_state(70);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(3,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(68);
							_la = recog.base.input.la(1);
							if { !(((((_la - 70)) & !0x3f) == 0 && ((1usize << (_la - 70)) & ((1usize << (EQ - 70)) | (1usize << (NEQ - 70)) | (1usize << (LT - 70)) | (1usize << (GT - 70)) | (1usize << (LTE - 70)) | (1usize << (GTE - 70)))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule add_sub_expression*/
							recog.base.set_state(69);
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

fn add_sub_expression(&self) -> Option<Rc<Add_sub_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
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

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule add_sub_expression*/
			recog.base.set_state(74);
			recog.add_sub_expression()?;

			recog.base.set_state(75);
			recog.base.match_token(EQ,&mut recog.err_handler)?;

			recog.base.set_state(77); 
			recog.err_handler.sync(&mut recog.base)?;
			_alt = 1;
			loop {
				match _alt {
				    x if x == 1=>
					{
					{
					recog.base.set_state(76);
					recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

					}
					}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				recog.base.set_state(79); 
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
				if _alt==2 || _alt==INVALID_ALT { break }
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
			recog.base.set_state(81);
			recog.mult_div_implicit_expression()?;

			recog.base.set_state(86);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(6,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(82);
					_la = recog.base.input.la(1);
					if { !(((((_la - 62)) & !0x3f) == 0 && ((1usize << (_la - 62)) & ((1usize << (PLUS - 62)) | (1usize << (MINUS - 62)) | (1usize << (PM - 62)))) != 0)) } {
						recog.err_handler.recover_inline(&mut recog.base)?;

					}
					else {
						if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
						recog.err_handler.report_match(&mut recog.base);
						recog.base.consume(&mut recog.err_handler);
					}
					/*InvokeRule mult_div_implicit_expression*/
					recog.base.set_state(83);
					recog.mult_div_implicit_expression()?;

					}
					} 
				}
				recog.base.set_state(88);
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
			recog.base.set_state(89);
			recog.unary_op_expression()?;

			recog.base.set_state(95);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					recog.base.set_state(93);
					recog.err_handler.sync(&mut recog.base)?;
					match recog.base.input.la(1) {
					 STAR | FSLASH | TIMES | DIV 
						=> {
							{
							recog.base.set_state(90);
							_la = recog.base.input.la(1);
							if { !(((((_la - 64)) & !0x3f) == 0 && ((1usize << (_la - 64)) & ((1usize << (STAR - 64)) | (1usize << (FSLASH - 64)) | (1usize << (TIMES - 64)) | (1usize << (DIV - 64)))) != 0)) } {
								recog.err_handler.recover_inline(&mut recog.base)?;

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule unary_op_expression*/
							recog.base.set_state(91);
							recog.unary_op_expression()?;

							}
						}

					 BUILTIN_KEYWORD_FUNC_NAME | INTEGRAL | D_LOWERCASE | DERIV | PARTIAL |
					 LIM | SQRT | ROOT | FRAC | TEXT | MAT | DET | TRANSPOSE | PI_CONST |
					 E_CONST | I_CONST | INFINITY_CONST | GAMMA_CONST | PHI_CONST | TRUE_CONST |
					 FALSE_CONST | NAN_CONST | PLUS | MINUS | HAT | UNDERSCORE | PRIME |
					 LPAREN | LBRACKET | LBRACE | ABS | L_ANGLE | GREEK_LETTER | IDENTIFIER |
					 NUMBER | NUMBER_WITH_COMMAS | CURRENCY_NUMBER | STRING 
						=> {
							{
							/*InvokeRule unary_op_expression*/
							recog.base.set_state(92);
							recog.unary_op_expression()?;

							}
						}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
					}
					} 
				}
				recog.base.set_state(97);
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
	AppliedDByDxPrefixContext(AppliedDByDxPrefixContext<'input>),
	UnaryPlusMinusContext(UnaryPlusMinusContext<'input>),
	AppliedDByDxFunctionContext(AppliedDByDxFunctionContext<'input>),
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
			AppliedDByDxPrefixContext(inner) => inner,
			UnaryPlusMinusContext(inner) => inner,
			AppliedDByDxFunctionContext(inner) => inner,
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

pub type AppliedDByDxPrefixContext<'input> = BaseParserRuleContext<'input,AppliedDByDxPrefixContextExt<'input>>;

pub trait AppliedDByDxPrefixContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn d_dx_prefix_operator(&self) -> Option<Rc<D_dx_prefix_operatorContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn script_op_expression(&self) -> Option<Rc<Script_op_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AppliedDByDxPrefixContextAttrs<'input> for AppliedDByDxPrefixContext<'input>{}

pub struct AppliedDByDxPrefixContextExt<'input>{
	base:Unary_op_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AppliedDByDxPrefixContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for AppliedDByDxPrefixContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for AppliedDByDxPrefixContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_appliedDByDxPrefix(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_appliedDByDxPrefix(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for AppliedDByDxPrefixContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_appliedDByDxPrefix(self);
	}
}

impl<'input> CustomRuleContext<'input> for AppliedDByDxPrefixContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_op_expression }
}

impl<'input> Borrow<Unary_op_expressionContextExt<'input>> for AppliedDByDxPrefixContext<'input>{
	fn borrow(&self) -> &Unary_op_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Unary_op_expressionContextExt<'input>> for AppliedDByDxPrefixContext<'input>{
	fn borrow_mut(&mut self) -> &mut Unary_op_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Unary_op_expressionContextAttrs<'input> for AppliedDByDxPrefixContext<'input> {}

impl<'input> AppliedDByDxPrefixContextExt<'input>{
	fn new(ctx: &dyn Unary_op_expressionContextAttrs<'input>) -> Rc<Unary_op_expressionContextAll<'input>>  {
		Rc::new(
			Unary_op_expressionContextAll::AppliedDByDxPrefixContext(
				BaseParserRuleContext::copy_from(ctx,AppliedDByDxPrefixContextExt{
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

pub type AppliedDByDxFunctionContext<'input> = BaseParserRuleContext<'input,AppliedDByDxFunctionContextExt<'input>>;

pub trait AppliedDByDxFunctionContextAttrs<'input>: AsciiMath2ParserContext<'input>{
	fn d_dx_function(&self) -> Option<Rc<D_dx_functionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AppliedDByDxFunctionContextAttrs<'input> for AppliedDByDxFunctionContext<'input>{}

pub struct AppliedDByDxFunctionContextExt<'input>{
	base:Unary_op_expressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AppliedDByDxFunctionContextExt<'a>}

impl<'input> AsciiMath2ParserContext<'input> for AppliedDByDxFunctionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for AppliedDByDxFunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_appliedDByDxFunction(self);
	}
	fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
		listener.exit_appliedDByDxFunction(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for AppliedDByDxFunctionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_appliedDByDxFunction(self);
	}
}

impl<'input> CustomRuleContext<'input> for AppliedDByDxFunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_op_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_op_expression }
}

impl<'input> Borrow<Unary_op_expressionContextExt<'input>> for AppliedDByDxFunctionContext<'input>{
	fn borrow(&self) -> &Unary_op_expressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Unary_op_expressionContextExt<'input>> for AppliedDByDxFunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Unary_op_expressionContextExt<'input> { &mut self.base }
}

impl<'input> Unary_op_expressionContextAttrs<'input> for AppliedDByDxFunctionContext<'input> {}

impl<'input> AppliedDByDxFunctionContextExt<'input>{
	fn new(ctx: &dyn Unary_op_expressionContextAttrs<'input>) -> Rc<Unary_op_expressionContextAll<'input>>  {
		Rc::new(
			Unary_op_expressionContextAll::AppliedDByDxFunctionContext(
				BaseParserRuleContext::copy_from(ctx,AppliedDByDxFunctionContextExt{
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

			recog.base.set_state(105);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
				1 =>{
					let tmp = UnaryPlusMinusContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(98);
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
					recog.base.set_state(99);
					recog.script_op_expression()?;

					}
				}
			,
				2 =>{
					let tmp = AppliedDByDxFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule d_dx_function*/
					recog.base.set_state(100);
					recog.d_dx_function()?;

					}
				}
			,
				3 =>{
					let tmp = AppliedDByDxPrefixContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule d_dx_prefix_operator*/
					recog.base.set_state(101);
					recog.d_dx_prefix_operator()?;

					/*InvokeRule script_op_expression*/
					recog.base.set_state(102);
					recog.script_op_expression()?;

					}
				}
			,
				4 =>{
					let tmp = NoUnaryOperatorContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					/*InvokeRule script_op_expression*/
					recog.base.set_state(104);
					recog.script_op_expression()?;

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
//------------------- d_dx_function ----------------
pub type D_dx_functionContextAll<'input> = D_dx_functionContext<'input>;


pub type D_dx_functionContext<'input> = BaseParserRuleContext<'input,D_dx_functionContextExt<'input>>;

#[derive(Clone)]
pub struct D_dx_functionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for D_dx_functionContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for D_dx_functionContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_d_dx_function(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_d_dx_function(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for D_dx_functionContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_d_dx_function(self);
	}
}

impl<'input> CustomRuleContext<'input> for D_dx_functionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_d_dx_function }
	//fn type_rule_index() -> usize where Self: Sized { RULE_d_dx_function }
}
antlr_rust::tid!{D_dx_functionContextExt<'a>}

impl<'input> D_dx_functionContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<D_dx_functionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,D_dx_functionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait D_dx_functionContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<D_dx_functionContextExt<'input>>{

fn d_dx_prefix_operator(&self) -> Option<Rc<D_dx_prefix_operatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> D_dx_functionContextAttrs<'input> for D_dx_functionContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn d_dx_function(&mut self,)
	-> Result<Rc<D_dx_functionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = D_dx_functionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_d_dx_function);
        let mut _localctx: Rc<D_dx_functionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule d_dx_prefix_operator*/
			recog.base.set_state(107);
			recog.d_dx_prefix_operator()?;

			recog.base.set_state(108);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule primary_expression*/
			recog.base.set_state(109);
			recog.primary_expression()?;

			recog.base.set_state(110);
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
//------------------- d_dx_prefix_operator ----------------
pub type D_dx_prefix_operatorContextAll<'input> = D_dx_prefix_operatorContext<'input>;


pub type D_dx_prefix_operatorContext<'input> = BaseParserRuleContext<'input,D_dx_prefix_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct D_dx_prefix_operatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> AsciiMath2ParserContext<'input> for D_dx_prefix_operatorContext<'input>{}

impl<'input,'a> Listenable<dyn AsciiMath2Listener<'input> + 'a> for D_dx_prefix_operatorContext<'input>{
		fn enter(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_d_dx_prefix_operator(self);
		}
		fn exit(&self,listener: &mut (dyn AsciiMath2Listener<'input> + 'a)) {
			listener.exit_d_dx_prefix_operator(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn AsciiMath2Visitor<'input> + 'a> for D_dx_prefix_operatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn AsciiMath2Visitor<'input> + 'a)) {
		visitor.visit_d_dx_prefix_operator(self);
	}
}

impl<'input> CustomRuleContext<'input> for D_dx_prefix_operatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = AsciiMath2ParserContextType;
	fn get_rule_index(&self) -> usize { RULE_d_dx_prefix_operator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_d_dx_prefix_operator }
}
antlr_rust::tid!{D_dx_prefix_operatorContextExt<'a>}

impl<'input> D_dx_prefix_operatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn AsciiMath2ParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<D_dx_prefix_operatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,D_dx_prefix_operatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait D_dx_prefix_operatorContextAttrs<'input>: AsciiMath2ParserContext<'input> + BorrowMut<D_dx_prefix_operatorContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token D_LOWERCASE
/// Returns `None` if there is no child corresponding to token D_LOWERCASE
fn D_LOWERCASE(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(D_LOWERCASE, 0)
}
/// Retrieves first TerminalNode corresponding to token FSLASH
/// Returns `None` if there is no child corresponding to token FSLASH
fn FSLASH(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
	self.get_token(FSLASH, 0)
}
fn differential(&self) -> Option<Rc<DifferentialContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> D_dx_prefix_operatorContextAttrs<'input> for D_dx_prefix_operatorContext<'input>{}

impl<'input, I, H> AsciiMath2Parser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn d_dx_prefix_operator(&mut self,)
	-> Result<Rc<D_dx_prefix_operatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = D_dx_prefix_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_d_dx_prefix_operator);
        let mut _localctx: Rc<D_dx_prefix_operatorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(112);
			recog.base.match_token(D_LOWERCASE,&mut recog.err_handler)?;

			recog.base.set_state(113);
			recog.base.match_token(FSLASH,&mut recog.err_handler)?;

			/*InvokeRule differential*/
			recog.base.set_state(114);
			recog.differential()?;

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
        recog.base.enter_rule(_localctx.clone(), 20, RULE_differential);
        let mut _localctx: Rc<DifferentialContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(116);
			recog.base.match_token(D_LOWERCASE,&mut recog.err_handler)?;

			recog.base.set_state(117);
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
        recog.base.enter_rule(_localctx.clone(), 22, RULE_script_op_expression);
        let mut _localctx: Rc<Script_op_expressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(142);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
				1 =>{
					let tmp = PowerSubscriptExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule primary_expression*/
					recog.base.set_state(119);
					recog.primary_expression()?;

					recog.base.set_state(128);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(11,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							{
							recog.base.set_state(120);
							recog.base.match_token(HAT,&mut recog.err_handler)?;

							/*InvokeRule primary_expression*/
							recog.base.set_state(121);
							recog.primary_expression()?;

							recog.base.set_state(124);
							recog.err_handler.sync(&mut recog.base)?;
							match  recog.interpreter.adaptive_predict(10,&mut recog.base)? {
								x if x == 1=>{
									{
									recog.base.set_state(122);
									recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

									/*InvokeRule primary_expression*/
									recog.base.set_state(123);
									recog.primary_expression()?;

									}
								}

								_ => {}
							}
							}
							}
							} 
						}
						recog.base.set_state(130);
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
					recog.base.set_state(131);
					recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(132);
					recog.primary_expression()?;

					recog.base.set_state(135);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(12,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(133);
							recog.base.match_token(HAT,&mut recog.err_handler)?;

							/*InvokeRule primary_expression*/
							recog.base.set_state(134);
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
					recog.base.set_state(137);
					recog.base.match_token(HAT,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(138);
					recog.primary_expression()?;

					}
				}
			,
				4 =>{
					let tmp = SubscriptExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(139);
					recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(140);
					recog.primary_expression()?;

					}
				}
			,
				5 =>{
					let tmp = PrimeExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(141);
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
	/// Retrieves first TerminalNode corresponding to token DERIV
	/// Returns `None` if there is no child corresponding to token DERIV
	fn DERIV(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(DERIV, 0)
	}
	fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn wrt_argument(&self) -> Option<Rc<Wrt_argumentContextAll<'input>>> where Self:Sized{
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
	/// Retrieves first TerminalNode corresponding to token BUILTIN_KEYWORD_FUNC_NAME
	/// Returns `None` if there is no child corresponding to token BUILTIN_KEYWORD_FUNC_NAME
	fn BUILTIN_KEYWORD_FUNC_NAME(&self) -> Option<Rc<TerminalNode<'input,AsciiMath2ParserContextType>>> where Self:Sized{
		self.get_token(BUILTIN_KEYWORD_FUNC_NAME, 0)
	}
	fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
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
        recog.base.enter_rule(_localctx.clone(), 24, RULE_primary_expression);
        let mut _localctx: Rc<Primary_expressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(282);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(24,&mut recog.base)? {
				1 =>{
					let tmp = ExplicitIdentifierCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(144);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(150);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PRIME {
						{
						recog.base.set_state(146); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						loop {
							{
							{
							recog.base.set_state(145);
							recog.base.match_token(PRIME,&mut recog.err_handler)?;

							}
							}
							recog.base.set_state(148); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if !(_la==PRIME) {break}
						}
						}
					}

					recog.base.set_state(152);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule arguments*/
					recog.base.set_state(153);
					recog.arguments()?;

					recog.base.set_state(154);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = ExplicitKeywordCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(156);
					recog.base.match_token(BUILTIN_KEYWORD_FUNC_NAME,&mut recog.err_handler)?;

					recog.base.set_state(157);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule arguments*/
					recog.base.set_state(158);
					recog.arguments()?;

					recog.base.set_state(159);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = SimpleKeywordCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(161);
					recog.base.match_token(BUILTIN_KEYWORD_FUNC_NAME,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(162);
					recog.primary_expression()?;

					}
				}
			,
				4 =>{
					let tmp = ParenColumnVectorContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(163);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule paren_element_for_column_vector*/
					recog.base.set_state(164);
					recog.paren_element_for_column_vector()?;

					recog.base.set_state(169);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(165);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule paren_element_for_column_vector*/
						recog.base.set_state(166);
						recog.paren_element_for_column_vector()?;

						}
						}
						recog.base.set_state(171);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(172);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					let tmp = ParenMatrixContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					recog.base.set_state(174);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule matrix_content*/
					recog.base.set_state(175);
					recog.matrix_content()?;

					recog.base.set_state(176);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					let tmp = ParenExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					recog.base.set_state(178);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(179);
					recog.expression()?;

					recog.base.set_state(180);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					let tmp = BracketMatrixContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					recog.base.set_state(182);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					/*InvokeRule matrix_content*/
					recog.base.set_state(183);
					recog.matrix_content()?;

					recog.base.set_state(184);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					let tmp = AngleBracketRowVectorContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(186);
					recog.base.match_token(L_ANGLE,&mut recog.err_handler)?;

					/*InvokeRule matrix_row*/
					recog.base.set_state(187);
					recog.matrix_row()?;

					recog.base.set_state(188);
					recog.base.match_token(R_ANGLE,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					let tmp = BraceExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9);
					_localctx = tmp;
					{
					recog.base.set_state(190);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(191);
					recog.expression()?;

					recog.base.set_state(192);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

					}
				}
			,
				10 =>{
					let tmp = AbsExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10);
					_localctx = tmp;
					{
					recog.base.set_state(194);
					recog.base.match_token(ABS,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(195);
					recog.expression()?;

					recog.base.set_state(196);
					recog.base.match_token(ABS,&mut recog.err_handler)?;

					}
				}
			,
				11 =>{
					let tmp = ExplicitIdentifierCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11);
					_localctx = tmp;
					{
					recog.base.set_state(198);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(204);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==PRIME {
						{
						recog.base.set_state(200); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						loop {
							{
							{
							recog.base.set_state(199);
							recog.base.match_token(PRIME,&mut recog.err_handler)?;

							}
							}
							recog.base.set_state(202); 
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							if !(_la==PRIME) {break}
						}
						}
					}

					recog.base.set_state(206);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule arguments*/
					recog.base.set_state(207);
					recog.arguments()?;

					recog.base.set_state(208);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				12 =>{
					let tmp = ExplicitKeywordCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12);
					_localctx = tmp;
					{
					recog.base.set_state(210);
					recog.base.match_token(BUILTIN_KEYWORD_FUNC_NAME,&mut recog.err_handler)?;

					recog.base.set_state(211);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule arguments*/
					recog.base.set_state(212);
					recog.arguments()?;

					recog.base.set_state(213);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				13 =>{
					let tmp = SimpleKeywordCallContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 13);
					_localctx = tmp;
					{
					recog.base.set_state(215);
					recog.base.match_token(BUILTIN_KEYWORD_FUNC_NAME,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(216);
					recog.primary_expression()?;

					}
				}
			,
				14 =>{
					let tmp = SqrtFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 14);
					_localctx = tmp;
					{
					recog.base.set_state(217);
					recog.base.match_token(SQRT,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(218);
					recog.primary_expression()?;

					}
				}
			,
				15 =>{
					let tmp = RootFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 15);
					_localctx = tmp;
					{
					recog.base.set_state(219);
					recog.base.match_token(ROOT,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(220);
					recog.primary_expression()?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(221);
					recog.primary_expression()?;

					}
				}
			,
				16 =>{
					let tmp = FracFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 16);
					_localctx = tmp;
					{
					recog.base.set_state(223);
					recog.base.match_token(FRAC,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(224);
					recog.primary_expression()?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(225);
					recog.primary_expression()?;

					}
				}
			,
				17 =>{
					let tmp = TextFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 17);
					_localctx = tmp;
					{
					recog.base.set_state(227);
					recog.base.match_token(TEXT,&mut recog.err_handler)?;

					recog.base.set_state(228);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule text_argument*/
					recog.base.set_state(229);
					recog.text_argument()?;

					recog.base.set_state(230);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				18 =>{
					let tmp = IntegralExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 18);
					_localctx = tmp;
					{
					recog.base.set_state(232);
					recog.base.match_token(INTEGRAL,&mut recog.err_handler)?;

					recog.base.set_state(235);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==UNDERSCORE {
						{
						recog.base.set_state(233);
						recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

						/*InvokeRule primary_expression*/
						recog.base.set_state(234);
						recog.primary_expression()?;

						}
					}

					recog.base.set_state(239);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==HAT {
						{
						recog.base.set_state(237);
						recog.base.match_token(HAT,&mut recog.err_handler)?;

						/*InvokeRule primary_expression*/
						recog.base.set_state(238);
						recog.primary_expression()?;

						}
					}

					/*InvokeRule primary_expression*/
					recog.base.set_state(241);
					recog.primary_expression()?;

					recog.base.set_state(243);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(21,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule differential*/
							recog.base.set_state(242);
							recog.differential()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				19 =>{
					let tmp = DerivativeFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 19);
					_localctx = tmp;
					{
					recog.base.set_state(245);
					recog.base.match_token(DERIV,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(246);
					recog.primary_expression()?;

					recog.base.set_state(248);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule wrt_argument*/
							recog.base.set_state(247);
							recog.wrt_argument()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				20 =>{
					let tmp = PartialFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 20);
					_localctx = tmp;
					{
					recog.base.set_state(250);
					recog.base.match_token(PARTIAL,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(251);
					recog.primary_expression()?;

					recog.base.set_state(253);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(23,&mut recog.base)? {
						x if x == 1=>{
							{
							/*InvokeRule wrt_argument*/
							recog.base.set_state(252);
							recog.wrt_argument()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				21 =>{
					let tmp = FractionLeibnizContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 21);
					_localctx = tmp;
					{
					/*InvokeRule differential*/
					recog.base.set_state(255);
					recog.differential()?;

					recog.base.set_state(256);
					recog.base.match_token(FSLASH,&mut recog.err_handler)?;

					/*InvokeRule differential*/
					recog.base.set_state(257);
					recog.differential()?;

					}
				}
			,
				22 =>{
					let tmp = LimitExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 22);
					_localctx = tmp;
					{
					recog.base.set_state(259);
					recog.base.match_token(LIM,&mut recog.err_handler)?;

					recog.base.set_state(260);
					recog.base.match_token(UNDERSCORE,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(261);
					recog.primary_expression()?;

					recog.base.set_state(262);
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
					recog.base.set_state(263);
					recog.primary_expression()?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(264);
					recog.primary_expression()?;

					}
				}
			,
				23 =>{
					let tmp = MatFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 23);
					_localctx = tmp;
					{
					recog.base.set_state(266);
					recog.base.match_token(MAT,&mut recog.err_handler)?;

					recog.base.set_state(267);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule matrix_content*/
					recog.base.set_state(268);
					recog.matrix_content()?;

					recog.base.set_state(269);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

					}
				}
			,
				24 =>{
					let tmp = DetFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 24);
					_localctx = tmp;
					{
					recog.base.set_state(271);
					recog.base.match_token(DET,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(272);
					recog.primary_expression()?;

					}
				}
			,
				25 =>{
					let tmp = TransposeFunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 25);
					_localctx = tmp;
					{
					recog.base.set_state(273);
					recog.base.match_token(TRANSPOSE,&mut recog.err_handler)?;

					/*InvokeRule primary_expression*/
					recog.base.set_state(274);
					recog.primary_expression()?;

					}
				}
			,
				26 =>{
					let tmp = IdentifierAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 26);
					_localctx = tmp;
					{
					recog.base.set_state(275);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				27 =>{
					let tmp = NumberAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 27);
					_localctx = tmp;
					{
					recog.base.set_state(276);
					recog.base.match_token(NUMBER,&mut recog.err_handler)?;

					}
				}
			,
				28 =>{
					let tmp = NumberWithCommasAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 28);
					_localctx = tmp;
					{
					recog.base.set_state(277);
					recog.base.match_token(NUMBER_WITH_COMMAS,&mut recog.err_handler)?;

					}
				}
			,
				29 =>{
					let tmp = CurrencyNumberAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 29);
					_localctx = tmp;
					{
					recog.base.set_state(278);
					recog.base.match_token(CURRENCY_NUMBER,&mut recog.err_handler)?;

					}
				}
			,
				30 =>{
					let tmp = GreekLetterAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 30);
					_localctx = tmp;
					{
					recog.base.set_state(279);
					recog.base.match_token(GREEK_LETTER,&mut recog.err_handler)?;

					}
				}
			,
				31 =>{
					let tmp = ConstantAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 31);
					_localctx = tmp;
					{
					/*InvokeRule constant_symbol*/
					recog.base.set_state(280);
					recog.constant_symbol()?;

					}
				}
			,
				32 =>{
					let tmp = StringAtomContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 32);
					_localctx = tmp;
					{
					recog.base.set_state(281);
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
        recog.base.enter_rule(_localctx.clone(), 26, RULE_paren_element_for_column_vector);
        let mut _localctx: Rc<Paren_element_for_column_vectorContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(284);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(285);
			recog.expression()?;

			recog.base.set_state(286);
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
        recog.base.enter_rule(_localctx.clone(), 28, RULE_arguments);
        let mut _localctx: Rc<ArgumentsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(297);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BUILTIN_KEYWORD_FUNC_NAME | INTEGRAL | D_LOWERCASE | DERIV | PARTIAL |
			 LIM | SQRT | ROOT | FRAC | TEXT | MAT | DET | TRANSPOSE | PI_CONST |
			 E_CONST | I_CONST | INFINITY_CONST | GAMMA_CONST | PHI_CONST | TRUE_CONST |
			 FALSE_CONST | NAN_CONST | PLUS | MINUS | HAT | UNDERSCORE | PRIME | LPAREN |
			 LBRACKET | LBRACE | ABS | L_ANGLE | GREEK_LETTER | IDENTIFIER | NUMBER |
			 NUMBER_WITH_COMMAS | CURRENCY_NUMBER | STRING 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expression*/
					recog.base.set_state(288);
					recog.expression()?;

					recog.base.set_state(293);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(289);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(290);
						recog.expression()?;

						}
						}
						recog.base.set_state(295);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					}
				}

			 RPAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
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
        recog.base.enter_rule(_localctx.clone(), 30, RULE_text_argument);
        let mut _localctx: Rc<Text_argumentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(301);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(299);
					recog.base.match_token(STRING,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(300);
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
        recog.base.enter_rule(_localctx.clone(), 32, RULE_wrt_argument);
        let mut _localctx: Rc<Wrt_argumentContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(303);
			recog.base.match_token(COMMA,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(304);
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
        recog.base.enter_rule(_localctx.clone(), 34, RULE_matrix_content);
        let mut _localctx: Rc<Matrix_contentContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule matrix_row*/
			recog.base.set_state(306);
			recog.matrix_row()?;

			recog.base.set_state(311);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SEMICOLON {
				{
				{
				recog.base.set_state(307);
				recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

				/*InvokeRule matrix_row*/
				recog.base.set_state(308);
				recog.matrix_row()?;

				}
				}
				recog.base.set_state(313);
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
        recog.base.enter_rule(_localctx.clone(), 36, RULE_matrix_row);
        let mut _localctx: Rc<Matrix_rowContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(314);
			recog.expression()?;

			recog.base.set_state(319);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(315);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(316);
				recog.expression()?;

				}
				}
				recog.base.set_state(321);
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
        recog.base.enter_rule(_localctx.clone(), 38, RULE_constant_symbol);
        let mut _localctx: Rc<Constant_symbolContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(322);
			_la = recog.base.input.la(1);
			if { !(((((_la - 53)) & !0x3f) == 0 && ((1usize << (_la - 53)) & ((1usize << (PI_CONST - 53)) | (1usize << (E_CONST - 53)) | (1usize << (I_CONST - 53)) | (1usize << (INFINITY_CONST - 53)) | (1usize << (GAMMA_CONST - 53)) | (1usize << (PHI_CONST - 53)) | (1usize << (TRUE_CONST - 53)) | (1usize << (FALSE_CONST - 53)) | (1usize << (NAN_CONST - 53)))) != 0)) } {
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
	\x7e\u{147}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x03\x02\x03\x02\x03\x02\x07\
	\x02\x2e\x0a\x02\x0c\x02\x0e\x02\x31\x0b\x02\x03\x02\x07\x02\x34\x0a\x02\
	\x0c\x02\x0e\x02\x37\x0b\x02\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x03\
	\x04\x03\x04\x07\x04\x40\x0a\x04\x0c\x04\x0e\x04\x43\x0b\x04\x03\x05\x03\
	\x05\x03\x05\x03\x05\x05\x05\x49\x0a\x05\x05\x05\x4b\x0a\x05\x03\x06\x03\
	\x06\x03\x06\x06\x06\x50\x0a\x06\x0d\x06\x0e\x06\x51\x03\x07\x03\x07\x03\
	\x07\x07\x07\x57\x0a\x07\x0c\x07\x0e\x07\x5a\x0b\x07\x03\x08\x03\x08\x03\
	\x08\x03\x08\x07\x08\x60\x0a\x08\x0c\x08\x0e\x08\x63\x0b\x08\x03\x09\x03\
	\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x05\x09\x6c\x0a\x09\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x05\x0d\x7f\x0a\
	\x0d\x07\x0d\u{81}\x0a\x0d\x0c\x0d\x0e\x0d\u{84}\x0b\x0d\x03\x0d\x03\x0d\
	\x03\x0d\x03\x0d\x05\x0d\u{8a}\x0a\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\
	\x0d\x05\x0d\u{91}\x0a\x0d\x03\x0e\x03\x0e\x06\x0e\u{95}\x0a\x0e\x0d\x0e\
	\x0e\x0e\u{96}\x05\x0e\u{99}\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x07\x0e\u{aa}\x0a\x0e\x0c\x0e\x0e\x0e\u{ad}\x0b\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x06\x0e\u{cb}\x0a\x0e\x0d\x0e\x0e\x0e\u{cc}\x05\x0e\u{cf}\x0a\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x05\x0e\u{ee}\x0a\x0e\x03\x0e\x03\x0e\x05\x0e\u{f2}\x0a\x0e\x03\
	\x0e\x03\x0e\x05\x0e\u{f6}\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{fb}\
	\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{100}\x0a\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{11d}\
	\x0a\x0e\x03\x0f\x03\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x10\x07\x10\
	\u{126}\x0a\x10\x0c\x10\x0e\x10\u{129}\x0b\x10\x03\x10\x05\x10\u{12c}\x0a\
	\x10\x03\x11\x03\x11\x05\x11\u{130}\x0a\x11\x03\x12\x03\x12\x03\x12\x03\
	\x13\x03\x13\x03\x13\x07\x13\u{138}\x0a\x13\x0c\x13\x0e\x13\u{13b}\x0b\x13\
	\x03\x14\x03\x14\x03\x14\x07\x14\u{140}\x0a\x14\x0c\x14\x0e\x14\u{143}\x0b\
	\x14\x03\x15\x03\x15\x03\x15\x02\x02\x16\x02\x04\x06\x08\x0a\x0c\x0e\x10\
	\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x02\x0a\x03\x02\x54\x55\
	\x03\x02\x48\x4d\x04\x02\x40\x41\x51\x51\x04\x02\x42\x43\x52\x53\x03\x02\
	\x40\x41\x03\x02\x76\x77\x03\x02\x4e\x4f\x03\x02\x37\x3f\x02\u{173}\x02\
	\x2a\x03\x02\x02\x02\x04\x3a\x03\x02\x02\x02\x06\x3c\x03\x02\x02\x02\x08\
	\x4a\x03\x02\x02\x02\x0a\x4c\x03\x02\x02\x02\x0c\x53\x03\x02\x02\x02\x0e\
	\x5b\x03\x02\x02\x02\x10\x6b\x03\x02\x02\x02\x12\x6d\x03\x02\x02\x02\x14\
	\x72\x03\x02\x02\x02\x16\x76\x03\x02\x02\x02\x18\u{90}\x03\x02\x02\x02\x1a\
	\u{11c}\x03\x02\x02\x02\x1c\u{11e}\x03\x02\x02\x02\x1e\u{12b}\x03\x02\x02\
	\x02\x20\u{12f}\x03\x02\x02\x02\x22\u{131}\x03\x02\x02\x02\x24\u{134}\x03\
	\x02\x02\x02\x26\u{13c}\x03\x02\x02\x02\x28\u{144}\x03\x02\x02\x02\x2a\x2f\
	\x05\x04\x03\x02\x2b\x2c\x07\x7d\x02\x02\x2c\x2e\x05\x04\x03\x02\x2d\x2b\
	\x03\x02\x02\x02\x2e\x31\x03\x02\x02\x02\x2f\x2d\x03\x02\x02\x02\x2f\x30\
	\x03\x02\x02\x02\x30\x35\x03\x02\x02\x02\x31\x2f\x03\x02\x02\x02\x32\x34\
	\x07\x7d\x02\x02\x33\x32\x03\x02\x02\x02\x34\x37\x03\x02\x02\x02\x35\x33\
	\x03\x02\x02\x02\x35\x36\x03\x02\x02\x02\x36\x38\x03\x02\x02\x02\x37\x35\
	\x03\x02\x02\x02\x38\x39\x07\x02\x02\x03\x39\x03\x03\x02\x02\x02\x3a\x3b\
	\x05\x06\x04\x02\x3b\x05\x03\x02\x02\x02\x3c\x41\x05\x08\x05\x02\x3d\x3e\
	\x09\x02\x02\x02\x3e\x40\x05\x08\x05\x02\x3f\x3d\x03\x02\x02\x02\x40\x43\
	\x03\x02\x02\x02\x41\x3f\x03\x02\x02\x02\x41\x42\x03\x02\x02\x02\x42\x07\
	\x03\x02\x02\x02\x43\x41\x03\x02\x02\x02\x44\x4b\x05\x0a\x06\x02\x45\x48\
	\x05\x0c\x07\x02\x46\x47\x09\x03\x02\x02\x47\x49\x05\x0c\x07\x02\x48\x46\
	\x03\x02\x02\x02\x48\x49\x03\x02\x02\x02\x49\x4b\x03\x02\x02\x02\x4a\x44\
	\x03\x02\x02\x02\x4a\x45\x03\x02\x02\x02\x4b\x09\x03\x02\x02\x02\x4c\x4d\
	\x05\x0c\x07\x02\x4d\x4f\x07\x48\x02\x02\x4e\x50\x07\x7d\x02\x02\x4f\x4e\
	\x03\x02\x02\x02\x50\x51\x03\x02\x02\x02\x51\x4f\x03\x02\x02\x02\x51\x52\
	\x03\x02\x02\x02\x52\x0b\x03\x02\x02\x02\x53\x58\x05\x0e\x08\x02\x54\x55\
	\x09\x04\x02\x02\x55\x57\x05\x0e\x08\x02\x56\x54\x03\x02\x02\x02\x57\x5a\
	\x03\x02\x02\x02\x58\x56\x03\x02\x02\x02\x58\x59\x03\x02\x02\x02\x59\x0d\
	\x03\x02\x02\x02\x5a\x58\x03\x02\x02\x02\x5b\x61\x05\x10\x09\x02\x5c\x5d\
	\x09\x05\x02\x02\x5d\x60\x05\x10\x09\x02\x5e\x60\x05\x10\x09\x02\x5f\x5c\
	\x03\x02\x02\x02\x5f\x5e\x03\x02\x02\x02\x60\x63\x03\x02\x02\x02\x61\x5f\
	\x03\x02\x02\x02\x61\x62\x03\x02\x02\x02\x62\x0f\x03\x02\x02\x02\x63\x61\
	\x03\x02\x02\x02\x64\x65\x09\x06\x02\x02\x65\x6c\x05\x18\x0d\x02\x66\x6c\
	\x05\x12\x0a\x02\x67\x68\x05\x14\x0b\x02\x68\x69\x05\x18\x0d\x02\x69\x6c\
	\x03\x02\x02\x02\x6a\x6c\x05\x18\x0d\x02\x6b\x64\x03\x02\x02\x02\x6b\x66\
	\x03\x02\x02\x02\x6b\x67\x03\x02\x02\x02\x6b\x6a\x03\x02\x02\x02\x6c\x11\
	\x03\x02\x02\x02\x6d\x6e\x05\x14\x0b\x02\x6e\x6f\x07\x57\x02\x02\x6f\x70\
	\x05\x1a\x0e\x02\x70\x71\x07\x58\x02\x02\x71\x13\x03\x02\x02\x02\x72\x73\
	\x07\x05\x02\x02\x73\x74\x07\x43\x02\x02\x74\x75\x05\x16\x0c\x02\x75\x15\
	\x03\x02\x02\x02\x76\x77\x07\x05\x02\x02\x77\x78\x09\x07\x02\x02\x78\x17\
	\x03\x02\x02\x02\x79\u{82}\x05\x1a\x0e\x02\x7a\x7b\x07\x44\x02\x02\x7b\x7e\
	\x05\x1a\x0e\x02\x7c\x7d\x07\x45\x02\x02\x7d\x7f\x05\x1a\x0e\x02\x7e\x7c\
	\x03\x02\x02\x02\x7e\x7f\x03\x02\x02\x02\x7f\u{81}\x03\x02\x02\x02\u{80}\
	\x7a\x03\x02\x02\x02\u{81}\u{84}\x03\x02\x02\x02\u{82}\u{80}\x03\x02\x02\
	\x02\u{82}\u{83}\x03\x02\x02\x02\u{83}\u{91}\x03\x02\x02\x02\u{84}\u{82}\
	\x03\x02\x02\x02\u{85}\u{86}\x07\x45\x02\x02\u{86}\u{89}\x05\x1a\x0e\x02\
	\u{87}\u{88}\x07\x44\x02\x02\u{88}\u{8a}\x05\x1a\x0e\x02\u{89}\u{87}\x03\
	\x02\x02\x02\u{89}\u{8a}\x03\x02\x02\x02\u{8a}\u{91}\x03\x02\x02\x02\u{8b}\
	\u{8c}\x07\x44\x02\x02\u{8c}\u{91}\x05\x1a\x0e\x02\u{8d}\u{8e}\x07\x45\x02\
	\x02\u{8e}\u{91}\x05\x1a\x0e\x02\u{8f}\u{91}\x07\x46\x02\x02\u{90}\x79\x03\
	\x02\x02\x02\u{90}\u{85}\x03\x02\x02\x02\u{90}\u{8b}\x03\x02\x02\x02\u{90}\
	\u{8d}\x03\x02\x02\x02\u{90}\u{8f}\x03\x02\x02\x02\u{91}\x19\x03\x02\x02\
	\x02\u{92}\u{98}\x07\x77\x02\x02\u{93}\u{95}\x07\x46\x02\x02\u{94}\u{93}\
	\x03\x02\x02\x02\u{95}\u{96}\x03\x02\x02\x02\u{96}\u{94}\x03\x02\x02\x02\
	\u{96}\u{97}\x03\x02\x02\x02\u{97}\u{99}\x03\x02\x02\x02\u{98}\u{94}\x03\
	\x02\x02\x02\u{98}\u{99}\x03\x02\x02\x02\u{99}\u{9a}\x03\x02\x02\x02\u{9a}\
	\u{9b}\x07\x57\x02\x02\u{9b}\u{9c}\x05\x1e\x10\x02\u{9c}\u{9d}\x07\x58\x02\
	\x02\u{9d}\u{11d}\x03\x02\x02\x02\u{9e}\u{9f}\x07\x03\x02\x02\u{9f}\u{a0}\
	\x07\x57\x02\x02\u{a0}\u{a1}\x05\x1e\x10\x02\u{a1}\u{a2}\x07\x58\x02\x02\
	\u{a2}\u{11d}\x03\x02\x02\x02\u{a3}\u{a4}\x07\x03\x02\x02\u{a4}\u{11d}\x05\
	\x1a\x0e\x02\u{a5}\u{a6}\x07\x57\x02\x02\u{a6}\u{ab}\x05\x1c\x0f\x02\u{a7}\
	\u{a8}\x07\x60\x02\x02\u{a8}\u{aa}\x05\x1c\x0f\x02\u{a9}\u{a7}\x03\x02\x02\
	\x02\u{aa}\u{ad}\x03\x02\x02\x02\u{ab}\u{a9}\x03\x02\x02\x02\u{ab}\u{ac}\
	\x03\x02\x02\x02\u{ac}\u{ae}\x03\x02\x02\x02\u{ad}\u{ab}\x03\x02\x02\x02\
	\u{ae}\u{af}\x07\x58\x02\x02\u{af}\u{11d}\x03\x02\x02\x02\u{b0}\u{b1}\x07\
	\x57\x02\x02\u{b1}\u{b2}\x05\x24\x13\x02\u{b2}\u{b3}\x07\x58\x02\x02\u{b3}\
	\u{11d}\x03\x02\x02\x02\u{b4}\u{b5}\x07\x57\x02\x02\u{b5}\u{b6}\x05\x04\
	\x03\x02\u{b6}\u{b7}\x07\x58\x02\x02\u{b7}\u{11d}\x03\x02\x02\x02\u{b8}\
	\u{b9}\x07\x59\x02\x02\u{b9}\u{ba}\x05\x24\x13\x02\u{ba}\u{bb}\x07\x5a\x02\
	\x02\u{bb}\u{11d}\x03\x02\x02\x02\u{bc}\u{bd}\x07\x5e\x02\x02\u{bd}\u{be}\
	\x05\x26\x14\x02\u{be}\u{bf}\x07\x5f\x02\x02\u{bf}\u{11d}\x03\x02\x02\x02\
	\u{c0}\u{c1}\x07\x5b\x02\x02\u{c1}\u{c2}\x05\x04\x03\x02\u{c2}\u{c3}\x07\
	\x5c\x02\x02\u{c3}\u{11d}\x03\x02\x02\x02\u{c4}\u{c5}\x07\x5d\x02\x02\u{c5}\
	\u{c6}\x05\x04\x03\x02\u{c6}\u{c7}\x07\x5d\x02\x02\u{c7}\u{11d}\x03\x02\
	\x02\x02\u{c8}\u{ce}\x07\x77\x02\x02\u{c9}\u{cb}\x07\x46\x02\x02\u{ca}\u{c9}\
	\x03\x02\x02\x02\u{cb}\u{cc}\x03\x02\x02\x02\u{cc}\u{ca}\x03\x02\x02\x02\
	\u{cc}\u{cd}\x03\x02\x02\x02\u{cd}\u{cf}\x03\x02\x02\x02\u{ce}\u{ca}\x03\
	\x02\x02\x02\u{ce}\u{cf}\x03\x02\x02\x02\u{cf}\u{d0}\x03\x02\x02\x02\u{d0}\
	\u{d1}\x07\x57\x02\x02\u{d1}\u{d2}\x05\x1e\x10\x02\u{d2}\u{d3}\x07\x58\x02\
	\x02\u{d3}\u{11d}\x03\x02\x02\x02\u{d4}\u{d5}\x07\x03\x02\x02\u{d5}\u{d6}\
	\x07\x57\x02\x02\u{d6}\u{d7}\x05\x1e\x10\x02\u{d7}\u{d8}\x07\x58\x02\x02\
	\u{d8}\u{11d}\x03\x02\x02\x02\u{d9}\u{da}\x07\x03\x02\x02\u{da}\u{11d}\x05\
	\x1a\x0e\x02\u{db}\u{dc}\x07\x30\x02\x02\u{dc}\u{11d}\x05\x1a\x0e\x02\u{dd}\
	\u{de}\x07\x31\x02\x02\u{de}\u{df}\x05\x1a\x0e\x02\u{df}\u{e0}\x05\x1a\x0e\
	\x02\u{e0}\u{11d}\x03\x02\x02\x02\u{e1}\u{e2}\x07\x32\x02\x02\u{e2}\u{e3}\
	\x05\x1a\x0e\x02\u{e3}\u{e4}\x05\x1a\x0e\x02\u{e4}\u{11d}\x03\x02\x02\x02\
	\u{e5}\u{e6}\x07\x33\x02\x02\u{e6}\u{e7}\x07\x57\x02\x02\u{e7}\u{e8}\x05\
	\x20\x11\x02\u{e8}\u{e9}\x07\x58\x02\x02\u{e9}\u{11d}\x03\x02\x02\x02\u{ea}\
	\u{ed}\x07\x04\x02\x02\u{eb}\u{ec}\x07\x45\x02\x02\u{ec}\u{ee}\x05\x1a\x0e\
	\x02\u{ed}\u{eb}\x03\x02\x02\x02\u{ed}\u{ee}\x03\x02\x02\x02\u{ee}\u{f1}\
	\x03\x02\x02\x02\u{ef}\u{f0}\x07\x44\x02\x02\u{f0}\u{f2}\x05\x1a\x0e\x02\
	\u{f1}\u{ef}\x03\x02\x02\x02\u{f1}\u{f2}\x03\x02\x02\x02\u{f2}\u{f3}\x03\
	\x02\x02\x02\u{f3}\u{f5}\x05\x1a\x0e\x02\u{f4}\u{f6}\x05\x16\x0c\x02\u{f5}\
	\u{f4}\x03\x02\x02\x02\u{f5}\u{f6}\x03\x02\x02\x02\u{f6}\u{11d}\x03\x02\
	\x02\x02\u{f7}\u{f8}\x07\x06\x02\x02\u{f8}\u{fa}\x05\x1a\x0e\x02\u{f9}\u{fb}\
	\x05\x22\x12\x02\u{fa}\u{f9}\x03\x02\x02\x02\u{fa}\u{fb}\x03\x02\x02\x02\
	\u{fb}\u{11d}\x03\x02\x02\x02\u{fc}\u{fd}\x07\x08\x02\x02\u{fd}\u{ff}\x05\
	\x1a\x0e\x02\u{fe}\u{100}\x05\x22\x12\x02\u{ff}\u{fe}\x03\x02\x02\x02\u{ff}\
	\u{100}\x03\x02\x02\x02\u{100}\u{11d}\x03\x02\x02\x02\u{101}\u{102}\x05\
	\x16\x0c\x02\u{102}\u{103}\x07\x43\x02\x02\u{103}\u{104}\x05\x16\x0c\x02\
	\u{104}\u{11d}\x03\x02\x02\x02\u{105}\u{106}\x07\x09\x02\x02\u{106}\u{107}\
	\x07\x45\x02\x02\u{107}\u{108}\x05\x1a\x0e\x02\u{108}\u{109}\x09\x08\x02\
	\x02\u{109}\u{10a}\x05\x1a\x0e\x02\u{10a}\u{10b}\x05\x1a\x0e\x02\u{10b}\
	\u{11d}\x03\x02\x02\x02\u{10c}\u{10d}\x07\x34\x02\x02\u{10d}\u{10e}\x07\
	\x57\x02\x02\u{10e}\u{10f}\x05\x24\x13\x02\u{10f}\u{110}\x07\x58\x02\x02\
	\u{110}\u{11d}\x03\x02\x02\x02\u{111}\u{112}\x07\x35\x02\x02\u{112}\u{11d}\
	\x05\x1a\x0e\x02\u{113}\u{114}\x07\x36\x02\x02\u{114}\u{11d}\x05\x1a\x0e\
	\x02\u{115}\u{11d}\x07\x77\x02\x02\u{116}\u{11d}\x07\x78\x02\x02\u{117}\
	\u{11d}\x07\x79\x02\x02\u{118}\u{11d}\x07\x7a\x02\x02\u{119}\u{11d}\x07\
	\x76\x02\x02\u{11a}\u{11d}\x05\x28\x15\x02\u{11b}\u{11d}\x07\x7c\x02\x02\
	\u{11c}\u{92}\x03\x02\x02\x02\u{11c}\u{9e}\x03\x02\x02\x02\u{11c}\u{a3}\
	\x03\x02\x02\x02\u{11c}\u{a5}\x03\x02\x02\x02\u{11c}\u{b0}\x03\x02\x02\x02\
	\u{11c}\u{b4}\x03\x02\x02\x02\u{11c}\u{b8}\x03\x02\x02\x02\u{11c}\u{bc}\
	\x03\x02\x02\x02\u{11c}\u{c0}\x03\x02\x02\x02\u{11c}\u{c4}\x03\x02\x02\x02\
	\u{11c}\u{c8}\x03\x02\x02\x02\u{11c}\u{d4}\x03\x02\x02\x02\u{11c}\u{d9}\
	\x03\x02\x02\x02\u{11c}\u{db}\x03\x02\x02\x02\u{11c}\u{dd}\x03\x02\x02\x02\
	\u{11c}\u{e1}\x03\x02\x02\x02\u{11c}\u{e5}\x03\x02\x02\x02\u{11c}\u{ea}\
	\x03\x02\x02\x02\u{11c}\u{f7}\x03\x02\x02\x02\u{11c}\u{fc}\x03\x02\x02\x02\
	\u{11c}\u{101}\x03\x02\x02\x02\u{11c}\u{105}\x03\x02\x02\x02\u{11c}\u{10c}\
	\x03\x02\x02\x02\u{11c}\u{111}\x03\x02\x02\x02\u{11c}\u{113}\x03\x02\x02\
	\x02\u{11c}\u{115}\x03\x02\x02\x02\u{11c}\u{116}\x03\x02\x02\x02\u{11c}\
	\u{117}\x03\x02\x02\x02\u{11c}\u{118}\x03\x02\x02\x02\u{11c}\u{119}\x03\
	\x02\x02\x02\u{11c}\u{11a}\x03\x02\x02\x02\u{11c}\u{11b}\x03\x02\x02\x02\
	\u{11d}\x1b\x03\x02\x02\x02\u{11e}\u{11f}\x07\x57\x02\x02\u{11f}\u{120}\
	\x05\x04\x03\x02\u{120}\u{121}\x07\x58\x02\x02\u{121}\x1d\x03\x02\x02\x02\
	\u{122}\u{127}\x05\x04\x03\x02\u{123}\u{124}\x07\x60\x02\x02\u{124}\u{126}\
	\x05\x04\x03\x02\u{125}\u{123}\x03\x02\x02\x02\u{126}\u{129}\x03\x02\x02\
	\x02\u{127}\u{125}\x03\x02\x02\x02\u{127}\u{128}\x03\x02\x02\x02\u{128}\
	\u{12c}\x03\x02\x02\x02\u{129}\u{127}\x03\x02\x02\x02\u{12a}\u{12c}\x03\
	\x02\x02\x02\u{12b}\u{122}\x03\x02\x02\x02\u{12b}\u{12a}\x03\x02\x02\x02\
	\u{12c}\x1f\x03\x02\x02\x02\u{12d}\u{130}\x07\x7c\x02\x02\u{12e}\u{130}\
	\x05\x04\x03\x02\u{12f}\u{12d}\x03\x02\x02\x02\u{12f}\u{12e}\x03\x02\x02\
	\x02\u{130}\x21\x03\x02\x02\x02\u{131}\u{132}\x07\x60\x02\x02\u{132}\u{133}\
	\x05\x04\x03\x02\u{133}\x23\x03\x02\x02\x02\u{134}\u{139}\x05\x26\x14\x02\
	\u{135}\u{136}\x07\x61\x02\x02\u{136}\u{138}\x05\x26\x14\x02\u{137}\u{135}\
	\x03\x02\x02\x02\u{138}\u{13b}\x03\x02\x02\x02\u{139}\u{137}\x03\x02\x02\
	\x02\u{139}\u{13a}\x03\x02\x02\x02\u{13a}\x25\x03\x02\x02\x02\u{13b}\u{139}\
	\x03\x02\x02\x02\u{13c}\u{141}\x05\x04\x03\x02\u{13d}\u{13e}\x07\x60\x02\
	\x02\u{13e}\u{140}\x05\x04\x03\x02\u{13f}\u{13d}\x03\x02\x02\x02\u{140}\
	\u{143}\x03\x02\x02\x02\u{141}\u{13f}\x03\x02\x02\x02\u{141}\u{142}\x03\
	\x02\x02\x02\u{142}\x27\x03\x02\x02\x02\u{143}\u{141}\x03\x02\x02\x02\u{144}\
	\u{145}\x09\x09\x02\x02\u{145}\x29\x03\x02\x02\x02\x20\x2f\x35\x41\x48\x4a\
	\x51\x58\x5f\x61\x6b\x7e\u{82}\u{89}\u{90}\u{96}\u{98}\u{ab}\u{cc}\u{ce}\
	\u{ed}\u{f1}\u{f5}\u{fa}\u{ff}\u{11c}\u{127}\u{12b}\u{12f}\u{139}\u{141}";

