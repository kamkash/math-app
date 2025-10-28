/*
 ANTLR4 LaTeX Math Grammar

 Ported from latex2sympy by @augustt198 https://github.com/augustt198/latex2sympy See license in
 LICENSE.txt
 */

/*
 After changing this file, it is necessary to run `python setup.py antlr` in the root directory of
 the repository. This will regenerate the code in `sympy/parsing/latex/_antlr/*.py`.
 */


// ANTLR4 LaTeX Math Grammar
// Enhanced: Added 'block' rule for parsing multiple expressions, clarified precedence hierarchy.
grammar LaTeX;

// --- Top-level block rule for multiple expressions ---
block
	: (relation | expr) (SEPARATOR (relation | expr))* SEPARATOR* EOF
	| LATEX_BLOCK L_BRACE (relation | expr) (LATEX_NEWLINE (relation | expr))* LATEX_NEWLINE* R_BRACE LATEX_NEWLINE* SEPARATOR* EOF
	;


// @formatter:off
// astyle:off
// --- Operator precedence hierarchy (highest at bottom) ---
// block
//   └─ relation (==, !=, <, >, <=, >=)
//        └─ additive (+, -)
//             └─ mp (*, /, etc.)
//                  └─ unary (+, -)
//                       └─ postfix (!, eval_at)
//                            └─ exp (^)
//                                 └─ comp (group, abs, func, atom, floor, ceil)

math: relation;

relation:
	relation relop relation
	| expr ;

equality: expr EQUAL expr;

expr: additive;

sumop: ADD | SUB;
multop: MUL | CMD_TIMES | CMD_CDOT | DIV | CMD_DIV | COLON;
relop: EQUAL | LT | LTE | GT | GTE | NEQ;
powop: CARET;

additive: additive sumop mp
| mp ;

// mult part
mp:
	mp multop mp
	| exp
	| unary ;


// Recommended cleanup for the multiplicative expression rule
// mult part
//mp:
//	mp multop unary // A multiplicative expression is a series of unary expressions
//	| unary;

mp_nofunc:
	mp_nofunc multop mp_nofunc
	| unary_nofunc;

unary: sumop unary
	| postfix+    ;

unary_nofunc:
	sumop unary_nofunc
	| postfix postfix_nofunc*;

postfix: exp postfix_op*;
postfix_nofunc: exp_nofunc postfix_op*;
postfix_op: BANG | eval_at;

eval_at:
	BAR (eval_at_sup | eval_at_sub | eval_at_sup eval_at_sub);

eval_at_sub: UNDERSCORE L_BRACE (expr | equality) R_BRACE;

eval_at_sup: CARET L_BRACE (expr | equality) R_BRACE;

exp: exp powop (atom | L_BRACE expr R_BRACE) subexpr? | comp;

exp_nofunc:
	exp_nofunc powop (atom | L_BRACE expr R_BRACE) subexpr?
	| comp_nofunc;

comp:
	group
	| abs_group
	| func
	| atom
	| floor
	| ceil;

comp_nofunc:
	group
	| abs_group
	| atom
	| floor
	| ceil;

group:
	L_PAREN expr R_PAREN
	| L_BRACKET expr R_BRACKET
	| L_BRACE expr R_BRACE
	| L_BRACE_LITERAL expr R_BRACE_LITERAL;

abs_group: BAR expr BAR;

number: DIGIT+ (',' DIGIT DIGIT DIGIT)* ('.' DIGIT+)?;

atom: (VAR | SYMBOL) (subexpr? SINGLE_QUOTES? | SINGLE_QUOTES? subexpr?) # atomVariable
	| number                                                              # atomNumber
	| DIFFERENTIAL                                                        # atomDifferential
	| mathit                                                              # atomMathit
	| frac                                                                # atomFrac
	| binom                                                               # atomBinom
	| bra                                                                 # atomBra
	| ket                                                                 # atomKet
	;


bra: L_ANGLE expr (R_BAR | BAR);
ket: (L_BAR | BAR) expr R_ANGLE;

mathit: CMD_MATHIT L_BRACE mathit_text R_BRACE;
mathit_text: VAR?;

frac: CMD_FRAC (upperd = DIGIT | L_BRACE upper = expr R_BRACE)
	(lowerd = DIGIT | L_BRACE lower = expr R_BRACE);

binom:
	(CMD_BINOM | CMD_DBINOM | CMD_TBINOM) L_BRACE n = expr R_BRACE L_BRACE k = expr R_BRACE;

floor: L_FLOOR val = expr R_FLOOR;
ceil: L_CEIL val = expr R_CEIL;

var_sym: (VAR|SYMBOL) #atomVarSym;

func_normal:
	FUNC_EXP
	| FUNC_LOG
	| FUNC_LG
	| FUNC_LN
	| FUNC_SIN
	| FUNC_COS
	| FUNC_TAN
	| FUNC_CSC
	| FUNC_SEC
	| FUNC_COT
	| FUNC_ARCSIN
	| FUNC_ARCCOS
	| FUNC_ARCTAN
	| FUNC_ARCCSC
	| FUNC_ARCSEC
	| FUNC_ARCCOT
	| FUNC_SINH
	| FUNC_COSH
	| FUNC_TANH
	| FUNC_ARSINH
	| FUNC_ARCOSH
	| FUNC_ARTANH;

func:
	func_normal (subexpr? supexpr? | supexpr? subexpr?) (
		L_PAREN func_arg R_PAREN
		| func_arg_noparens
	) #fn_normal
	| var_sym (subexpr? SINGLE_QUOTES? | SINGLE_QUOTES? subexpr?) #fn_var // e.g. f(x), f_1'(x)
	| L_PAREN args R_PAREN  #fn_anonym
	| FUNC_INT (subexpr supexpr | supexpr subexpr)? (
		additive? DIFFERENTIAL
		| frac
		| additive
	) #fn_int
	| FUNC_SQRT (L_BRACKET root = expr R_BRACKET)? L_BRACE sqrbase = expr R_BRACE #fn_sqrt
	| FUNC_OVERLINE L_BRACE olbase = expr R_BRACE #fc_overline
	| (FUNC_SUM | FUNC_PROD) (subeq supexpr | supexpr subeq) mp #fn_sum_prod
	| FUNC_LIM limit_sub mp #fn_limit;

args: (expr ',' args) | expr;

limit_sub:
	UNDERSCORE L_BRACE (VAR | SYMBOL) LIM_APPROACH_SYM expr (
		CARET ((L_BRACE sumop R_BRACE) | sumop)
	)? R_BRACE;

func_arg: expr | (expr ',' func_arg);
func_arg_noparens: mp_nofunc;

subexpr: UNDERSCORE (atom | L_BRACE expr R_BRACE);
supexpr: CARET (atom | L_BRACE expr R_BRACE);

subeq: UNDERSCORE L_BRACE equality R_BRACE;
supeq: UNDERSCORE L_BRACE equality R_BRACE;

THINSPACE: ('\\,' | '\\thinspace') -> skip;
MEDSPACE: ('\\:' | '\\medspace') -> skip;
THICKSPACE: ('\\;' | '\\thickspace') -> skip;
QUAD: '\\quad' -> skip;
QQUAD: '\\qquad' -> skip;
NEGTHINSPACE: ('\\!' | '\\negthinspace') -> skip;
NEGMEDSPACE: '\\negmedspace' -> skip;
NEGTHICKSPACE: '\\negthickspace' -> skip;
CMD_LEFT: '\\left' -> skip;
CMD_RIGHT: '\\right' -> skip;

IGNORE:
	(
		'\\vrule'
		| '\\vcenter'
		| '\\vbox'
		| '\\vskip'
		| '\\vspace'
		| '\\hfil'
		| '\\*'
		| '\\-'
		| '\\.'
		| '\\/'
		| '\\"'
		| '\\('
		| '\\='
	) -> skip;

ADD: '+';
SUB: '-';
MUL: '*';
DIV: '/';

L_PAREN: '(';
R_PAREN: ')';
L_BRACE: '{';
R_BRACE: '}';
L_BRACE_LITERAL: '\\{';
R_BRACE_LITERAL: '\\}';
L_BRACKET: '[';
R_BRACKET: ']';

BAR: '|';

R_BAR: '\\right|';
L_BAR: '\\left|';

L_ANGLE: '\\langle';
R_ANGLE: '\\rangle';
FUNC_LIM: '\\lim';
LIM_APPROACH_SYM:
	'\\to'
	| '\\rightarrow'
	| '\\Rightarrow'
	| '\\longrightarrow'
	| '\\Longrightarrow';
FUNC_INT:
	'\\int'
	| '\\int\\limits';
FUNC_SUM: '\\sum';
FUNC_PROD: '\\prod';

FUNC_EXP: '\\exp';
FUNC_LOG: '\\log';
FUNC_LG: '\\lg';
FUNC_LN: '\\ln';
FUNC_SIN: '\\sin';
FUNC_COS: '\\cos';
FUNC_TAN: '\\tan';
FUNC_CSC: '\\csc';
FUNC_SEC: '\\sec';
FUNC_COT: '\\cot';

FUNC_ARCSIN: '\\arcsin';
FUNC_ARCCOS: '\\arccos';
FUNC_ARCTAN: '\\arctan';
FUNC_ARCCSC: '\\arccsc';
FUNC_ARCSEC: '\\arcsec';
FUNC_ARCCOT: '\\arccot';

FUNC_SINH: '\\sinh';
FUNC_COSH: '\\cosh';
FUNC_TANH: '\\tanh';
FUNC_ARSINH: '\\arsinh';
FUNC_ARCOSH: '\\arcosh';
FUNC_ARTANH: '\\artanh';

L_FLOOR: '\\lfloor';
R_FLOOR: '\\rfloor';
L_CEIL: '\\lceil';
R_CEIL: '\\rceil';

FUNC_SQRT: '\\sqrt';
FUNC_OVERLINE: '\\overline';

CMD_TIMES: '\\times';
CMD_CDOT: '\\cdot';
CMD_DIV: '\\div';
CMD_FRAC:
	'\\frac'
	| '\\dfrac'
	| '\\tfrac';
CMD_BINOM: '\\binom';
CMD_DBINOM: '\\dbinom';
CMD_TBINOM: '\\tbinom';

CMD_MATHIT: '\\mathit';

UNDERSCORE: '_';
CARET: '^';
COLON: ':';

fragment WS_CHAR: [ \t\r\n];
DIFFERENTIAL: 'd' WS_CHAR*? ([a-zA-Z] | '\\' [a-zA-Z]+);

DIGIT: [0-9];
// VAR: [a-zA-Z]+;
VAR: ('_'|'\\_')* [a-zA-Z] ([a-zA-Z0-9_]|'\\')*;

EQUAL: (('&' WS_CHAR*?)? '=') | ('=' (WS_CHAR*? '&')?);
NEQ: '\\neq';

LT: '<';
LTE: ('\\leq' | '\\le' | LTE_Q | LTE_S);
LTE_Q: '\\leqq';
LTE_S: '\\leqslant';
LATEX_BLOCK: '\\displaylines';
GT: '>';
GTE: ('\\geq' | '\\ge' | GTE_Q | GTE_S);
GTE_Q: '\\geqq';
GTE_S: '\\geqslant';

WS: [ \t]+ -> skip;
BANG: '!';
LATEX_NEWLINE: '\\\\';
SINGLE_QUOTES: '\''+;
SYMBOL: '\\' [a-zA-Z]+;
SEPARATOR: NEWLINE;
fragment NEWLINE: '\r'? '\n' | '\r';
