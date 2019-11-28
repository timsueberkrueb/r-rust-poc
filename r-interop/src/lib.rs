use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::os::raw::{c_double, c_int};

use r_internals::*;

pub use r_internals::SEXP;

pub trait FromSEXP {
    fn from_sexp(sexp: SEXP) -> Self;
}

pub trait IntoSEXP {
    fn into_sexp(self) -> SEXP;
}

pub trait SEXPType: Sized {
    fn sexp_type() -> SEXPTYPE;
    fn vector_elt(v: SEXP, idx: R_xlen_t) -> Self;
    fn set_vector_elt(v: SEXP, idx: R_xlen_t, val: Self);
}

impl SEXPType for String {
    fn sexp_type() -> SEXPTYPE {
        STRSXP
    }

    fn vector_elt(v: SEXP, idx: R_xlen_t) -> Self {
        let sexpr = unsafe { STRING_ELT(v, idx) };
        Self::from_sexp(sexpr)
    }

    fn set_vector_elt(v: SEXP, idx: R_xlen_t, val: Self) {
        unsafe { ALTSTRING_SET_ELT(v, idx, val.into_sexp()) };
    }
}

impl IntoSEXP for String {
    fn into_sexp(self) -> SEXP {
        let c_str = CString::new(self).unwrap();
        let p = c_str.as_ptr();
        unsafe { Rf_ScalarString(Rf_mkCharCE(p, cetype_t_CE_UTF8)) }
    }
}

impl FromSEXP for String {
    fn from_sexp(sexp: SEXP) -> Self {
        let sexp = unsafe { Rf_protect(Rf_coerceVector(sexp, STRSXP)) };
        let chrsxp = unsafe { STRING_ELT(sexp, 0) };
        let c_buff = unsafe { Rf_translateCharUTF8(chrsxp) };
        let c_str: &CStr = unsafe { CStr::from_ptr(c_buff) };
        let str_slice: &str = c_str
            .to_str()
            .expect("Failed to decode UTF-8 string from R");
        unsafe {
            Rf_unprotect(1);
        }
        str_slice.to_owned()
    }
}

impl SEXPType for c_int {
    fn sexp_type() -> SEXPTYPE {
        INTSXP
    }

    fn vector_elt(v: SEXP, idx: R_xlen_t) -> Self {
        unsafe { INTEGER_ELT(v, idx) }
    }

    fn set_vector_elt(v: SEXP, idx: R_xlen_t, val: Self) {
        unsafe { ALTINTEGER_SET_ELT(v, idx, val) };
    }
}

impl IntoSEXP for c_int {
    fn into_sexp(self) -> SEXP {
        unsafe { Rf_ScalarInteger(self) }
    }
}

impl FromSEXP for c_int {
    fn from_sexp(sexp: SEXP) -> Self {
        unsafe { Rf_asInteger(sexp) }
    }
}

impl SEXPType for c_double {
    fn sexp_type() -> SEXPTYPE {
        REALSXP
    }

    fn vector_elt(v: SEXP, idx: R_xlen_t) -> Self {
        unsafe { REAL_ELT(v, idx) }
    }

    fn set_vector_elt(v: SEXP, idx: R_xlen_t, val: Self) {
        unsafe { ALTREAL_SET_ELT(v, idx, val) };
    }
}

impl IntoSEXP for c_double {
    fn into_sexp(self) -> SEXP {
        unsafe { Rf_ScalarReal(self) }
    }
}

impl FromSEXP for c_double {
    fn from_sexp(sexp: SEXP) -> Self {
        unsafe { Rf_asReal(sexp) }
    }
}

impl SEXPType for bool {
    fn sexp_type() -> SEXPTYPE {
        LGLSXP
    }

    fn vector_elt(v: SEXP, idx: R_xlen_t) -> Self {
        unsafe { LOGICAL_ELT(v, idx) != 0 }
    }

    fn set_vector_elt(v: SEXP, idx: R_xlen_t, val: Self) {
        unsafe { ALTLOGICAL_SET_ELT(v, idx, val as c_int) };
    }
}

impl IntoSEXP for bool {
    fn into_sexp(self) -> SEXP {
        unsafe { Rf_ScalarLogical(self as i32) }
    }
}

impl FromSEXP for bool {
    fn from_sexp(sexp: SEXP) -> Self {
        let i = unsafe { Rf_asLogical(sexp) };
        i != 0
    }
}

impl IntoSEXP for () {
    fn into_sexp(self) -> SEXP {
        unsafe { R_NilValue }
    }
}

impl FromSEXP for () {
    fn from_sexp(_sexp: SEXP) -> () {
        ()
    }
}

impl<T: FromSEXP + IntoSEXP + SEXPType> SEXPType for Vec<T> {
    fn sexp_type() -> SEXPTYPE {
        VECSXP
    }

    fn vector_elt(v: SEXP, idx: R_xlen_t) -> Self {
        let sexp = unsafe { VECTOR_ELT(v, idx) };
        Self::from_sexp(sexp)
    }

    fn set_vector_elt(v: SEXP, idx: R_xlen_t, val: Self) {
        unsafe { SET_VECTOR_ELT(v, idx, val.into_sexp()) };
    }
}

impl<T: IntoSEXP + SEXPType> IntoSEXP for Vec<T> {
    fn into_sexp(self) -> SEXP {
        let len = self
            .len()
            .try_into()
            .expect("Failed to convert Vec length to the type expected by R");
        let v_sexp = unsafe { Rf_protect(Rf_allocVector(T::sexp_type(), len)) };
        for (idx, item) in self.into_iter().enumerate() {
            let r_idx = idx
                .try_into()
                .expect("Failed to convert vector index to the type expected by R");
            T::set_vector_elt(v_sexp, r_idx, item);
        }
        unsafe { Rf_unprotect(1) };
        v_sexp
    }
}

impl<T: FromSEXP + SEXPType> FromSEXP for Vec<T> {
    fn from_sexp(sexp: SEXP) -> Self {
        let sexp = unsafe { Rf_protect(Rf_coerceVector(sexp, T::sexp_type())) };
        let len = unsafe { XLENGTH(sexp) };
        let mut vec = Vec::with_capacity(len as usize);
        for idx in 0..len {
            let elem = T::vector_elt(sexp, idx);
            vec.push(elem);
        }
        unsafe { Rf_unprotect(1) };
        vec
    }
}
