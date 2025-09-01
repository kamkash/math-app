#[allow(warnings)]
pub mod giac_ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
pub mod context;
pub mod gen;
pub mod giac_vec;
