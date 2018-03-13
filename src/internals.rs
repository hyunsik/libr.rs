use std::fmt;

pub use super::ext::arith::*;
pub use super::ext::boolean::*;
pub use super::ext::complex::*;
pub use super::ext::error::*;
pub use super::ext::memory::*;
pub use super::ext::utils::*;
pub use super::ext::print::*;

pub use super::ext::libextern::*;

pub use self::SEXPTYPE::*;
pub use libc::ptrdiff_t;
use libc::FILE;

#[allow(non_camel_case_types)]
pub type Rbyte = ::libc::c_uchar;
#[allow(non_camel_case_types)]
pub type R_len_t = ::libc::c_int;
#[allow(non_camel_case_types)]
pub type R_xlen_t = ptrdiff_t;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct R_long_vec_hdr_t {
    pub lv_length: R_xlen_t,
    pub lv_truelength: R_xlen_t,
}

impl ::std::default::Default for R_long_vec_hdr_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
pub enum SEXPTYPE {
    NILSXP     = 0,   /* nil = NULL */
    SYMSXP     = 1,   /* symbols */
    LISTSXP    = 2,   /* lists of dotted pairs */
    CLOSXP     = 3,   /* closures */
    ENVSXP     = 4,   /* environments */
    PROMSXP    = 5,   /* promises: [un]evaluated closure arguments */
    LANGSXP    = 6,   /* language constructs (special lists) */
    SPECIALSXP = 7,   /* special forms */
    BUILTINSXP = 8,   /* builtin non-special forms */
    CHARSXP    = 9,   /* "scalar" string type (internal only)*/
    LGLSXP     = 10,  /* logical vectors */
    // 11 and 12 were factors and ordered factors in the 1990s
    INTSXP     = 13,  /* integer vectors */
    REALSXP    = 14,  /* real variables */
    CPLXSXP    = 15,  /* complex variables */
    STRSXP     = 16,  /* string vectors */
    DOTSXP     = 17,  /* dot-dot-dot object */
    ANYSXP     = 18,  /* make "any" args work. Used in specifying types for symbol registration to mean anything is okay  */
    VECSXP     = 19,  /* generic vectors */
    EXPRSXP    = 20,  /* expressions vectors */
    BCODESXP   = 21,  /* byte code */
    EXTPTRSXP  = 22,  /* external pointer */
    WEAKREFSXP = 23,  /* weak reference */
    RAWSXP     = 24,  /* raw bytes */
    S4SXP      = 25,  /* S4, non-vector */

    NEWSXP     = 30,  /* fresh node created in new page */
    FREESXP    = 31,  /* node released by GC */
    FUNSXP     = 99,  /* Closure or Builtin or Special */
}

impl SEXPTYPE {
    pub fn name(&self) -> &'static str {
        match *self {
            NILSXP     => "NILSXP",
            SYMSXP     => "SYMSXP",
            LISTSXP    => "LISTSXP",
            CLOSXP     => "CLOSXP",
            ENVSXP     => "ENVSXP",
            PROMSXP    => "PROMSXP",
            LANGSXP    => "LANGSXP",
            SPECIALSXP => "SPECIALSXP",
            BUILTINSXP => "BUILTINSXP",
            CHARSXP    => "CHARSXP",
            LGLSXP     => "LGLSXP",
            INTSXP     => "INTSXP",
            REALSXP    => "REALSXP",
            CPLXSXP    => "CPLXSXP",
            STRSXP     => "STRSXP",
            DOTSXP     => "DOTSXP",
            ANYSXP     => "ANYSXP",
            VECSXP     => "VECSXP",
            EXPRSXP    => "EXPRSXP",
            BCODESXP   => "BCODESXP",
            EXTPTRSXP  => "EXTPTRSXP",
            WEAKREFSXP => "WEAKREFSXP",
            RAWSXP     => "RAWSXP",
            S4SXP      => "S4SXP",
            NEWSXP     => "NEWSXP",
            FREESXP    => "FREESXP",
            FUNSXP     => "FUNSXP",
        }
    }

    pub fn from_isize(n: isize) -> SEXPTYPE {
        match n {
            0 => NILSXP,
            1 => SYMSXP,
            2 => LISTSXP,
            3 => CLOSXP,
            4 => ENVSXP,
            5 => PROMSXP,
            6 => LANGSXP,
            7 => SPECIALSXP,
            8 => BUILTINSXP,
            9 => CHARSXP,
            10 => LGLSXP,
            13 => INTSXP,
            14 => REALSXP,
            15 => CPLXSXP,
            16 => STRSXP,
            17 => DOTSXP,
            18 => ANYSXP,
            19 => VECSXP,
            20 => EXPRSXP,
            21 => BCODESXP,
            22 => EXTPTRSXP,
            23 => WEAKREFSXP,
            24 => RAWSXP,
            25 => S4SXP,
            30 => NEWSXP,
            31 => FREESXP,
            99 => FUNSXP,
            _ => panic!("unknown SEXPTYPE: {}", n)
        }
    }
}

impl fmt::Display for SEXPTYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Debug for SEXPTYPE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[allow(non_camel_case_types)]
pub enum Struct_SEXPREC { }
pub type SEXP = *mut Struct_SEXPREC;

#[allow(non_camel_case_types)]
pub type PROTECT_INDEX = ::libc::c_int;

#[allow(non_camel_case_types)]
pub enum Struct_R_allocator { }
#[allow(non_camel_case_types)]
pub type R_allocator_t = Struct_R_allocator;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum cetype_t {
    CE_NATIVE = 0,
    CE_UTF8 = 1,
    CE_LATIN1 = 2,
    CE_BYTES = 3,
    CE_SYMBOL = 5,
    CE_ANY = 99,
}

#[allow(non_camel_case_types)]
pub type R_CFinalizer_t =
    ::std::option::Option<extern "C" fn(arg1: SEXP) -> ()>;
#[allow(non_camel_case_types)]
pub type R_pstream_data_t = *mut ::libc::c_void;

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum R_pstream_format_t {
    R_pstream_any_format = 0,
    R_pstream_ascii_format,
    R_pstream_binary_format,
    R_pstream_xdr_format,
}

#[allow(non_camel_case_types)]
pub type R_outpstream_t = *mut Struct_R_outpstream_st;
#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy)]
pub struct Struct_R_outpstream_st {
    pub data: R_pstream_data_t,
    pub _type: R_pstream_format_t,
    pub version: ::libc::c_int,
    pub OutChar: ::std::option::Option<extern "C" fn(arg1: R_outpstream_t,
                                                     arg2: ::libc::c_int)
                                           -> ()>,
    pub OutBytes: ::std::option::Option<extern "C" fn(arg1: R_outpstream_t,
                                                      arg2:
                                                          *mut ::libc::c_void,
                                                      arg3: ::libc::c_int)
                                            -> ()>,
    pub OutPersistHookFunc: ::std::option::Option<extern "C" fn(arg1: SEXP,
                                                                arg2: SEXP)
                                                      -> SEXP>,
    pub OutPersistHookData: SEXP,
}
impl ::std::clone::Clone for Struct_R_outpstream_st {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_R_outpstream_st {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(non_camel_case_types)]
pub type R_inpstream_t = *mut Struct_R_inpstream_st;

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Copy)]
pub struct Struct_R_inpstream_st {
    pub data: R_pstream_data_t,
    pub _type: R_pstream_format_t,
    pub InChar: ::std::option::Option<extern "C" fn(arg1: R_inpstream_t)
                                          -> ::libc::c_int>,
    pub InBytes: ::std::option::Option<extern "C" fn(arg1: R_inpstream_t,
                                                     arg2:
                                                         *mut ::libc::c_void,
                                                     arg3: ::libc::c_int)
                                           -> ()>,
    pub InPersistHookFunc: ::std::option::Option<extern "C" fn(arg1: SEXP,
                                                               arg2: SEXP)
                                                     -> SEXP>,
    pub InPersistHookData: SEXP,
}
impl ::std::clone::Clone for Struct_R_inpstream_st {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_R_inpstream_st {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[link(name = "R")]
extern {
    pub static mut R_GlobalEnv: SEXP;
    pub static mut R_EmptyEnv: SEXP;
    pub static mut R_BaseEnv: SEXP;
    pub static mut R_BaseNamespace: SEXP;
    pub static mut R_NamespaceRegistry: SEXP;
    pub static mut R_Srcref: SEXP;
    pub static mut R_NilValue: SEXP;
    pub static mut R_UnboundValue: SEXP;
    pub static mut R_MissingArg: SEXP;
    pub static mut R_RestartToken: SEXP;
    pub static mut R_Bracket2Symbol: SEXP;
    pub static mut R_BracketSymbol: SEXP;
    pub static mut R_BraceSymbol: SEXP;
    pub static mut R_ClassSymbol: SEXP;
    pub static mut R_DeviceSymbol: SEXP;
    pub static mut R_DimNamesSymbol: SEXP;
    pub static mut R_DimSymbol: SEXP;
    pub static mut R_DollarSymbol: SEXP;
    pub static mut R_DotsSymbol: SEXP;
    pub static mut R_DropSymbol: SEXP;
    pub static mut R_LastvalueSymbol: SEXP;
    pub static mut R_LevelsSymbol: SEXP;
    pub static mut R_ModeSymbol: SEXP;
    pub static mut R_NameSymbol: SEXP;
    pub static mut R_NamesSymbol: SEXP;
    pub static mut R_NaRmSymbol: SEXP;
    pub static mut R_PackageSymbol: SEXP;
    pub static mut R_QuoteSymbol: SEXP;
    pub static mut R_RowNamesSymbol: SEXP;
    pub static mut R_SeedsSymbol: SEXP;
    pub static mut R_SourceSymbol: SEXP;
    pub static mut R_TspSymbol: SEXP;
    pub static mut R_dot_defined: SEXP;
    pub static mut R_dot_Method: SEXP;
    pub static mut R_dot_target: SEXP;
    pub static mut R_NaString: SEXP;
    pub static mut R_BlankString: SEXP;
}
#[link(name = "R")]
extern {
    pub fn R_CHAR(x: SEXP) -> *const ::libc::c_char;
    pub fn Rf_isNull(s: SEXP) -> Rboolean;
    pub fn Rf_isSymbol(s: SEXP) -> Rboolean;
    pub fn Rf_isLogical(s: SEXP) -> Rboolean;
    pub fn Rf_isReal(s: SEXP) -> Rboolean;
    pub fn Rf_isComplex(s: SEXP) -> Rboolean;
    pub fn Rf_isExpression(s: SEXP) -> Rboolean;
    pub fn Rf_isEnvironment(s: SEXP) -> Rboolean;
    pub fn Rf_isString(s: SEXP) -> Rboolean;
    pub fn Rf_isObject(s: SEXP) -> Rboolean;
    pub fn ATTRIB(x: SEXP) -> SEXP;
    pub fn OBJECT(x: SEXP) -> ::libc::c_int;
    pub fn MARK(x: SEXP) -> ::libc::c_int;
    pub fn TYPEOF(x: SEXP) -> ::libc::c_int;
    pub fn NAMED(x: SEXP) -> ::libc::c_int;
    pub fn REFCNT(x: SEXP) -> ::libc::c_int;
    pub fn SET_OBJECT(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_TYPEOF(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_NAMED(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_ATTRIB(x: SEXP, v: SEXP) -> ();
    pub fn DUPLICATE_ATTRIB(to: SEXP, from: SEXP) -> ();
    pub fn IS_S4_OBJECT(x: SEXP) -> ::libc::c_int;
    pub fn SET_S4_OBJECT(x: SEXP) -> ();
    pub fn UNSET_S4_OBJECT(x: SEXP) -> ();
    pub fn LENGTH(x: SEXP) -> ::libc::c_int;
    pub fn TRUELENGTH(x: SEXP) -> ::libc::c_int;
    pub fn SETLENGTH(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_TRUELENGTH(x: SEXP, v: ::libc::c_int) -> ();
    pub fn XLENGTH(x: SEXP) -> R_xlen_t;
    pub fn XTRUELENGTH(x: SEXP) -> R_xlen_t;
    pub fn IS_LONG_VEC(x: SEXP) -> ::libc::c_int;
    pub fn LEVELS(x: SEXP) -> ::libc::c_int;
    pub fn SETLEVELS(x: SEXP, v: ::libc::c_int) -> ::libc::c_int;
    pub fn LOGICAL(x: SEXP) -> *mut ::libc::c_int;
    pub fn INTEGER(x: SEXP) -> *mut ::libc::c_int;
    pub fn RAW(x: SEXP) -> *mut Rbyte;
    pub fn REAL(x: SEXP) -> *mut ::libc::c_double;
    pub fn COMPLEX(x: SEXP) -> *mut Rcomplex;
    pub fn STRING_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn VECTOR_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn SET_STRING_ELT(x: SEXP, i: R_xlen_t, v: SEXP) -> ();
    pub fn SET_VECTOR_ELT(x: SEXP, i: R_xlen_t, v: SEXP) -> SEXP;
    pub fn STRING_PTR(x: SEXP) -> *mut SEXP;
    pub fn VECTOR_PTR(x: SEXP) -> *mut SEXP;
    pub fn TAG(e: SEXP) -> SEXP;
    pub fn CAR(e: SEXP) -> SEXP;
    pub fn CDR(e: SEXP) -> SEXP;
    pub fn CAAR(e: SEXP) -> SEXP;
    pub fn CDAR(e: SEXP) -> SEXP;
    pub fn CADR(e: SEXP) -> SEXP;
    pub fn CDDR(e: SEXP) -> SEXP;
    pub fn CADDR(e: SEXP) -> SEXP;
    pub fn CADDDR(e: SEXP) -> SEXP;
    pub fn CAD4R(e: SEXP) -> SEXP;
    pub fn MISSING(x: SEXP) -> ::libc::c_int;
    pub fn SET_MISSING(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_TAG(x: SEXP, y: SEXP) -> ();
    pub fn SETCAR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCADR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCADDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCADDDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn SETCAD4R(e: SEXP, y: SEXP) -> SEXP;
    pub fn CONS_NR(a: SEXP, b: SEXP) -> SEXP;
    pub fn FORMALS(x: SEXP) -> SEXP;
    pub fn BODY(x: SEXP) -> SEXP;
    pub fn CLOENV(x: SEXP) -> SEXP;
    pub fn RDEBUG(x: SEXP) -> ::libc::c_int;
    pub fn RSTEP(x: SEXP) -> ::libc::c_int;
    pub fn RTRACE(x: SEXP) -> ::libc::c_int;
    pub fn SET_RDEBUG(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_RSTEP(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_RTRACE(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_FORMALS(x: SEXP, v: SEXP) -> ();
    pub fn SET_BODY(x: SEXP, v: SEXP) -> ();
    pub fn SET_CLOENV(x: SEXP, v: SEXP) -> ();
    pub fn PRINTNAME(x: SEXP) -> SEXP;
    pub fn SYMVALUE(x: SEXP) -> SEXP;
    pub fn INTERNAL(x: SEXP) -> SEXP;
    pub fn DDVAL(x: SEXP) -> ::libc::c_int;
    pub fn SET_DDVAL(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_PRINTNAME(x: SEXP, v: SEXP) -> ();
    pub fn SET_SYMVALUE(x: SEXP, v: SEXP) -> ();
    pub fn SET_INTERNAL(x: SEXP, v: SEXP) -> ();
    pub fn FRAME(x: SEXP) -> SEXP;
    pub fn ENCLOS(x: SEXP) -> SEXP;
    pub fn HASHTAB(x: SEXP) -> SEXP;
    pub fn ENVFLAGS(x: SEXP) -> ::libc::c_int;
    pub fn SET_ENVFLAGS(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_FRAME(x: SEXP, v: SEXP) -> ();
    pub fn SET_ENCLOS(x: SEXP, v: SEXP) -> ();
    pub fn SET_HASHTAB(x: SEXP, v: SEXP) -> ();
    pub fn PRCODE(x: SEXP) -> SEXP;
    pub fn PRENV(x: SEXP) -> SEXP;
    pub fn PRVALUE(x: SEXP) -> SEXP;
    pub fn PRSEEN(x: SEXP) -> ::libc::c_int;
    pub fn SET_PRSEEN(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_PRENV(x: SEXP, v: SEXP) -> ();
    pub fn SET_PRVALUE(x: SEXP, v: SEXP) -> ();
    pub fn SET_PRCODE(x: SEXP, v: SEXP) -> ();
    pub fn HASHASH(x: SEXP) -> ::libc::c_int;
    pub fn HASHVALUE(x: SEXP) -> ::libc::c_int;
    pub fn SET_HASHASH(x: SEXP, v: ::libc::c_int) -> ();
    pub fn SET_HASHVALUE(x: SEXP, v: ::libc::c_int) -> ();
    pub fn R_GetCurrentSrcref(arg1: ::libc::c_int) -> SEXP;
    pub fn R_GetSrcFilename(arg1: SEXP) -> SEXP;
    pub fn Rf_asChar(arg1: SEXP) -> SEXP;
    pub fn Rf_coerceVector(arg1: SEXP, arg2: SEXPTYPE) -> SEXP;
    pub fn Rf_PairToVectorList(x: SEXP) -> SEXP;
    pub fn Rf_VectorToPairList(x: SEXP) -> SEXP;
    pub fn Rf_asCharacterFactor(x: SEXP) -> SEXP;
    pub fn Rf_asLogical(x: SEXP) -> ::libc::c_int;
    pub fn Rf_asInteger(x: SEXP) -> ::libc::c_int;
    pub fn Rf_asReal(x: SEXP) -> ::libc::c_double;
    pub fn Rf_asComplex(x: SEXP) -> Rcomplex;
    pub fn Rf_acopy_string(arg1: *const ::libc::c_char) -> *mut ::libc::c_char;
    pub fn Rf_alloc3DArray(arg1: SEXPTYPE,
                           arg2: ::libc::c_int,
                           arg3: ::libc::c_int,
                           arg4: ::libc::c_int)
                           -> SEXP;
    pub fn Rf_allocArray(arg1: SEXPTYPE, arg2: SEXP) -> SEXP;
    pub fn Rf_allocMatrix(arg1: SEXPTYPE, arg2: ::libc::c_int, arg3: ::libc::c_int) -> SEXP;
    pub fn Rf_allocList(arg1: ::libc::c_int) -> SEXP;
    pub fn Rf_allocS4Object() -> SEXP;
    pub fn Rf_allocSExp(arg1: SEXPTYPE) -> SEXP;
    pub fn Rf_allocVector3(arg1: SEXPTYPE, arg2: R_xlen_t, arg3: *mut R_allocator_t) -> SEXP;
    pub fn Rf_any_duplicated(x: SEXP, from_last: Rboolean) -> R_xlen_t;
    pub fn Rf_any_duplicated3(x: SEXP, incomp: SEXP, from_last: Rboolean) -> R_xlen_t;
    pub fn Rf_applyClosure(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP, arg5: SEXP) -> SEXP;
    pub fn Rf_arraySubscript(arg1: ::libc::c_int, arg2: SEXP, arg3: SEXP,
                             arg4:
                                 ::std::option::Option<extern "C" fn(arg1:
                                                                         SEXP,
                                                                     arg2:
                                                                         SEXP)
                                                           -> SEXP>,
                             arg5:
                                 ::std::option::Option<extern "C" fn(arg1:
                                                                         SEXP,
                                                                     arg2:
                                                                         ::libc::c_int)
                                                           -> SEXP>,
                             arg6: SEXP) -> SEXP;
    pub fn Rf_classgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_cons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_copyMatrix(arg1: SEXP, arg2: SEXP, arg3: Rboolean) -> ();
    pub fn Rf_copyListMatrix(arg1: SEXP, arg2: SEXP, arg3: Rboolean) -> ();
    pub fn Rf_copyMostAttrib(arg1: SEXP, arg2: SEXP) -> ();
    pub fn Rf_copyVector(arg1: SEXP, arg2: SEXP) -> ();
    pub fn Rf_countContexts(arg1: ::libc::c_int, arg2: ::libc::c_int) -> ::libc::c_int;
    pub fn Rf_CreateTag(arg1: SEXP) -> SEXP;
    pub fn Rf_defineVar(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> ();
    pub fn Rf_dimgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_dimnamesgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_DropDims(arg1: SEXP) -> SEXP;
    pub fn Rf_duplicate(arg1: SEXP) -> SEXP;
    pub fn Rf_shallow_duplicate(arg1: SEXP) -> SEXP;
    pub fn Rf_lazy_duplicate(arg1: SEXP) -> SEXP;
    pub fn Rf_duplicated(arg1: SEXP, arg2: Rboolean) -> SEXP;
    pub fn R_envHasNoSpecialSymbols(arg1: SEXP) -> Rboolean;
    pub fn Rf_eval(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findFun(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findVar(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findVarInFrame(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findVarInFrame3(arg1: SEXP, arg2: SEXP, arg3: Rboolean) -> SEXP;
    pub fn Rf_getAttrib(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_GetArrayDimnames(arg1: SEXP) -> SEXP;
    pub fn Rf_GetColNames(arg1: SEXP) -> SEXP;
    pub fn Rf_GetMatrixDimnames(arg1: SEXP,
                                arg2: *mut SEXP,
                                arg3: *mut SEXP,
                                arg4: *mut *const ::libc::c_char,
                                arg5: *mut *const ::libc::c_char)
                                -> ();
    pub fn Rf_GetOption(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_GetOption1(arg1: SEXP) -> SEXP;
    pub fn Rf_GetOptionDigits() -> ::libc::c_int;
    pub fn Rf_GetOptionWidth() -> ::libc::c_int;
    pub fn Rf_GetRowNames(arg1: SEXP) -> SEXP;
    pub fn Rf_gsetVar(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> ();
    pub fn Rf_install(arg1: *const ::libc::c_char) -> SEXP;
    pub fn Rf_isFree(arg1: SEXP) -> Rboolean;
    pub fn Rf_isOrdered(arg1: SEXP) -> Rboolean;
    pub fn Rf_isUnordered(arg1: SEXP) -> Rboolean;
    pub fn Rf_isUnsorted(arg1: SEXP, arg2: Rboolean) -> Rboolean;
    pub fn Rf_lengthgets(arg1: SEXP, arg2: R_len_t) -> SEXP;
    pub fn Rf_xlengthgets(arg1: SEXP, arg2: R_xlen_t) -> SEXP;
    pub fn R_lsInternal(arg1: SEXP, arg2: Rboolean) -> SEXP;
    pub fn Rf_match(arg1: SEXP, arg2: SEXP, arg3: ::libc::c_int) -> SEXP;
    pub fn Rf_matchE(arg1: SEXP, arg2: SEXP, arg3: ::libc::c_int, arg4: SEXP) -> SEXP;
    pub fn Rf_namesgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_mkChar(arg1: *const ::libc::c_char) -> SEXP;
    pub fn Rf_mkCharLen(arg1: *const ::libc::c_char, arg2: ::libc::c_int) -> SEXP;
    pub fn Rf_NonNullStringMatch(arg1: SEXP, arg2: SEXP) -> Rboolean;
    pub fn Rf_ncols(arg1: SEXP) -> ::libc::c_int;
    pub fn Rf_nrows(arg1: SEXP) -> ::libc::c_int;
    pub fn Rf_nthcdr(arg1: SEXP, arg2: ::libc::c_int) -> SEXP;
    pub fn Rf_pmatch(arg1: SEXP, arg2: SEXP, arg3: Rboolean) -> Rboolean;
    pub fn Rf_psmatch(arg1: *const ::libc::c_char,
                      arg2: *const ::libc::c_char,
                      arg3: Rboolean)
                      -> Rboolean;
    pub fn Rf_PrintValue(arg1: SEXP) -> ();
    pub fn Rf_setAttrib(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
    pub fn Rf_setSVector(arg1: *mut SEXP, arg2: ::libc::c_int, arg3: SEXP) -> ();
    pub fn Rf_setVar(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> ();
    pub fn Rf_str2type(arg1: *const ::libc::c_char) -> SEXPTYPE;
    pub fn Rf_StringBlank(arg1: SEXP) -> Rboolean;
    pub fn Rf_substitute(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_translateChar(arg1: SEXP) -> *const ::libc::c_char;
    pub fn Rf_translateChar0(arg1: SEXP) -> *const ::libc::c_char;
    pub fn Rf_translateCharUTF8(arg1: SEXP) -> *const ::libc::c_char;
    pub fn Rf_type2char(arg1: SEXPTYPE) -> *const ::libc::c_char;
    pub fn Rf_type2str(arg1: SEXPTYPE) -> SEXP;
    pub fn Rf_unprotect_ptr(arg1: SEXP) -> ();
    pub fn R_signal_protect_error() -> ();
    pub fn R_signal_unprotect_error() -> ();
    pub fn R_signal_reprotect_error(i: PROTECT_INDEX) -> ();
    pub fn R_tryEval(arg1: SEXP, arg2: SEXP, arg3: *mut ::libc::c_int) -> SEXP;
    pub fn R_tryEvalSilent(arg1: SEXP, arg2: SEXP, arg3: *mut ::libc::c_int) -> SEXP;
    pub fn R_curErrorBuf() -> *const ::libc::c_char;
    pub fn Rf_isS4(arg1: SEXP) -> Rboolean;
    pub fn Rf_asS4(arg1: SEXP, arg2: Rboolean, arg3: ::libc::c_int) -> SEXP;
    pub fn Rf_S3Class(arg1: SEXP) -> SEXP;
    pub fn Rf_isBasicClass(arg1: *const ::libc::c_char) -> ::libc::c_int;
    pub fn R_cycle_detected(s: SEXP, child: SEXP) -> Rboolean;
    pub fn Rf_getCharCE(arg1: SEXP) -> cetype_t;
    pub fn Rf_mkCharCE(arg1: *const ::libc::c_char, arg2: cetype_t) -> SEXP;
    pub fn Rf_mkCharLenCE(arg1: *const ::libc::c_char,
                          arg2: ::libc::c_int,
                          arg3: cetype_t)
                          -> SEXP;
    pub fn Rf_reEnc(x: *const ::libc::c_char,
                    ce_in: cetype_t,
                    ce_out: cetype_t,
                    subst: ::libc::c_int)
                    -> *const ::libc::c_char;
    pub fn R_MakeExternalPtr(p: *mut ::libc::c_void, tag: SEXP, prot: SEXP) -> SEXP;
    pub fn R_ExternalPtrAddr(s: SEXP) -> *mut ::libc::c_void;
    pub fn R_ExternalPtrTag(s: SEXP) -> SEXP;
    pub fn R_ExternalPtrProtected(s: SEXP) -> SEXP;
    pub fn R_ClearExternalPtr(s: SEXP) -> ();
    pub fn R_SetExternalPtrAddr(s: SEXP, p: *mut ::libc::c_void) -> ();
    pub fn R_SetExternalPtrTag(s: SEXP, tag: SEXP) -> ();
    pub fn R_SetExternalPtrProtected(s: SEXP, p: SEXP) -> ();
    pub fn R_RegisterFinalizer(s: SEXP, fun: SEXP) -> ();
    pub fn R_RegisterCFinalizer(s: SEXP, fun: R_CFinalizer_t) -> ();
    pub fn R_RegisterFinalizerEx(s: SEXP, fun: SEXP, onexit: Rboolean) -> ();
    pub fn R_RegisterCFinalizerEx(s: SEXP, fun: R_CFinalizer_t, onexit: Rboolean) -> ();
    pub fn R_RunPendingFinalizers() -> ();
    pub fn R_MakeWeakRef(key: SEXP, val: SEXP, fin: SEXP, onexit: Rboolean) -> SEXP;
    pub fn R_MakeWeakRefC(key: SEXP, val: SEXP, fin: R_CFinalizer_t, onexit: Rboolean) -> SEXP;
    pub fn R_WeakRefKey(w: SEXP) -> SEXP;
    pub fn R_WeakRefValue(w: SEXP) -> SEXP;
    pub fn R_RunWeakRefFinalizer(w: SEXP) -> ();
    pub fn R_PromiseExpr(arg1: SEXP) -> SEXP;
    pub fn R_ClosureExpr(arg1: SEXP) -> SEXP;
    pub fn R_initialize_bcode() -> ();
    pub fn R_bcEncode(arg1: SEXP) -> SEXP;
    pub fn R_bcDecode(arg1: SEXP) -> SEXP;
    pub fn R_ToplevelExec(fun:
                              ::std::option::Option<extern "C" fn(arg1:
                                                                      *mut ::libc::c_void)
                                                        -> ()>,
                          data: *mut ::libc::c_void) -> Rboolean;
    pub fn R_ExecWithCleanup(fun:
                                 ::std::option::Option<extern "C" fn(arg1:
                                                                         *mut ::libc::c_void)
                                                           -> SEXP>,
                             data: *mut ::libc::c_void,
                             cleanfun:
                                 ::std::option::Option<extern "C" fn(arg1:
                                                                         *mut ::libc::c_void)
                                                           -> ()>,
                             cleandata: *mut ::libc::c_void) -> SEXP;
    pub fn R_RestoreHashCount(rho: SEXP) -> ();
    pub fn R_IsPackageEnv(rho: SEXP) -> Rboolean;
    pub fn R_PackageEnvName(rho: SEXP) -> SEXP;
    pub fn R_FindPackageEnv(info: SEXP) -> SEXP;
    pub fn R_IsNamespaceEnv(rho: SEXP) -> Rboolean;
    pub fn R_NamespaceEnvSpec(rho: SEXP) -> SEXP;
    pub fn R_FindNamespace(info: SEXP) -> SEXP;
    pub fn R_LockEnvironment(env: SEXP, bindings: Rboolean) -> ();
    pub fn R_EnvironmentIsLocked(env: SEXP) -> Rboolean;
    pub fn R_LockBinding(sym: SEXP, env: SEXP) -> ();
    pub fn R_unLockBinding(sym: SEXP, env: SEXP) -> ();
    pub fn R_MakeActiveBinding(sym: SEXP, fun: SEXP, env: SEXP) -> ();
    pub fn R_BindingIsLocked(sym: SEXP, env: SEXP) -> Rboolean;
    pub fn R_BindingIsActive(sym: SEXP, env: SEXP) -> Rboolean;
    pub fn R_HasFancyBindings(rho: SEXP) -> Rboolean;
    pub fn Rf_errorcall(arg1: SEXP, arg2: *const ::libc::c_char, ...) -> ();
    pub fn Rf_warningcall(arg1: SEXP, arg2: *const ::libc::c_char, ...) -> ();
    pub fn Rf_warningcall_immediate(arg1: SEXP, arg2: *const ::libc::c_char, ...) -> ();
    pub fn R_XDREncodeDouble(d: ::libc::c_double, buf: *mut ::libc::c_void) -> ();
    pub fn R_XDRDecodeDouble(buf: *mut ::libc::c_void) -> ::libc::c_double;
    pub fn R_XDREncodeInteger(i: ::libc::c_int, buf: *mut ::libc::c_void) -> ();
    pub fn R_XDRDecodeInteger(buf: *mut ::libc::c_void) -> ::libc::c_int;
    pub fn R_InitInPStream(stream: R_inpstream_t, data: R_pstream_data_t,
                           _type: R_pstream_format_t,
                           inchar:
                               ::std::option::Option<extern "C" fn(arg1:
                                                                       R_inpstream_t)
                                                         -> ::libc::c_int>,
                           inbytes:
                               ::std::option::Option<extern "C" fn(arg1:
                                                                       R_inpstream_t,
                                                                   arg2:
                                                                       *mut ::libc::c_void,
                                                                   arg3:
                                                                       ::libc::c_int)
                                                         -> ()>,
                           phook:
                               ::std::option::Option<extern "C" fn(arg1: SEXP,
                                                                   arg2: SEXP)
                                                         -> SEXP>,
                           pdata: SEXP) -> ();
    pub fn R_InitOutPStream(stream: R_outpstream_t, data: R_pstream_data_t,
                            _type: R_pstream_format_t, version: ::libc::c_int,
                            outchar:
                                ::std::option::Option<extern "C" fn(arg1:
                                                                        R_outpstream_t,
                                                                    arg2:
                                                                        ::libc::c_int)
                                                          -> ()>,
                            outbytes:
                                ::std::option::Option<extern "C" fn(arg1:
                                                                        R_outpstream_t,
                                                                    arg2:
                                                                        *mut ::libc::c_void,
                                                                    arg3:
                                                                        ::libc::c_int)
                                                          -> ()>,
                            phook:
                                ::std::option::Option<extern "C" fn(arg1:
                                                                        SEXP,
                                                                    arg2:
                                                                        SEXP)
                                                          -> SEXP>,
                            pdata: SEXP) -> ();
    pub fn R_InitFileInPStream(stream: R_inpstream_t, fp: *mut FILE,
                               _type: R_pstream_format_t,
                               phook:
                                   ::std::option::Option<extern "C" fn(arg1:
                                                                           SEXP,
                                                                       arg2:
                                                                           SEXP)
                                                             -> SEXP>,
                               pdata: SEXP) -> ();
    pub fn R_InitFileOutPStream(stream: R_outpstream_t, fp: *mut FILE,
                                _type: R_pstream_format_t,
                                version: ::libc::c_int,
                                phook:
                                    ::std::option::Option<extern "C" fn(arg1:
                                                                            SEXP,
                                                                        arg2:
                                                                            SEXP)
                                                              -> SEXP>,
                                pdata: SEXP) -> ();
    pub fn R_Serialize(s: SEXP, ops: R_outpstream_t) -> ();
    pub fn R_Unserialize(ips: R_inpstream_t) -> SEXP;
    pub fn R_do_slot(obj: SEXP, name: SEXP) -> SEXP;
    pub fn R_do_slot_assign(obj: SEXP, name: SEXP, value: SEXP) -> SEXP;
    pub fn R_has_slot(obj: SEXP, name: SEXP) -> ::libc::c_int;
    pub fn R_do_MAKE_CLASS(what: *const ::libc::c_char) -> SEXP;
    pub fn R_getClassDef(what: *const ::libc::c_char) -> SEXP;
    pub fn R_do_new_object(class_def: SEXP) -> SEXP;
    pub fn R_check_class_and_super(x: SEXP,
                                   valid: *mut *const ::libc::c_char,
                                   rho: SEXP)
                                   -> ::libc::c_int;
    pub fn R_check_class_etc(x: SEXP, valid: *mut *const ::libc::c_char) -> ::libc::c_int;
    pub fn R_PreserveObject(arg1: SEXP) -> ();
    pub fn R_ReleaseObject(arg1: SEXP) -> ();
    pub fn R_dot_Last() -> ();
    pub fn R_RunExitFinalizers() -> ();
    pub fn R_system(arg1: *const ::libc::c_char) -> ::libc::c_int;
    pub fn R_compute_identical(arg1: SEXP, arg2: SEXP, arg3: ::libc::c_int) -> Rboolean;
    pub fn R_orderVector(indx: *mut ::libc::c_int,
                         n: ::libc::c_int,
                         arglist: SEXP,
                         nalast: Rboolean,
                         decreasing: Rboolean)
                         -> ();
    pub fn Rf_allocVector(arg1: SEXPTYPE, arg2: R_xlen_t) -> SEXP;
    pub fn Rf_conformable(arg1: SEXP, arg2: SEXP) -> Rboolean;
    pub fn Rf_elt(arg1: SEXP, arg2: ::libc::c_int) -> SEXP;
    pub fn Rf_inherits(arg1: SEXP, arg2: *const ::libc::c_char) -> Rboolean;
    pub fn Rf_isArray(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFactor(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFrame(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFunction(arg1: SEXP) -> Rboolean;
    pub fn Rf_isInteger(arg1: SEXP) -> Rboolean;
    pub fn Rf_isLanguage(arg1: SEXP) -> Rboolean;
    pub fn Rf_isList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isMatrix(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNewList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNumber(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNumeric(arg1: SEXP) -> Rboolean;
    pub fn Rf_isPairList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isPrimitive(arg1: SEXP) -> Rboolean;
    pub fn Rf_isTs(arg1: SEXP) -> Rboolean;
    pub fn Rf_isUserBinop(arg1: SEXP) -> Rboolean;
    pub fn Rf_isValidString(arg1: SEXP) -> Rboolean;
    pub fn Rf_isValidStringF(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVector(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorAtomic(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorizable(arg1: SEXP) -> Rboolean;
    pub fn Rf_lang1(arg1: SEXP) -> SEXP;
    pub fn Rf_lang2(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_lang3(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
    pub fn Rf_lang4(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP) -> SEXP;
    pub fn Rf_lang5(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP, arg5: SEXP) -> SEXP;
    pub fn Rf_lang6(arg1: SEXP,
                    arg2: SEXP,
                    arg3: SEXP,
                    arg4: SEXP,
                    arg5: SEXP,
                    arg6: SEXP)
                    -> SEXP;
    pub fn Rf_lastElt(arg1: SEXP) -> SEXP;
    pub fn Rf_lcons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_length(arg1: SEXP) -> R_len_t;
    pub fn Rf_list1(arg1: SEXP) -> SEXP;
    pub fn Rf_list2(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_list3(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
    pub fn Rf_list4(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP) -> SEXP;
    pub fn Rf_list5(arg1: SEXP, arg2: SEXP, arg3: SEXP, arg4: SEXP, arg5: SEXP) -> SEXP;
    pub fn Rf_listAppend(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_mkNamed(arg1: SEXPTYPE, arg2: *mut *const ::libc::c_char) -> SEXP;
    pub fn Rf_mkString(arg1: *const ::libc::c_char) -> SEXP;
    pub fn Rf_nlevels(arg1: SEXP) -> ::libc::c_int;
    pub fn Rf_ScalarComplex(arg1: Rcomplex) -> SEXP;
    pub fn Rf_ScalarInteger(arg1: ::libc::c_int) -> SEXP;
    pub fn Rf_ScalarLogical(arg1: ::libc::c_int) -> SEXP;
    pub fn Rf_ScalarRaw(arg1: Rbyte) -> SEXP;
    pub fn Rf_ScalarReal(arg1: ::libc::c_double) -> SEXP;
    pub fn Rf_ScalarString(arg1: SEXP) -> SEXP;
    pub fn Rf_xlength(arg1: SEXP) -> R_xlen_t;
    pub fn Rf_protect(arg1: SEXP) -> SEXP;
    pub fn Rf_unprotect(arg1: ::libc::c_int) -> ();
    pub fn R_ProtectWithIndex(arg1: SEXP, arg2: *mut PROTECT_INDEX) -> ();
    pub fn R_Reprotect(arg1: SEXP, arg2: PROTECT_INDEX) -> ();
    pub fn R_FixupRHS(x: SEXP, y: SEXP) -> SEXP;
}
