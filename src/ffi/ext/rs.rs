// automatically generated by rust-bindgen
use libc::{c_void, size_t};

pub unsafe fn Calloc(n: size_t, t: size_t) -> *mut ::libc::c_void {
    R_chk_calloc(n, t)
}

pub unsafe fn Realloc(p: *mut c_void, n: size_t, t: size_t) -> *mut c_void {
    R_chk_realloc(p, n * t)
}

pub unsafe fn Free(p: *mut c_void) -> () {
    R_chk_free(p)
}
#[link(name = "R")]
extern {
    pub fn R_chk_calloc(arg1: size_t, arg2: size_t) -> *mut ::libc::c_void;
    pub fn R_chk_realloc(arg1: *mut ::libc::c_void, arg2: size_t) -> *mut ::libc::c_void;
    pub fn R_chk_free(arg1: *mut ::libc::c_void) -> ();
    pub fn call_R(arg1: *mut ::libc::c_char,
                  arg2: ::libc::c_long,
                  arg3: *mut *mut ::libc::c_void,
                  arg4: *mut *mut ::libc::c_char,
                  arg5: *mut ::libc::c_long,
                  arg6: *mut *mut ::libc::c_char,
                  arg7: ::libc::c_long,
                  arg8: *mut *mut ::libc::c_char)
                  -> ();
}
