## parser, tokenizer
```bash
java -cp ./antlr-4.13.2-complete.jar org.antlr.v4.Tool -visitor calculator.g4
```
## testrig
```bash
export CLASSPATH=`pwd`/antlr-4.13.2-complete.jar:$CLASSPATH
alias grun='java org.antlr.v4.gui.TestRig'

java -cp ./antlr-4.13.2-complete.jar org.antlr.v4.Tool -visitor calculator.g4
javac *.java
grun calculator block -tree -gui block1.txt
```
