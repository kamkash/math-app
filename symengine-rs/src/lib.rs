#[allow(warnings)]
pub mod symengine_ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub mod basic;
