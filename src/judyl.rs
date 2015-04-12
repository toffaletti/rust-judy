use super::capi::*;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::mem::{transmute, transmute_copy};

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
            let v = JudyLGet(self.m, index, null_mut());
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
        JudyLIterator{ m: self.m, i: 0, value_type: PhantomData}
    }

    pub fn count(&self, index1: Word_t, index2: Word_t) -> Word_t {
        unsafe {
            JudyLCount(self.m, index1, index2, null_mut())
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
