extern crate libc;

use self::capi::*;
use std::mem::size_of;
use std::mem::{transmute, transmute_copy};
use std::ptr::null_mut;
use std::marker::PhantomData;

pub mod capi {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

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
    }
}

pub struct JudyL<V> {
    m: Pvoid_t,
    value_type: PhantomData<V>,
}

impl<V> JudyL<V> {
    pub fn new() -> JudyL<V> {
        JudyL{m: null_mut(), value_type: PhantomData}
    }

    pub fn insert(&mut self, index: Word_t, value: &V) -> bool {
        unsafe {
            let v = JudyLIns(&mut self.m, index, null_mut());
            if v == null_mut() {
                false
            } else if *v != null_mut() {
                false
            } else {
                *v = transmute(value);
                true
            }
        }
    }

    pub fn get<'a>(&'a self, index: Word_t) -> Option<&'a V> {
        unsafe {
            let v = JudyLGet(self.m as Pcvoid_t, index, null_mut());
            if v == null_mut() {
                None
            } else {
                Some(transmute(*v))
            }
        }
    }

    pub fn free(&mut self) -> Word_t {
        if self.m != null_mut() {
            unsafe {
                let ret = JudyLFreeArray(&mut self.m, null_mut());
                assert!(self.m == null_mut());
                ret
            }
        } else {
            0
        }
    }

    pub fn iter(& self) -> JudyLIterator<V> {
        JudyLIterator{ m: self.m as Pcvoid_t, i: 0, value_type: PhantomData}
    }

    pub fn count(&self, index1: Word_t, index2: Word_t) -> Word_t {
        unsafe {
            JudyLCount(self.m as Pcvoid_t, index1, index2, null_mut())
        }
    }
}

pub struct JudyHS<K, V> {
    m: Pvoid_t,
    key_type: PhantomData<K>,
    value_type: PhantomData<V>,
}

impl<K, V> JudyHS<K, V> {
    pub fn new() -> JudyHS<K, V> {
        JudyHS{m: null_mut(), key_type: PhantomData, value_type: PhantomData}
    }

    pub fn insert(&mut self, key: K, value: &V) -> bool {
        unsafe {
            let kk = &key as *const K;
            let v = JudyHSIns(&mut self.m, kk as Pcvoid_t, size_of::<K>() as Word_t, null_mut());
            if v == null_mut() {
                false
            } else if *v != null_mut() {
                false
            } else {
                *v = transmute(value);
                true
            }
        }
    }

    pub fn get<'a>(&'a self, key: K) -> Option<&'a V> {
        unsafe {
            let kk = &key as *const K;
            let v = JudyHSGet(self.m as Pcvoid_t, kk as Pcvoid_t, size_of::<K>() as Word_t);
            if v == null_mut() {
                None
            } else {
                Some(transmute(*v))
            }
        }
    }

    pub fn free(&mut self) -> Word_t {
        if self.m != null_mut() {
            let ret = unsafe { JudyHSFreeArray(&mut self.m, null_mut()) };
            assert!(self.m == null_mut());
            ret
        } else {
            0
        }
    }
}

#[derive(Clone)]
pub struct JudyLIterator<V> {
    m: Pcvoid_t,
    i: Word_t,
    value_type: PhantomData<V>,
}

impl<V> Iterator for JudyLIterator<V> {
    type Item = (Word_t, V);

    fn next(&mut self) -> Option<(Word_t, V)> {
        unsafe {
            let v = JudyLNext(self.m, &mut self.i, null_mut());
            if v == null_mut() {
                None
            } else {
                let vv = *v as *mut V;
                Some((self.i, transmute_copy(&*vv)))
            }
        }
    }
}

impl<V> Drop for JudyL<V> {
    fn drop(&mut self) {
        self.free();
    }
}

impl<K, V> Drop for JudyHS<K, V> {
    fn drop(&mut self) {
        self.free();
    }
}

//impl<V> RandomAccessIterator<(Word_t, V)> for JudyLIterator<V> {
//    type Item = (Word_t, V);
//
//    fn indexable(&self) -> usize {
//        unsafe {
//            JudyLCount(self.m, 0, -1, null_mut()) as usize
//        }
//    }
//
//    fn idx(&self, index: usize) -> Option<(Word_t, V)> {
//        unsafe {
//            // TODO: maybe JudyLByCount would be better here?
//            let v = JudyLGet(self.m, index as Word_t, null_mut());
//            if v == null_mut() {
//                None
//            } else {
//                let vv = *v as *mut V;
//                Some((self.i, transmute_copy(&*vv)))
//
//            }
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_judyhs() {
        let mut h = JudyHS::<u32, u32>::new();
        assert!(h.insert(123, &456));
        match h.get(123) {
            Some(x) => assert_eq!(456, *x),
            None => panic!(),
        }
        assert!(h.free() > 0);
    }

    #[test]
    fn test_judyl() {
        let mut h = JudyL::<u32>::new();
        assert!(h.insert(123, &456));
        match h.get(123) {
            Some(x) => assert_eq!(456, *x),
            None => panic!(),
        }

        let mut it = h.iter();
        assert_eq!(Some((123, 456)), it.next());
        assert_eq!(None, it.next());
        for (i, v) in h.iter() {
            println!("i: {:?} v: {:?}", i, v);
        }
        assert!(h.free() > 0);
    }
}
