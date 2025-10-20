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
https://github.com/rrevenantt/antlr4rust


## Rust parsers
## LALRPOP
https://lalrpop.github.io/lalrpop/
https://github.com/lalrpop/lalrpop