// automatically generated by rust-bindgen

use super::ext::boolean::*;

#[link(name = "R")]
extern {
    pub static mut R_DirtyImage: ::libc::c_int;
    pub static mut R_TempDir: *mut ::libc::c_char;
}
#[link(name = "R")]
extern {
    pub fn Rf_initEmbeddedR(argc: ::libc::c_int, argv: *mut *mut ::libc::c_char) -> ::libc::c_int;
    pub fn Rf_endEmbeddedR(fatal: ::libc::c_int) -> ();
    pub fn Rf_initialize_R(ac: ::libc::c_int, av: *mut *mut ::libc::c_char) -> ::libc::c_int;
    pub fn setup_Rmainloop() -> ();
    pub fn R_ReplDLLinit() -> ();
    pub fn R_ReplDLLdo1() -> ::libc::c_int;
    pub fn R_setStartTime() -> ();
    pub fn R_RunExitFinalizers() -> ();
    pub fn CleanEd() -> ();
    pub fn Rf_KillAllDevices() -> ();
    pub fn R_CleanTempDir() -> ();
    pub fn R_SaveGlobalEnv() -> ();
    pub fn fpu_setup(start: Rboolean) -> ();
}