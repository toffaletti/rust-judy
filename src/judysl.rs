#![allow(clippy::upper_case_acronyms)]

use super::capi::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::null_mut;

pub struct JudySL {
    m: Pvoid_t,
}

impl Default for JudySL {
    fn default() -> Self {
        Self::new()
    }
}

impl JudySL {
    pub fn new() -> JudySL {
        JudySL { m: null_mut() }
    }

    pub fn insert(&mut self, key: &str, value: Word_t) -> bool {
        unsafe {
            let ks = CString::from_vec_unchecked(key.into());
            let v = JudySLIns(&mut self.m, ks.as_ptr() as *const u8, null_mut());
            if v.is_null() || !(*v).is_null() {
                false
            } else {
                *v = value as Pvoid_t;
                true
            }
        }
    }

    pub fn get(&self, key: &str) -> Option<Word_t> {
        unsafe {
            let ks = CString::from_vec_unchecked(key.into());
            let v = JudySLGet(self.m, ks.as_ptr() as *const u8);
            if v.is_null() {
                None
            } else {
                Some(*v as Word_t)
            }
        }
    }

    pub fn remove(&mut self, key: &str) -> bool {
        unsafe {
            let ks = CString::from_vec_unchecked(key.into());
            1 == JudySLDel(&mut self.m, ks.as_ptr() as *const u8, null_mut())
        }
    }

    pub fn free(&mut self) -> Word_t {
        if !self.m.is_null() {
            let ret = unsafe { JudySLFreeArray(&mut self.m, null_mut()) };
            assert!(self.m.is_null());
            ret
        } else {
            0
        }
    }

    pub fn iter(&self) -> JudySLIterator<'_> {
        JudySLIterator {
            sl: self,
            k: [0; 1024],
        }
    }

    pub fn clear(&mut self) {
        self.free();
    }

    pub fn is_empty(&self) -> bool {
        self.m.is_null()
    }
}

impl Drop for JudySL {
    fn drop(&mut self) {
        self.free();
    }
}

pub struct JudySLIterator<'a> {
    sl: &'a JudySL,
    k: [u8; 1024],
}

impl<'a> Iterator for JudySLIterator<'a> {
    type Item = (&'a CStr, Word_t);

    fn next(&mut self) -> Option<(&'a CStr, Word_t)> {
        unsafe {
            let v = JudySLNext(self.sl.m, self.k.as_mut_ptr(), null_mut());
            if v.is_null() {
                None
            } else {
                Some((CStr::from_ptr(self.k.as_ptr() as *const i8), *v as Word_t))
            }
        }
    }
}
