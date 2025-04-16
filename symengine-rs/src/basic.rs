use std::ffi::{CStr, CString};
use std::fmt;
use crate::symengine_ffi::*;

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
