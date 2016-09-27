extern crate mferuby;

use std::ffi::CString;
use std::thread;
use mferuby::*;

macro_rules! cstr {
    ( $slice:expr ) => {
        CString::new($slice).unwrap().as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn mrb_rust_hello(mrb: *mut mrb_state, selfie: mrb_value) -> mrb_value {
    unsafe { mrb_str_new_cstr(mrb, cstr!("Hello")) }
}

#[no_mangle]
pub extern "C" fn mrb_mruby_rust_hello_world_gem_init(mrb: *mut mrb_state) {
    unsafe {
        let klass = mrb_define_module(mrb, cstr!("Rust"));
        mrb_define_class_method(mrb, klass, cstr!("hello"), mrb_rust_hello as *const mrb_func_t, MRB_ARGS_NONE());
    }
}

#[no_mangle]
pub extern "C" fn mrb_mruby_rust_hello_world_gem_final(mrb: *mut mrb_state) {
}
