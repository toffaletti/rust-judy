use super::capi::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::null_mut;

pub struct JudySL {
    m: Pvoid_t,
}

impl JudySL {
    pub fn new() -> JudySL {
        JudySL { m: null_mut() }
    }

    pub fn insert(&mut self, key: &str, value: Word_t) -> bool {
        unsafe {
            let ks = CString::from_vec_unchecked(key.into());
            let v = JudySLIns(&mut self.m, ks.as_ptr() as *const u8, null_mut());
            if v == null_mut() {
                false
            } else if *v != null_mut() {
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
            if v == null_mut() {
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
        if self.m != null_mut() {
            let ret = unsafe { JudySLFreeArray(&mut self.m, null_mut()) };
            assert!(self.m == null_mut());
            ret
        } else {
            0
        }
    }

    pub fn iter<'a>(&'a self) -> JudySLIterator<'a> {
        JudySLIterator {
            sl: self,
            k: [0; 1024],
        }
    }

    pub fn clear(&mut self) {
        self.free();
    }

    pub fn is_empty(&self) -> bool {
        self.m == null_mut()
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
            if v == null_mut() {
                None
            } else {
                Some((CStr::from_ptr(self.k.as_ptr() as *const i8), *v as Word_t))
            }
        }
    }
}
