use crate::giac_ffi::*;

extern "C" {
    // Note: these match the signatures emitted by bindgen (include context pointer)
    pub fn gen_is_vecteur(g: *const gen_t, ctx: *mut context_t) -> i32;
    pub fn gen_vecteur_len(g: *const gen_t, ctx: *mut context_t) -> usize;
    pub fn gen_vecteur_get(g: *const gen_t, i: usize, ctx: *mut context_t) -> *mut gen_t;
}

/// Safety: the returned pointer from `gen_vecteur_get` is allocated by the C++ wrapper.
/// The caller must free it via `gen_free` when done (use via `Gen::from_raw_ptr_owned`).
pub unsafe fn is_vecteur(g: *const gen_t, ctx: *mut context_t) -> bool {
    if g.is_null() || ctx.is_null() { return false; }
    gen_is_vecteur(g, ctx) == 1
}

pub unsafe fn vecteur_len(g: *const gen_t, ctx: *mut context_t) -> usize {
    if g.is_null() || ctx.is_null() { return 0; }
    gen_vecteur_len(g, ctx)
}

pub unsafe fn vecteur_get(g: *const gen_t, i: usize, ctx: *mut context_t) -> *mut gen_t {
    if g.is_null() || ctx.is_null() { return std::ptr::null_mut(); }
    gen_vecteur_get(g, i, ctx)
}
