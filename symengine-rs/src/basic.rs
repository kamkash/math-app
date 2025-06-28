use crate::symengine_ffi::*;
use lazy_static::lazy_static;
use log::info;
use regex::Regex;
use std::ffi::{CStr, CString};
use std::fmt;
use std::rc::Rc;

lazy_static! {
    pub static ref RE_LOG_BASE: Regex = Regex::new(r"^log_(\d+)$").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinFunction {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Csc,
    Sec,
    Exp,
    Log,
    Logb,
    Ln, // natural logarithm
    Sqrt,
}
impl BuiltinFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            BuiltinFunction::Sin => "sin",
            BuiltinFunction::Cos => "cos",
            BuiltinFunction::Tan => "tan",
            BuiltinFunction::Asin => "asin",
            BuiltinFunction::Acos => "acos",
            BuiltinFunction::Atan => "atan",
            BuiltinFunction::Csc => "csc",
            BuiltinFunction::Sec => "sec",
            BuiltinFunction::Exp => "exp",
            BuiltinFunction::Log => "log",
            BuiltinFunction::Ln => "ln",
            BuiltinFunction::Sqrt => "sqrt",
            BuiltinFunction::Logb => "logb",
        }
    }
}
impl BuiltinFunction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "sin" => Some(BuiltinFunction::Sin),
            "cos" => Some(BuiltinFunction::Cos),
            "tan" => Some(BuiltinFunction::Tan),
            "asin" => Some(BuiltinFunction::Asin),
            "acos" => Some(BuiltinFunction::Acos),
            "atan" => Some(BuiltinFunction::Atan),
            "csc" => Some(BuiltinFunction::Csc),
            "sec" => Some(BuiltinFunction::Sec),
            "exp" => Some(BuiltinFunction::Exp),
            "log" => Some(BuiltinFunction::Log),
            s if s.contains("log_") => Some(BuiltinFunction::Logb),
            "ln" => Some(BuiltinFunction::Ln),
            "sqrt" => Some(BuiltinFunction::Sqrt),
            _ => None,
        }
    }
}

// Operator enum for idiomatic Rust usage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Operator {
    pub fn as_str(&self) -> &'static str {
        match self {
            Operator::Add => "__ADD__",
            Operator::Sub => "__SUB__",
            Operator::Mul => "__MUL__",
            Operator::Div => "__DIV__",
            Operator::Pow => "__POW__",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "__ADD__" => Some(Operator::Add),
            "__SUB__" => Some(Operator::Sub),
            "__MUL__" => Some(Operator::Mul),
            "__DIV__" => Some(Operator::Div),
            "__POW__" => Some(Operator::Pow),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogicalOperator {
    Eq,       // '='
    DoubleEq, // '=='
    Neq,      // '!=' | '<>'
    Lt,       // '<'
    Gt,       // '>'
    Lte,      // '<=' | 'le'
    Gte,      // '>=' | 'ge'
}

impl LogicalOperator {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogicalOperator::Eq => "__EQ__",
            LogicalOperator::DoubleEq => "__EQEQ__",
            LogicalOperator::Neq => "__NEQ__", // canonical string
            LogicalOperator::Lt => "__LT__",
            LogicalOperator::Gt => "__GT__",
            LogicalOperator::Lte => "__LE__",
            LogicalOperator::Gte => "__GE__",
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "__EQ__" => Some(LogicalOperator::Eq),
            "__EQEQ__" => Some(LogicalOperator::DoubleEq),
            "__NE__" => Some(LogicalOperator::Neq),
            "__LT__" => Some(LogicalOperator::Lt),
            "__GT__" => Some(LogicalOperator::Gt),
            "__LE__" => Some(LogicalOperator::Lte),
            "__GE__" => Some(LogicalOperator::Gte),
            _ => None,
        }
    }

    pub fn from_str_token(s: &str) -> Option<Self> {
        match s {
            "=" => Some(LogicalOperator::Eq),
            "==" => Some(LogicalOperator::DoubleEq),
            "!=" | "<>" => Some(LogicalOperator::Neq),
            "<" => Some(LogicalOperator::Lt),
            ">" => Some(LogicalOperator::Gt),
            "<=" | "le" => Some(LogicalOperator::Lte),
            ">=" | "ge" => Some(LogicalOperator::Gte),
            _ => None,
        }
    }
}

pub struct Basic {
    inner: *mut basic,
}

impl std::hash::Hash for Basic {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Convert the Basic to a string and hash the string.
        // This is a simple way to generate a hash code.
        self.to_string().hash(state);
    }
}

impl PartialEq for Basic {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for Basic {}

impl Basic {
    pub const DOUBLE_PRECISION_BITS: u64 = 53;

    fn heap_alloc() -> Result<*mut basic_struct, &'static str> {
        unsafe {
            let ptr: *mut basic_struct = basic_new_heap();
            if ptr.is_null() {
                Err("Failed to allocate SymEngine basic struct")
            } else {
                Ok(ptr)
            }
        }
    }

    fn _stack_alloc() -> *mut basic_struct {
        unsafe {
            let ptr: *mut basic_struct = std::mem::zeroed();
            basic_new_stack(ptr);
            ptr
        }
    }

    /// Creates a new, uninitialized Basic instance.
    pub fn new() -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        Self {
            inner: b as *mut basic,
        }
    }

    fn real_double_get_d_safe(ptr: *const basic_struct) -> f64 {
        unsafe { real_double_get_d(ptr) }
    }

    /// Parses a string into a Basic expression using SymEngine's parser.
    pub fn parse(expr: &str) -> Result<Self, &'static str> {
        let b = Basic::new();
        let cstr = CString::new(expr).map_err(|_| "CString conversion failed")?;
        let status = unsafe { basic_parse(b.inner as *mut basic_struct, cstr.as_ptr()) };
        if status == 0 {
            Ok(b)
        } else {
            Err("Failed to parse expression")
        }
    }

    pub fn symbols<const N: usize>(names: [&str; N]) -> [Basic; N] {
        names.map(|name| Basic::symbol(name))
    }

    // ===========================
    // Constants
    // ===========================

    /// Creates a `Basic` instance representing the constant zero.
    pub fn zero() -> Self {
        let b = Basic::new();
        unsafe {
            basic_const_zero(b.inner as *mut basic_struct);
        }
        b
    }

    /// Creates a `Basic` instance representing the constant one.
    pub fn one() -> Self {
        let b = Basic::new();
        unsafe {
            basic_const_one(b.inner as *mut basic_struct);
        }
        b
    }

    /// Creates a `Basic` instance representing the constant minus one.
    pub fn minus_one() -> Self {
        let b = Basic::new();
        unsafe {
            basic_const_minus_one(b.inner as *mut basic_struct);
        }
        b
    }

    /// Returns true if this Basic number is zero.
    pub fn is_zero(&self) -> bool {
        unsafe { number_is_zero(self.inner as *const basic_struct) != 0 }
    }

    /// Returns true if this Basic number is negative.
    pub fn is_negative(&self) -> bool {
        unsafe { number_is_negative(self.inner as *const basic_struct) != 0 }
    }

    /// Returns true if this Basic number is positive.
    pub fn is_positive(&self) -> bool {
        unsafe { number_is_positive(self.inner as *const basic_struct) != 0 }
    }

    /// Creates a `Basic` instance representing the imaginary unit `i`.
    pub fn i() -> Self {
        let b = Basic::new();
        unsafe {
            basic_const_I(b.inner as *mut basic_struct);
        }
        b
    }

    /// Creates a `Basic` instance representing the constant π.
    pub fn pi() -> Self {
        let b = Basic::new();
        unsafe {
            basic_const_pi(b.inner as *mut basic_struct);
        }
        b
    }

    /// Creates a `Basic` instance representing Euler's number `e`.
    pub fn e() -> Self {
        let b = Basic::new();
        unsafe {
            basic_const_E(b.inner as *mut basic_struct);
        }
        b
    }

    // ==========================
    // function constructors
    // ==========================

    /// Constructs a function by name, supporting both exact and regex-based matches.
    pub fn function(name: &str, args: &[Rc<Basic>]) -> Self {
        assert!(
            !name.is_empty() && args.len() > 0,
            "Function name and args must not be empty"
        );
        if let Some(captures) = RE_LOG_BASE.captures(name) {
            if let Some(cap_base) = captures.get(1) {
                info!("log base {}", cap_base.as_str());
                return Basic::logb(&args[0], cap_base.as_str().parse::<i64>().unwrap());
            }
        }
        match name {
            "sin" => Basic::sin(&args[0]),
            "cos" => Basic::cos(&args[0]),
            "tan" => Basic::tan(&args[0]),
            "exp" => Basic::exp(&args[0]),
            "log" => Basic::log(&args[0]),
            "ln" => Basic::ln(&args[0]),
            "sqrt" => Basic::sqrt(&args[0]),
            _ => Basic::symbol(name), // Fallback to symbol for unknown functions
        }
    }

    // ===========================
    // log() with base support
    // symengine Basic::log() only supports natural logarithm (ln).
    // ===========================

    /// Computes the natural logarithm (log) of the given `Basic` instance.
    pub fn natural_log(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_log(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    /// Computes the natural logarithm (log) of the given `Basic` instance.
    pub fn log(symbol: &Basic) -> Self {
        Basic::log10(symbol)
    }

    pub fn log10(symbol: &Basic) -> Self {
        Basic::logb(symbol, 10)
    }

    pub fn ln(symbol: &Basic) -> Self {
        Basic::natural_log(symbol)
    }

    pub fn logb(symbol: &Basic, base: i64) -> Self {
        let b = Basic::natural_log(&Basic::integer(base)).evalf(Basic::DOUBLE_PRECISION_BITS, true);
        let n = Basic::natural_log(symbol);
        Basic::div(&n, &b)
    }

    // ===========================
    // Arithmetic Operations
    // ===========================

    pub fn add(&self, rhs: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_add(
                b.inner as *mut basic_struct,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        b
    }

    pub fn sub(&self, rhs: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_sub(
                b.inner as *mut basic_struct,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        b
    }

    pub fn mul(&self, rhs: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_mul(
                b.inner as *mut basic_struct,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        b
    }

    pub fn div(&self, rhs: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_div(
                b.inner as *mut basic_struct,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        b
    }

    pub fn div_int(&self, val: i64) -> Self {
        let b = Basic::new();
        unsafe {
            basic_div(
                b.inner as *mut basic_struct,
                self.inner as *mut basic_struct,
                Basic::integer(val).inner as *mut basic_struct,
            );
        }
        b
    }

    pub fn pow(&self, rhs: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_pow(
                b.inner as *mut basic_struct,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        b
    }

    /// Computes the square of the given `Basic` instance.
    pub fn sqr(&self) -> Self {
        self.pow(&Basic::integer(2))
    }

    /// Computes the square root of the given `Basic` instance.
    pub fn sqrt(&self) -> Self {
        let b = Basic::new();
        unsafe {
            basic_sqrt(
                b.inner as *mut basic_struct,
                self.inner as *mut basic_struct,
            );
        }
        b
    }

    /// Computes the exponential (exp) of the given `Basic` instance.
    pub fn exp(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_exp(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    // ===========================
    // Trigonometric Functions
    // ===========================

    pub fn sin(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_sin(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    pub fn cos(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_cos(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    pub fn tan(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_tan(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    /// Computes the arcsine (asin) of the given `Basic` instance.
    pub fn asin(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_asin(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    /// Computes the arccosine (acos) of the given `Basic` instance.
    pub fn acos(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_acos(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    /// Computes the arctangent (atan) of the given `Basic` instance.
    pub fn atan(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_atan(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    /// Computes the cosecant (csc) of the given `Basic` instance.
    pub fn csc(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_csc(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    /// Computes the secant (sec) of the given `Basic` instance.
    pub fn sec(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_sec(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
            );
        }
        b
    }

    // ===========================
    // Utility Functions
    // ===========================

    pub fn abs(&self) -> Self {
        let b = Basic::new();
        unsafe {
            basic_abs(
                b.inner as *mut basic_struct,
                self.inner as *mut basic_struct,
            );
        }
        b
    }

    pub fn min(args: Vec<&Basic>) -> Self {
        let bv = BasicVec::from_slice(&args).unwrap();
        let b = Basic::new();
        unsafe {
            basic_min(b.inner as *mut basic_struct, bv.inner as *const CVecBasic);
        }
        b
    }

    pub fn max(args: Vec<&Basic>) -> Self {
        let bv = BasicVec::from_slice(&args).unwrap();
        let b = Basic::new();
        unsafe {
            basic_max(b.inner as *mut basic_struct, bv.inner as *const CVecBasic);
        }
        b
    }

    pub fn subs<'a>(exp: &Basic, pairs: &[(&'a Basic, &'a Basic)]) -> Self {
        let mb = BasicMap::from_pairs(pairs).unwrap();
        let b = Basic::new();
        unsafe {
            basic_subs(
                b.inner as *mut basic_struct,
                exp.inner as *mut basic_struct,
                mb.inner as *const CMapBasicBasic,
            );
        }
        b
    }

    pub fn rc_subs<'a>(exp: &Basic, rcpairs: &[(&'a Rc<Basic>, &'a Rc<Basic>)]) -> Self {
        let mb = BasicMap::from_rc_pairs(rcpairs).unwrap();
        let b = Basic::new();
        unsafe {
            basic_subs(
                b.inner as *mut basic_struct,
                exp.inner as *mut basic_struct,
                mb.inner as *const CMapBasicBasic,
            );
        }
        b
    }

    pub fn equals(&self, rhs: &Basic) -> bool {
        unsafe {
            basic_eq(
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            ) != 0
        }
    }

    pub fn to_string(&self) -> String {
        if self.is_null() {
            String::new()
        } else {
            unsafe {
                let c_str = basic_str(self.inner as *mut basic_struct);
                let s = CStr::from_ptr(c_str).to_string_lossy().into_owned();
                basic_str_free(c_str);
                s
            }
        }
    }

    // operators
    pub fn div_op() -> Self {
        Basic::symbol(Operator::Div.as_str())
    }

    pub fn mul_op() -> Self {
        Basic::symbol(Operator::Mul.as_str())
    }

    pub fn add_op() -> Self {
        Basic::symbol(Operator::Add.as_str())
    }

    pub fn sub_op() -> Self {
        Basic::symbol(Operator::Sub.as_str())
    }

    pub fn pow_op() -> Self {
        Basic::symbol(Operator::Pow.as_str())
    }

    /// Returns true if this Basic is a Number.
    pub fn is_number(&self) -> bool {
        unsafe { is_a_Number(self.inner as *const basic_struct) != 0 }
    }

    /// Returns true if this Basic is an Integer.
    pub fn is_integer(&self) -> bool {
        unsafe { is_a_Integer(self.inner as *const basic_struct) != 0 }
    }

    /// Returns true if this Basic is a Rational.
    pub fn is_rational(&self) -> bool {
        unsafe { is_a_Rational(self.inner as *const basic_struct) != 0 }
    }

    /// Returns true if this Basic is a Symbol.
    pub fn is_symbol(&self) -> bool {
        unsafe { is_a_Symbol(self.inner as *const basic_struct) != 0 }
    }

    /// Returns true if this Basic is a Complex.
    pub fn is_complex(&self) -> bool {
        unsafe { is_a_Complex(self.inner as *const basic_struct) != 0 }
    }

    pub fn is_add_op(&self) -> bool {
        if self.is_null() {
            return false;
        }
        Basic::symbol(Operator::Add.as_str()).equals(self)
    }
    pub fn is_sub_op(&self) -> bool {
        if self.is_null() {
            return false;
        }
        Basic::symbol(Operator::Sub.as_str()).equals(self)
    }

    pub fn is_mul_op(&self) -> bool {
        if self.is_null() {
            return false;
        }
        Basic::symbol(Operator::Mul.as_str()).equals(self)
    }
    pub fn is_div_op(&self) -> bool {
        if self.is_null() {
            return false;
        }
        Basic::symbol(Operator::Div.as_str()).equals(self)
    }

    /// Returns true if this Basic expression is a power (Pow).
    pub fn is_pow_op(&self) -> bool {
        if self.is_null() {
            return false;
        }
        Basic::symbol(Operator::Pow.as_str()).equals(self)
    }

    pub fn is_op(&self) -> bool {
        self.is_add_op()
            || self.is_sub_op()
            || self.is_mul_op()
            || self.is_div_op()
            || self.is_pow_op()
    }

    /// Returns (true, base) if this Basic is a logb (log with base) function symbol, e.g. "log_2", "log_10".
    /// Returns (false, 0) otherwise.
    pub fn is_logb_func_sym(&self) -> (bool, i64) {
        if self.is_symbol() {
            let s = self.to_string();
            if let Some(caps) = RE_LOG_BASE.captures(&s) {
                if let Some(m) = caps.get(1) {
                    if let Ok(base) = m.as_str().parse::<i64>() {
                        return (true, base);
                    }
                }
            }
        }
        (false, 0)
    }

    pub fn is_default(&self) -> bool {
        self.is_null() || self.inner.is_null()
    }

    // ===========================
    // Logical Operations
    // ===========================

    /// Create a Basic representing a logical operator symbol
    pub fn logical_op(op: LogicalOperator) -> Self {
        Basic::symbol(op.as_str())
    }

    /// Returns true if this Basic is a logical operator
    pub fn is_logical_op(&self) -> bool {
        [
            LogicalOperator::Eq,
            LogicalOperator::DoubleEq,
            LogicalOperator::Neq,
            LogicalOperator::Lt,
            LogicalOperator::Gt,
            LogicalOperator::Lte,
            LogicalOperator::Gte,
        ]
        .iter()
        .any(|&op| Basic::symbol(op.as_str()).equals(self))
    }

    pub fn is_eq_op(&self) -> bool {
        Basic::symbol(LogicalOperator::Eq.as_str()).equals(self)
    }
    pub fn is_double_eq_op(&self) -> bool {
        Basic::symbol(LogicalOperator::DoubleEq.as_str()).equals(self)
    }
    pub fn is_neq_op(&self) -> bool {
        Basic::symbol(LogicalOperator::Neq.as_str()).equals(self)
    }
    pub fn is_lt_op(&self) -> bool {
        Basic::symbol(LogicalOperator::Lt.as_str()).equals(self)
    }
    pub fn is_gt_op(&self) -> bool {
        Basic::symbol(LogicalOperator::Gt.as_str()).equals(self)
    }
    pub fn is_lte_op(&self) -> bool {
        Basic::symbol(LogicalOperator::Lte.as_str()).equals(self)
    }
    pub fn is_gte_op(&self) -> bool {
        Basic::symbol(LogicalOperator::Gte.as_str()).equals(self)
    }

    /// Convenience constructors for each logical operator
    pub fn eq_op() -> Self {
        Basic::symbol(LogicalOperator::Eq.as_str())
    }
    pub fn double_eq_op() -> Self {
        Basic::symbol(LogicalOperator::DoubleEq.as_str())
    }
    pub fn neq_op() -> Self {
        Basic::symbol(LogicalOperator::Neq.as_str())
    }
    pub fn lt_op() -> Self {
        Basic::symbol(LogicalOperator::Lt.as_str())
    }
    pub fn gt_op() -> Self {
        Basic::symbol(LogicalOperator::Gt.as_str())
    }
    pub fn lte_op() -> Self {
        Basic::symbol(LogicalOperator::Lte.as_str())
    }
    pub fn gte_op() -> Self {
        Basic::symbol(LogicalOperator::Gte.as_str())
    }

    /// Convenience constructors for each function
    /// corresponding to a BuiltinFunction.
    pub fn sin_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Sin.as_str())
    }

    pub fn cos_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Cos.as_str())
    }

    pub fn tan_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Tan.as_str())
    }

    pub fn asin_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Asin.as_str())
    }
    pub fn acos_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Acos.as_str())
    }
    pub fn atan_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Atan.as_str())
    }
    pub fn csc_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Csc.as_str())
    }
    pub fn sec_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Sec.as_str())
    }
    pub fn exp_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Exp.as_str())
    }
    pub fn log_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Log.as_str())
    }
    pub fn ln_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Ln.as_str())
    }
    pub fn logb_func_sym(base: i64) -> Self {
        let s = format!("{}_{}", BuiltinFunction::Log.as_str(), base);
        Basic::symbol(&s)
    }
    pub fn sqrt_func_sym() -> Self {
        Basic::symbol(BuiltinFunction::Sqrt.as_str())
    }

    pub fn func_sym_from_str(name: &str) -> Self {
        if let Some(func) = BuiltinFunction::from_str(name) {
            Basic::symbol(func.as_str())
        } else {
            Basic::symbol(name)
        }
    }

    // ===========================
    // Conversion Functions
    // ===========================

    pub fn evalf(&self, bits: u64, real: bool) -> Basic {
        let s = Basic::new();

        let real_c_int = if real { 1 } else { 0 };
        unsafe {
            basic_evalf(
                s.inner as *mut basic_struct,      // s: *mut basic_struct
                self.inner as *const basic_struct, // s: *const basic_struct
                bits,                              // bits: c_ulong
                real_c_int,                        // real: c_int
            );
        }
        s
    }

    pub fn to_f64(&self) -> Option<f64> {
        Some(Self::real_double_get_d_safe(
            self.inner as *const basic_struct,
        ))
    }

    pub fn to_i64(&self) -> Option<i64> {
        let result = unsafe { integer_get_si(self.inner as *const basic_struct) };
        Some(result)
    }

    pub fn to_u64(&self) -> Option<u64> {
        let result = unsafe { integer_get_ui(self.inner as *const basic_struct) };
        Some(result)
    }

    pub fn neg(&self) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_neg(b, self.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn integer(val: i64) -> Self {
        let b: *mut basic_struct = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            integer_set_si(b, val);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn real(val: f64) -> Self {
        let b: *mut basic_struct = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            real_double_set_d(b, val);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn symbol(name: &str) -> Self {
        let b: *mut basic_struct = Self::heap_alloc().unwrap() as *mut basic_struct;
        let cstr = CString::new(name).unwrap();
        unsafe {
            symbol_set(b, cstr.as_ptr());
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Checks if the given Basic is a function corresponding to a BuiltinFunction.
    pub fn is_function(b: &Basic) -> bool {
        let s = b.to_string();
        BuiltinFunction::from_str(&s).is_some()
    }

    /// Checks if the `inner` pointer is null.
    pub fn is_null(&self) -> bool {
        self.inner.is_null()
    }

    pub fn get_type(&self) -> u32 {
        unsafe { basic_get_type(self.inner as *const basic_struct) }
    }

    #[allow(non_upper_case_globals)]
    pub fn get_type_str(&self) -> &'static str {
        match self.get_type() {
            TypeID_SYMENGINE_INTEGER => "Integer",
            TypeID_SYMENGINE_RATIONAL => "Rational",
            TypeID_SYMENGINE_REAL_DOUBLE => "Real",
            TypeID_SYMENGINE_COMPLEX_MPC => "Complex",
            TypeID_SYMENGINE_SYMBOL => "Symbol",
            TypeID_SYMENGINE_FUNCTIONSYMBOL => "Function",
            TypeID_SYMENGINE_ADD => "Add",
            TypeID_SYMENGINE_MUL => "Multiply",
            TypeID_SYMENGINE_POW => "Power",
            _ => "Unknown",
        }
    }

    // ===========================
    // solvers
    // ===========================

    // Solves the univariate polynomial equation `self == 0` for the given symbol.
    // Returns a Vec of Basic instances representing the roots.
    pub fn solve_poly(&self, symbol: &Basic) -> Vec<Basic> {
        let mut roots = Vec::new();
        let basic_set = BasicSet::new().unwrap();
        if basic_set.inner.is_null() {
            return roots;
        }
        unsafe {
            basic_solve_poly(
                basic_set.inner,
                self.inner as *const basic_struct,
                symbol.inner as *const basic_struct,
            );
            let n = setbasic_size(basic_set.inner);
            for i in 0..n {
                let elem = basic_set.get(i);
                if !elem.is_none() {
                    roots.push(elem.unwrap());
                }
            }
        }
        roots
    }

    // ===========================
    // Calculus
    // ===========================
    // differentiate self(exp), WRT var
    pub fn diff(&self, var: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_diff(
                b.inner as *mut basic_struct,
                self.inner as *const basic_struct,
                var.inner as *const basic_struct,
            );
        }
        b
    }
}

impl Drop for Basic {
    fn drop(&mut self) {
        unsafe {
            basic_free_heap(self.inner as *mut basic_struct);
        }
    }
}

impl Clone for Basic {
    fn clone(&self) -> Self {
        let b = Self::heap_alloc().expect("Failed to allocate memory for clone");
        unsafe {
            basic_assign(b, self.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }
}

impl Default for Basic {
    fn default() -> Self {
        Self {
            inner: std::ptr::null_mut(),
        }
    }
}

impl fmt::Display for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Basic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl From<i64> for Basic {
    fn from(val: i64) -> Self {
        let b = Self::heap_alloc().expect("Failed to allocate memory for integer");
        unsafe {
            integer_set_si(b, val);
        }
        Self {
            inner: b as *mut basic,
        }
    }
}

impl From<f64> for Basic {
    fn from(val: f64) -> Self {
        let b = Self::heap_alloc().expect("Failed to allocate memory for real");
        unsafe {
            real_double_set_d(b, val);
        }
        Self {
            inner: b as *mut basic,
        }
    }
}

impl Into<i64> for Basic {
    fn into(self) -> i64 {
        self.to_i64().unwrap()
    }
}

impl Into<u64> for Basic {
    fn into(self) -> u64 {
        self.to_u64().unwrap()
    }
}

impl Into<f64> for Basic {
    fn into(self) -> f64 {
        self.to_f64().unwrap()
    }
}

#[derive(Debug)]
pub struct BasicVec {
    inner: *mut CVecBasic,
}

impl BasicVec {
    pub fn from_slice(slice: &[&Basic]) -> Result<Self, &'static str> {
        unsafe {
            let ptr = vecbasic_new();
            if ptr.is_null() {
                return Err("Failed to allocate memory for BasicVec");
            }
            for &b in slice {
                vecbasic_push_back(ptr, b.inner as *mut basic_struct);
            }
            Ok(Self { inner: ptr })
        }
    }
}

impl Drop for BasicVec {
    fn drop(&mut self) {
        unsafe { vecbasic_free(self.inner) }
    }
}

#[derive(Debug)]
pub struct BasicMap {
    inner: *mut CMapBasicBasic,
}

impl BasicMap {
    pub fn from_pairs<'a>(pairs: &[(&'a Basic, &'a Basic)]) -> Result<Self, &'static str> {
        unsafe {
            let ptr = mapbasicbasic_new();
            if ptr.is_null() {
                return Err("Failed to allocate memory for BasicMap");
            }
            for (basic_key, basic_value) in pairs {
                mapbasicbasic_insert(
                    ptr,
                    basic_key.inner as *mut basic_struct,
                    basic_value.inner as *mut basic_struct,
                );
            }
            Ok(Self { inner: ptr })
        }
    }

    pub fn from_rc_pairs<'a>(
        pairs: &[(&'a Rc<Basic>, &'a Rc<Basic>)],
    ) -> Result<Self, &'static str> {
        unsafe {
            let ptr = mapbasicbasic_new();
            if ptr.is_null() {
                return Err("Failed to allocate memory for BasicMap");
            }
            for (basic_key, basic_value) in pairs {
                mapbasicbasic_insert(
                    ptr,
                    basic_key.inner as *mut basic_struct,
                    basic_value.inner as *mut basic_struct,
                );
            }
            Ok(Self { inner: ptr })
        }
    }
}

impl Drop for BasicMap {
    fn drop(&mut self) {
        unsafe { mapbasicbasic_free(self.inner) }
    }
}

#[derive(Debug)]
pub struct BasicSet {
    inner: *mut CSetBasic,
}

impl BasicSet {
    pub fn new() -> Result<Self, &'static str> {
        unsafe {
            let ptr = setbasic_new();
            if ptr.is_null() {
                return Err("Failed to allocate memory for BasicSet");
            }
            Ok(Self { inner: ptr })
        }
    }

    /// Creates a BasicSet from an iterator of &Basic.
    pub fn from_slice<'a, I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = &'a Basic>,
    {
        let set = Self::new()?;
        for b in iter {
            unsafe {
                setbasic_insert(set.inner, b.inner as *mut basic_struct);
            }
        }
        Ok(set)
    }

    /// Returns the number of elements in the set.
    pub fn len(&self) -> usize {
        unsafe { setbasic_size(self.inner) }
    }

    /// Returns true if the set contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the element at the given index, if any.
    pub fn get(&self, index: usize) -> Option<Basic> {
        if index >= self.len().try_into().unwrap() {
            return None;
        }
        unsafe {
            let b = Basic::new();
            setbasic_get(
                self.inner,
                index.try_into().unwrap(),
                b.inner as *mut basic_struct,
            );
            if b.inner.is_null() {
                None
            } else {
                Some(b)
            }
        }
    }
}

impl Drop for BasicSet {
    fn drop(&mut self) {
        unsafe { setbasic_free(self.inner) }
    }
}
