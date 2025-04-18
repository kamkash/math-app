use crate::symengine_ffi::*;
use std::ffi::{CStr, CString};
use std::fmt;

pub struct Basic {
    inner: *mut basic,
}

impl Basic {
    fn heap_alloc() -> *mut basic_struct {
        unsafe {
            let ptr: *mut basic_struct = basic_new_heap();
            debug_assert!(!ptr.is_null(), "Failed to allocate SymEngine basic struct");
            ptr
        }
    }

    fn _stack_alloc() -> *mut basic_struct {
        unsafe {
            let ptr: *mut basic_struct = std::mem::zeroed();
            basic_new_stack(ptr);
            ptr
        }
    }

    pub fn to_f64(&self) -> f64 {
        let result = unsafe { real_double_get_d(self.inner as *const basic_struct) };
        result
    }

    pub fn to_i64(&self) -> i64 {
        let result = unsafe { integer_get_si(self.inner as *const basic_struct) };
        result
    }    

    pub fn to_u64(&self) -> u64 {
        let result = unsafe { integer_get_ui(self.inner as *const basic_struct) };
        result
    }    

    pub fn symbol(name: &str) -> Self {
        let b: *mut basic_struct = Self::heap_alloc() as *mut basic_struct;
        let cstr = CString::new(name).unwrap();
        unsafe {
            symbol_set(b, cstr.as_ptr());
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn integer(val: i64) -> Self {
        let b: *mut basic_struct = Self::heap_alloc() as *mut basic_struct;
        unsafe {
            integer_set_si(b, val);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn real(val: f64) -> Self {
        let b: *mut basic_struct = Self::heap_alloc() as *mut basic_struct;
        unsafe {
            real_double_set_d(b, val);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn sin(symbol: &Basic) -> Self {
        let b = Self::heap_alloc() as *mut basic_struct;
        unsafe {
            basic_sin(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn cos(symbol: &Basic) -> Self {
        let b = Self::heap_alloc() as *mut basic_struct;
        unsafe {
            basic_cos(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn tan(symbol: &Basic) -> Self {
        let b = Self::heap_alloc() as *mut basic_struct;
        unsafe {
            basic_tan(b, symbol.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn pi() -> Self {
        let b = Self::heap_alloc() as *mut basic_struct;
        unsafe {
            basic_const_pi(b);
        }
        Self {
            inner: b as *mut basic,
        }
    }
    pub fn div_int(&self, val: i64) -> Self {
        let b = Self::heap_alloc() as *mut basic_struct;
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
        let b = Self::heap_alloc() as *mut basic_struct;
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

    pub fn sub(&self, rhs: &Basic) -> Self {
        let b = Self::heap_alloc() as *mut basic_struct;
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
        let b = Self::heap_alloc() as *mut basic_struct;
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
        let b = Self::heap_alloc() as *mut basic_struct;
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

    pub fn add(&self, rhs: &Basic) -> Self {
        let b: *mut basic_struct = Self::heap_alloc() as *mut basic_struct;
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

    pub fn equals(&self, rhs: &Basic) -> bool {
        unsafe {
            basic_eq(
                self.inner as *mut basic_struct,
                rhs.inner as *mut basic_struct,
            ) != 0
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let c_str = basic_str(self.inner as *mut basic_struct);
            let s = CStr::from_ptr(c_str).to_string_lossy().into_owned();
            basic_str_free(c_str);
            s
        }
    }

    pub fn abs(&self) -> Self {
        let b = Self::heap_alloc() as *mut basic_struct;
        unsafe {
            basic_abs(b, self.inner as *mut basic_struct);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn min(args: Vec<&Basic>) -> Self {
        let bv = BasicVec::from_slice(&args);
        let b = Self::heap_alloc() as *mut basic_struct;

        unsafe {
            basic_min(b, bv.inner as *const CVecBasic);
        }

        Self {
            inner: b as *mut basic,
        }
    }

    pub fn max(args: Vec<&Basic>) -> Self {
        let bv = BasicVec::from_slice(&args);
        let b = Self::heap_alloc() as *mut basic_struct;

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
        let mb = BasicMap::from_tuples(pairs);
        let b = Self::heap_alloc() as *mut basic_struct;
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
}

impl Drop for Basic {
    fn drop(&mut self) {
        unsafe {
            basic_free_heap(self.inner as *mut basic_struct);
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
        write!(f, "Basic({})", self.to_string())
    }
}

pub struct BasicVec {
    inner: *mut CVecBasic,
}

impl BasicVec {
    pub fn from_slice(slice: &[&Basic]) -> Self {
        unsafe {
            let ptr = vecbasic_new();
            for &b in slice {
                vecbasic_push_back(ptr, b.inner as *mut basic_struct);
            }
            Self {
                inner: ptr as *mut CVecBasic,
            }
        }
    }
}

impl Drop for BasicVec {
    fn drop(&mut self) {
        unsafe { vecbasic_free(self.inner as *mut CVecBasic) }
    }
}

pub struct BasicMap {
    inner: *mut CMapBasicBasic,
}

impl BasicMap {
    pub fn from_tuples<'a, I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (&'a Basic, &'a Basic)>,
    {
        unsafe {
            let ptr = mapbasicbasic_new();
            for (basic_key, basic_value) in iter {
                mapbasicbasic_insert(
                    ptr,
                    basic_key.inner as *mut basic_struct,
                    basic_value.inner as *mut basic_struct,
                );
            }
            Self {
                inner: ptr as *mut CMapBasicBasic,
            }
        }
    }
}
impl Drop for BasicMap {
    fn drop(&mut self) {
        unsafe { mapbasicbasic_free(self.inner as *mut CMapBasicBasic) }
    }
}
