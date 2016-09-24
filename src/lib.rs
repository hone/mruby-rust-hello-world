extern crate libc;
use std::ffi::CString;
use libc::{c_void, c_char, c_int, c_double};
use std::thread;

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
pub extern "C" fn mrb_mruby_rust_gem_init(mrb: *mut mrb_state) {
    unsafe {
        let klass = mrb_define_module(mrb, cstr!("Rust"));
        mrb_define_class_method(mrb, klass, cstr!("hello"), mrb_rust_hello as *const mrb_func_t, MRB_ARGS_NONE());
    }
}

#[no_mangle]
pub extern "C" fn mrb_mruby_rust_gem_final(mrb: *mut mrb_state) {
}

extern {
    #[link_name = "tmrb_nil_value"]
    fn nil() -> mrb_value;
    #[link_name = "tmrb_fixnum_value"]
    fn fixnum(i: c_int) -> mrb_value;
    #[link_name = "tmrb_float_value"]
    fn float(mrb: *mut mrb_state, f: c_double) -> mrb_value;

    fn mrb_open() -> *mut mrb_state;
    fn mrb_str_new_cstr(mrb: *mut mrb_state, c_str: *const c_char) -> mrb_value;
    fn mrb_define_module(mrb: *mut mrb_state, name: *const c_char) -> *mut RClass;
    fn mrb_define_class_method(mrb: *mut mrb_state, klass: *mut RClass, name: *const c_char, function: *const mrb_func_t, spec: mrb_aspec);

    #[link_name = "TMRB_ARGS_REQ"]
    fn MRB_ARGS_REQ(count: u32) -> mrb_aspec;
    #[link_name = "TMRB_ARGS_NONE"]
    fn MRB_ARGS_NONE() -> mrb_aspec;
}

#[allow(non_camel_case_types)]
pub type mrb_aspec = u32;

#[repr(C)]
pub struct mrb_state {
    padding: [u8; 376],
}

#[repr(C)]
pub struct mrb_value {
    padding: [usize; 2],
}

pub type RClass = c_void;
pub type mrb_func_t = c_void;
