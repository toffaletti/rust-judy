use super::capi::*;
use std::ptr::null_mut;

pub struct Judy1 {
    m: Pvoid_t,
}

impl Judy1 {
    pub fn new() -> Judy1 {
        Judy1{m: null_mut()}
    }

    pub fn set(&mut self, index: Word_t) -> bool {
        let prev = unsafe { Judy1Set(&mut self.m, index, null_mut()) };
        prev == 1
    }

    pub fn unset(&mut self, index: Word_t) -> bool {
        let prev = unsafe { Judy1Unset(&mut self.m, index, null_mut()) };
        prev == 1
    }

    pub fn test(&mut self, index: Word_t) -> bool {
        1 == unsafe { Judy1Test(self.m, index, null_mut()) }
    }

    pub fn free(&mut self) -> Word_t {
        if self.m != null_mut() {
            unsafe {
                let ret = Judy1FreeArray(&mut self.m, null_mut());
                assert!(self.m == null_mut());
                ret
            }
        } else {
            0
        }
    }

    pub fn iter(& self) -> Judy1Iterator {
        Judy1Iterator{m: self.m, i: 0}
    }
}

#[derive(Clone)]
pub struct Judy1Iterator {
    m: Pcvoid_t,
    i: Word_t,
}

impl Iterator for Judy1Iterator {
    type Item = Word_t;

    fn next(&mut self) -> Option<(Word_t)> {
        unsafe {
            let v = Judy1Next(self.m, &mut self.i, null_mut());
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


