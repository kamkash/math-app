// Generated from ArrayInit.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
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
use super::arrayinitlistener::*;
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
		pub const T__2:isize=3; 
		pub const INT:isize=4; 
		pub const WS:isize=5;
	pub const RULE_init:usize = 0; 
	pub const RULE_value:usize = 1;
	pub const ruleNames: [&'static str; 2] =  [
		"init", "value"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;4] = [
		None, Some("'{'"), Some("','"), Some("'}'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;6]  = [
		None, None, None, None, Some("INT"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,ArrayInitParserExt, I, ArrayInitParserContextType , dyn ArrayInitListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type ArrayInitTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, ArrayInitParserContextType , dyn ArrayInitListener<'input> + 'a>;

/// Parser for ArrayInit grammar
pub struct ArrayInitParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> ArrayInitParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				ArrayInitParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> ArrayInitParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> ArrayInitParser<'input, I, DefaultErrorStrategy<'input,ArrayInitParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for ArrayInitParser
pub trait ArrayInitParserContext<'input>:
	for<'x> Listenable<dyn ArrayInitListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=ArrayInitParserContextType>
{}

impl<'input> ArrayInitParserContext<'input> for TerminalNode<'input,ArrayInitParserContextType> {}
impl<'input> ArrayInitParserContext<'input> for ErrorNode<'input,ArrayInitParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn ArrayInitParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn ArrayInitListener<'input> + 'input{}

pub struct ArrayInitParserContextType;
antlr_rust::type_id!{ArrayInitParserContextType}

impl<'input> ParserNodeType<'input> for ArrayInitParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn ArrayInitParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for ArrayInitParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for ArrayInitParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct ArrayInitParserExt{
}

impl ArrayInitParserExt{
}


impl<'input> TokenAware<'input> for ArrayInitParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for ArrayInitParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for ArrayInitParserExt{
	fn get_grammar_file_name(&self) -> & str{ "ArrayInit.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
}
//------------------- init ----------------
pub type InitContextAll<'input> = InitContext<'input>;


pub type InitContext<'input> = BaseParserRuleContext<'input,InitContextExt<'input>>;

#[derive(Clone)]
pub struct InitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ArrayInitParserContext<'input> for InitContext<'input>{}

impl<'input,'a> Listenable<dyn ArrayInitListener<'input> + 'a> for InitContext<'input>{
	fn enter(&self,listener: &mut (dyn ArrayInitListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_init(self);
	}
}

impl<'input> CustomRuleContext<'input> for InitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ArrayInitParserContextType;
	fn get_rule_index(&self) -> usize { RULE_init }
	//fn type_rule_index() -> usize where Self: Sized { RULE_init }
}
antlr_rust::type_id!{InitContextExt<'a>}

impl<'input> InitContextExt<'input>{
	fn new(parent: Option<Rc<dyn ArrayInitParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<InitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,InitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait InitContextAttrs<'input>: ArrayInitParserContext<'input> + BorrowMut<InitContextExt<'input>>{

fn value_all(&self) ->  Vec<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn value(&self, i: usize) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> InitContextAttrs<'input> for InitContext<'input>{}

impl<'input, I, H> ArrayInitParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn init(&mut self,)
	-> Result<Rc<InitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = InitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_init);
        let mut _localctx: Rc<InitContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(4);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			/*InvokeRule value*/
			recog.base.set_state(5);
			recog.value()?;

			recog.base.set_state(10);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==T__1 {
				{
				{
				recog.base.set_state(6);
				recog.base.match_token(T__1,&mut recog.err_handler)?;

				/*InvokeRule value*/
				recog.base.set_state(7);
				recog.value()?;

				}
				}
				recog.base.set_state(12);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(13);
			recog.base.match_token(T__2,&mut recog.err_handler)?;

			}
		};
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
//------------------- value ----------------
pub type ValueContextAll<'input> = ValueContext<'input>;


pub type ValueContext<'input> = BaseParserRuleContext<'input,ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> ArrayInitParserContext<'input> for ValueContext<'input>{}

impl<'input,'a> Listenable<dyn ArrayInitListener<'input> + 'a> for ValueContext<'input>{
	fn enter(&self,listener: &mut (dyn ArrayInitListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_value(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = ArrayInitParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::type_id!{ValueContextExt<'a>}

impl<'input> ValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn ArrayInitParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ValueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ValueContextAttrs<'input>: ArrayInitParserContext<'input> + BorrowMut<ValueContextExt<'input>>{

fn init(&self) -> Option<Rc<InitContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INT
/// Returns `None` if there is no child corresponding to token INT
fn INT(&self) -> Option<Rc<TerminalNode<'input,ArrayInitParserContextType>>> where Self:Sized{
	self.get_token(INT, 0)
}

}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input>{}

impl<'input, I, H> ArrayInitParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn value(&mut self,)
	-> Result<Rc<ValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(17);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__0 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule init*/
					recog.base.set_state(15);
					recog.init()?;

					}
				}

			 INT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(16);
					recog.base.match_token(INT,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
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
	\x07\x16\x04\x02\x09\x02\x04\x03\x09\x03\x03\x02\x03\x02\x03\x02\x03\x02\
	\x07\x02\x0b\x0a\x02\x0c\x02\x0e\x02\x0e\x0b\x02\x03\x02\x03\x02\x03\x03\
	\x03\x03\x05\x03\x14\x0a\x03\x03\x03\x02\x02\x04\x02\x04\x02\x02\x02\x15\
	\x02\x06\x03\x02\x02\x02\x04\x13\x03\x02\x02\x02\x06\x07\x07\x03\x02\x02\
	\x07\x0c\x05\x04\x03\x02\x08\x09\x07\x04\x02\x02\x09\x0b\x05\x04\x03\x02\
	\x0a\x08\x03\x02\x02\x02\x0b\x0e\x03\x02\x02\x02\x0c\x0a\x03\x02\x02\x02\
	\x0c\x0d\x03\x02\x02\x02\x0d\x0f\x03\x02\x02\x02\x0e\x0c\x03\x02\x02\x02\
	\x0f\x10\x07\x05\x02\x02\x10\x03\x03\x02\x02\x02\x11\x14\x05\x02\x02\x02\
	\x12\x14\x07\x06\x02\x02\x13\x11\x03\x02\x02\x02\x13\x12\x03\x02\x02\x02\
	\x14\x05\x03\x02\x02\x02\x04\x0c\x13";

