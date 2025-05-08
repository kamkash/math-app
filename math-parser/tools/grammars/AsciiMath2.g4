grammar AsciiMath2;

// Parser Rules

// Entry point
block: expression (SEPARATOR expression)* SEPARATOR* EOF;

expression: logical_expression; // Start with lowest precedence

// Logical operators (lowest precedence if needed, e.g. and, or, not) For simplicity, we'll skip
// explicit logical operators for now and go to relations
logical_expression:
	relation_expression ((AND | OR) relation_expression)*;

// Relational operators (e.g., =, <, >, <=, >=, !=)
relation_expression:
	add_sub_expression (
		(EQ | NEQ | LT | GT | LTE | GTE) add_sub_expression
	)?; // Allow only one relational op for simplicity here

// Addition and Subtraction (Left associative)
add_sub_expression:
	mult_div_implicit_expression (
		(PLUS | MINUS | PM) mult_div_implicit_expression
	)*;

// Multiplication, Division, and Implicit Multiplication (Left associative) Order: Handle explicit
// ops first, then try to parse implicit. This is a tricky part. Implicit multiplication 'atom atom'
// has high precedence. Let's group them:
mult_div_implicit_expression:
	unary_op_expression (
		(STAR | FSLASH | TIMES | DIV) unary_op_expression
		| unary_op_expression
	)*; // Last 'unary_op_expression' is for implicit: 2x, (a)x

// Unary Plus/Minus (Prefix)
unary_op_expression: (PLUS | MINUS) script_op_expression	# unaryPrefixExpression
	| script_op_expression									# unaryExpression;

// Superscript (Power), Subscript AsciiMath usually handles scripts after the base: base^script or
// base_script Power is often right-associative: a^b^c -> a^(b^c) Subscripts/Superscripts have
// higher precedence than mult/div.
script_op_expression:
	primary_expression (
		(HAT primary_expression (UNDERSCORE primary_expression)?)
	)*															# powerSubscriptExpression
	| (UNDERSCORE primary_expression (HAT primary_expression)?)	# subscriptPowerExpression
	| HAT primary_expression									# powerExpression
	| UNDERSCORE primary_expression								# subscriptExpression
	| PRIME														# primeExpression ; // Allow multiple scripts like f'_1^2 but needs care

// Primary expressions - the highest precedence
primary_expression:
	LPAREN expression RPAREN			# parenExpression // (expression)
	| LBRACE expression RBRACE			# braceExpression // {expression}
	| LBRACKET matrix_content RBRACKET	# matrixExpression // [a,b;c,d]
	| ABS expression ABS				# absExpression // |expression|  (abs() preferred)
	| L_ANGLE expression R_ANGLE		# angleBracketExpression // (: expression :) or << expression >>

	// Standard Functions
	| func_name LPAREN arguments RPAREN	# functionCall // sin(x), log(x,y)
	| func_name primary_expression		# functionCallSimple
	// sin x (less common for multi-char func name but possible)

	// AsciiMath specific functions/keywords as prefix
	| SQRT primary_expression						# sqrtFunction // sqrt x or sqrt(x)
	| ROOT primary_expression primary_expression	# rootFunction // root n x
	| FRAC primary_expression primary_expression	# fracFunction // frac num den
	| TEXT LPAREN text_argument RPAREN				# textFunction // text("some text") or text(x)

	// Calculus
	| INTEGRAL (UNDERSCORE primary_expression)? (
		HAT primary_expression
	)? primary_expression (differential_dx)?				# integralExpression // int_a^b f(x) dx
	| DERIV primary_expression (WTRT primary_expression)?	# derivativeFunction
	// deriv(f) or deriv(f,x)
	| PARTIAL primary_expression (WTRT primary_expression)? # partialFunction
	// partial(f) or partial(f,x)
	| D_LOWERCASE primary_expression FSLASH D_LOWERCASE primary_expression					# LeibnizNotation // dy/dx
	| LIM UNDERSCORE primary_expression (TO | RARROW) primary_expression primary_expression	#
		limitExpression // lim_(x->a) f(x)

	// Linear Algebra specific constructs or functions
	| VEC primary_expression			# vecFunction // vec(x)
	| MAT LPAREN matrix_content RPAREN	# matFunction // mat([a,b];[c,d])
	| DET primary_expression			# detFunction // det(A)
	| TRANSPOSE primary_expression		# transposeFunction // T A or transpose(A)

	// Atoms
	| NUMBER			# numberAtom
	| IDENTIFIER		# identifierAtom
	| GREEK_LETTER		# greekLetterAtom
	| constant_symbol	# constantAtom
	| STRING			# stringAtom; // "text"

arguments:
	expression (COMMA expression)*
	|; // Allows empty arguments for f()

text_argument: STRING | expression; // text("hello") or text(var)

differential_dx:
	D_LOWERCASE (IDENTIFIER | GREEK_LETTER); // dx, dy, dtheta

// For derivatives like deriv(f,x) or partial(f,x) meaning 'with respect to x' wrt_expression: WRT
// primary_expression;

// Matrix content: rows separated by semicolon, elements by comma
matrix_content: matrix_row (SEMICOLON matrix_row)*;

matrix_row: expression (COMMA expression)*;

func_name:
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
	| ABS_FUNC // abs as a function name
	| NORM
	| SUM
	| PROD
	| CARD
	| IDENTIFIER; // For any other function name like f, g, myFunc

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

// Lexer Rules (Tokens) Keywords should be defined before IDENTIFIER

// Calculus Keywords
INTEGRAL: 'int' | '\u222B';
DERIV: 'deriv'; // Using 'deriv' for d/dx as a function
PARTIAL: 'partial' | 'del' | '\u2202';
LIM: 'lim';
D_LOWERCASE:
	'd'; // For dy/dx, needs to be distinct from identifier 'd' if possible, context helps

// Standard Function Keywords (many are similar to IDENTIFIER, context in parser helps)
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
SQRT: 'sqrt' | '\u221A';
ROOT: 'root';
FRAC: 'frac';
TEXT: 'text';
FLOOR: 'floor';
CEIL: 'ceil';
ROUND: 'round';
MIN: 'min';
MAX: 'max';
NORM: 'norm';
CARD: 'card';
ABS_FUNC:
	'abs'; // abs as a function name to distinguish from |x|

// Linear Algebra Keywords
VEC: 'vec';
MAT: 'mat'; // For mat(...) construct
DET: 'det';
TRANSPOSE:
	'transpose'
	| ('T' ~[a-zA-Z0-9]); // T not followed by letter/digit (e.g. T A)

// Summation/Product Keywords
SUM: 'sum' | '\u2211';
PROD: 'prod' | '\u220F';

// Constant Keywords
PI_CONST: 'pi' | '\u03C0';
E_CONST:
	'e'; // Euler's number 'e' can be tricky if also a variable
I_CONST: 'i'; // Imaginary unit 'i'
INFINITY_CONST: 'oo' | 'infty' | '\u221E';
GAMMA_CONST:
	'gamma'
	| '\u03B3'; // If 'gamma' is always Euler-Mascheroni, otherwise GREEK_LETTER
PHI_CONST:
	'phi'
	| '\u03C6'; // Golden ratio, if always this, otherwise GREEK_LETTER
TRUE_CONST: 'true';
FALSE_CONST: 'false';
NAN_CONST: 'NaN';

// Operators (Order can matter if ambiguous, e.g. '-' vs '->')
PLUS: '+';
MINUS: '-';
STAR: '*'; // Explicit multiplication
FSLASH: '/'; // Division or fraction
HAT: '^';
UNDERSCORE: '_';
PRIME: '\'';
BANG: '!'; // Factorial
EQ: '='; // Equality, not assignment in expressions
NEQ: '!=' | '<>';
LT: '<';
GT: '>';
LTE: '<=' | 'le';
GTE: '>=' | 'ge';
TO: 'to'; // For limits x->a
RARROW: '->' | '\u2192'; // Right arrow
LARROW: '<-' | '\u2190'; // Left arrow (if needed)
PM: '+-' | '\u00B1'; // Plus-minus
TIMES:
	'xx'
	| 'cdot'
	| '\u00D7'
	| '\u22C5'; // Multiplication cross/dot
DIV: '-:' | 'div' | '\u00F7'; // Division symbol
WTRT: 'wrt'; // "with respect to" for derivatives

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
ABS: '|'; // Absolute value bars
L_ANGLE:
	'(:'
	| '<<'
	| '\u2329'; // Invisible paren, or angle bracket
R_ANGLE:
	':)'
	| '>>'
	| '\u232A'; // Invisible paren, or angle bracket

// Separators
COMMA: ',';
SEMICOLON: ';';

// Greek Letters (Common ones, more can be added) These should come before IDENTIFIER if they are
// reserved
ALPHA_G: 'alpha';
BETA_G: 'beta'; //GAMMA_G: 'gamma'; // covered by GAMMA_CONST
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
//PI_G: 'pi'; // covered by PI_CONST
RHO_G: 'rho';
SIGMA_G: 'sigma';
TAU_G: 'tau';
UPSILON_G: 'upsilon'; //PHI_G: 'phi'; // covered by PHI_CONST
CHI_G: 'chi';
PSI_G: 'psi';
OMEGA_G: 'omega';
// Capital Greek letters if needed, e.g., Delta, Sigma, Omega (often IDENTIFIERs)
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

// General Identifier (for variables, unrecognized functions) Must be after all keywords that might
// look like identifiers
IDENTIFIER:
	[_]* [a-zA-Z] [a-zA-Z0-9_]*; // Allows underscore in variable names, e.g. x_1

// Numbers: Integers and decimals, scientific notation optional
NUMBER:
	MINUS? DIGITS ('.' DIGITS)? ([eE] MINUS? DIGITS)?
	| MINUS? '.' DIGITS ( [eE] MINUS? DIGITS)?;
fragment DIGITS: [0-9]+;

// String Literals for text("...")
STRING:
	'"' (~["\r\n] | '""')*? '"'; // Allows "" for escaped quote

SEPARATOR: NEWLINE;

fragment NEWLINE: '\r'? '\n' | '\r';

// Whitespace: Skipped by the parser WS: [ \t\r\n]+ -> skip;
WS: [ \t]+ -> skip;

// Comments (AsciiMath doesn't have a formal comment syntax, but if it were needed) LINE_COMMENT:
// '//' ~[\r\n]* -> skip;