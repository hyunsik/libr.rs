//! automatically generated by rust-bindgen


pub use super::libextern::*;

#[link(name = "R")]
extern {
    pub static mut R_NaN: ::libc::c_double;
    pub static mut R_PosInf: ::libc::c_double;
    pub static mut R_NegInf: ::libc::c_double;
    pub static mut R_NaReal: ::libc::c_double;
    pub static mut R_NaInt: ::libc::c_int;
}
#[link(name = "R")]
extern {
    pub fn isnan(__value: ::libc::c_double) -> ::libc::c_int;
    pub fn finite(__value: ::libc::c_double) -> ::libc::c_int;
    pub fn R_IsNA(arg1: ::libc::c_double) -> ::libc::c_int;
    pub fn R_IsNaN(arg1: ::libc::c_double) -> ::libc::c_int;
    pub fn R_finite(arg1: ::libc::c_double) -> ::libc::c_int;
}

pub unsafe fn ISNA(x: ::libc::c_double) -> ::libc::c_int {
    R_IsNA(x)
}
pub unsafe fn ISNAN(x: ::libc::c_double) -> ::libc::c_int {
    (isnan(x) != 0) as ::libc::c_int
}

pub unsafe fn R_FINITE(x: ::libc::c_double) -> ::libc::c_int {
    R_finite(x)
}
