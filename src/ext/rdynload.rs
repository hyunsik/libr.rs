#![allow(non_camel_case_types)]
use super::boolean::*;

#[allow(dead_code)]
const SINGLESXP: u32 = 302;

#[allow(non_camel_case_types)]
pub type DL_FUNC =
    ::std::option::Option<extern "C" fn() -> *mut ::libc::c_void>;
#[allow(non_camel_case_types)]
pub type R_NativePrimitiveArgType = ::libc::c_uint;
#[allow(non_camel_case_types)]
pub type R_NativeObjectArgType = ::libc::c_uint;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum R_NativeArgStyle {
    R_ARG_IN = 0,
    R_ARG_OUT,
    R_ARG_IN_OUT,
    R_IRRELEVANT,
}

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct R_CMethodDef {
    pub name: *const ::libc::c_char,
    pub fun: DL_FUNC,
    pub numArgs: ::libc::c_int,
    pub types: *mut R_NativePrimitiveArgType,
    pub styles: *mut R_NativeArgStyle,
}

impl ::std::clone::Clone for R_CMethodDef {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for R_CMethodDef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(non_camel_case_types)]
pub type R_FortranMethodDef = R_CMethodDef;

#[repr(C)]
#[derive(Copy)]
#[allow(non_snake_case)]
pub struct R_CallMethodDef {
    pub name: *const ::libc::c_char,
    pub fun: DL_FUNC,
    pub numArgs: ::libc::c_int,
}
impl ::std::clone::Clone for R_CallMethodDef {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for R_CallMethodDef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[allow(non_camel_case_types)]
pub type R_ExternalMethodDef = R_CallMethodDef;
#[allow(non_camel_case_types)]
pub enum Struct__DllInfo { }
pub type DllInfo = Struct__DllInfo;

#[allow(non_camel_case_types)]
pub enum Struct_Rf_RegisteredNativeSymbol { }
#[allow(non_camel_case_types)]
pub type R_RegisteredNativeSymbol = Struct_Rf_RegisteredNativeSymbol;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum NativeSymbolType {
    R_ANY_SYM = 0,
    R_C_SYM,
    R_CALL_SYM,
    R_FORTRAN_SYM,
    R_EXTERNAL_SYM,
}

#[link(name = "R")]
extern {
    pub fn R_registerRoutines(info: *mut DllInfo,
                              croutines: *const R_CMethodDef,
                              callRoutines: *const R_CallMethodDef,
                              fortranRoutines: *const R_FortranMethodDef,
                              externalRoutines: *const R_ExternalMethodDef)
                              -> ::libc::c_int;
    pub fn R_useDynamicSymbols(info: *mut DllInfo, value: Rboolean) -> Rboolean;
    pub fn R_forceSymbols(info: *mut DllInfo, value: Rboolean) -> Rboolean;
    pub fn R_getDllInfo(name: *const ::libc::c_char) -> *mut DllInfo;
    pub fn R_getEmbeddingDllInfo() -> *mut DllInfo;
    pub fn R_FindSymbol(arg1: *const ::libc::c_char,
                        arg2: *const ::libc::c_char,
                        symbol: *mut R_RegisteredNativeSymbol)
                        -> DL_FUNC;
    pub fn R_RegisterCCallable(package: *const ::libc::c_char,
                               name: *const ::libc::c_char,
                               fptr: DL_FUNC)
                               -> ();
    pub fn R_GetCCallable(package: *const ::libc::c_char, name: *const ::libc::c_char) -> DL_FUNC;
}
