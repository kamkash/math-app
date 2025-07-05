use crate::giac_ffi::*;
use std::ffi::{CStr, CString};

pub struct Gen {
    ptr: *mut gen_t,
    ctx: *mut context_t
}

impl Gen {
    pub fn new(expr: &str, ctx: &crate::context::Context) -> Option<Self> {
        let cstr = CString::new(expr).ok()?;
        let ptr = unsafe { gen_new(cstr.as_ptr(), ctx.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: ctx.as_ptr() as *mut context_opaque,
            })
        }
    }

    pub fn from_f64(value: f64, ctx: &crate::context::Context) -> Option<Self> {
        let ptr = unsafe { gen_new_from_double(value, ctx.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: ctx.as_ptr() as *mut context_opaque,
            })
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let cstr = gen_to_string(self.ptr, self.ctx as *mut context_opaque);
            CStr::from_ptr(cstr).to_string_lossy().into_owned()
        }
    }

    pub fn simplify(&self) -> Option<Self> {
        let ptr = unsafe { gen_simplify(self.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn diff(&self, var: &str) -> Option<Self> {
        let cvar = CString::new(var).ok()?;
        let ptr = unsafe { gen_diff(self.ptr, cvar.as_ptr(), self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn integrate(&self) -> Option<Self> {
        let ptr = unsafe { gen_integrate(self.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn add(&self, other: &Gen) -> Option<Self> {
        let ptr = unsafe { gen_add(self.ptr, other.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn sub(&self, other: &Gen) -> Option<Self> {
        let ptr = unsafe { gen_sub(self.ptr, other.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn mul(&self, other: &Gen) -> Option<Self> {
        let ptr = unsafe { gen_mul(self.ptr, other.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn div(&self, other: &Gen) -> Option<Self> {
        let ptr = unsafe { gen_div(self.ptr, other.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn pow(&self, other: &Gen) -> Option<Self> {
        let ptr = unsafe { gen_pow(self.ptr, other.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn symb_plus(&self, other: &Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_plus(self.ptr, other.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn symb_mult(&self, other: &Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_mult(self.ptr, other.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn symb_pow(&self, other: &Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_pow(self.ptr, other.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn pi(ctx: &crate::context::Context) -> Option<Self> {
        let ptr = unsafe { gen_pi(ctx.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: ctx.as_ptr() as *mut context_opaque })
        }
    }

    pub fn e(ctx: &crate::context::Context) -> Option<Self> {
        let ptr = unsafe { gen_e(ctx.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: ctx.as_ptr() as *mut context_opaque })
        }
    }

    pub fn subs(&self, vars: &[&str], values: &[&Gen]) -> Option<Self> {
        if vars.len() != values.len() || vars.is_empty() {
            return None;
        }
        let c_vars: Vec<std::ffi::CString> = vars.iter().map(|&v| std::ffi::CString::new(v).ok()).collect::<Option<_>>()?;
        let c_vars_ptrs: Vec<*const std::os::raw::c_char> = c_vars.iter().map(|c| c.as_ptr()).collect();
        let value_ptrs: Vec<*mut gen_t> = values.iter().map(|g: &&Gen| g.ptr as *mut gen_t).collect();
        let ptr = unsafe {
            gen_subs(
                self.ptr,
                c_vars_ptrs.as_ptr() as *mut *const std::os::raw::c_char,
                value_ptrs.as_ptr() as *mut *mut gen_t,
                vars.len(),
                self.ctx,
            )
        };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn eval(&self) -> Option<Self> {
        let ptr = unsafe { gen_eval(self.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }
}

impl Drop for Gen {
    fn drop(&mut self) {
        unsafe {
            gen_free(self.ptr);
        }
    }
}

extern "C" {
    pub fn gen_new_from_double(value: f64, ctx: *mut context_t) -> *mut gen_t;
}
