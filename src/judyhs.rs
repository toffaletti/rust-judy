#![allow(clippy::upper_case_acronyms)]

use super::capi::*;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ptr::null_mut;

pub trait SizedPtr {
    fn len(&self) -> usize;
    fn as_ptr(&self) -> *const u8;
}

impl SizedPtr for str {
    fn len(&self) -> usize {
        self.len()
    }

    fn as_ptr(&self) -> *const u8 {
        self.as_ptr()
    }
}

impl<K> SizedPtr for [K] {
    fn len(&self) -> usize {
        self.len()
    }

    fn as_ptr(&self) -> *const u8 {
        self.as_ptr() as *const u8
    }
}

impl<K> SizedPtr for K {
    fn len(&self) -> usize {
        size_of::<K>()
    }

    fn as_ptr(&self) -> *const u8 {
        self as *const K as *const u8
    }
}

pub struct JudyHS<K: ?Sized> {
    m: Pvoid_t,
    key_type: PhantomData<*const K>,
}

impl<K: ?Sized> Default for JudyHS<K> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: ?Sized> JudyHS<K> {
    pub fn new() -> JudyHS<K> {
        JudyHS {
            m: null_mut(),
            key_type: PhantomData,
        }
    }

    pub fn free(&mut self) -> Word_t {
        if !self.m.is_null() {
            let ret = unsafe { JudyHSFreeArray(&mut self.m, null_mut()) };
            assert!(self.m.is_null());
            ret
        } else {
            0
        }
    }

    pub fn clear(&mut self) {
        self.free();
    }

    pub fn is_empty(&self) -> bool {
        self.m.is_null()
    }
}

impl<K: SizedPtr + ?Sized> JudyHS<K> {
    pub fn insert(&mut self, key: &K, value: Word_t) -> bool {
        unsafe {
            let v = JudyHSIns(
                &mut self.m,
                key.as_ptr() as Pcvoid_t,
                key.len() as Word_t,
                null_mut(),
            );
            if v.is_null() || !(*v).is_null() {
                false
            } else {
                *v = value as Pvoid_t;
                true
            }
        }
    }

    pub fn get(&self, key: &K) -> Option<Word_t> {
        unsafe {
            let v = JudyHSGet(self.m, key.as_ptr() as Pcvoid_t, key.len() as Word_t);
            if v.is_null() {
                None
            } else {
                Some(*v as Word_t)
            }
        }
    }

    pub fn remove(&mut self, key: &K) -> bool {
        // TODO: couldn't find a good way to take a &K
        // shouldn't need to consume key
        unsafe {
            1 == JudyHSDel(
                &mut self.m,
                key.as_ptr() as Pcvoid_t,
                key.len() as Word_t,
                null_mut(),
            )
        }
    }
}

impl<K: ?Sized> Drop for JudyHS<K> {
    fn drop(&mut self) {
        self.free();
    }
}
