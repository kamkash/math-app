use crate::symengine_ffi::*;
// use log::*;
use std::ffi::{CStr, CString};
use std::fmt;
use std::rc::Rc;

// Constants for operator symbols
const ADD_OP: &str = "__ADD__";
const SUB_OP: &str = "__SUB__";
const MUL_OP: &str = "__MUL__";
const DIV_OP: &str = "__DIV__";
const POW_OP: &str = "__POW__";

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

    /// Computes the natural logarithm (log) of the given `Basic` instance.
    pub fn log(symbol: &Basic) -> Self {
        let b = Basic::new();
        unsafe {
            basic_log(
                b.inner as *mut basic_struct,
                symbol.inner as *mut basic_struct,
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

    pub fn subs<'a, I>(exp: &Basic, pairs: I) -> Self
    where
        I: IntoIterator<Item = (&'a Basic, &'a Basic)>,
    {
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

    pub fn rc_subs<'a, I>(exp: &Basic, rcpairs: I) -> Self
    where
        I: IntoIterator<Item = (&'a Rc<Basic>, &'a Rc<Basic>)>,
    {
        let mb = BasicMap::from_rc_pairs(
            rcpairs
                .into_iter()
                .map(|(k, v)| (Rc::clone(k), Rc::clone(v))),
        )
        .unwrap();
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
        Basic::symbol(ADD_OP).equals(self)
    }
    pub fn is_sub_op(&self) -> bool {
        if self.is_null() {
            return false;
        }
        Basic::symbol(SUB_OP).equals(self)
    }

    pub fn is_mul_op(&self) -> bool {
        if self.is_null() {
            return false;
        }
        Basic::symbol(MUL_OP).equals(self)
    }
    pub fn is_div_op(&self) -> bool {
        if self.is_null() {
            return false;
        }
        Basic::symbol(DIV_OP).equals(self)
    }

    /// Returns true if this Basic expression is a power (Pow).
    pub fn is_pow_op(&self) -> bool {
        if self.is_null() {
            return false;
        }
        Basic::symbol(POW_OP).equals(self)
    }

    pub fn is_op(&self) -> bool {
        self.is_add_op()
            || self.is_sub_op()
            || self.is_mul_op()
            || self.is_div_op()
            || self.is_pow_op()
    }

    /// Returns the arguments of this Basic expression
    /// Returns an empty Vec for atomic types (e.g., Symbol, Integer) or on error.
    pub fn get_args(&self) -> Vec<Basic> {
        let mut args_vec = Vec::new();
        if self.is_null() {
            return args_vec;
        }
        unsafe {
            let c_vec_basic = vecbasic_new();
            if c_vec_basic.is_null() {
                return args_vec;
            } // Allocation failed

            basic_get_args(self.inner as *const basic_struct, c_vec_basic);
            let n = vecbasic_size(c_vec_basic);
            for i in 0..n {
                let elem_basic = Basic::new();
                vecbasic_get(c_vec_basic, i, elem_basic.inner as *mut basic_struct);
                if !elem_basic.is_null() {
                    args_vec.push(elem_basic);
                }
            }
            vecbasic_free(c_vec_basic);
        }
        args_vec
    }

    pub fn div_op() -> Self {
        Basic::symbol(DIV_OP)
    }

    pub fn mul_op() -> Self {
        Basic::symbol(MUL_OP)
    }

    pub fn add_op() -> Self {
        Basic::symbol(ADD_OP)
    }

    pub fn sub_op() -> Self {
        Basic::symbol(SUB_OP)
    }

    pub fn pow_op() -> Self {
        Basic::symbol(POW_OP)
    }

    // ===========================
    // Conversion Functions
    // ===========================

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
                basic_set.inner as *mut CSetBasic,
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
            Ok(Self {
                inner: ptr as *mut CVecBasic,
            })
        }
    }
}

impl Drop for BasicVec {
    fn drop(&mut self) {
        unsafe { vecbasic_free(self.inner as *mut CVecBasic) }
    }
}

#[derive(Debug)]
pub struct BasicMap {
    inner: *mut CMapBasicBasic,
}

impl BasicMap {
    pub fn from_pairs<'a, I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = (&'a Basic, &'a Basic)>,
    {
        unsafe {
            let ptr = mapbasicbasic_new();
            if ptr.is_null() {
                return Err("Failed to allocate memory for BasicMap");
            }
            for (basic_key, basic_value) in iter {
                mapbasicbasic_insert(
                    ptr,
                    basic_key.inner as *mut basic_struct,
                    basic_value.inner as *mut basic_struct,
                );
            }
            Ok(Self {
                inner: ptr as *mut CMapBasicBasic,
            })
        }
    }

    pub fn from_rc_pairs<I>(iter: I) -> Result<Self, &'static str>
    where
        I: IntoIterator<Item = (Rc<Basic>, Rc<Basic>)>,
    {
        unsafe {
            let ptr = mapbasicbasic_new();
            if ptr.is_null() {
                return Err("Failed to allocate memory for BasicMap");
            }
            for (basic_key, basic_value) in iter {
                mapbasicbasic_insert(
                    ptr,
                    basic_key.inner as *mut basic_struct,
                    basic_value.inner as *mut basic_struct,
                );
            }
            Ok(Self {
                inner: ptr as *mut CMapBasicBasic,
            })
        }
    }
}

impl Drop for BasicMap {
    fn drop(&mut self) {
        unsafe { mapbasicbasic_free(self.inner as *mut CMapBasicBasic) }
    }
}

#[derive(Debug)]
pub struct BasicSet {
    inner: *mut CSetBasic,
}

impl BasicSet {
    /// Creates a new empty BasicSet.
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
