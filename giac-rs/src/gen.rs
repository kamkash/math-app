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
}

impl Drop for Gen {
    fn drop(&mut self) {
        unsafe {
            gen_free(self.ptr);
        }
    }
}
