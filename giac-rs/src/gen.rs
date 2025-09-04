//! GIAC FFI ownership and usage notes
//!
//! - Ownership model:
//!   - `Gen` is an owning wrapper for the underlying `gen_t` returned by the GIAC FFI.
//!     It calls the FFI `gen_free` in `Drop` (unless the `ctx` is null for library-owned singletons).
//!   - `Clone`/`deep_clone()` create an owned duplicate via the FFI `gen_clone`.
//!   - Use `Gen::deep_clone()` when you need a separate owned copy of the underlying `gen_t`.
//! - Cheap, non-owning views:
//!   - `GenRef` is a lightweight, non-owning, `Copy` view containing the raw pointers.
//!     It does not manage memory and must not outlive the owning `Gen` or its `Context`.
//!   - Prefer passing `&Gen` (or `GenRef`) to APIs rather than moving `Gen` by value.
//! - CI/tests:
//!   - The underlying GIAC library is not guaranteed thread-safe. For deterministic tests
//!     that exercise the GIAC FFI, run test suites single-threaded in CI:
//!       RUST_TEST_THREADS=1 cargo test --workspace
use math_core::common::LogicalOperator;
use regex::Regex;

use crate::{context::Context, giac_ffi::*};
use crate::giac_vec;
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

/// A lightweight, non-owning view of a `Gen` suitable for cheap copies.
///
/// `GenRef` is `Copy` and `Clone` and simply contains the raw pointers
/// from a `Gen`. It does not manage the underlying memory and therefore
/// must not outlive the owning `Gen` (or the context it refers to).
/// Use `Gen::as_ref()` to obtain a `GenRef` from a `&Gen`.
#[derive(Copy, Clone)]
pub struct GenRef {
    ptr: *mut gen_t,
    ctx: *mut context_t,
}
impl GenRef {
    pub fn ptr(&self) -> *mut gen_t {
        self.ptr
    }

    pub fn ctx(&self) -> *mut context_t {
        self.ctx
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let cstr = gen_to_string(self.ptr, self.ctx as *mut context_opaque);
            CStr::from_ptr(cstr).to_string_lossy().into_owned()
        }
    }
}

impl fmt::Display for GenRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Debug for GenRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Gen {
    /// Return a non-owning, Copyable view of this `Gen`.
    pub fn as_ref(&self) -> GenRef {
        GenRef {
            ptr: self.ptr,
            ctx: self.ctx,
        }
    }
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
    pub fn new(expr: &str, ctx: &Context) -> Option<Self> {
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

    /// Parse a string expression into a `Gen` using the explicit C parse wrapper.
    /// This delegates to the FFI `gen_parse` which is an alias around `giac::gen`.
    pub fn parse(expr: &str, ctx: &Context) -> Option<Self> {
        let cstr = CString::new(expr).ok()?;
        let ptr = unsafe { gen_parse(cstr.as_ptr(), ctx.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen {
                ptr,
                ctx: ctx.as_ptr() as *mut context_opaque,
            })
        }
    }

    pub fn from_f64(value: f64, ctx: &Context) -> Option<Self> {
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

    pub fn symbols<const N: usize>(names: [&str; N], ctx: &Context) -> [Gen; N] {
        names.map(|name| {
            Gen::symbol(name, ctx).unwrap_or_else(|| {
                panic!("Failed to create symbol for '{}'", name);
            })
        })
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

    /// Returns the symbolic square root of the given Gen (implemented as symb_pow(x, 1/2)).
    pub fn symb_sqrt(&self) -> Option<Self> {
        unsafe {
            let ptr = gen_symb_sqrt(self.ptr, self.ctx);
            if ptr.is_null() {
                return None;
            }
            // try to evaluate the symbolic sqrt to get a plain numeric when possible
            let eval_ptr = gen_eval(ptr, self.ctx);
            if !eval_ptr.is_null() {
                // free original symbolic pointer and use evaluated pointer
                gen_free(ptr);
                return Some(Gen {
                    ptr: eval_ptr,
                    ctx: self.ctx,
                });
            }
            Some(Gen { ptr, ctx: self.ctx })
        }
    }

    /// Returns the symbolic nth root of the given Gen (i.e., x^(1/n)).
    pub fn symb_root(&self, n: f64) -> Option<Self> {
        if n == 0.0 {
            return None;
        }
        unsafe {
            let exp = 1.0f64 / n;
            let exp_ptr = gen_new_from_double(exp, self.ctx);
            if exp_ptr.is_null() {
                return None;
            }
            let ptr = gen_symb_pow(self.ptr, exp_ptr, self.ctx);
            gen_free(exp_ptr);
            if ptr.is_null() {
                None
            } else {
                Some(Gen { ptr, ctx: self.ctx })
            }
        }
    }

    pub fn pi(ctx: &Context) -> Option<Self> {
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

    pub fn e(ctx: &Context) -> Option<Self> {
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
        let c_vars: Vec<CString> = vars
            .iter()
            .map(|&v| CString::new(v).ok())
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
    pub fn sin(symbol: &Gen) -> Option<Self> {
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
    pub fn cos(symbol: &Gen) -> Option<Self> {
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
    pub fn tan(symbol: &Gen) -> Option<Self> {
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
    pub fn log(symbol: &Gen) -> Option<Self> {
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

    pub fn logb(symbol: &Gen, base: f64) -> Option<Self> {
        let ctx = Context::new();
        let base_gen = Gen::from_f64(base, &ctx)?;
        let b = Gen::ln(&base_gen)?;
        let n = Gen::ln(symbol)?;
        n.div(&b)
    }

    pub fn log10(symbol: &Gen) -> Option<Self> {
        Gen::logb(symbol, 10.0)
    }

    /// Returns the symbolic natural logarithm (ln) of the given Gen.
    pub fn ln(symbol: &Gen) -> Option<Self> {
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
    pub fn exp(symbol: &Gen) -> Option<Self> {
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
    pub fn symbol(name: &str, ctx: &Context) -> Option<Self> {
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

    /// Attempts to convert the Gen to a f64 if it represents a number.
    pub fn to_f64(&self) -> Option<f64> {
        unsafe {
            let mut out: f64 = 0.0;
            let success = gen_to_f64(self.ptr, &mut out as *mut f64);
            if success == 1 {
                return Some(out);
            }
            // Fallback: some GIAC operations return a singleton vecteur (e.g. "[10.0]").
            // Try to extract a numeric element from a singleton vecteur using the FFI helpers.
            if !self.ptr.is_null() && !self.ctx.is_null() && giac_vec::is_vecteur(self.ptr as *const gen_t, self.ctx) {
                let len = giac_vec::vecteur_len(self.ptr as *const gen_t, self.ctx);
                if len == 1 {
                    let elem_ptr = giac_vec::vecteur_get(self.ptr as *const gen_t, 0, self.ctx);
                    if !elem_ptr.is_null() {
                        let mut out2: f64 = 0.0;
                        let ok = gen_to_f64(elem_ptr as *mut gen_t, &mut out2 as *mut f64);
                        gen_free(elem_ptr as *mut gen_t);
                        if ok == 1 {
                            return Some(out2);
                        }
                    }
                }
            }
            None
        }
    }

    /// If this Gen represents a singleton list (e.g. `[10.0]`) return its numeric element.
    /// This uses the C++ vecteur helpers to avoid brittle string parsing.
    pub fn extract_singleton_f64(&self) -> Option<f64> {
        unsafe {
            // direct numeric
            if let Some(v) = self.to_f64() {
                return Some(v);
            }
            // check vecteur using the context
            if giac_vec::is_vecteur(self.ptr as *const gen_t, self.ctx) {
                let len = giac_vec::vecteur_len(self.ptr as *const gen_t, self.ctx);
                if len == 1 {
                    let elem_ptr = giac_vec::vecteur_get(self.ptr as *const gen_t, 0, self.ctx);
                    if elem_ptr.is_null() {
                        return None;
                    }
                    // elem_ptr was allocated by the C++ wrapper; try to convert to f64
                    let mut out: f64 = 0.0;
                    let ok = gen_to_f64(elem_ptr as *mut gen_t, &mut out as *mut f64);
                    // free the temporary gen_t returned by vecteur_get
                    gen_free(elem_ptr as *mut gen_t);
                    if ok == 1 {
                        return Some(out);
                    }
                }
            }
            None
        }
    }

    /// Create an owned duplicate of the underlying `gen_t` using the FFI `gen_clone`.
    /// Returns `None` if cloning failed.
    pub fn deep_clone(&self) -> Option<Self> {
        if self.ptr.is_null() {
            return None;
        }
        let ptr = unsafe { gen_clone(self.ptr, self.ctx) };
        if ptr.is_null() {
            None
        } else {
            Some(Gen { ptr, ctx: self.ctx })
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

// Note: `Gen` is intentionally Copy and Clone (trivial bitwise copy).
// Cloning is a cheap pointer copy. If you need an owned duplicate of the
// underlying `gen_t`, call the FFI `gen_clone` manually via a helper if
// required (not provided here to keep the type Copy).
impl Drop for Gen {
    fn drop(&mut self) {
        // Only free if we have a non-null context; static or library-owned
        // gen_t pointers are created with a null context in this crate
        // (see the GEN_* lazy_static values) and must not be freed here.
        unsafe {
            if !self.ptr.is_null() && !self.ctx.is_null() {
                gen_free(self.ptr);
            }
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
