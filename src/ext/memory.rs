// automatically generated by rust-bindgen
use libc::size_t;

#[link(name = "R")]
extern {
    pub fn vmaxget() -> *mut ::libc::c_void;
    pub fn vmaxset(arg1: *const ::libc::c_void) -> ();
    pub fn R_gc() -> ();
    pub fn R_gc_running() -> ::libc::c_int;
    pub fn R_alloc(arg1: size_t, arg2: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn R_allocLD(nelem: size_t) -> *mut ::libc::c_double;
    pub fn S_alloc(arg1: ::libc::c_long, arg2: ::libc::c_int) -> *mut ::libc::c_char;
    pub fn S_realloc(arg1: *mut ::libc::c_char,
                     arg2: ::libc::c_long,
                     arg3: ::libc::c_long,
                     arg4: ::libc::c_int)
                     -> *mut ::libc::c_char;
}