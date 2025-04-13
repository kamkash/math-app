#![allow(unused)]

mod r#mod;
mod gen {
    use log::info;
    mod calculatorlexer;
    mod calculatorlistener;
    mod calculatorparser;
    mod calculatorvisitor;

    use crate::gen::calculatorlexer::calculatorLexer;
    use crate::gen::calculatorparser::calculatorParser;
    use antlr_rust::tree::ParseTreeVisitorCompat;
    use antlr_rust::InputStream;
    use antlr_rust::{common_token_stream::CommonTokenStream, tree::ParseTree};
    use calculatorparser::{
        calculatorParserContextType, AtomContext, ConstantContext, EquationContext,
        EquationContextAttrs, ExpressionContext, Func_Context, FuncnameContext,
        MultiplyingExpressionContext, PowExpressionContext, RelopContext, ScientificContext,
        SignedAtomContext, VariableContext,
    };
    use calculatorvisitor::calculatorVisitorCompat;

    #[test]
    fn test_calculator_parser() {
        struct CalcVisitor(isize);

        impl ParseTreeVisitorCompat<'_> for CalcVisitor {
            type Node = calculatorParserContextType;
            type Return = isize;

            fn temp_result(&mut self) -> &mut Self::Return {
                &mut self.0
            }

            fn aggregate_results(
                &self,
                _aggregate: Self::Return,
                _next: Self::Return,
            ) -> Self::Return {
                // Custom logic for aggregating results
                self.0
            }
        }

        impl<'input> calculatorVisitorCompat<'input> for CalcVisitor {
            fn visit_block(
                &mut self,
                ctx: &calculatorparser::BlockContext<'input>,
            ) -> Self::Return {
                dbg!(ctx.get_text());
                self.visit_children(ctx)
            }

            fn visit_equation(&mut self, ctx: &EquationContext<'input>) -> Self::Return {
                // dbg!(ctx.expression_all());
                // if let Some(relop_ctx) = ctx.relop() {
                //     dbg!(relop_ctx.get_text());
                // } else {
                //     dbg!("No relop found");
                // }
                dbg!(ctx.get_text());
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

        let input = r#"
                            a=cos(3), 
                            b=5000.0, 
                            c=2,
                            d=$1000.00,
                            p=3.1415926"#;
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
}
