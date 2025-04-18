use antlr_rust::tree::ParseTree;
use antlr_rust::tree::{ParseTreeVisitorCompat, Tree};

use crate::gen_calc_parser::calculatorparser::{
    calculatorParserContextType, AtomContext, BlockContext, ConstantContext, EquationContext,
    ExpressionContext, Func_Context, FuncnameContext, FunctionDefinitionContext,
    MultiplyingExpressionContext, PowExpressionContext, RelopContext, ScientificContext,
    SignedAtomContext, VariableContext,
};
use crate::gen_calc_parser::calculatorvisitor::calculatorVisitorCompat;

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
        self.visit_children(ctx)
    }

    fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) -> Self::Return {
        dbg!(ctx.get_text());
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
