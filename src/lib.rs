extern crate libc;
use std::ffi::CString;
use libc::{c_void, c_char};
use std::thread;

#[no_mangle]
pub extern "C" fn mrb_rust_hello() -> *mut c_void {
    println!("Hello World");
    unsafe { nil() }
}

macro_rules! cstr {
    ( $slice:expr ) => {
        CString::new($slice).unwrap().as_ptr()
    }
}

#[no_mangle]
pub extern "C" fn mrb_mruby_rust_gem_init(mrb: &mut mrb_state) {
    let klass = unsafe { mrb_define_module(mrb, cstr!("Rust")) };
    unsafe { mrb_define_class_method(mrb, klass, cstr!("hello"), mrb_rust_hello as *const c_void, TMRB_ARGS_REQ(0)) };
}

#[no_mangle]
pub extern "C" fn mrb_mruby_rust_gem_final(mrb: &mut mrb_state) {
}

extern {
    #[link_name = "tmrb_nil_value"]
    fn nil() -> *mut c_void;

    fn mrb_define_module(mrb: &mut mrb_state, name: *const c_char) -> &mut RClass;
    fn mrb_define_class_method(mrb: &mut mrb_state, klass: &mut RClass, name: *const c_char, function: *const c_void, spec: ReqArgsSpec);

    fn TMRB_ARGS_REQ(count: u32) -> ReqArgsSpec;
}

pub type mrb_state = c_void;
pub type RClass = c_void;
pub type ReqArgsSpec = u32;
