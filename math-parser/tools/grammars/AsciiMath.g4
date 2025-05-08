grammar AsciiMath;

// ========== Parser Rules ==========

block: (functionDefinition | equation | expr) (
		SEPARATOR (functionDefinition | equation | expr)
	)* SEPARATOR* WS* EOF;

functionDefinition: ID LPAREN atom (COMMA atom)* RPAREN EQ expr;

equation: expr EQ expr;

expr: sumExpr (relOp sumExpr)* # RelationalExpression;

sumExpr:
	productExpr ((ADD | SUB) productExpr)* # AddSubExpression;

productExpr:
	powerExpr ((MUL | DIV)? powerExpr)* # MulDivOrImplicitExpression;

powerExpr:
	signedAtom (POW powerExpr)? (UND powerExpr)? # PowerSubscriptExpression;

signedAtom: ADD signedAtom | SUB signedAtom | func_ | atom;

atom:
	LPAREN expr RPAREN		# Parenthesized
	| FRAC expr DIV expr	# FractionExpression
	| derivExpr				# DerivativeExpression
	| intExpr				# IntegralExpression
	| limExpr				# LimitExpression
	| matrix				# MatrixExpression
	| VECTOR_OP atom		# VectorExpression
	| SYMBOL				# Symbol
	| GREEK					# GreekLetter
	| ID					# Identifier
	| NUMBER				# Number;

relOp: EQ | NEQ | LT | GT | LE | GE;

intExpr: INT (UND expr)? (POW expr)? expr # IntegralWithLimits;

limExpr: LIM (UND expr)? expr # LimitWithSubscript;

derivExpr: (D DIV D ID | PARTIAL DIV PARTIAL ID) expr # DerivExpression;

matrix: LBRACK LBRACK row (COMMA row)* RBRACK RBRACK # Mat;

row: expr (COMMA expr)* # MatrixRow;

func_: FUNC LPAREN expr (COMMA expr)* RPAREN;

// ========== Lexer Rules ==========

// Keywords
FUNC:
	'sqrt'
	| 'sin'
	| 'cos'
	| 'tan'
	| 'sec'
	| 'csc'
	| 'cot'
	| 'log'
	| 'ln'
	| 'exp';
FRAC: 'frac';
INT: 'int';
LIM: 'lim';
D: 'd';
PARTIAL: 'partial';
VECTOR_OP: 'vec' | 'hat' | 'bar' | 'tilde';

// Symbols and relations
SYMBOL:
	'pi'
	| 'infty'
	| 'oo'
	| '->'
	| '<-'
	| '<->'
	| '=>'
	| 'xx'
	| '.';

GREEK:
	'alpha'
	| 'beta'
	| 'gamma'
	| 'delta'
	| 'epsilon'
	| 'zeta'
	| 'eta'
	| 'theta'
	| 'iota'
	| 'kappa'
	| 'lambda'
	| 'mu'
	| 'nu'
	| 'xi'
	| 'omicron'
	| 'rho'
	| 'sigma'
	| 'tau'
	| 'upsilon'
	| 'phi'
	| 'chi'
	| 'psi'
	| 'omega';

// Operators
ADD: '+';
SUB: '-';
MUL: '*';
DIV: '/';
POW: '^';
UND: '_';
EQ: '=';
NEQ: '!=';
LT: '<';
GT: '>';
LE: '<=';
GE: '>=';
COMMA: ',';

// Brackets
LPAREN: '(';
RPAREN: ')';
LBRACK: '[';
RBRACK: ']';

// Identifiers and numbers
ID: [a-zA-Z] [a-zA-Z0-9_]*;
NUMBER: [0-9]+ ('.' [0-9]+)?;

SEPARATOR: ';' | NEWLINE;

fragment NEWLINE: '\r'? '\n' | '\r';

// Whitespace
WS: [ \t]+ -> skip;