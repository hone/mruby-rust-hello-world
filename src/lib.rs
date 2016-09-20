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
        mrb_define_class_method(mrb, klass, cstr!("hello"), mrb_rust_hello as *const c_void, MRB_ARGS_NONE());
    }
}

#[no_mangle]
pub extern "C" fn mrb_mruby_rust_gem_final(mrb: &mut mrb_state) {
}

extern {
    #[link_name = "tmrb_nil_value"]
    fn nil() -> *mut c_void;
    #[link_name = "tmrb_fixnum_value"]
    fn fixnum(i: c_int) -> *mut c_void;
    #[link_name = "tmrb_float_value"]
    fn float(mrb: &mut mrb_state, f: c_double) -> *mut c_void;

    fn mrb_str_new_cstr(mrb: &mut mrb_state, c_str: *const c_char) -> *mut c_void;
    fn mrb_open() -> *mut mrb_state;
    fn mrb_define_module(mrb: *mut mrb_state, name: *const c_char) -> *mut RClass;
    fn mrb_define_class_method(mrb: *mut mrb_state, klass: *mut RClass, name: *const c_char, function: *const c_void, spec: ReqArgsSpec);

    #[link_name = "TMRB_ARGS_REQ"]
    fn MRB_ARGS_REQ(count: u32) -> ReqArgsSpec;
    #[link_name = "TMRB_ARGS_NONE"]
    fn MRB_ARGS_NONE() -> ReqArgsSpec;
}

#[allow(non_camel_case_types)]
pub type mrb_state = c_void;
pub type RClass = c_void;
pub type ReqArgsSpec = u32;

#[repr(C)]
pub struct mrb_value {
    padding: [usize; 2],
}

#[repr(C)]
pub struct RClass {
    tt: mrb_vtype,
    color: u32,
    flags: u32,
    c: *mut RClass,
    gcnext: *mut RBasic,
    iv: *mut c_void,
    mt: *mut c_void,
    sup: *mut RClass,
}

#[repr(C)]
enum mrb_vtype {
  MRB_TT_FALSE = 0,   /*   0 */
  MRB_TT_FREE,        /*   1 */
  MRB_TT_TRUE,        /*   2 */
  MRB_TT_FIXNUM,      /*   3 */
  MRB_TT_SYMBOL,      /*   4 */
  MRB_TT_UNDEF,       /*   5 */
  MRB_TT_FLOAT,       /*   6 */
  MRB_TT_CPTR,        /*   7 */
  MRB_TT_OBJECT,      /*   8 */
  MRB_TT_CLASS,       /*   9 */
  MRB_TT_MODULE,      /*  10 */
  MRB_TT_ICLASS,      /*  11 */
  MRB_TT_SCLASS,      /*  12 */
  MRB_TT_PROC,        /*  13 */
  MRB_TT_ARRAY,       /*  14 */
  MRB_TT_HASH,        /*  15 */
  MRB_TT_STRING,      /*  16 */
  MRB_TT_RANGE,       /*  17 */
  MRB_TT_EXCEPTION,   /*  18 */
  MRB_TT_FILE,        /*  19 */
  MRB_TT_ENV,         /*  20 */
  MRB_TT_DATA,        /*  21 */
  MRB_TT_FIBER,       /*  22 */
  MRB_TT_MAXDEFINE,    /*  23 */
}

#[repr(C)]
pub struct RBasic {
    tt: mrb_vtype,
    color: u32,
    flags: u32,
    c: *mut RClass,
    gcnext: *mut RBasic,
}
