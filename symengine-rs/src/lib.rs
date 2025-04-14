#[allow(warnings)]
mod symengine_ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// use log::info;
use std::ffi::{CStr, CString};
use std::fmt;
use symengine_ffi::*;

#[ctor::ctor]
fn init_logger() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .try_init();
}

pub struct Basic {
    inner: *mut basic,
}

impl Basic {
    fn alloc() -> *mut basic_struct {
        unsafe {
            let ptr: *mut basic_struct = basic_new_heap();
            debug_assert!(!ptr.is_null(), "Failed to allocate SymEngine basic struct");
            ptr
        }
    }

    fn _new() -> Self {
        let mut b: basic_struct = unsafe { std::mem::zeroed() }; // Allocate on the stack
        unsafe {
            basic_new_stack(&mut b as *mut basic_struct); // Initialize the struct
        }
        Self {
            inner: &mut b as *mut basic_struct as *mut basic,
        }
    }

    fn _alloc() -> *mut basic_struct {
        unsafe {
            let b: Box<[basic_struct; 1]> = Box::new([std::mem::zeroed()]);
            let ptr = Box::into_raw(b);
            basic_new_stack(ptr as *mut basic_struct);
            ptr as *mut basic_struct
        }
    }

    pub fn symbol(name: &str) -> Self {
        let b: *mut basic_struct = Self::alloc();
        let cstr = CString::new(name).unwrap();
        unsafe {
            symbol_set(b, cstr.as_ptr());
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn integer(val: i64) -> Self {
        let b: *mut basic_struct = Self::alloc();
        unsafe {
            integer_set_si(b, val);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn real(val: f64) -> Self {
        let b: *mut basic_struct = Self::alloc();
        unsafe {
            real_double_set_d(b, val);
        }
        Self {
            inner: b as *mut basic,
        }
    }

    pub fn add(&self, rhs: &Basic) -> Self {
        let b: *mut basic_struct = Self::alloc();
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
        let b = Self::alloc();
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
        let b = Self::alloc();
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

// impl Drop for Basic {
//     fn drop(&mut self) {
//         unsafe {
//             basic_free_heap(self.inner as *mut basic_struct);
//         }
//     }
// }

impl Drop for Basic {
    fn drop(&mut self) {
        unsafe {
            basic_free_stack(self.inner as *mut basic_struct);
            drop(Box::from_raw(self.inner));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_symbol_add() {
        let x = Basic::symbol("x");
        let y = Basic::symbol("y");
        let sum = x.add(&y);
        assert_eq!(sum.to_string(), "x + y");
    }

    #[test]
    fn test_basic_integer_real() {
        let int = Basic::integer(42);
        let real = Basic::real(3.14);
        assert_eq!(int.to_string(), "42");
        assert_eq!(real.to_string(), "3.14");
    }

    #[test]
    fn test_basic_mul_pow_eq() {
        let x = Basic::symbol("x");
        let y = Basic::symbol("y");
        let product = x.mul(&y);
        let power = x.pow(&Basic::integer(2));
        assert_eq!(product.to_string(), "x*y");
        assert_eq!(power.to_string(), "x**2");
        assert!(x.equals(&Basic::symbol("x")));
        assert!(!x.equals(&y));
    }
}
