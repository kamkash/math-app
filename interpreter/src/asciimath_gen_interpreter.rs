use std::collections::VecDeque;
use std::rc::Rc;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};
use antlr_rust::InputStream;
use asciimath2lexer::AsciiMath2Lexer;
use giac_rs::context::Context;
use giac_rs::gen::{Gen, GEN_DIV, GEN_MUL, GEN_POW};
use giac_rs::gen::{GEN_ADD, GEN_SUB};
use log::{error, info};
use math_core::common::LogicalOperator;
use math_parser::gen_parsers::asciimath2lexer;
use math_parser::gen_parsers::asciimath2parser::{
    AsciiMath2Parser, AsciiMath2ParserContextType, ExplicitKeywordCallContext,
    ExplicitKeywordCallContextAttrs, IdentifierAtomContext, IntegralExpressionContext,
    MultopContext, NumberAtomContext, Power_expressionContext, PowopContext, RelopContext,
    Scripted_op_expressionContext, SumopContext,
};
use math_parser::gen_parsers::asciimath2visitor::AsciiMath2VisitorCompat;

macro_rules! filter_optional_children_texts {
    ($ctx:expr) => {
        $ctx.get_children()
            .filter(|child| {
                let child_text = child.get_text();
                child_text != "(" && child_text != ")"
            })
            .map(|child| child.get_text())
            .collect::<Vec<_>>()
    };
}

pub struct SymEquationGen {
    pub left: Rc<Gen>,
    pub right: Rc<Gen>,
    pub op: Rc<Gen>,
}

impl SymEquationGen {
    pub fn new(left: Rc<Gen>, right: Rc<Gen>, op: Rc<Gen>) -> Self {
        SymEquationGen { left, right, op }
    }
}

impl std::fmt::Debug for SymEquationGen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.left, self.op, self.right)
    }
}

impl std::fmt::Display for SymEquationGen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.op, self.right)
    }
}

pub struct AsciiMathGenVisitor {
    pub tmp_result: Rc<Gen>,
    pub block_expressions: Vec<SymEquationGen>,
    pub symbol_table: std::collections::HashMap<Rc<Gen>, Rc<Gen>>, // Use String for symbol names
    pub result_table: std::collections::HashMap<Rc<Gen>, Rc<Gen>>, // Use String for symbol names
    pub visitor_stack: Vec<Rc<Gen>>,
    pub giac_context: Rc<Context>,
}

impl AsciiMathGenVisitor {
    pub fn new() -> Self {
        let ctx = Rc::new(Context::new());
        AsciiMathGenVisitor {
            tmp_result: Rc::new(Gen::new("0", &ctx).unwrap()), // Initialize with a default Gen
            block_expressions: Vec::new(),
            symbol_table: std::collections::HashMap::new(),
            result_table: std::collections::HashMap::new(),
            visitor_stack: Vec::new(),
            giac_context: ctx,
        }
    }

    fn build_symbol_table(&mut self) {
        self.symbol_table
            .extend(self.block_expressions.iter().filter_map(|sym_eq| {
                if sym_eq.left.is_symbol() {
                    Some((Rc::clone(&sym_eq.left), Rc::clone(&sym_eq.right)))
                } else if sym_eq.right.is_symbol() {
                    Some((Rc::clone(&sym_eq.right), Rc::clone(&sym_eq.left)))
                } else {
                    None
                }
            }));
        info!("symbol table: {:?}", self.symbol_table);
    }

    fn build_result_table(&mut self) {
        // evaluate and store assignables in giac context
        self.symbol_table
            .iter()
            .filter(|(_sym, expr)| expr.is_number())
            .for_each(|(sym, expr)| {
                let e = format!("{} := {}", sym, expr);
                Gen::new(e.as_str(), &self.giac_context).unwrap().eval();
            });

        self.result_table = self
            .symbol_table
            .iter()
            .map(|(sym, expr)| {
                if expr.is_number() {
                    (Rc::clone(sym), Rc::clone(expr))
                } else {
                    let value = expr.eval().unwrap();
                    (Rc::clone(sym), Rc::new(value))
                }
            })
            .collect();
        info!("result table: {:?}", self.result_table);
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for AsciiMathGenVisitor {
    type Node = AsciiMath2ParserContextType;
    type Return = Rc<Gen>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.tmp_result
    }

    fn aggregate_results(&self, _aggregate: Self::Return, next: Self::Return) -> Self::Return {
        next
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        Rc::new(Gen::new("0", &self.giac_context).unwrap()) // Default Gen
    }
}

impl<'input> AsciiMath2VisitorCompat<'input> for AsciiMathGenVisitor {
    fn visit_block(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::BlockContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        self.build_symbol_table();
        self.build_result_table();
        res
    }

    fn visit_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::ExpressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = self.visitor_stack.len();
        if len >= 3 {
            let right = self.visitor_stack.pop().unwrap();
            let op = self.visitor_stack.pop().unwrap(); // Operator is now a string representation of Gen
            let left = self.visitor_stack.pop().unwrap();
            let equation = SymEquationGen::new(left, right, op);
            self.block_expressions.push(equation);
        }
        res
    }

    fn visit_relation_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Relation_expressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_relation_expression_no_rhs(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Relation_expression_no_rhsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_add_sub_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Add_sub_expressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 {
            let remove_at = stack_len - len;
            let mut left = self.visitor_stack.remove(remove_at);
            for _ in 0..(len - 1) / 2 {
                let op = self.visitor_stack.remove(remove_at);
                if op.is_add() {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.add(&right).unwrap());
                } else if op.is_sub() {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.sub(&right).unwrap());
                } else {
                    error!("Invalid operator in add_sub_expression: {}", op.to_string());
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_mult_div_expression(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::Mult_div_expressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 {
            let remove_at = stack_len - len;
            let mut mult_stack: VecDeque<Rc<Gen>> = VecDeque::new();
            for index in 0..len {
                let left_item = self.visitor_stack.remove(remove_at);
                if index % 2 == 0 {
                    mult_stack.push_front(left_item);
                } else {
                    if left_item.is_mul() || left_item.is_div() {
                        mult_stack.push_front(left_item);
                    } else {
                        mult_stack.push_front(Rc::new(GEN_MUL.clone()));
                        mult_stack.push_front(left_item);
                    }
                }
            }
            let mut left: Rc<Gen> = Rc::new(Gen::new("0", &self.giac_context).unwrap()); // Initialize with a default Gen
            let mut right: Rc<Gen>;
            let mut op: Rc<Gen>;
            while mult_stack.len() > 0 {
                if left.to_string() == "0" {
                    // Check for default Gen
                    left = Rc::clone(&mult_stack.pop_back().unwrap());
                    op = Rc::clone(&mult_stack.pop_back().unwrap());
                    right = Rc::clone(&mult_stack.pop_back().unwrap());
                } else {
                    op = Rc::clone(&mult_stack.pop_back().unwrap());
                    right = Rc::clone(&mult_stack.pop_back().unwrap());
                }
                if op.is_mul() {
                    left = Rc::new(left.symb_mult(&right).unwrap());
                } else if op.is_div() {
                    left = Rc::new(left.div(&right).unwrap());
                } else {
                    error!(
                        "Unexpected operator in mult_div_expression: {}",
                        op.to_string()
                    );
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_power_expression(&mut self, ctx: &Power_expressionContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 {
            let remove_at = stack_len - len;
            let mut left = self.visitor_stack.remove(remove_at);
            for _ in 0..(len - 1) / 2 {
                let op_gen = self.visitor_stack.remove(remove_at);
                if op_gen.is_pow() {
                    let right = self.visitor_stack.remove(remove_at);
                    left = Rc::new(left.symb_pow(&right).unwrap());
                } else {
                    info!(
                        "Unexpected operator in power_expression: {}",
                        op_gen.to_string()
                    );
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_identifierAtom(&mut self, ctx: &IdentifierAtomContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let var_text = ctx.get_text();
        let var_symbol = Rc::new(Gen::symbol(&var_text, &self.giac_context).unwrap());
        self.visitor_stack.push(var_symbol);
        res
    }

    fn visit_numberAtom(&mut self, ctx: &NumberAtomContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let sci_text = ctx.get_text();
        let filtered: String = sci_text
            .chars()
            .filter(|c| c.is_numeric() || *c == '.' || *c == '-')
            .collect();
        let value: f64 = filtered.parse().unwrap_or(0.0);
        let result = Rc::new(Gen::from_f64(value, &self.giac_context).unwrap());
        self.visitor_stack.push(result);
        res
    }

    fn visit_relop(&mut self, ctx: &RelopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        let rel_op = LogicalOperator::from_str_token(&op_text);
        let gen_op = Gen::logical_op(rel_op.unwrap_or(LogicalOperator::Eq));
        self.visitor_stack.push(Rc::new(gen_op.unwrap()));
        res
    }

    fn visit_sumop(&mut self, ctx: &SumopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "+" {
            self.visitor_stack.push(Rc::new(GEN_ADD.clone()));
        } else {
            self.visitor_stack.push(Rc::new(GEN_SUB.clone()));
        }
        res
    }

    fn visit_multop(&mut self, ctx: &MultopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "*" {
            self.visitor_stack.push(Rc::new(GEN_MUL.clone()));
        } else {
            self.visitor_stack.push(Rc::new(GEN_DIV.clone()));
        }
        res
    }

    fn visit_powop(&mut self, ctx: &PowopContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let op_text = ctx.get_text();
        if op_text == "^" || op_text == "**" {
            self.visitor_stack.push(Rc::new(GEN_POW.clone()));
        }
        res
    }

    fn visit_scripted_op_expression(
        &mut self,
        ctx: &Scripted_op_expressionContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let children: Vec<String> = filter_optional_children_texts!(ctx);

        // For Gen, we need to construct the function call as a string or use specific Gen functions if available.
        // This is a placeholder and needs proper mapping from AsciiMath functions to Giac functions.
        if !children.is_empty() {
            let func_name = &children[0];
            let arg = self.visitor_stack.pop().unwrap(); // Assuming one argument for now

            // This part is highly dependent on how Giac handles functions.
            // For now, we'll just create a string representation of the function call.
            let func_call_str = format!("{}({})", func_name, arg.to_string());
            let sym_func = Rc::new(Gen::new(&func_call_str, &self.giac_context).unwrap());
            self.visitor_stack.push(sym_func);
        }
        res
    }

    fn visit_explicitKeywordCall(
        &mut self,
        ctx: &ExplicitKeywordCallContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);

        let children: Vec<String> = filter_optional_children_texts!(ctx);
        let mut args = Vec::new();
        // Pop arguments from the stack in reverse order of appearance in children
        for _i in 1..children.len() {
            if let Some(arg_gen) = self.visitor_stack.pop() {
                args.push(arg_gen);
            }
        }
        args.reverse(); // Re-reverse to get them in correct order

        let func_name = if ctx.scripted_op_expression().is_some() {
            let func = self.visitor_stack.pop().unwrap();
            func.to_string() // Get the function name as a string
        } else {
            ctx.get_child(0).unwrap().get_text()
        };

        // Construct the function call string for Gen
        let args_str: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
        let func_call_str = format!("{}({})", func_name, args_str.join(", "));
        let func_gen = Rc::new(Gen::new(&func_call_str, &self.giac_context).unwrap());
        self.visitor_stack.push(func_gen);
        res
    }

    fn visit_integralExpression(
        &mut self,
        ctx: &IntegralExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_constantAtom(
        &mut self,
        ctx: &math_parser::gen_parsers::asciimath2parser::ConstantAtomContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let constant_text = ctx.get_text();
        let constant_value = match constant_text.as_str() {
            "pi" => Rc::new(Gen::pi(&self.giac_context).unwrap()),
            "e" => Rc::new(Gen::e(&self.giac_context).unwrap()),
            _ => Rc::new(Gen::new(&constant_text, &self.giac_context).unwrap()), // Fallback to symbol
        };
        self.visitor_stack.push(constant_value);
        res
    }
}

pub fn evaluate_ascii_math_block_gen(input: &str) -> Result<String, String> {
    info!("evaluate_ascii_math_block_gen: {}", input);
    let mut visitor = AsciiMathGenVisitor::new();
    let lexer = AsciiMath2Lexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = AsciiMath2Parser::new(token_stream);
    let result = parser.block();
    match result {
        Ok(context) => {
            let _ = visitor.visit(&*context);
            // Convert result_table keys (String) and values (Rc<Gen>) to a displayable format.
            let formatted_results: Vec<String> = visitor
                .result_table
                .iter()
                .map(|(key, value)| format!("{}: {}", key, value.to_string()))
                .collect();
            Ok(format!("{{ {} }}", formatted_results.join(", ")))
        }
        Err(e) => Err(format!("parser error {}", e).to_string()),
    }
}
