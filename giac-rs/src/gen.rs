use math_core::common::LogicalOperator;
use regex::Regex;

use crate::giac_ffi::*;
use std::{
    ffi::{CStr, CString},
    fmt,
};

lazy_static::lazy_static! {
    pub static ref RE_LOG_BASE: Regex = Regex::new(r"^log_(\d+)$").unwrap();
    pub static ref GEN_ADD: Gen = Gen {
        ptr: unsafe {
            get_add_op()
        },
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_SUB: Gen = Gen {
        ctx: std::ptr::null_mut(),
        ptr: unsafe{get_sub_op()},
    };

    pub static ref GEN_MUL: Gen = Gen {
        ptr: unsafe{get_mul_op()},
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_DIV: Gen = Gen {
        ptr: unsafe{get_div_op()},
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_POW: Gen = Gen {
        ptr: unsafe{get_pow_op()},
        ctx: std::ptr::null_mut(),
    };

    pub static ref GEN_EQ: Gen = Gen {
        ptr: unsafe { get_eq_op() },
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_NE: Gen = Gen {
        ptr: unsafe { get_ne_op() },
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_LT: Gen = Gen {
        ptr: unsafe { get_lt_op() },
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_LE: Gen = Gen {
        ptr: unsafe { get_le_op() },
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_GT: Gen = Gen {
        ptr: unsafe { get_gt_op() },
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_GE: Gen = Gen {
        ptr: unsafe { get_ge_op() },
        ctx: std::ptr::null_mut(),
    };

    pub static ref GEN_AND: Gen = Gen {
        ptr: unsafe { get_and_op() },
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_OR: Gen = Gen {
        ptr: unsafe { get_or_op() },
        ctx: std::ptr::null_mut(),
    };
    pub static ref GEN_NOT: Gen = Gen {
        ptr: unsafe { get_not_op() },
        ctx: std::ptr::null_mut(),
    };

}

pub struct Gen {
    ptr: *mut gen_t,
    ctx: *mut context_t,
}

// SAFETY: Gen only contains raw pointers and does not manage thread-local state.
// You must ensure thread safety when using Gen across threads.
unsafe impl Send for Gen {}
unsafe impl Sync for Gen {}

impl std::hash::Hash for Gen {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Convert the Basic to a string and hash the string.
        // This is a simple way to generate a hash code.
        self.to_string().hash(state);
    }
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

    /// Returns the symbolic division of self and other (i.e., symbolic(at_divide, ...)).
    pub fn symb_div(&self, other: &Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_div(self.ptr, other.ptr, self.ctx) };
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

    pub fn symb_log(&self) -> Option<Self> {
        let ptr = unsafe { gen_symb_log(self.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn symb_ln(&self) -> Option<Self> {
        let ptr = unsafe { gen_symb_ln(self.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn symb_exp(&self) -> Option<Self> {
        let ptr = unsafe { gen_symb_exp(self.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn symb_sin(&self) -> Option<Self> {
        let ptr = unsafe { gen_symb_sin(self.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn symb_cos(&self) -> Option<Self> {
        let ptr = unsafe { gen_symb_cos(self.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    pub fn symb_tan(&self) -> Option<Self> {
        let ptr = unsafe { gen_symb_tan(self.ptr, self.ctx) };
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
            Some(Gen {
                ptr,
                ctx: ctx.as_ptr() as *mut context_opaque,
            })
        }
    }

    pub fn e(ctx: &crate::context::Context) -> Option<Self> {
        let ptr = unsafe { gen_e(ctx.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: ctx.as_ptr() as *mut context_opaque,
            })
        }
    }

    pub fn subs(&self, vars: &[&str], values: &[&Gen]) -> Option<Self> {
        if vars.len() != values.len() || vars.is_empty() {
            return None;
        }
        let c_vars: Vec<std::ffi::CString> = vars
            .iter()
            .map(|&v| std::ffi::CString::new(v).ok())
            .collect::<Option<_>>()?;
        let c_vars_ptrs: Vec<*const std::os::raw::c_char> =
            c_vars.iter().map(|c| c.as_ptr()).collect();
        let value_ptrs: Vec<*mut gen_t> =
            values.iter().map(|g: &&Gen| g.ptr as *mut gen_t).collect();
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

    pub fn is_number(&self) -> bool {
        unsafe { is_number(self.ptr) == 1 }
    }

    pub fn is_constant(&self) -> bool {
        unsafe { is_constant(self.ptr) == 1 }
    }

    pub fn is_symbol(&self) -> bool {
        unsafe { is_symbol(self.ptr) == 1 }
    }

    pub fn equals(&self, other: &Gen) -> bool {
        unsafe { equals(self.ptr, other.ptr) == 1 }
    }

    pub fn is_add(&self) -> bool {
        unsafe { is_add(self.ptr) == 1 }
    }

    pub fn is_sub(&self) -> bool {
        unsafe { is_sub(self.ptr) == 1 }
    }

    pub fn is_mul(&self) -> bool {
        unsafe { is_mul(self.ptr) == 1 }
    }

    pub fn is_div(&self) -> bool {
        unsafe { is_div(self.ptr) == 1 }
    }

    pub fn is_pow(&self) -> bool {
        unsafe { is_pow(self.ptr) == 1 }
    }

    pub fn is_op(&self) -> bool {
        self.is_add() || self.is_sub() || self.is_mul() || self.is_div() || self.is_pow()
    }

    pub fn is_and(&self) -> bool {
        unsafe { is_and(self.ptr) == 1 }
    }

    pub fn is_or(&self) -> bool {
        unsafe { is_or(self.ptr) == 1 }
    }

    pub fn is_not(&self) -> bool {
        unsafe { is_not(self.ptr) == 1 }
    }

    pub fn is_eq(&self) -> bool {
        unsafe { is_eq(self.ptr) == 1 }
    }

    pub fn is_ne(&self) -> bool {
        unsafe { is_ne(self.ptr) == 1 }
    }

    pub fn is_lt(&self) -> bool {
        unsafe { is_lt(self.ptr) == 1 }
    }

    pub fn is_le(&self) -> bool {
        unsafe { is_le(self.ptr) == 1 }
    }

    pub fn is_gt(&self) -> bool {
        unsafe { is_gt(self.ptr) == 1 }
    }

    pub fn is_ge(&self) -> bool {
        unsafe { is_ge(self.ptr) == 1 }
    }

    pub fn is_logical_op(&self) -> bool {
        self.is_and()
            || self.is_or()
            || self.is_not()
            || self.is_eq()
            || self.is_ne()
            || self.is_lt()
            || self.is_le()
            || self.is_gt()
            || self.is_ge()
    }

    pub fn logical_op(op: LogicalOperator) -> Option<Gen> {
        match op {
            LogicalOperator::Eq => Some(GEN_EQ.clone()),
            LogicalOperator::DoubleEq => Some(GEN_EQ.clone()),
            LogicalOperator::Lt => Some(GEN_LT.clone()),
            LogicalOperator::Gt => Some(GEN_GT.clone()),
            LogicalOperator::Lte => Some(GEN_LE.clone()),
            LogicalOperator::Gte => Some(GEN_GE.clone()),
            _ => None,
        }
    }

    pub fn ptr(&self) -> *mut gen_t {
        self.ptr
    }

    pub fn ctx(&self) -> *mut context_t {
        self.ctx
    }

    /// Returns the symbolic sine of the given Gen.
    pub fn sin(symbol: Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_sin(symbol.ptr, symbol.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: symbol.ctx,
            })
        }
    }

    /// Returns the symbolic cosine of the given Gen.
    pub fn cos(symbol: Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_cos(symbol.ptr, symbol.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: symbol.ctx,
            })
        }
    }

    /// Returns the symbolic tangent of the given Gen.
    pub fn tan(symbol: Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_tan(symbol.ptr, symbol.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: symbol.ctx,
            })
        }
    }

    /// Returns the symbolic logarithm (base 10) of the given Gen.
    pub fn log(symbol: Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_log(symbol.ptr, symbol.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: symbol.ctx,
            })
        }
    }

    /// Returns the symbolic natural logarithm (ln) of the given Gen.
    pub fn ln(symbol: Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_ln(symbol.ptr, symbol.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: symbol.ctx,
            })
        }
    }

    /// Returns the symbolic exponential of the given Gen.
    pub fn exp(symbol: Gen) -> Option<Self> {
        let ptr = unsafe { gen_symb_exp(symbol.ptr, symbol.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: symbol.ctx,
            })
        }
    }

    /// Creates a new symbolic variable with the given name in the given context.
    pub fn symbol(name: &str, ctx: &crate::context::Context) -> Option<Self> {
        let cstr = CString::new(name).ok()?;
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

    /// Attempts to convert the Gen to an f64 if it represents a number.
    pub fn to_f64(&self) -> Option<f64> {
        unsafe {
            let mut out: f64 = 0.0;
            let success = gen_to_f64(self.ptr, &mut out as *mut f64);
            if success == 1 {
                Some(out)
            } else {
                None
            }
        }
    }
}

impl Default for Gen {
    fn default() -> Self {
        Self {
            ptr: std::ptr::null_mut(),
            ctx: std::ptr::null_mut(),
        }
    }
}

impl fmt::Display for Gen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for Gen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl PartialEq for Gen {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for Gen {}

impl Drop for Gen {
    fn drop(&mut self) {
        unsafe {
            gen_free(self.ptr);
        }
    }
}

impl Clone for Gen {
    fn clone(&self) -> Self {
        let ptr = unsafe { gen_clone(self.ptr, self.ctx) };
        if ptr.is_null() {
            Gen::default()
        } else {
            Gen { ptr, ctx: self.ctx }
        }
    }
}
