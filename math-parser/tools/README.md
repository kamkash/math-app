```bash
(webappenv) Mac-Studio:tools kamran$ history | grep antlr
java -jar antlr-4.13.2-complete.jar -Dlanguage=TypeScript calculator.g4
java -jar antlr4-4.8-2-rust-SNAPSHOT-complete.jar -Dlanguage=Rust calculator.g4
java -jar antlr-4.13.2-complete.jar calculator.g4
java -cp antlr-4.13.2-complete.jar org.antlr.v4.runtime.misc.TestRig
java -cp antlr-4.13.2-complete.jar org.antlr.v4.runtime.misc.TestRig calculator.g4
java -cp antlr-4.13.2-complete.jar org.antlr.v4.runtime.misc.TestRig calculator r -tokens
java -cp .:../antlr-4.13.2-complete.jar org.antlr.v4.gui.TestRig calculator multiplyingExpression -tokens
java -cp .:../antlr-4.13.2-complete.jar org.antlr.v4.gui.TestRig calculator multiplyingExpression -tree
java -cp .:../antlr-4.13.2-complete.jar org.antlr.v4.gui.TestRig calculator multiplyingExpression -tree -gui


java -cp ../antlr-4.13.2-complete.jar org.antlr.v4.Tool -visitor LaTeX.g4 
javac *.java
alias grun='java org.antlr.v4.gui.TestRig'
export CLASSPATH=`pwd`/../antlr-4.13.2-complete.jar:`pwd`:$CLASSPATH
CLASSPATH=.:../antlr-4.13.2-complete.jar grun LaTeX  block -tree -gui block.tex 


```

## antlr-rust

<https://github.com/rrevenantt/antlr4rust>

## Rust parsers

## LALRPOP

<https://lalrpop.github.io/lalrpop/>
<https://github.com/lalrpop/lalrpop>

## LaTeX Math Syntax for Symbolic Algebra Parsing

Since you're parsing for **semantic meaning** (not typesetting), you only need to handle the syntactic forms that encode mathematical structure. Forget spacing commands, font switches, and layout environments.

---

### Inline — What to Support

Both delimiters are common in the wild:

```
$...$
\(...\)
```

```latex
The derivative is $\frac{d}{dx} f(x)$ at the point.
The derivative is \(\frac{d}{dx} f(x)\) at the point.
```

---

### Block — What to Support

```
\[...\]
\begin{equation}...\end{equation}
\begin{equation*}...\end{equation*}
```

```latex
\[ x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a} \]

\begin{equation}
  x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
\end{equation}

\begin{equation*}
  x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
\end{equation*}
```

---

### Multi-line Block — What to Support

The only ones worth handling for algebra are `align` and `align*`. The others (`gather`, `multline`) are purely typographic.

```
\begin{align}...\end{align}
\begin{align*}...\end{align*}
```

The `&` marks the **alignment point** (usually at `=`), and `\\` is the **line separator**. For a parser, strip `&` and split on `\\` to get individual expressions:

```latex
\begin{align}
  f(x)  &= x^3 - 2x^2 + x - 5 \\
  f'(x) &= 3x^2 - 4x + 1 \\
  f''(x) &= 6x - 4
\end{align}
```

```latex
\begin{align*}
  (a + b)^2 &= a^2 + 2ab + b^2 \\
  (a - b)^2 &= a^2 - 2ab + b^2
\end{align*}
```

---

### Label and Reference — Handle or Strip

Common in academic documents. Either resolve the reference or strip both:

```latex
\begin{equation}
  E = mc^2
  \label{eq:energy}
\end{equation}

As shown in \eqref{eq:energy}, energy scales with $c^2$.
```

---

### Summary: Minimal Parser Target

| Syntax | Type | Lines |
|---|---|---|
| `$...$` | inline | single |
| `\(...\)` | inline | single |
| `\[...\]` | block | single or multi |
| `\begin{equation}` | block, numbered | single |
| `\begin{equation*}` | block, unnumbered | single |
| `\begin{align}` | block, numbered | multi |
| `\begin{align*}` | block, unnumbered | multi |

Everything else (`gather`, `multline`, `split`, `cases`, `array`) is either rare in algebra contexts or reducible to the above. `cases` is the one exception worth adding if you need piecewise functions:

```latex
f(x) = \begin{cases}
  x^2      & x \geq 0 \\
  -x       & x < 0
\end{cases}
```

For `align` and `cases`, your parse strategy is the same: **split on `\\`**, then **split each line on `&`** to separate left-hand side from right-hand side or condition.

### `\[...\]` Single Line

```latex
\[ x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a} \]
```

```latex
\[ \sum_{n=1}^{\infty} \frac{1}{n^2} = \frac{\pi^2}{6} \]
```

---

### `\[...\]` Multi-line

No special environment needed — just use `\\` to break lines. The content is still one logical block:

```latex
\[
  f(x) = x^3 - 2x^2 + x - 5 \\
  f'(x) = 3x^2 - 4x + 1 \\
  f''(x) = 6x - 4
\]
```

With alignment via `&` (requires `amsmath`):

```latex
\[
  f(x)   &= x^3 - 2x^2 + x - 5 \\
  f'(x)  &= 3x^2 - 4x + 1 \\
  f''(x) &= 6x - 4
\]
```

> **Parser note:** `\[...\]` is a single block delimiter — extract everything between `\[` and `\]`, then split on `\\` to get individual lines, and split each line on `&` to separate LHS from RHS.

## ANTLR4 Grammar Rule for Custom Commands

The pattern is simple and uniform across all your examples:

```
\commandName { arg1 } { arg2 } ... { argN }
```

---

### Lexer Rules

```antlr
BACKSLASH       : '\\' ;
LBRACE          : '{' ;
RBRACE          : '}' ;
COMMAND_NAME    : '\\' [a-zA-Z]+ ;
```

---

### Parser Rules

```antlr
// Top-level command invocation
customCommand
    : COMMAND_NAME commandArgs
    ;

// Zero or more brace-delimited arguments
commandArgs
    : commandArg*
    ;

commandArg
    : LBRACE expr RBRACE
    ;
```

That's it. Every one of your examples fits this single rule:

```
\solve    { x^2 - 5x + 6 = 0 }     →  COMMAND_NAME  commandArg
\factor   { x^2 - 5x + 6 }         →  COMMAND_NAME  commandArg
\diff     { x^3 + 2x } { x }       →  COMMAND_NAME  commandArg  commandArg
\integrate{ x^2 } { 0 } { 1 }      →  COMMAND_NAME  commandArg  commandArg  commandArg
```

---

### In Context of a Broader Expression Grammar

```antlr
expr
    : expr ('+' | '-') expr         # addSub
    | expr ('*' | '/' | '^') expr   # mulDivPow
    | LPAREN expr RPAREN            # parenExpr
    | customCommand                 # commandExpr
    | NUMBER                        # number
    | VARIABLE                      # variable
    ;

customCommand
    : COMMAND_NAME commandArg*
    ;

commandArg
    : LBRACE expr RBRACE
    ;
```

---

### Key Design Point

The argument count is **not encoded in the grammar** — `commandArg*` accepts any number. You validate arity separately in your visitor/listener after parsing:

```rust
// In your visitor
fn visit_custom_command(name: &str, args: Vec<Expr>) -> Result<Expr> {
    match name {
        "\\solve"     => expect_args(args, 1),
        "\\factor"    => expect_args(args, 1),
        "\\diff"      => expect_args(args, 2),
        "\\integrate" => expect_args(args, 3),
        _ => Err(UnknownCommand(name))
    }
}
```

This keeps the grammar **open** — adding a new command like `\limit` requires zero grammar changes, only a new match arm in the visitor.
