grammar AsciiMath2;

// Parser Rules

block: expression (SEPARATOR expression)* SEPARATOR* EOF;

expression: logical_expression; // Start with lowest precedence

// Logical operators (lowest precedence if needed, e.g. and, or, not) For simplicity, we'll skip
// explicit logical operators for now and go to relations
logical_expression:
	relation_expression ((AND | OR) relation_expression)*;

relation_expression:
	relation_expression_no_rhs
	| add_sub_expression (
		(EQ | NEQ | LT | GT | LTE | GTE) add_sub_expression
	)?;

relation_expression_no_rhs: add_sub_expression EQ SEPARATOR+;

add_sub_expression:
	mult_div_implicit_expression (
		(PLUS | MINUS | PM) mult_div_implicit_expression
	)*;

mult_div_implicit_expression:
	unary_op_expression (
		(STAR | FSLASH | TIMES | DIV) unary_op_expression
		| unary_op_expression
	)*;

// Unary operations
unary_op_expression: (PLUS | MINUS) script_op_expression	# unaryPlusMinus
	| d_dx_prefix_operator script_op_expression				# appliedDByDxPrefix // For d/dx f(x)
	| script_op_expression									# noUnaryOperator;

d_dx_function:
	d_dx_prefix_operator LPAREN primary_expression RPAREN;
d_dx_prefix_operator: D_LOWERCASE FSLASH differential;
differential: D_LOWERCASE (IDENTIFIER | GREEK_LETTER);

script_op_expression:
	primary_expression (
		(HAT primary_expression (UNDERSCORE primary_expression)?)
	)*															# powerSubscriptExpression
	| (UNDERSCORE primary_expression (HAT primary_expression)?)	# subscriptPowerExpression
	| HAT primary_expression									# powerExpression
	| UNDERSCORE primary_expression								# subscriptExpression
	| PRIME														# primeExpression; // Allow multiple scripts like f'_1^2 but needs care

// Primary expressions - the highest precedence
primary_expression:
	// MODIFICATION: Rule for f(x), f'(x), f''(x) etc. This rule takes an IDENTIFIER, optionally
	// followed by one or more PRIME symbols, then a parenthesized argument list.
	IDENTIFIER (PRIME+)? LPAREN arguments RPAREN # explicitIdentifierCall

	// Rule for built-in function calls like sin(x), log(x) if they can't be primed or have
	// different syntax BUILTIN_KEYWORD_FUNC_NAME should be a rule or token set for sin, cos, log
	// etc.
	| BUILTIN_KEYWORD_FUNC_NAME LPAREN arguments RPAREN # explicitKeywordCall

	// Rule for built-in functions that don't use parentheses in AsciiMath like sin x (If different
	// from SQRT primary_expression etc. which are already prefix ops)
	| BUILTIN_KEYWORD_FUNC_NAME primary_expression # simpleKeywordCall // e.g., sin x

	// Specific parenthesized structures first: 1. Column vectors like ((a),(b),(c)) where comma
	// separates rows of parenthesized elements
	| LPAREN paren_element_for_column_vector (
		COMMA paren_element_for_column_vector
	)* RPAREN # parenColumnVector

	// 2. Parentheses for matrices (e.g. (a,b; c,d) ) or row vectors (e.g. (a,b,c) ) This rule will
	// also catch single expressions like (x) if not caught by parenExpression first or if
	// paren_column_vector_row fails.
	| LPAREN matrix_content RPAREN # parenMatrix

	// 3. General parentheses for grouping any expression (fallback for simple grouping)
	| LPAREN expression RPAREN # parenExpression

	// Standard matrix with square brackets (can represent row or column vectors too)
	| LBRACKET matrix_content RBRACKET # bracketMatrix

	// Angle bracket vectors (typically row vectors like <x,y,z> or (:x,y,z:))
	| L_ANGLE matrix_row R_ANGLE						# angleBracketRowVector
	| LBRACE expression RBRACE							# braceExpression // e.g. {a+b}
	| ABS expression ABS								# absExpression // |expression|
	| IDENTIFIER (PRIME+)? LPAREN arguments RPAREN		# explicitIdentifierCall // f(x), f'(x)
	| BUILTIN_KEYWORD_FUNC_NAME LPAREN arguments RPAREN	# explicitKeywordCall // sin(x), vec(x,y,z)
	| BUILTIN_KEYWORD_FUNC_NAME primary_expression		# simpleKeywordCall // e.g., sin x
	| SQRT primary_expression							# sqrtFunction
	| ROOT primary_expression primary_expression		# rootFunction
	| FRAC primary_expression primary_expression		# fracFunction
	| TEXT LPAREN text_argument RPAREN					# textFunction
	| INTEGRAL (UNDERSCORE primary_expression)? (
		HAT primary_expression
	)? primary_expression (differential)?													# integralExpression
	| DERIV primary_expression (wrt_argument)?												# derivativeFunction
	| PARTIAL primary_expression (wrt_argument)?											# partialFunction
	| differential FSLASH differential														# fractionLeibniz
	| LIM UNDERSCORE primary_expression (TO | RARROW) primary_expression primary_expression	#
		limitExpression

	// Explicit MAT constructor (if different from bracketMatrix/parenMatrix)
	| MAT LPAREN matrix_content RPAREN	# matFunction // mat((a,b];[c,d]))
	| DET primary_expression			# detFunction
	| TRANSPOSE primary_expression		# transposeFunction
	| IDENTIFIER						# identifierAtom
	| NUMBER							# numberAtom
	| NUMBER_WITH_COMMAS				# numberWithCommasAtom
	| CURRENCY_NUMBER					# currencyNumberAtom
	| GREEK_LETTER						# greekLetterAtom
	| constant_symbol					# constantAtom
	| STRING							# stringAtom;

// Rule for elements of the specific ((a),(b)) column vector style
paren_element_for_column_vector: LPAREN expression RPAREN;

arguments: expression (COMMA expression)* |;
text_argument: STRING | expression;
wrt_argument: COMMA expression;

matrix_content: matrix_row (SEMICOLON matrix_row)*;
// Can be a full matrix, a row vector (1 row), or a column vector (1 col)

matrix_row:
	expression (COMMA expression)*; // Represents a single row with comma-separated elements

// For functions like sin, cos, log, and now vec
BUILTIN_KEYWORD_FUNC_NAME:
	SIN
	| COS
	| TAN
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
	| LOG
	| LN
	| EXP
	| FLOOR
	| CEIL
	| ROUND
	| MIN
	| MAX
	| ABS_FUNC
	| NORM
	| CARD
	| SUM
	| PROD // if used as name(args)
	| VEC; // Added VEC here
// Note: SQRT, FRAC, ROOT, TEXT, DET, TRANSPOSE are handled by their own specific rules in
// primary_expression MAT is also a specific rule (matFunction) if it's a keyword based
// constructor.;
constant_symbol:
	PI_CONST
	| E_CONST
	| I_CONST
	| INFINITY_CONST
	| GAMMA_CONST
	| TRUE_CONST
	| FALSE_CONST
	| NAN_CONST
	| PHI_CONST;

// --- Lexer Rules (Tokens) --- (Ensure these are complete and correctly ordered, Keywords before
// IDENTIFIER)

INTEGRAL: 'int' | '\u222B';
D_LOWERCASE: 'd';
DERIV: 'deriv' | DBYD;
DBYD:
	D_LOWERCASE WS* FSLASH D_LOWERCASE (
		IDENTIFIER
		| GREEK_LETTER
	);
PARTIAL: 'partial' | 'del' | '\u2202';
LIM: 'lim';

// Function name keywords
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

// Other keywords used as prefix operators or specific structures
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
E_CONST: 'e';
I_CONST: 'i';
INFINITY_CONST: 'oo' | 'infty' | '\u221E';
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
UNDERSCORE: '_';
PRIME: '\'';
BANG: '!';
EQ: '=';
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

// General Identifier
IDENTIFIER: [_]* [a-zA-Z] [a-zA-Z0-9_]*;

// Numbers
NUMBER:
	MINUS? DIGITS ('.' DIGITS)? ([eE] MINUS? DIGITS)?
	| MINUS? '.' DIGITS ( [eE] MINUS? DIGITS)?;
fragment DIGITS: [0-9]+;

NUMBER_WITH_COMMAS: DIGIT+ (',' DIGIT_THREE)* ('.' DIGIT+)?;

CURRENCY_NUMBER:
	CURRENCY_SYMBOL (NUMBER_WITH_COMMAS | SCIENTIFIC_NUMBER);

SCIENTIFIC_NUMBER: NUMBER ((E1 | E2) SIGN? NUMBER)?;

fragment DIGIT: [0-9];

fragment DIGIT_THREE: DIGIT DIGIT DIGIT;

// String Literals
STRING: '"' ( ~["\r\n] | '""')*? '"';

SEPARATOR: NEWLINE;

fragment NEWLINE: '\r'? '\n' | '\r';

// Whitespace: Skipped by the parser WS: [ \t\r\n]+ -> skip;
WS: [ \t]+ -> skip;

// Comments (AsciiMath doesn't have a formal comment syntax, but if it were needed) LINE_COMMENT:
// '//' ~[\r\n]* -> skip;