use crate::giac_ffi::*;

use std::marker::PhantomData;
use std::rc::Rc;

pub struct Context {
    ptr: *mut context_t,
    // Prevent Send and Sync by including Rc, which is neither Send nor Sync
    _not_send_sync: PhantomData<Rc<()>>,
}

// SAFETY: GIAC context is !Send and !Sync — don't allow cross-thread access

impl Context {
    pub fn new() -> Self {
        let ptr: *mut context_t = unsafe { context_new() };
        assert!(!ptr.is_null());
        Context { ptr, _not_send_sync: PhantomData }
    }

    pub fn as_ptr(&self) -> *mut context_t {
        self.ptr
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            context_free(self.ptr);
        }
    }
}
