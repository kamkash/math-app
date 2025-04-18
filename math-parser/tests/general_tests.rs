use log::info;
use antlr_rust::tree::{ParseTreeVisitorCompat, Tree};
use antlr_rust::InputStream;
use antlr_rust::{common_token_stream::CommonTokenStream, tree::ParseTree};

use math_parser::gen_calc_parser::calculatorlexer::calculatorLexer;
use math_parser::gen_calc_parser::calculatorparser::calculatorParser;

use math_parser::gen_calc_parser::calculatorparser::{
    calculatorParserContextType, AtomContext, BlockContext, ConstantContext, EquationContext,
    ExpressionContext, Func_Context, FuncnameContext,
    FunctionDefinitionContext, MultiplyingExpressionContext, PowExpressionContext, RelopContext,
    ScientificContext, SignedAtomContext, VariableContext,
};
use math_parser::gen_calc_parser::calculatorvisitor::calculatorVisitorCompat;

#[test]
fn test_calculator_parser() {
    struct CalcVisitor(isize);

    impl ParseTreeVisitorCompat<'_> for CalcVisitor {
        type Node = calculatorParserContextType;
        type Return = isize;

        fn temp_result(&mut self) -> &mut Self::Return {
            &mut self.0
        }

        fn aggregate_results(&self, _aggregate: Self::Return, _next: Self::Return) -> Self::Return {
            // Custom logic for aggregating results
            self.0
        }
    }

    impl<'input> calculatorVisitorCompat<'input> for CalcVisitor {
        fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
            dbg!(ctx.get_text());
            self.visit_children(ctx)
        }

        fn visit_functionDefinition(
            &mut self,
            ctx: &FunctionDefinitionContext<'input>,
        ) -> Self::Return {
            dbg!(ctx.get_text());
            self.visit_children(ctx)
        }

        fn visit_equation(&mut self, ctx: &EquationContext<'input>) -> Self::Return {
            dbg!(ctx.get_text());
            for child in ctx.get_children() {
                if child.get_child_count() > 0 {
                    for grand_child in child.get_children() {
                        if grand_child.get_child_count() > 0 {
                            for grand_grand_child in grand_child.get_children() {
                                if grand_grand_child.get_child_count() > 0 {
                                    for grand_grand_grand_child in grand_grand_child.get_children()
                                    {
                                        dbg!(grand_grand_grand_child.get_text());
                                    }
                                } else {
                                    dbg!(grand_grand_child.get_text());
                                }
                            }
                        } else {
                            dbg!(grand_child.get_text());
                        }
                    }
                } else {
                    dbg!(child.get_text());
                }
            }
            self.visit_children(ctx)
        }

        fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
            self.visit_children(ctx)
        }

        fn visit_multiplyingExpression(
            &mut self,
            ctx: &MultiplyingExpressionContext<'input>,
        ) -> Self::Return {
            self.visit_children(ctx)
        }
        fn visit_powExpression(&mut self, ctx: &PowExpressionContext<'input>) -> Self::Return {
            self.visit_children(ctx)
        }

        fn visit_signedAtom(&mut self, ctx: &SignedAtomContext<'input>) -> Self::Return {
            self.visit_children(ctx)
        }

        fn visit_atom(&mut self, ctx: &AtomContext<'input>) -> Self::Return {
            self.visit_children(ctx)
        }

        fn visit_scientific(&mut self, ctx: &ScientificContext<'input>) -> Self::Return {
            self.visit_children(ctx)
        }

        fn visit_constant(&mut self, ctx: &ConstantContext<'input>) -> Self::Return {
            self.visit_children(ctx)
        }

        fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
            self.visit_children(ctx)
        }

        fn visit_func_(&mut self, ctx: &Func_Context<'input>) -> Self::Return {
            self.visit_children(ctx)
        }

        fn visit_funcname(&mut self, ctx: &FuncnameContext<'input>) -> Self::Return {
            self.visit_children(ctx)
        }

        fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
            self.visit_children(ctx)
        }
    }

    let input = r#"10 + 21 = ans,
                            a=cos(y)^2 + sin(x)^2 , 
                            b=5000.0 , c=2 ,
                            d=$1000.00,
                            d1=$100,000.00,
                            e = c*x^3 - 1/y + k*exp(-1/t),
                            p=3.1415926,
                            f = $30,000 * (1.0 + 0.05)^n,
                            g=$100,000,
                            f(x)=x^2+2*x-1,
                            g(x)=a*exp(-t/(k*x)),
                            f(x,y)=__ans__,
                            f(3,4,9) = 3.1415;
                            f(2.86) = _a_
                            "#;
    let mut visitor = CalcVisitor(0);

    info!("Input: {}", input);
    let lexer = calculatorLexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = calculatorParser::new(token_stream);
    let parse_tree = parser.block().unwrap();
    let result = visitor.visit(parse_tree.as_ref());
    info!("Result: {}", result);
    info!("Parsed result: {:?}", parse_tree);
}
