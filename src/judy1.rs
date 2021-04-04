use super::capi::*;
use std::ptr::null_mut;

pub struct Judy1 {
    m: Pvoid_t,
}

impl Default for Judy1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Judy1 {
    pub fn new() -> Judy1 {
        Judy1 { m: null_mut() }
    }

    pub fn set(&mut self, index: Word_t) -> bool {
        let prev = unsafe { Judy1Set(&mut self.m, index, null_mut()) };
        prev == 1
    }

    pub fn set_bulk_sorted(&mut self, count: Word_t, keys: &[Word_t]) -> bool {
        assert!(
            keys.len() as u64 >= count,
            "Judy1::set_bulk_sorted: Keys array shorter than count argument!"
        );
        unsafe {
            let result = Judy1SetArray(&mut self.m, count, keys.as_ptr(), null_mut());
            result == 0 || result == 1
        }
    }

    pub fn unset(&mut self, index: Word_t) -> bool {
        let prev = unsafe { Judy1Unset(&mut self.m, index, null_mut()) };
        prev == 1
    }

    pub fn test(&mut self, index: Word_t) -> bool {
        1 == unsafe { Judy1Test(self.m, index, null_mut()) }
    }

    pub fn free(&mut self) -> Word_t {
        if self.m.is_null() {
            unsafe {
                let ret = Judy1FreeArray(&mut self.m, null_mut());
                assert!(self.m.is_null());
                ret
            }
        } else {
            0
        }
    }

    pub fn iter(&self) -> Judy1Iterator<'_> {
        Judy1Iterator { j: self, i: 0 }
    }

    pub fn count(&self, index1: Word_t, index2: Word_t) -> Word_t {
        unsafe { Judy1Count(self.m, index1, index2, null_mut()) }
    }

    pub fn clear(&mut self) {
        self.free();
    }

    pub fn len(&self) -> usize {
        self.count(0, Word_t::max_value()) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.m.is_null()
    }
}

pub struct Judy1Iterator<'a> {
    j: &'a Judy1,
    i: Word_t,
}

impl<'a> Iterator for Judy1Iterator<'a> {
    type Item = Word_t;

    fn next(&mut self) -> Option<Word_t> {
        unsafe {
            let v = Judy1Next(self.j.m, &mut self.i, null_mut());
            if v == 0 {
                None
            } else {
                Some(self.i)
            }
        }
    }
}

impl Drop for Judy1 {
    fn drop(&mut self) {
        self.free();
    }
}
