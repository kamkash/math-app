use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::context::ContextOpaque;

#[repr(C)]
struct GenOpaque;
type GenPtr = *mut GenOpaque;

extern "C" {
    fn gen_new(expr: *const c_char, ctx: *mut ContextOpaque) -> GenPtr;
    fn gen_to_string(g: GenPtr, ctx: *mut ContextOpaque) -> *const c_char;
    fn gen_free(g: GenPtr);

    fn gen_simplify(g: GenPtr, ctx: *mut GenOpaque) -> GenPtr;
    fn gen_diff(g: GenPtr, var: *const c_char, ctx: *mut GenOpaque) -> GenPtr;
    fn gen_integrate(g: GenPtr, var: *const c_char, ctx: *mut GenOpaque) -> GenPtr;
}

pub struct Gen {
    ptr: GenPtr,
    ctx: *mut GenOpaque,
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
                ctx:ctx.as_ptr() as *mut GenOpaque,
            })
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let cstr = gen_to_string(self.ptr, self.ctx as *mut ContextOpaque);
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

    pub fn integrate(&self, var: &str) -> Option<Self> {
        let cvar = CString::new(var).ok()?;
        let ptr = unsafe { gen_integrate(self.ptr, cvar.as_ptr(), self.ctx) };
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
