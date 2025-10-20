use crate::asciimath_gen_interpreter::SymEquationGen;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree};
use antlr_rust::InputStream;
use giac_rs::context::Context;
use giac_rs::gen::Gen;
use giac_rs::gen::{GEN_ADD, GEN_DIV, GEN_MUL, GEN_POW, GEN_SUB};
use math_core::common::LogicalOperator;
use math_parser::gen_parsers::latexlexer::LaTeXLexer;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use log::{error, info};
// Import the generated parser context type
use math_parser::gen_parsers::latexparser::{
    AdditiveContext, AtomVariableContext, BlockContext, EqualityContext, ExpContext, ExprContext,
    FracContext, LaTeXParser, LaTeXParserContextType, MathContext, MpContext, MultopContext,
    PowopContext, RelationContext, RelopContext, SumopContext,
};
use math_parser::gen_parsers::latexvisitor::LaTeXVisitorCompat;

pub fn eval_symbol_to_f64(visitor: &mut LaTeXGenVisitor, var_name: &str) -> f64 {
    let actual_sym = visitor
        .result_table
        .get(&Gen::symbol(var_name, &visitor.giac_context).unwrap())
        .unwrap_or_else(|| panic!("Variable {} not found in result_table", var_name));

    // Try direct conversion
    if let Some(v) = actual_sym.to_f64() {
        return v;
    }

    // Try evaluating stored gen
    if let Some(evaled) = actual_sym.eval() {
        if let Some(v) = evaled.to_f64() {
            return v;
        }
    }

    // Try simplifying then evaluating
    if let Some(simp) = actual_sym.simplify() {
        if let Some(evaled) = simp.eval() {
            if let Some(v) = evaled.to_f64() {
                return v;
            }
        }
    }

    // Fallback: assign the expression to the symbol in the context and read it back
    let sym = Gen::symbol(var_name, &visitor.giac_context).unwrap();
    let expr_str = visitor
        .symbol_table
        .get(&sym)
        .map(|g| g.to_string())
        .unwrap_or_else(|| panic!("No expression found for {}", var_name));
    let assign = format!("{} := {}", var_name, expr_str);
    Gen::new(assign.as_str(), &visitor.giac_context)
        .unwrap()
        .eval();
    let read = Gen::new(var_name, &visitor.giac_context).unwrap().eval();
    if let Some(r) = read {
        if let Some(v) = r.to_f64() {
            return v;
        }
    }

    // Some GIAC evaluations produce singleton-list string forms like "[10.0]".
    // Try to parse a numeric value out of a singleton list before failing.
    let s = actual_sym.to_string();
    if s.starts_with('[') && s.ends_with(']') {
        let inner = s.trim_start_matches('[').trim_end_matches(']').trim();
        if let Ok(v) = inner.parse::<f64>() {
            return v;
        }
    }

    panic!("Variable {} did not evaluate to f64, got: {}", var_name, s);
}

pub struct LaTeXGenVisitor {
    pub tmp_result: Rc<Gen>,
    pub visitor_stack: Vec<Rc<Gen>>,
    pub block_expressions: Vec<SymEquationGen>,
    pub giac_context: Rc<Context>,
    pub symbol_table: HashMap<Rc<Gen>, Rc<Gen>>,
    pub result_table: HashMap<Rc<Gen>, Rc<Gen>>,
}

impl LaTeXGenVisitor {
    pub fn new() -> Self {
        let ctx = Rc::new(Context::new());
        Self {
            tmp_result: Rc::new(Gen::new("0", &ctx).unwrap()),
            visitor_stack: Vec::new(),
            block_expressions: Vec::new(),
            giac_context: ctx,
            symbol_table: HashMap::new(),
            result_table: HashMap::new(),
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

impl<'input> ParseTreeVisitorCompat<'input> for LaTeXGenVisitor {
    type Node = LaTeXParserContextType;
    type Return = Rc<Gen>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.tmp_result
    }

    fn aggregate_results(&self, _aggregate: Self::Return, next: Self::Return) -> Self::Return {
        next
    }

    fn visit_terminal(&mut self, _node: &TerminalNode<'_, Self::Node>) -> Self::Return {
        Rc::new(Gen::new("0", &self.giac_context).unwrap())
    }
}

impl<'input> LaTeXVisitorCompat<'input> for LaTeXGenVisitor {
    fn visit_block(&mut self, ctx: &BlockContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        self.build_symbol_table();
        self.build_result_table();
        res
    }

    fn visit_math(&mut self, ctx: &MathContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        res
    }

    fn visit_relation(&mut self, ctx: &RelationContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        // dbg!(ctx.get_text());
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len >= 3 && stack_len >= 3 {
            let right = self.visitor_stack.pop().unwrap();
            let op = self.visitor_stack.pop().unwrap();
            let left = self.visitor_stack.pop().unwrap();
            let equation = SymEquationGen::new(left, right, op);
            self.block_expressions.push(equation);
        }
        res
    }

    fn visit_equality(&mut self, ctx: &EqualityContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_expr(&mut self, ctx: &ExprContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    fn visit_additive(&mut self, ctx: &AdditiveContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 && stack_len >= len {
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
                    error!("Invalid operator in additive: {}", op.to_string());
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_mp(&mut self, ctx: &MpContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        // dbg!(ctx.get_text());
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 && stack_len >= len {
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
            let mut left: Rc<Gen> = Rc::new(Gen::new("0", &self.giac_context).unwrap());
            let mut right: Rc<Gen>;
            let mut op: Rc<Gen>;
            while mult_stack.len() > 0 {
                if left.to_string() == "0" {
                    left = Rc::clone(&mult_stack.pop_back().unwrap());
                    op = Rc::clone(&mult_stack.pop_back().unwrap());
                    right = Rc::clone(&mult_stack.pop_back().unwrap());
                } else {
                    op = Rc::clone(&mult_stack.pop_back().unwrap());
                    right = Rc::clone(&mult_stack.pop_back().unwrap());
                }
                if op.is_mul() {
                    left = Rc::new(left.mul(&right).unwrap());
                } else if op.is_div() {
                    left = Rc::new(left.div(&right).unwrap());
                } else {
                    error!("Unexpected operator in mp: {}", op.to_string());
                }
            }
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_frac(&mut self, ctx: &FracContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        dbg!(ctx.get_text());
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 && stack_len >= 2 {
            let right_item = self.visitor_stack.pop().unwrap();
            dbg!(right_item.to_string());
            let left_item = self.visitor_stack.pop().unwrap();
            dbg!(left_item.to_string());
            let frac = Rc::new(left_item.div(&right_item).unwrap());
            self.visitor_stack.push(frac);
        }
        res
    }

    fn visit_exp(&mut self, ctx: &ExpContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let len = ctx.get_child_count();
        let stack_len = self.visitor_stack.len();
        if len > 1 && stack_len >= 3 {
            let right = self.visitor_stack.pop().unwrap();
            let op_gen = self.visitor_stack.pop().unwrap();
            let mut left = self.visitor_stack.pop().unwrap();
            // info!("left {}, op {}, right {}", left.to_string(), op_gen.to_string(), right.to_string());
            assert!(op_gen.is_pow());
            left = Rc::new(left.symb_pow(&right).unwrap());
            self.visitor_stack.push(left);
        }
        res
    }

    fn visit_fn_sqrt(
        &mut self,
        ctx: &math_parser::gen_parsers::latexparser::Fn_sqrtContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);

        // Get the base expression (sqrbase)
        let base = self.visitor_stack.pop().unwrap();

        // Check if a root parameter is specified
        if let Some(_root_expr) = &ctx.root {
            // If we have a root parameter, we need to pop it from the stack too
            // The root parameter is the second-to-last item on the stack
            if self.visitor_stack.len() > 0 {
                let root = self.visitor_stack.pop().unwrap();
                let nth_root = Rc::new(base.symb_root(root.to_f64().unwrap()).unwrap());
                self.visitor_stack.push(nth_root);
            } else {
                // Fallback to square root if something went wrong
                let sqrt = Rc::new(base.symb_sqrt().unwrap());
                self.visitor_stack.push(sqrt);
            }
        } else {
            // No root parameter, just calculate the square root
            let sqrt = Rc::new(base.symb_sqrt().unwrap());
            self.visitor_stack.push(sqrt);
        }
        res
    }

    fn visit_func_normal(
        &mut self,
        ctx: &math_parser::gen_parsers::latexparser::Func_normalContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let func_name = ctx.get_text();
        let func_name_text = format!("__{}__", func_name.replace("\\", ""));
        let f = Rc::new(Gen::symbol(&func_name_text, &self.giac_context).unwrap());
        info!("func_name : {} {} ", func_name, f);
        self.visitor_stack.push(f);
        res
    }

    fn visit_fn_normal(
        &mut self,
        ctx: &math_parser::gen_parsers::latexparser::Fn_normalContext<'input>,
    ) -> Self::Return {
        // Record stack length before visiting children so we can determine
        // how many Gen values the children pushed (these represent args,
        // including any sub/superscript expressions which the user wants
        // treated as additional args).
        let before = self.visitor_stack.len();
        info!("stack {:?} ", self.visitor_stack);
        let res = self.visit_children(ctx);
        info!("stack {:?} ", self.visitor_stack);
        let after = self.visitor_stack.len();
        let mut pushed = after.saturating_sub(before);

        if pushed > 0 {
            info!("stack {:?} ", self.visitor_stack);
            let func_name = self.visitor_stack.remove(before);
            // Remove single quotes, commas, and backslashes from function name
            let func_name_text = func_name.to_string();
            info!("func_name : {} {}", func_name_text, func_name);
            pushed -= 1;
            info!("stack {:?} ", self.visitor_stack);
            // Pop the pushed items (they're in left-to-right visit order, but the
            // stack's top is the last pushed; collect and reverse to restore
            // left-to-right argument order).
            let mut args_rc: Vec<Rc<Gen>> = Vec::with_capacity(pushed);
            for _ in 0..pushed {
                args_rc.push(self.visitor_stack.pop().unwrap());
            }
            args_rc.reverse();

            // Prepare a slice of &Gen for the FFI helper
            let arg_refs: Vec<&Gen> = args_rc.iter().map(|rc| rc.as_ref()).collect();

            match Gen::function_call(&func_name_text, &arg_refs, &self.giac_context) {
                Some(g) => {
                    let rc = Rc::new(g);
                    self.visitor_stack.push(Rc::clone(&rc));
                    info!("func_name (call): {} {}", func_name_text, rc);
                }
                None => {
                    // If constructing the function failed, push back the original
                    // args followed by the function symbol (preserve stack state
                    // somewhat predictably).
                    for a in args_rc.into_iter() {
                        self.visitor_stack.push(a);
                    }
                    let sym = Rc::new(Gen::symbol(&func_name_text, &self.giac_context).unwrap());
                    self.visitor_stack.push(Rc::clone(&sym));
                    info!("func_name (fallback symbol): {} {}", func_name_text, sym);
                }
            }
        }
        res
    }

    fn visit_atomVarSym(
        &mut self,
        ctx: &math_parser::gen_parsers::latexparser::AtomVarSymContext<'input>,
    ) -> Self::Return {
        let res = self.visit_children(ctx);
        let var_text = ctx.get_text();
        let filtered = var_text.replace("\\", "");
        let var_symbol = Rc::new(Gen::symbol(&filtered, &self.giac_context).unwrap());
        self.visitor_stack.push(var_symbol);
        res
    }

    fn visit_atomVariable(&mut self, ctx: &AtomVariableContext<'input>) -> Self::Return {
        let res = self.visit_children(ctx);
        let var_text = ctx.get_text();
        let filtered = var_text.replace("\\", "");
        let var_symbol = Rc::new(Gen::symbol(&filtered, &self.giac_context).unwrap());
        self.visitor_stack.push(var_symbol);
        res
    }

    fn visit_atomNumber(
        &mut self,
        ctx: &math_parser::gen_parsers::latexparser::AtomNumberContext<'input>,
    ) -> Self::Return {
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
        if op_text == "*" || op_text == "\\times" || op_text == "\\cdot" {
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
}

pub fn evaluate_latex_block(input: &str) -> Result<String, String> {
    info!("evaluate_latex_block: {}", input);
    let mut visitor = LaTeXGenVisitor::new();
    let lexer = LaTeXLexer::new(InputStream::new(input));
    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = LaTeXParser::new(token_stream);
    let result = parser.block();
    match result {
        Ok(context) => {
            let _ = visitor.visit(&*context);
            let result = format!("{:?}", visitor.result_table);
            Ok(result)
        }
        Err(e) => Err(format!("parser error {}", e).to_string()),
    }
}
