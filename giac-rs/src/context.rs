use std::os::raw::c_void;

#[repr(C)]
pub struct ContextOpaque(c_void);
pub type ContextPtr = *mut ContextOpaque;

extern "C" {
    fn context_new() -> ContextPtr;
    fn context_free(ctx: ContextPtr);
}

pub struct Context {
    ptr: ContextPtr,
}

impl Context {
    pub fn new() -> Self {
        let ptr = unsafe { context_new() };
        assert!(!ptr.is_null());
        Context { ptr }
    }

    pub fn as_ptr(&self) -> ContextPtr {
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
