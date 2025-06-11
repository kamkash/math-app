grammar AsciiMath2;

// Parser Rules

block: (integral_expression | expression) (
		SEPARATOR (integral_expression | expression)
	)* SEPARATOR* EOF;

expression: logical_expression; // Start with lowest precedence

// Logical operators (lowest precedence if needed, e.g. and, or, not) For simplicity, we'll skip
// explicit logical operators for now and go to relations
logical_expression:
	relation_expression ((AND | OR) relation_expression)* # relationalExpression;

relation_expression:
	relation_expression_no_rhs
	| add_sub_expression (relop add_sub_expression)?;

relation_expression_no_rhs: (add_sub_expression | function_call) EQ;

add_sub_expression:
	mult_div_expression (sumop mult_div_expression)*;

mult_div_expression:
	power_expression (multop? power_expression)*;

power_expression:
	primary_expression (powop primary_expression)*;

unary_op_expression: (PLUS | MINUS) primary_expression	# unaryPlusMinus
	| primary_expression								# noUnaryOperator;

//differential_other: LPAREN? DVAR RPAREN?;   // can't get this to work
differential_other:
	LPAREN? (
		'du'
		| 'dv'
		| 'dw'
		| 'dalpha'
		| 'beta'
		| 'dgamma'
		| 'd\u03B1'
		| 'd\u03B2'
		| 'd\u03B3'
	) RPAREN?;

differential: ('dx' | 'dy' | 'dz' | 'dt' | differential_other);

integral_expression:
	LPAREN? INTEGRAL RPAREN? (integral_lower_limit)? (
		integral_upper_limit
	)? op_body differential EQ? # integralExpression;

scripted_op_expression:
    LPAREN? BUILTIN_KEYWORD_FUNC_NAME RPAREN? WS* (UNDERSCORE add_sub_expression)? (HAT add_sub_expression)? ;

op_body: LPAREN? add_sub_expression RPAREN?;

integral_upper_limit: HAT ('oo' | '+oo' | '-oo' | NUMBER);

integral_lower_limit:
	UNDERSCORE ('oo' | '+oo' | '-oo' | NUMBER);

// Primary expressions - the highest precedence
primary_expression:
	LPAREN logical_expression RPAREN														# parenExpression
	| (BUILTIN_KEYWORD_FUNC_NAME | scripted_op_expression) LPAREN arguments RPAREN			# explicitKeywordCall // sin(x), vec(x,y,z)
	| BUILTIN_KEYWORD_FUNC_NAME primary_expression											# simpleKeywordCall // e.g., sin x
	| LBRACE logical_expression RBRACE														# braceExpression // e.g. {a+b}
	| ABS logical_expression ABS															# absExpression // |expression|
	| ROOT primary_expression primary_expression											# rootFunction
	| FRAC primary_expression primary_expression											# fracFunction
	| TEXT LPAREN text_argument RPAREN														# textFunction
	| derivative																			# derivativeFunction
	| partial_derivative																	# partialFunction
	| differential FSLASH differential														# fractionLeibniz
	| LIM UNDERSCORE primary_expression (TO | RARROW) primary_expression primary_expression	#
		limitExpression
	| LPAREN paren_element_for_column_vector (
		COMMA paren_element_for_column_vector
	)* RPAREN # parenColumnVector
	// | LPAREN matrix_content RPAREN # parenMatrix | LBRACKET matrix_content RBRACKET #
	// bracketMatrix | L_ANGLE matrix_row R_ANGLE # angleBracketRowVector | MAT LPAREN
	// matrix_content RPAREN # matFunction // mat((a,b];[c,d]))
	| DET primary_expression		# detFunction
	| TRANSPOSE primary_expression	# transposeFunction
	| IDENTIFIER					# identifierAtom
	| NUMBER						# numberAtom
	| NUMBER_WITH_COMMAS			# numberWithCommasAtom
	| CURRENCY_NUMBER				# currencyNumberAtom
	| GREEK_LETTER					# greekLetterAtom
	| constant_symbol				# constantAtom
	| STRING						# stringAtom;

// Rule for elements of the specific ((a),(b)) column vector style
paren_element_for_column_vector: LPAREN expression RPAREN;

arguments: (logical_expression | unary_op_expression) (
		COMMA (expression | unary_op_expression)
	)*;
text_argument: STRING | expression;
wrt_argument: COMMA expression;

matrix_content: matrix_row (SEMICOLON matrix_row)*;
// Can be a full matrix, a row vector (1 row), or a column vector (1 col)

matrix_row:
	expression (COMMA expression)*; // Represents a single row with comma-separated elements

deriv_function: DERIV;
d_by_d: DBYD;
derivative: (deriv_function | d_by_d) primary_expression (
		wrt_argument
	)?;

partial_derivative: PARTIAL primary_expression (wrt_argument)?;

// MODIFICATION: Rule for f(x), f'(x), f''(x) etc. This rule takes an IDENTIFIER, optionally
// followed by one or more PRIME symbols, then a parenthesized argument list.
function_call:
	IDENTIFIER LPAREN arguments RPAREN
	| IDENTIFIER (PRIME+)? LPAREN arguments RPAREN;

// --- Lexer Rules (Tokens) ---

// For functions like sin, cos, log, and now vec
BUILTIN_KEYWORD_FUNC_NAME:
	SQRT
	| SIN
	| COS
	| TAN
	| LOG
	| LN
	| EXP
	| FLOOR
	| CEIL
	| ROUND
	| MIN
	| MAX
	| CSC
	| SEC
	| COT
	| ASIN
	| ACOS
	| ATAN
	| ACSC
	| ASEC
	| ACOT
	| SINH
	| COSH
	| TANH
	| CSCH
	| SECH
	| COTH
	| ASINH
	| ACOSH
	| ATANH
	| ACSCH
	| ASECH
	| ACOTH
	| ABS_FUNC
	| NORM
	| CARD
	| SUM
	| PROD
	| VEC
	| SOLVE;

constant_symbol:
	PI_CONST
	// | E_CONST | I_CONST
	| INFINITY_CONST
	| GAMMA_CONST
	| TRUE_CONST
	| FALSE_CONST
	| NAN_CONST
	| PHI_CONST;

relop: GT | LT | LTE | GTE | EQ | NEQ | DOUBLE_EQ;

sumop: PLUS | MINUS | PM;

multop: STAR | FSLASH | TIMES | DIV;

powop: HAT | POW;

// --- Lexer Rules (Tokens) --- (Ensure these are complete and correctly ordered, Keywords before
// IDENTIFIER)

INTEGRAL: 'int' | '\u222B';
// D_LOWERCASE: 'd';
DERIV: 'deriv';
DBYD:
	LPAREN? 'd' RPAREN? FSLASH LPAREN? 'd' WS* (
		IDENTIFIER
		| GREEK_LETTER
	) RPAREN?;
PARTIAL: 'partial' | 'del' | '\u2202';
LIM: 'lim';

// Function name keywords
SOLVE: 'solve';
SIN: 'sin';
COS: 'cos';
TAN: 'tan';
CSC: 'csc';
SEC: 'sec';
COT: 'cot';
ASIN: 'asin' | 'arcsin';
ACOS: 'acos' | 'arccos';
ATAN: 'atan' | 'arctan';
ACSC: 'acsc' | 'arccsc';
ASEC: 'asec' | 'arcsec';
ACOT: 'acot' | 'arccot';
SINH: 'sinh';
COSH: 'cosh';
TANH: 'tanh';
CSCH: 'csch';
SECH: 'sech';
COTH: 'coth';
ASINH: 'asinh' | 'arsinh';
ACOSH: 'acosh' | 'arcosh';
ATANH: 'atanh' | 'artanh';
ACSCH: 'acsch' | 'arcsch';
ASECH: 'asech' | 'arcsech';
ACOTH: 'acoth' | 'arcoth';
LOG: 'log';
LN: 'ln';
EXP: 'exp';
FLOOR: 'floor';
CEIL: 'ceil';
ROUND: 'round';
MIN: 'min';
MAX: 'max';
NORM: 'norm';
CARD: 'card';
ABS_FUNC: 'abs'; // abs as a function name
SUM: 'sum' | '\u2211';
PROD: 'prod' | '\u220F';
VEC: 'vec'; // Token for the vec function
SQRT: 'sqrt' | '\u221A';
ROOT: 'root';
FRAC: 'frac';
TEXT: 'text';
MAT:
	'mat'; // Keyword for explicit matrix constructor e.g. mat(...)
DET: 'det';
TRANSPOSE:
	'transpose'
	| ('T' ~[a-zA-Z0-9]); // T not followed by letter/digit (e.g. T A)

// Constant Keywords
PI_CONST: 'pi' | '\u03C0';
// E_CONST: 'e'; I_CONST: 'i';
INFINITY_CONST: 'oo' | 'infty' | '\u221E';
SIGNED_INFINITY_CONST: (PLUS | MINUS)? INFINITY_CONST;
GAMMA_CONST: 'gamma' | '\u03B3';
PHI_CONST: 'phi' | '\u03C6';
TRUE_CONST: 'true';
FALSE_CONST: 'false';
NAN_CONST: 'NaN';

// Operators
PLUS: '+';
MINUS: '-';
STAR: '*';
FSLASH: '/';
HAT: '^';
POW: '**';
UNDERSCORE: '_';
PRIME: '\'';
BANG: '!';
EQ: '=';
DOUBLE_EQ: '==';
NEQ: '!=' | '<>';
LT: '<';
GT: '>';
LTE: '<=' | 'le';
GTE: '>=' | 'ge';
TO: 'to';
RARROW: '->' | '\u2192';
LARROW: '<-' | '\u2190';
PM: '+-' | '\u00B1';
TIMES: 'xx' | 'cdot' | '\u00D7' | '\u22C5';
DIV: '-:' | 'div' | '\u00F7';

AND: 'and';
OR: 'or';
NOT: 'not';

// Grouping Symbols
LPAREN: '(';
RPAREN: ')';
LBRACKET: '[';
RBRACKET: ']';
LBRACE: '{';
RBRACE: '}';
ABS: '|';
L_ANGLE: '(:' | '<<' | '\u2329';
R_ANGLE: ':)' | '>>' | '\u232A';

// Separators
COMMA: ',';
SEMICOLON: ';';

// Greek Letters
ALPHA_G: 'alpha';
BETA_G: 'beta';
DELTA_G: 'delta';
EPSILON_G: 'epsilon';
ZETA_G: 'zeta';
ETA_G: 'eta';
THETA_G: 'theta';
IOTA_G: 'iota';
KAPPA_G: 'kappa';
LAMBDA_G: 'lambda';
MU_G: 'mu';
NU_G: 'nu';
XI_G: 'xi';
RHO_G: 'rho';
SIGMA_G: 'sigma';
TAU_G: 'tau';
UPSILON_G: 'upsilon';
CHI_G: 'chi';
PSI_G: 'psi';
OMEGA_G: 'omega';
GREEK_LETTER:
	ALPHA_G
	| BETA_G
	| DELTA_G
	| EPSILON_G
	| ZETA_G
	| ETA_G
	| THETA_G
	| IOTA_G
	| KAPPA_G
	| LAMBDA_G
	| MU_G
	| NU_G
	| XI_G
	| RHO_G
	| SIGMA_G
	| TAU_G
	| UPSILON_G
	| CHI_G
	| PSI_G
	| OMEGA_G;

fragment CURRENCY_SYMBOL:
	'$'
	| '€'
	| '£'
	| '¥'; // Add more symbols as needed

fragment E1: 'E';

fragment E2: 'e';

fragment SIGN: '+' | '-';

fragment RESERVED_WORDS: 'oo' | 'infty';

// General Identifier
IDENTIFIER: [_]* [a-zA-Z] [a-zA-Z0-9_]*;
LETTERS: [a-zA-Z];
LOWERCASE_LETTER: [a-z];
fragment DVAR: 'du' | 'dv' | 'dw';

// Numbers
NUMBER:
	DIGITS ('.' DIGITS)? ([eE] MINUS? DIGITS)?
	| '.' DIGITS ( [eE] MINUS? DIGITS)?;

NUMBER_WITH_COMMAS: DIGIT+ (',' DIGIT_THREE)* ('.' DIGIT+)?;

CURRENCY_NUMBER:
	CURRENCY_SYMBOL (NUMBER_WITH_COMMAS | SCIENTIFIC_NUMBER);

SCIENTIFIC_NUMBER: NUMBER ((E1 | E2) SIGN? NUMBER)?;

fragment DIGITS: [0-9]+;
fragment DIGIT: [0-9];
fragment DIGIT_THREE: DIGIT DIGIT DIGIT;

// String Literals
STRING: '"' ( ~["\r\n] | '""')*? '"';

SEPARATOR: NEWLINE;

fragment NEWLINE: '\r'? '\n' | '\r';

// Whitespace: Skipped by the parser WS: [ \t\r\n]+ -> skip;
WS: [ \t]+ -> skip;