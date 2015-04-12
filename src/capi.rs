#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)] 

extern crate libc;
use self::libc::{c_void, c_int, c_ulong};
pub type Pvoid_t = *mut c_void;
pub type PPvoid_t = *mut Pvoid_t;
pub type Pcvoid_t = *const c_void;
pub type Word_t = c_ulong;
//pub type PWord_t = *mut Word_t;

pub type JU_Errno_t = c_int;

pub static JU_ERRNO_NONE: JU_Errno_t           = 0;
pub static JU_ERRNO_FULL: JU_Errno_t           = 1;
pub static JU_ERRNO_NFMAX: JU_Errno_t          = 1; // JU_ERRNO_FULL
pub static JU_ERRNO_NOMEM: JU_Errno_t          = 2;
pub static JU_ERRNO_NULLPPARRAY: JU_Errno_t    = 3;
pub static JU_ERRNO_NONNULLPARRAY: JU_Errno_t  = 10;
pub static JU_ERRNO_NULLPINDEX: JU_Errno_t     = 4;
pub static JU_ERRNO_NULLPVALUE: JU_Errno_t     = 11;
pub static JU_ERRNO_NOTJUDY1: JU_Errno_t       = 5;
pub static JU_ERRNO_NOTJUDYL: JU_Errno_t       = 6;
pub static JU_ERRNO_NOTJUDYSL: JU_Errno_t      = 7;
pub static JU_ERRNO_UNSORTED: JU_Errno_t       = 12;
pub static JU_ERRNO_OVERRUN: JU_Errno_t        = 8;
pub static JU_ERRNO_CORRUPT: JU_Errno_t        = 9;

#[repr(C)]
pub struct JError_t {
    je_Errno: JU_Errno_t,
    je_ErrID: c_int,
    je_reserved: [Word_t; 4],
}
pub type PJError_t = *mut JError_t;

impl JError_t {
    pub fn new() -> JError_t {
        JError_t{
            je_Errno: JU_ERRNO_NONE,
            je_ErrID: 0,
            je_reserved: [0; 4],
        }
    }
}

extern {
    pub fn JudyHSGet(array: Pcvoid_t, key: *const c_void, size: Word_t) -> PPvoid_t;
    pub fn JudyHSIns(array: PPvoid_t, key: *const c_void, size: Word_t, err: PJError_t) -> PPvoid_t;
    pub fn JudyHSDel(array: PPvoid_t, key: *const c_void, size: Word_t, err: PJError_t) -> c_int;
    pub fn JudyHSFreeArray(array: PPvoid_t, err: PJError_t) -> Word_t;

    pub fn JudyLIns(array: PPvoid_t, index: Word_t, err: PJError_t) -> PPvoid_t;
    pub fn JudyLDel(array: PPvoid_t, index: Word_t, err: PJError_t) -> c_int;
    pub fn JudyLGet(array: Pcvoid_t, index: Word_t, err: PJError_t) -> PPvoid_t;
    pub fn JudyLCount(array: Pcvoid_t, index1: Word_t, index2: Word_t, err: PJError_t) -> Word_t;
    pub fn JudyLByCount(array: Pcvoid_t, nth: Word_t, pindex: *mut Word_t, err: PJError_t) -> PPvoid_t;
    pub fn JudyLFreeArray(array: PPvoid_t, err: PJError_t) -> Word_t;
    pub fn JudyLMemUsed(array: Pcvoid_t) -> Word_t;
    pub fn JudyLFirst(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> PPvoid_t;
    pub fn JudyLNext(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> PPvoid_t;
    pub fn JudyLLast(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> PPvoid_t;
    pub fn JudyLPrev(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> PPvoid_t;
    pub fn JudyLFirstEmpty(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn JudyLNextEmpty(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn JudyLLastEmpty(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn JudyLPrevEmpty(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;

    pub fn Judy1Set(array: PPvoid_t, index: Word_t, err: PJError_t) -> c_int;
    pub fn Judy1Unset(array: PPvoid_t, index: Word_t, err: PJError_t) -> c_int;
    pub fn Judy1Test(array: Pcvoid_t, index: Word_t, err: PJError_t) -> c_int;
    pub fn Judy1Count(array: Pcvoid_t, index1: Word_t, index2: Word_t, err: PJError_t) -> Word_t;
    pub fn Judy1ByCount(array: Pcvoid_t, nth: Word_t, pindex: *mut Word_t, err: PJError_t) -> PPvoid_t;
    pub fn Judy1FreeArray(array: PPvoid_t, err: PJError_t) -> Word_t;
    pub fn Judy1MemUsed(array: Pcvoid_t) -> Word_t;
    pub fn Judy1First(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn Judy1Next(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn Judy1Last(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn Judy1Prev(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn Judy1FirstEmpty(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn Judy1NextEmpty(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn Judy1LastEmpty(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;
    pub fn Judy1PrevEmpty(array: Pcvoid_t, pindex: *mut Word_t, err: PJError_t) -> c_int;

    pub fn JudySLGet(array: Pcvoid_t, index: *const u8) -> PPvoid_t;
    pub fn JudySLIns(array: PPvoid_t, index: *const u8, err: PJError_t) -> PPvoid_t;
    pub fn JudySLDel(array: PPvoid_t, index: *const u8, err: PJError_t) -> c_int;
    pub fn JudySLFreeArray(array: PPvoid_t, err: PJError_t) -> Word_t;
    pub fn JudySLFirst(array: Pcvoid_t, pindex: *mut u8, err: PJError_t) -> PPvoid_t;
    pub fn JudySLNext(array: Pcvoid_t, pindex: *mut u8, err: PJError_t) -> PPvoid_t;
    pub fn JudySLLast(array: Pcvoid_t, pindex: *mut u8, err: PJError_t) -> PPvoid_t;
    pub fn JudySLPrev(array: Pcvoid_t, pindex: *mut u8, err: PJError_t) -> PPvoid_t;
}
