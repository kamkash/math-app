// Generated from calculator.g4 by ANTLR 4.8
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
use super::calculatorlistener::*;
use super::calculatorvisitor::*;

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

		pub const COS:isize=1; 
		pub const SIN:isize=2; 
		pub const TAN:isize=3; 
		pub const ACOS:isize=4; 
		pub const ASIN:isize=5; 
		pub const ATAN:isize=6; 
		pub const LN:isize=7; 
		pub const LOG:isize=8; 
		pub const EXP:isize=9; 
		pub const SQRT:isize=10; 
		pub const LPAREN:isize=11; 
		pub const RPAREN:isize=12; 
		pub const PLUS:isize=13; 
		pub const MINUS:isize=14; 
		pub const TIMES:isize=15; 
		pub const DIV:isize=16; 
		pub const GT:isize=17; 
		pub const LT:isize=18; 
		pub const EQ:isize=19; 
		pub const DOUBLE_EQ:isize=20; 
		pub const NE:isize=21; 
		pub const SEPARATOR:isize=22; 
		pub const COMMA:isize=23; 
		pub const POINT:isize=24; 
		pub const POW:isize=25; 
		pub const PI:isize=26; 
		pub const EULER:isize=27; 
		pub const VARIABLE:isize=28; 
		pub const SCIENTIFIC_NUMBER:isize=29; 
		pub const CURRENCY_NUMBER:isize=30; 
		pub const WS:isize=31;
	pub const RULE_block:usize = 0; 
	pub const RULE_functionDefinition:usize = 1; 
	pub const RULE_equation:usize = 2; 
	pub const RULE_relational_expression:usize = 3; 
	pub const RULE_expression:usize = 4; 
	pub const RULE_multiplyingExpression:usize = 5; 
	pub const RULE_powExpression:usize = 6; 
	pub const RULE_signedAtom:usize = 7; 
	pub const RULE_atom:usize = 8; 
	pub const RULE_scientific:usize = 9; 
	pub const RULE_currency:usize = 10; 
	pub const RULE_constant:usize = 11; 
	pub const RULE_variable:usize = 12; 
	pub const RULE_func_:usize = 13; 
	pub const RULE_funcname:usize = 14; 
	pub const RULE_relop:usize = 15; 
	pub const RULE_sumop:usize = 16; 
	pub const RULE_multop:usize = 17;
	pub const ruleNames: [&'static str; 18] =  [
		"block", "functionDefinition", "equation", "relational_expression", "expression", 
		"multiplyingExpression", "powExpression", "signedAtom", "atom", "scientific", 
		"currency", "constant", "variable", "func_", "funcname", "relop", "sumop", 
		"multop"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;27] = [
		None, Some("'cos'"), Some("'sin'"), Some("'tan'"), Some("'acos'"), Some("'asin'"), 
		Some("'atan'"), Some("'ln'"), Some("'log'"), Some("'exp'"), Some("'sqrt'"), 
		Some("'('"), Some("')'"), Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), 
		Some("'>'"), Some("'<'"), Some("'='"), Some("'=='"), Some("'!='"), None, 
		Some("','"), Some("'.'"), Some("'^'"), Some("'pi'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;32]  = [
		None, Some("COS"), Some("SIN"), Some("TAN"), Some("ACOS"), Some("ASIN"), 
		Some("ATAN"), Some("LN"), Some("LOG"), Some("EXP"), Some("SQRT"), Some("LPAREN"), 
		Some("RPAREN"), Some("PLUS"), Some("MINUS"), Some("TIMES"), Some("DIV"), 
		Some("GT"), Some("LT"), Some("EQ"), Some("DOUBLE_EQ"), Some("NE"), Some("SEPARATOR"), 
		Some("COMMA"), Some("POINT"), Some("POW"), Some("PI"), Some("EULER"), 
		Some("VARIABLE"), Some("SCIENTIFIC_NUMBER"), Some("CURRENCY_NUMBER"), 
		Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,calculatorParserExt<'input>, I, calculatorParserContextType , dyn calculatorListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type calculatorTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, calculatorParserContextType , dyn calculatorListener<'input> + 'a>;

/// Parser for calculator grammar
pub struct calculatorParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> calculatorParser<'input, I, H>
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
				calculatorParserExt{
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

impl<'input, I> calculatorParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> calculatorParser<'input, I, DefaultErrorStrategy<'input,calculatorParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for calculatorParser
pub trait calculatorParserContext<'input>:
	for<'x> Listenable<dyn calculatorListener<'input> + 'x > + 
	for<'x> Visitable<dyn calculatorVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=calculatorParserContextType>
{}

antlr_rust::coerce_from!{ 'input : calculatorParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn calculatorParserContext<'input> + 'input
where
    T: calculatorVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn calculatorVisitor<'input> + 'x))
    }
}

impl<'input> calculatorParserContext<'input> for TerminalNode<'input,calculatorParserContextType> {}
impl<'input> calculatorParserContext<'input> for ErrorNode<'input,calculatorParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn calculatorParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn calculatorListener<'input> + 'input }

pub struct calculatorParserContextType;
antlr_rust::tid!{calculatorParserContextType}

impl<'input> ParserNodeType<'input> for calculatorParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn calculatorParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct calculatorParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> calculatorParserExt<'input>{
}
antlr_rust::tid! { calculatorParserExt<'a> }

impl<'input> TokenAware<'input> for calculatorParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for calculatorParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for calculatorParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "calculator.g4"}

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

impl<'input> calculatorParserContext<'input> for BlockContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for BlockContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_block(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_block(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for BlockContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_block(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_block }
	//fn type_rule_index() -> usize where Self: Sized { RULE_block }
}
antlr_rust::tid!{BlockContextExt<'a>}

impl<'input> BlockContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BlockContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BlockContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BlockContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<BlockContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn functionDefinition_all(&self) ->  Vec<Rc<FunctionDefinitionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn functionDefinition(&self, i: usize) -> Option<Rc<FunctionDefinitionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn equation_all(&self) ->  Vec<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn equation(&self, i: usize) -> Option<Rc<EquationContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn relational_expression_all(&self) ->  Vec<Rc<Relational_expressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn relational_expression(&self, i: usize) -> Option<Rc<Relational_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SEPARATOR in current rule
fn SEPARATOR_all(&self) -> Vec<Rc<TerminalNode<'input,calculatorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEPARATOR, starting from 0.
/// Returns `None` if number of children corresponding to token SEPARATOR is less or equal than `i`.
fn SEPARATOR(&self, i: usize) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(SEPARATOR, i)
}
/// Retrieves all `TerminalNode`s corresponding to token WS in current rule
fn WS_all(&self) -> Vec<Rc<TerminalNode<'input,calculatorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token WS, starting from 0.
/// Returns `None` if number of children corresponding to token WS is less or equal than `i`.
fn WS(&self, i: usize) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(WS, i)
}

}

impl<'input> BlockContextAttrs<'input> for BlockContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
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

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(39);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule functionDefinition*/
					recog.base.set_state(36);
					recog.functionDefinition()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule equation*/
					recog.base.set_state(37);
					recog.equation()?;

					}
				}
			,
				3 =>{
					{
					/*InvokeRule relational_expression*/
					recog.base.set_state(38);
					recog.relational_expression()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(49);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SEPARATOR {
				{
				{
				recog.base.set_state(41);
				recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

				recog.base.set_state(45);
				recog.err_handler.sync(&mut recog.base)?;
				match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
					1 =>{
						{
						/*InvokeRule functionDefinition*/
						recog.base.set_state(42);
						recog.functionDefinition()?;

						}
					}
				,
					2 =>{
						{
						/*InvokeRule equation*/
						recog.base.set_state(43);
						recog.equation()?;

						}
					}
				,
					3 =>{
						{
						/*InvokeRule relational_expression*/
						recog.base.set_state(44);
						recog.relational_expression()?;

						}
					}

					_ => {}
				}
				}
				}
				recog.base.set_state(51);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(55);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==WS {
				{
				{
				recog.base.set_state(52);
				recog.base.match_token(WS,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(57);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(58);
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
//------------------- functionDefinition ----------------
pub type FunctionDefinitionContextAll<'input> = FunctionDefinitionContext<'input>;


pub type FunctionDefinitionContext<'input> = BaseParserRuleContext<'input,FunctionDefinitionContextExt<'input>>;

#[derive(Clone)]
pub struct FunctionDefinitionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for FunctionDefinitionContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for FunctionDefinitionContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_functionDefinition(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_functionDefinition(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for FunctionDefinitionContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_functionDefinition(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionDefinitionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_functionDefinition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_functionDefinition }
}
antlr_rust::tid!{FunctionDefinitionContextExt<'a>}

impl<'input> FunctionDefinitionContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FunctionDefinitionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FunctionDefinitionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FunctionDefinitionContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<FunctionDefinitionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VARIABLE
/// Returns `None` if there is no child corresponding to token VARIABLE
fn VARIABLE(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(VARIABLE, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn signedAtom_all(&self) ->  Vec<Rc<SignedAtomContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn signedAtom(&self, i: usize) -> Option<Rc<SignedAtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token SEPARATOR in current rule
fn SEPARATOR_all(&self) -> Vec<Rc<TerminalNode<'input,calculatorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEPARATOR, starting from 0.
/// Returns `None` if number of children corresponding to token SEPARATOR is less or equal than `i`.
fn SEPARATOR(&self, i: usize) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(SEPARATOR, i)
}

}

impl<'input> FunctionDefinitionContextAttrs<'input> for FunctionDefinitionContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn functionDefinition(&mut self,)
	-> Result<Rc<FunctionDefinitionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FunctionDefinitionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_functionDefinition);
        let mut _localctx: Rc<FunctionDefinitionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(60);
			recog.base.match_token(VARIABLE,&mut recog.err_handler)?;

			recog.base.set_state(61);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule signedAtom*/
			recog.base.set_state(62);
			recog.signedAtom()?;

			recog.base.set_state(67);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==SEPARATOR {
				{
				{
				recog.base.set_state(63);
				recog.base.match_token(SEPARATOR,&mut recog.err_handler)?;

				/*InvokeRule signedAtom*/
				recog.base.set_state(64);
				recog.signedAtom()?;

				}
				}
				recog.base.set_state(69);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(70);
			recog.base.match_token(RPAREN,&mut recog.err_handler)?;

			recog.base.set_state(71);
			recog.base.match_token(EQ,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(72);
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
//------------------- equation ----------------
pub type EquationContextAll<'input> = EquationContext<'input>;


pub type EquationContext<'input> = BaseParserRuleContext<'input,EquationContextExt<'input>>;

#[derive(Clone)]
pub struct EquationContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for EquationContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for EquationContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_equation(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_equation(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for EquationContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_equation(self);
	}
}

impl<'input> CustomRuleContext<'input> for EquationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equation }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equation }
}
antlr_rust::tid!{EquationContextExt<'a>}

impl<'input> EquationContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EquationContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EquationContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EquationContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<EquationContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token EQ
/// Returns `None` if there is no child corresponding to token EQ
fn EQ(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(EQ, 0)
}

}

impl<'input> EquationContextAttrs<'input> for EquationContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn equation(&mut self,)
	-> Result<Rc<EquationContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EquationContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_equation);
        let mut _localctx: Rc<EquationContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(74);
			recog.expression()?;

			recog.base.set_state(75);
			recog.base.match_token(EQ,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(76);
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
//------------------- relational_expression ----------------
pub type Relational_expressionContextAll<'input> = Relational_expressionContext<'input>;


pub type Relational_expressionContext<'input> = BaseParserRuleContext<'input,Relational_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Relational_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for Relational_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for Relational_expressionContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relational_expression(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_relational_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for Relational_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_relational_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Relational_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relational_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relational_expression }
}
antlr_rust::tid!{Relational_expressionContextExt<'a>}

impl<'input> Relational_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Relational_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Relational_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Relational_expressionContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<Relational_expressionContextExt<'input>>{

fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn relop(&self) -> Option<Rc<RelopContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Relational_expressionContextAttrs<'input> for Relational_expressionContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relational_expression(&mut self,)
	-> Result<Rc<Relational_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Relational_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_relational_expression);
        let mut _localctx: Rc<Relational_expressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule expression*/
			recog.base.set_state(78);
			recog.expression()?;

			/*InvokeRule relop*/
			recog.base.set_state(79);
			recog.relop()?;

			/*InvokeRule expression*/
			recog.base.set_state(80);
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
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for ExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_expression(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_expression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn multiplyingExpression_all(&self) ->  Vec<Rc<MultiplyingExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn multiplyingExpression(&self, i: usize) -> Option<Rc<MultiplyingExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token PLUS in current rule
fn PLUS_all(&self) -> Vec<Rc<TerminalNode<'input,calculatorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token PLUS, starting from 0.
/// Returns `None` if number of children corresponding to token PLUS is less or equal than `i`.
fn PLUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(PLUS, i)
}
/// Retrieves all `TerminalNode`s corresponding to token MINUS in current rule
fn MINUS_all(&self) -> Vec<Rc<TerminalNode<'input,calculatorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token MINUS, starting from 0.
/// Returns `None` if number of children corresponding to token MINUS is less or equal than `i`.
fn MINUS(&self, i: usize) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(MINUS, i)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_expression);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule multiplyingExpression*/
			recog.base.set_state(82);
			recog.multiplyingExpression()?;

			recog.base.set_state(87);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==PLUS || _la==MINUS {
				{
				{
				recog.base.set_state(83);
				_la = recog.base.input.la(1);
				if { !(_la==PLUS || _la==MINUS) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule multiplyingExpression*/
				recog.base.set_state(84);
				recog.multiplyingExpression()?;

				}
				}
				recog.base.set_state(89);
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
//------------------- multiplyingExpression ----------------
pub type MultiplyingExpressionContextAll<'input> = MultiplyingExpressionContext<'input>;


pub type MultiplyingExpressionContext<'input> = BaseParserRuleContext<'input,MultiplyingExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct MultiplyingExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for MultiplyingExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for MultiplyingExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_multiplyingExpression(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_multiplyingExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for MultiplyingExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_multiplyingExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultiplyingExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplyingExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplyingExpression }
}
antlr_rust::tid!{MultiplyingExpressionContextExt<'a>}

impl<'input> MultiplyingExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultiplyingExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultiplyingExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultiplyingExpressionContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<MultiplyingExpressionContextExt<'input>>{

fn powExpression_all(&self) ->  Vec<Rc<PowExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn powExpression(&self, i: usize) -> Option<Rc<PowExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token TIMES in current rule
fn TIMES_all(&self) -> Vec<Rc<TerminalNode<'input,calculatorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token TIMES, starting from 0.
/// Returns `None` if number of children corresponding to token TIMES is less or equal than `i`.
fn TIMES(&self, i: usize) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(TIMES, i)
}
/// Retrieves all `TerminalNode`s corresponding to token DIV in current rule
fn DIV_all(&self) -> Vec<Rc<TerminalNode<'input,calculatorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token DIV, starting from 0.
/// Returns `None` if number of children corresponding to token DIV is less or equal than `i`.
fn DIV(&self, i: usize) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(DIV, i)
}

}

impl<'input> MultiplyingExpressionContextAttrs<'input> for MultiplyingExpressionContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multiplyingExpression(&mut self,)
	-> Result<Rc<MultiplyingExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultiplyingExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_multiplyingExpression);
        let mut _localctx: Rc<MultiplyingExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule powExpression*/
			recog.base.set_state(90);
			recog.powExpression()?;

			recog.base.set_state(95);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==TIMES || _la==DIV {
				{
				{
				recog.base.set_state(91);
				_la = recog.base.input.la(1);
				if { !(_la==TIMES || _la==DIV) } {
					recog.err_handler.recover_inline(&mut recog.base)?;

				}
				else {
					if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
					recog.err_handler.report_match(&mut recog.base);
					recog.base.consume(&mut recog.err_handler);
				}
				/*InvokeRule powExpression*/
				recog.base.set_state(92);
				recog.powExpression()?;

				}
				}
				recog.base.set_state(97);
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
//------------------- powExpression ----------------
pub type PowExpressionContextAll<'input> = PowExpressionContext<'input>;


pub type PowExpressionContext<'input> = BaseParserRuleContext<'input,PowExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct PowExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for PowExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for PowExpressionContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_powExpression(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_powExpression(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for PowExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_powExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for PowExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_powExpression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_powExpression }
}
antlr_rust::tid!{PowExpressionContextExt<'a>}

impl<'input> PowExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PowExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PowExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PowExpressionContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<PowExpressionContextExt<'input>>{

fn signedAtom_all(&self) ->  Vec<Rc<SignedAtomContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn signedAtom(&self, i: usize) -> Option<Rc<SignedAtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token POW in current rule
fn POW_all(&self) -> Vec<Rc<TerminalNode<'input,calculatorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token POW, starting from 0.
/// Returns `None` if number of children corresponding to token POW is less or equal than `i`.
fn POW(&self, i: usize) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(POW, i)
}

}

impl<'input> PowExpressionContextAttrs<'input> for PowExpressionContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn powExpression(&mut self,)
	-> Result<Rc<PowExpressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PowExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_powExpression);
        let mut _localctx: Rc<PowExpressionContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule signedAtom*/
			recog.base.set_state(98);
			recog.signedAtom()?;

			recog.base.set_state(103);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==POW {
				{
				{
				recog.base.set_state(99);
				recog.base.match_token(POW,&mut recog.err_handler)?;

				/*InvokeRule signedAtom*/
				recog.base.set_state(100);
				recog.signedAtom()?;

				}
				}
				recog.base.set_state(105);
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
//------------------- signedAtom ----------------
pub type SignedAtomContextAll<'input> = SignedAtomContext<'input>;


pub type SignedAtomContext<'input> = BaseParserRuleContext<'input,SignedAtomContextExt<'input>>;

#[derive(Clone)]
pub struct SignedAtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for SignedAtomContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for SignedAtomContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_signedAtom(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_signedAtom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for SignedAtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_signedAtom(self);
	}
}

impl<'input> CustomRuleContext<'input> for SignedAtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_signedAtom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_signedAtom }
}
antlr_rust::tid!{SignedAtomContextExt<'a>}

impl<'input> SignedAtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SignedAtomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SignedAtomContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SignedAtomContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<SignedAtomContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
fn signedAtom(&self) -> Option<Rc<SignedAtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}
fn func_(&self) -> Option<Rc<Func_ContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn atom(&self) -> Option<Rc<AtomContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> SignedAtomContextAttrs<'input> for SignedAtomContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn signedAtom(&mut self,)
	-> Result<Rc<SignedAtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SignedAtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_signedAtom);
        let mut _localctx: Rc<SignedAtomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(112);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(8,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(106);
					recog.base.match_token(PLUS,&mut recog.err_handler)?;

					/*InvokeRule signedAtom*/
					recog.base.set_state(107);
					recog.signedAtom()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(108);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					/*InvokeRule signedAtom*/
					recog.base.set_state(109);
					recog.signedAtom()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule func_*/
					recog.base.set_state(110);
					recog.func_()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule atom*/
					recog.base.set_state(111);
					recog.atom()?;

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
//------------------- atom ----------------
pub type AtomContextAll<'input> = AtomContext<'input>;


pub type AtomContext<'input> = BaseParserRuleContext<'input,AtomContextExt<'input>>;

#[derive(Clone)]
pub struct AtomContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for AtomContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for AtomContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_atom(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_atom(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for AtomContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_atom(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atom }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atom }
}
antlr_rust::tid!{AtomContextExt<'a>}

impl<'input> AtomContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AtomContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AtomContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<AtomContextExt<'input>>{

fn scientific(&self) -> Option<Rc<ScientificContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn currency(&self) -> Option<Rc<CurrencyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}

}

impl<'input> AtomContextAttrs<'input> for AtomContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atom(&mut self,)
	-> Result<Rc<AtomContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_atom);
        let mut _localctx: Rc<AtomContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(123);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EOF | RPAREN | PLUS | MINUS | TIMES | DIV | GT | LT | EQ | DOUBLE_EQ |
			 NE | SEPARATOR | COMMA | POW | WS 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					}
				}

			 SCIENTIFIC_NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule scientific*/
					recog.base.set_state(115);
					recog.scientific()?;

					}
				}

			 VARIABLE 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule variable*/
					recog.base.set_state(116);
					recog.variable()?;

					}
				}

			 PI | EULER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule constant*/
					recog.base.set_state(117);
					recog.constant()?;

					}
				}

			 CURRENCY_NUMBER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule currency*/
					recog.base.set_state(118);
					recog.currency()?;

					}
				}

			 LPAREN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(119);
					recog.base.match_token(LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(120);
					recog.expression()?;

					recog.base.set_state(121);
					recog.base.match_token(RPAREN,&mut recog.err_handler)?;

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
//------------------- scientific ----------------
pub type ScientificContextAll<'input> = ScientificContext<'input>;


pub type ScientificContext<'input> = BaseParserRuleContext<'input,ScientificContextExt<'input>>;

#[derive(Clone)]
pub struct ScientificContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for ScientificContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for ScientificContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_scientific(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_scientific(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for ScientificContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_scientific(self);
	}
}

impl<'input> CustomRuleContext<'input> for ScientificContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_scientific }
	//fn type_rule_index() -> usize where Self: Sized { RULE_scientific }
}
antlr_rust::tid!{ScientificContextExt<'a>}

impl<'input> ScientificContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ScientificContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ScientificContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ScientificContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<ScientificContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SCIENTIFIC_NUMBER
/// Returns `None` if there is no child corresponding to token SCIENTIFIC_NUMBER
fn SCIENTIFIC_NUMBER(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(SCIENTIFIC_NUMBER, 0)
}

}

impl<'input> ScientificContextAttrs<'input> for ScientificContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn scientific(&mut self,)
	-> Result<Rc<ScientificContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ScientificContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_scientific);
        let mut _localctx: Rc<ScientificContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(125);
			recog.base.match_token(SCIENTIFIC_NUMBER,&mut recog.err_handler)?;

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
//------------------- currency ----------------
pub type CurrencyContextAll<'input> = CurrencyContext<'input>;


pub type CurrencyContext<'input> = BaseParserRuleContext<'input,CurrencyContextExt<'input>>;

#[derive(Clone)]
pub struct CurrencyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for CurrencyContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for CurrencyContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_currency(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_currency(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for CurrencyContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_currency(self);
	}
}

impl<'input> CustomRuleContext<'input> for CurrencyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_currency }
	//fn type_rule_index() -> usize where Self: Sized { RULE_currency }
}
antlr_rust::tid!{CurrencyContextExt<'a>}

impl<'input> CurrencyContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CurrencyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CurrencyContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait CurrencyContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<CurrencyContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CURRENCY_NUMBER
/// Returns `None` if there is no child corresponding to token CURRENCY_NUMBER
fn CURRENCY_NUMBER(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(CURRENCY_NUMBER, 0)
}

}

impl<'input> CurrencyContextAttrs<'input> for CurrencyContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn currency(&mut self,)
	-> Result<Rc<CurrencyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CurrencyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_currency);
        let mut _localctx: Rc<CurrencyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(127);
			recog.base.match_token(CURRENCY_NUMBER,&mut recog.err_handler)?;

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
//------------------- constant ----------------
pub type ConstantContextAll<'input> = ConstantContext<'input>;


pub type ConstantContext<'input> = BaseParserRuleContext<'input,ConstantContextExt<'input>>;

#[derive(Clone)]
pub struct ConstantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for ConstantContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for ConstantContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_constant(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_constant(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for ConstantContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_constant(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}
antlr_rust::tid!{ConstantContextExt<'a>}

impl<'input> ConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstantContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ConstantContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<ConstantContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PI
/// Returns `None` if there is no child corresponding to token PI
fn PI(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(PI, 0)
}
/// Retrieves first TerminalNode corresponding to token EULER
/// Returns `None` if there is no child corresponding to token EULER
fn EULER(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(EULER, 0)
}

}

impl<'input> ConstantContextAttrs<'input> for ConstantContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constant(&mut self,)
	-> Result<Rc<ConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_constant);
        let mut _localctx: Rc<ConstantContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(129);
			_la = recog.base.input.la(1);
			if { !(_la==PI || _la==EULER) } {
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
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;


pub type VariableContext<'input> = BaseParserRuleContext<'input,VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for VariableContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_variable(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_variable(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr_rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait VariableContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<VariableContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VARIABLE
/// Returns `None` if there is no child corresponding to token VARIABLE
fn VARIABLE(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(VARIABLE, 0)
}

}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn variable(&mut self,)
	-> Result<Rc<VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(131);
			recog.base.match_token(VARIABLE,&mut recog.err_handler)?;

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
//------------------- func_ ----------------
pub type Func_ContextAll<'input> = Func_Context<'input>;


pub type Func_Context<'input> = BaseParserRuleContext<'input,Func_ContextExt<'input>>;

#[derive(Clone)]
pub struct Func_ContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for Func_Context<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for Func_Context<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_func_(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_func_(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for Func_Context<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_func_(self);
	}
}

impl<'input> CustomRuleContext<'input> for Func_ContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_ }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_ }
}
antlr_rust::tid!{Func_ContextExt<'a>}

impl<'input> Func_ContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Func_ContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_ContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Func_ContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<Func_ContextExt<'input>>{

fn funcname(&self) -> Option<Rc<FuncnameContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(LPAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(RPAREN, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,calculatorParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> Func_ContextAttrs<'input> for Func_Context<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn func_(&mut self,)
	-> Result<Rc<Func_ContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_ContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_func_);
        let mut _localctx: Rc<Func_ContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule funcname*/
			recog.base.set_state(133);
			recog.funcname()?;

			recog.base.set_state(134);
			recog.base.match_token(LPAREN,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(135);
			recog.expression()?;

			recog.base.set_state(140);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==COMMA {
				{
				{
				recog.base.set_state(136);
				recog.base.match_token(COMMA,&mut recog.err_handler)?;

				/*InvokeRule expression*/
				recog.base.set_state(137);
				recog.expression()?;

				}
				}
				recog.base.set_state(142);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(143);
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
//------------------- funcname ----------------
pub type FuncnameContextAll<'input> = FuncnameContext<'input>;


pub type FuncnameContext<'input> = BaseParserRuleContext<'input,FuncnameContextExt<'input>>;

#[derive(Clone)]
pub struct FuncnameContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for FuncnameContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for FuncnameContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_funcname(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_funcname(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for FuncnameContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_funcname(self);
	}
}

impl<'input> CustomRuleContext<'input> for FuncnameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcname }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcname }
}
antlr_rust::tid!{FuncnameContextExt<'a>}

impl<'input> FuncnameContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FuncnameContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncnameContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FuncnameContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<FuncnameContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COS
/// Returns `None` if there is no child corresponding to token COS
fn COS(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(COS, 0)
}
/// Retrieves first TerminalNode corresponding to token TAN
/// Returns `None` if there is no child corresponding to token TAN
fn TAN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(TAN, 0)
}
/// Retrieves first TerminalNode corresponding to token SIN
/// Returns `None` if there is no child corresponding to token SIN
fn SIN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(SIN, 0)
}
/// Retrieves first TerminalNode corresponding to token ACOS
/// Returns `None` if there is no child corresponding to token ACOS
fn ACOS(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(ACOS, 0)
}
/// Retrieves first TerminalNode corresponding to token ATAN
/// Returns `None` if there is no child corresponding to token ATAN
fn ATAN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(ATAN, 0)
}
/// Retrieves first TerminalNode corresponding to token ASIN
/// Returns `None` if there is no child corresponding to token ASIN
fn ASIN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(ASIN, 0)
}
/// Retrieves first TerminalNode corresponding to token LOG
/// Returns `None` if there is no child corresponding to token LOG
fn LOG(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(LOG, 0)
}
/// Retrieves first TerminalNode corresponding to token LN
/// Returns `None` if there is no child corresponding to token LN
fn LN(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(LN, 0)
}
/// Retrieves first TerminalNode corresponding to token EXP
/// Returns `None` if there is no child corresponding to token EXP
fn EXP(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(EXP, 0)
}
/// Retrieves first TerminalNode corresponding to token SQRT
/// Returns `None` if there is no child corresponding to token SQRT
fn SQRT(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(SQRT, 0)
}

}

impl<'input> FuncnameContextAttrs<'input> for FuncnameContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn funcname(&mut self,)
	-> Result<Rc<FuncnameContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncnameContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_funcname);
        let mut _localctx: Rc<FuncnameContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(145);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << COS) | (1usize << SIN) | (1usize << TAN) | (1usize << ACOS) | (1usize << ASIN) | (1usize << ATAN) | (1usize << LN) | (1usize << LOG) | (1usize << EXP) | (1usize << SQRT))) != 0)) } {
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

impl<'input> calculatorParserContext<'input> for RelopContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for RelopContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_relop(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_relop(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for RelopContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_relop(self);
	}
}

impl<'input> CustomRuleContext<'input> for RelopContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relop }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relop }
}
antlr_rust::tid!{RelopContextExt<'a>}

impl<'input> RelopContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RelopContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RelopContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RelopContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<RelopContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GT
/// Returns `None` if there is no child corresponding to token GT
fn GT(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(GT, 0)
}
/// Retrieves first TerminalNode corresponding to token LT
/// Returns `None` if there is no child corresponding to token LT
fn LT(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(LT, 0)
}
/// Retrieves first TerminalNode corresponding to token NE
/// Returns `None` if there is no child corresponding to token NE
fn NE(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(NE, 0)
}
/// Retrieves first TerminalNode corresponding to token DOUBLE_EQ
/// Returns `None` if there is no child corresponding to token DOUBLE_EQ
fn DOUBLE_EQ(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(DOUBLE_EQ, 0)
}

}

impl<'input> RelopContextAttrs<'input> for RelopContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn relop(&mut self,)
	-> Result<Rc<RelopContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RelopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_relop);
        let mut _localctx: Rc<RelopContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(147);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << GT) | (1usize << LT) | (1usize << DOUBLE_EQ) | (1usize << NE))) != 0)) } {
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
//------------------- sumop ----------------
pub type SumopContextAll<'input> = SumopContext<'input>;


pub type SumopContext<'input> = BaseParserRuleContext<'input,SumopContextExt<'input>>;

#[derive(Clone)]
pub struct SumopContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> calculatorParserContext<'input> for SumopContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for SumopContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_sumop(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_sumop(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for SumopContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_sumop(self);
	}
}

impl<'input> CustomRuleContext<'input> for SumopContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_sumop }
	//fn type_rule_index() -> usize where Self: Sized { RULE_sumop }
}
antlr_rust::tid!{SumopContextExt<'a>}

impl<'input> SumopContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<SumopContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,SumopContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait SumopContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<SumopContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PLUS
/// Returns `None` if there is no child corresponding to token PLUS
fn PLUS(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(PLUS, 0)
}
/// Retrieves first TerminalNode corresponding to token MINUS
/// Returns `None` if there is no child corresponding to token MINUS
fn MINUS(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(MINUS, 0)
}

}

impl<'input> SumopContextAttrs<'input> for SumopContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn sumop(&mut self,)
	-> Result<Rc<SumopContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = SumopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_sumop);
        let mut _localctx: Rc<SumopContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(149);
			_la = recog.base.input.la(1);
			if { !(_la==PLUS || _la==MINUS) } {
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

impl<'input> calculatorParserContext<'input> for MultopContext<'input>{}

impl<'input,'a> Listenable<dyn calculatorListener<'input> + 'a> for MultopContext<'input>{
		fn enter(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_multop(self);
		}
		fn exit(&self,listener: &mut (dyn calculatorListener<'input> + 'a)) {
			listener.exit_multop(self);
			listener.exit_every_rule(self);
		}
}

impl<'input,'a> Visitable<dyn calculatorVisitor<'input> + 'a> for MultopContext<'input>{
	fn accept(&self,visitor: &mut (dyn calculatorVisitor<'input> + 'a)) {
		visitor.visit_multop(self);
	}
}

impl<'input> CustomRuleContext<'input> for MultopContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = calculatorParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multop }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multop }
}
antlr_rust::tid!{MultopContextExt<'a>}

impl<'input> MultopContextExt<'input>{
	fn new(parent: Option<Rc<dyn calculatorParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MultopContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MultopContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait MultopContextAttrs<'input>: calculatorParserContext<'input> + BorrowMut<MultopContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token TIMES
/// Returns `None` if there is no child corresponding to token TIMES
fn TIMES(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(TIMES, 0)
}
/// Retrieves first TerminalNode corresponding to token DIV
/// Returns `None` if there is no child corresponding to token DIV
fn DIV(&self) -> Option<Rc<TerminalNode<'input,calculatorParserContextType>>> where Self:Sized{
	self.get_token(DIV, 0)
}

}

impl<'input> MultopContextAttrs<'input> for MultopContext<'input>{}

impl<'input, I, H> calculatorParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn multop(&mut self,)
	-> Result<Rc<MultopContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MultopContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_multop);
        let mut _localctx: Rc<MultopContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(151);
			_la = recog.base.input.la(1);
			if { !(_la==TIMES || _la==DIV) } {
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
	\x21\u{9c}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\x05\
	\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\x0a\
	\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\x0e\
	\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\x13\
	\x09\x13\x03\x02\x03\x02\x03\x02\x05\x02\x2a\x0a\x02\x03\x02\x03\x02\x03\
	\x02\x03\x02\x05\x02\x30\x0a\x02\x07\x02\x32\x0a\x02\x0c\x02\x0e\x02\x35\
	\x0b\x02\x03\x02\x07\x02\x38\x0a\x02\x0c\x02\x0e\x02\x3b\x0b\x02\x03\x02\
	\x03\x02\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x07\x03\x44\x0a\x03\x0c\
	\x03\x0e\x03\x47\x0b\x03\x03\x03\x03\x03\x03\x03\x03\x03\x03\x04\x03\x04\
	\x03\x04\x03\x04\x03\x05\x03\x05\x03\x05\x03\x05\x03\x06\x03\x06\x03\x06\
	\x07\x06\x58\x0a\x06\x0c\x06\x0e\x06\x5b\x0b\x06\x03\x07\x03\x07\x03\x07\
	\x07\x07\x60\x0a\x07\x0c\x07\x0e\x07\x63\x0b\x07\x03\x08\x03\x08\x03\x08\
	\x07\x08\x68\x0a\x08\x0c\x08\x0e\x08\x6b\x0b\x08\x03\x09\x03\x09\x03\x09\
	\x03\x09\x03\x09\x03\x09\x05\x09\x73\x0a\x09\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x05\x0a\x7e\x0a\x0a\x03\x0b\
	\x03\x0b\x03\x0c\x03\x0c\x03\x0d\x03\x0d\x03\x0e\x03\x0e\x03\x0f\x03\x0f\
	\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{8d}\x0a\x0f\x0c\x0f\x0e\x0f\u{90}\x0b\
	\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x03\x11\x03\x11\x03\x12\x03\x12\x03\
	\x13\x03\x13\x03\x13\x02\x02\x14\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\
	\x16\x18\x1a\x1c\x1e\x20\x22\x24\x02\x07\x03\x02\x0f\x10\x03\x02\x11\x12\
	\x03\x02\x1c\x1d\x03\x02\x03\x0c\x04\x02\x13\x14\x16\x17\x02\u{9c}\x02\x29\
	\x03\x02\x02\x02\x04\x3e\x03\x02\x02\x02\x06\x4c\x03\x02\x02\x02\x08\x50\
	\x03\x02\x02\x02\x0a\x54\x03\x02\x02\x02\x0c\x5c\x03\x02\x02\x02\x0e\x64\
	\x03\x02\x02\x02\x10\x72\x03\x02\x02\x02\x12\x7d\x03\x02\x02\x02\x14\x7f\
	\x03\x02\x02\x02\x16\u{81}\x03\x02\x02\x02\x18\u{83}\x03\x02\x02\x02\x1a\
	\u{85}\x03\x02\x02\x02\x1c\u{87}\x03\x02\x02\x02\x1e\u{93}\x03\x02\x02\x02\
	\x20\u{95}\x03\x02\x02\x02\x22\u{97}\x03\x02\x02\x02\x24\u{99}\x03\x02\x02\
	\x02\x26\x2a\x05\x04\x03\x02\x27\x2a\x05\x06\x04\x02\x28\x2a\x05\x08\x05\
	\x02\x29\x26\x03\x02\x02\x02\x29\x27\x03\x02\x02\x02\x29\x28\x03\x02\x02\
	\x02\x2a\x33\x03\x02\x02\x02\x2b\x2f\x07\x18\x02\x02\x2c\x30\x05\x04\x03\
	\x02\x2d\x30\x05\x06\x04\x02\x2e\x30\x05\x08\x05\x02\x2f\x2c\x03\x02\x02\
	\x02\x2f\x2d\x03\x02\x02\x02\x2f\x2e\x03\x02\x02\x02\x30\x32\x03\x02\x02\
	\x02\x31\x2b\x03\x02\x02\x02\x32\x35\x03\x02\x02\x02\x33\x31\x03\x02\x02\
	\x02\x33\x34\x03\x02\x02\x02\x34\x39\x03\x02\x02\x02\x35\x33\x03\x02\x02\
	\x02\x36\x38\x07\x21\x02\x02\x37\x36\x03\x02\x02\x02\x38\x3b\x03\x02\x02\
	\x02\x39\x37\x03\x02\x02\x02\x39\x3a\x03\x02\x02\x02\x3a\x3c\x03\x02\x02\
	\x02\x3b\x39\x03\x02\x02\x02\x3c\x3d\x07\x02\x02\x03\x3d\x03\x03\x02\x02\
	\x02\x3e\x3f\x07\x1e\x02\x02\x3f\x40\x07\x0d\x02\x02\x40\x45\x05\x10\x09\
	\x02\x41\x42\x07\x18\x02\x02\x42\x44\x05\x10\x09\x02\x43\x41\x03\x02\x02\
	\x02\x44\x47\x03\x02\x02\x02\x45\x43\x03\x02\x02\x02\x45\x46\x03\x02\x02\
	\x02\x46\x48\x03\x02\x02\x02\x47\x45\x03\x02\x02\x02\x48\x49\x07\x0e\x02\
	\x02\x49\x4a\x07\x15\x02\x02\x4a\x4b\x05\x0a\x06\x02\x4b\x05\x03\x02\x02\
	\x02\x4c\x4d\x05\x0a\x06\x02\x4d\x4e\x07\x15\x02\x02\x4e\x4f\x05\x0a\x06\
	\x02\x4f\x07\x03\x02\x02\x02\x50\x51\x05\x0a\x06\x02\x51\x52\x05\x20\x11\
	\x02\x52\x53\x05\x0a\x06\x02\x53\x09\x03\x02\x02\x02\x54\x59\x05\x0c\x07\
	\x02\x55\x56\x09\x02\x02\x02\x56\x58\x05\x0c\x07\x02\x57\x55\x03\x02\x02\
	\x02\x58\x5b\x03\x02\x02\x02\x59\x57\x03\x02\x02\x02\x59\x5a\x03\x02\x02\
	\x02\x5a\x0b\x03\x02\x02\x02\x5b\x59\x03\x02\x02\x02\x5c\x61\x05\x0e\x08\
	\x02\x5d\x5e\x09\x03\x02\x02\x5e\x60\x05\x0e\x08\x02\x5f\x5d\x03\x02\x02\
	\x02\x60\x63\x03\x02\x02\x02\x61\x5f\x03\x02\x02\x02\x61\x62\x03\x02\x02\
	\x02\x62\x0d\x03\x02\x02\x02\x63\x61\x03\x02\x02\x02\x64\x69\x05\x10\x09\
	\x02\x65\x66\x07\x1b\x02\x02\x66\x68\x05\x10\x09\x02\x67\x65\x03\x02\x02\
	\x02\x68\x6b\x03\x02\x02\x02\x69\x67\x03\x02\x02\x02\x69\x6a\x03\x02\x02\
	\x02\x6a\x0f\x03\x02\x02\x02\x6b\x69\x03\x02\x02\x02\x6c\x6d\x07\x0f\x02\
	\x02\x6d\x73\x05\x10\x09\x02\x6e\x6f\x07\x10\x02\x02\x6f\x73\x05\x10\x09\
	\x02\x70\x73\x05\x1c\x0f\x02\x71\x73\x05\x12\x0a\x02\x72\x6c\x03\x02\x02\
	\x02\x72\x6e\x03\x02\x02\x02\x72\x70\x03\x02\x02\x02\x72\x71\x03\x02\x02\
	\x02\x73\x11\x03\x02\x02\x02\x74\x7e\x03\x02\x02\x02\x75\x7e\x05\x14\x0b\
	\x02\x76\x7e\x05\x1a\x0e\x02\x77\x7e\x05\x18\x0d\x02\x78\x7e\x05\x16\x0c\
	\x02\x79\x7a\x07\x0d\x02\x02\x7a\x7b\x05\x0a\x06\x02\x7b\x7c\x07\x0e\x02\
	\x02\x7c\x7e\x03\x02\x02\x02\x7d\x74\x03\x02\x02\x02\x7d\x75\x03\x02\x02\
	\x02\x7d\x76\x03\x02\x02\x02\x7d\x77\x03\x02\x02\x02\x7d\x78\x03\x02\x02\
	\x02\x7d\x79\x03\x02\x02\x02\x7e\x13\x03\x02\x02\x02\x7f\u{80}\x07\x1f\x02\
	\x02\u{80}\x15\x03\x02\x02\x02\u{81}\u{82}\x07\x20\x02\x02\u{82}\x17\x03\
	\x02\x02\x02\u{83}\u{84}\x09\x04\x02\x02\u{84}\x19\x03\x02\x02\x02\u{85}\
	\u{86}\x07\x1e\x02\x02\u{86}\x1b\x03\x02\x02\x02\u{87}\u{88}\x05\x1e\x10\
	\x02\u{88}\u{89}\x07\x0d\x02\x02\u{89}\u{8e}\x05\x0a\x06\x02\u{8a}\u{8b}\
	\x07\x19\x02\x02\u{8b}\u{8d}\x05\x0a\x06\x02\u{8c}\u{8a}\x03\x02\x02\x02\
	\u{8d}\u{90}\x03\x02\x02\x02\u{8e}\u{8c}\x03\x02\x02\x02\u{8e}\u{8f}\x03\
	\x02\x02\x02\u{8f}\u{91}\x03\x02\x02\x02\u{90}\u{8e}\x03\x02\x02\x02\u{91}\
	\u{92}\x07\x0e\x02\x02\u{92}\x1d\x03\x02\x02\x02\u{93}\u{94}\x09\x05\x02\
	\x02\u{94}\x1f\x03\x02\x02\x02\u{95}\u{96}\x09\x06\x02\x02\u{96}\x21\x03\
	\x02\x02\x02\u{97}\u{98}\x09\x02\x02\x02\u{98}\x23\x03\x02\x02\x02\u{99}\
	\u{9a}\x09\x03\x02\x02\u{9a}\x25\x03\x02\x02\x02\x0d\x29\x2f\x33\x39\x45\
	\x59\x61\x69\x72\x7d\u{8e}";

