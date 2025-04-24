use crate::symengine_ffi::*;
use std::ffi::{CStr, CString};
use std::fmt;
use std::rc::Rc;

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

    fn real_double_get_d_safe(ptr: *const basic_struct) -> f64 {
        unsafe { real_double_get_d(ptr) }
    }

    // ===========================
    // Constants
    // ===========================

    /// Creates a `Basic` instance representing the constant zero.
    pub fn zero() -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_const_zero(b);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Creates a `Basic` instance representing the constant one.
    pub fn one() -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_const_one(b);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Creates a `Basic` instance representing the constant minus one.
    pub fn minus_one() -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_const_minus_one(b);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Creates a `Basic` instance representing the imaginary unit `i`.
    pub fn i() -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_const_I(b);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Creates a `Basic` instance representing the constant π.
    pub fn pi() -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_const_pi(b);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Creates a `Basic` instance representing Euler's number `e`.
    pub fn e() -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_const_E(b);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    // ===========================
    // Arithmetic Operations
    // ===========================

    pub fn add(&self, rhs: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_add(
                b,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn sub(&self, rhs: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_sub(
                b,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn mul(&self, rhs: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_mul(
                b,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn div(&self, rhs: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_div(
                b,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn div_int(&self, val: i64) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_div(
                b,
                self.inner as *mut basic_struct,
                Basic::integer(val).inner as *mut basic_struct,
            );
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn pow(&self, rhs: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_pow(
                b,
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            );
        }
        Self {
            inner: b as *mut basic,
        }
    }

    // ===========================
    // Trigonometric Functions
    // ===========================

    pub fn sin(symbol: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_sin(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn cos(symbol: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_cos(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn tan(symbol: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_tan(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Computes the arcsine (asin) of the given `Basic` instance.
    pub fn asin(symbol: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_asin(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Computes the arccosine (acos) of the given `Basic` instance.
    pub fn acos(symbol: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_acos(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Computes the arctangent (atan) of the given `Basic` instance.
    pub fn atan(symbol: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_atan(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Computes the cosecant (csc) of the given `Basic` instance.
    pub fn csc(symbol: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_csc(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    /// Computes the secant (sec) of the given `Basic` instance.
    pub fn sec(symbol: &Basic) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_sec(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    // ===========================
    // Utility Functions
    // ===========================

    pub fn abs(&self) -> Self {
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_abs(b, self.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn min(args: Vec<&Basic>) -> Self {
        let bv = BasicVec::from_slice(&args).unwrap();
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;

        unsafe {
            basic_min(b, bv.inner as *const CVecBasic);
        }

        Self {
            inner: b as *mut basic,
        }
    }

    pub fn max(args: Vec<&Basic>) -> Self {
        let bv = BasicVec::from_slice(&args).unwrap();
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;

        unsafe {
            basic_max(b, bv.inner as *const CVecBasic);
        }

        Self {
            inner: b as *mut basic,
        }
    }

    pub fn subs<'a, I>(exp: &Basic, pairs: I) -> Self
    where
        I: IntoIterator<Item = (&'a Basic, &'a Basic)>,
    {
        let mb = BasicMap::from_pairs(pairs).unwrap();
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_subs(
                b,
                exp.inner as *mut basic_struct,
                mb.inner as *const CMapBasicBasic,
            );
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn rc_subs<I>(exp: &Basic, rcpairs: I) -> Self
    where
        I: IntoIterator<Item = (Rc<Basic>, Rc<Basic>)>,
    {
        let mb = BasicMap::from_rc_pairs(rcpairs).unwrap();
        let b = Self::heap_alloc().unwrap() as *mut basic_struct;
        unsafe {
            basic_subs(
                b,
                exp.inner as *mut basic_struct,
                mb.inner as *const CMapBasicBasic,
            );
        }
        Self {
            inner: b as *mut basic,
        }
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

    // ===========================
    // Conversion Functions
    // ===========================

    pub fn to_f64(&self) -> f64 {
        Self::real_double_get_d_safe(self.inner as *const basic_struct)
    }

    pub fn to_i64(&self) -> i64 {
        let result = unsafe { integer_get_si(self.inner as *const basic_struct) };
        result
    }

    pub fn to_u64(&self) -> u64 {
        let result = unsafe { integer_get_ui(self.inner as *const basic_struct) };
        result
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
        self.to_i64()
    }
}

impl Into<u64> for Basic {
    fn into(self) -> u64 {
        self.to_u64()
    }
}

impl Into<f64> for Basic {
    fn into(self) -> f64 {
        self.to_f64()
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
