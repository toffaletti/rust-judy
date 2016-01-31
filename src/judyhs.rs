use super::capi::*;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::mem::size_of;

trait SizedPtr {
    fn len(&self) -> usize;
    fn as_ptr(&self) -> *const u8;
}

impl SizedPtr for str {
    fn len(&self) -> usize {
        return self.len()
    }

    fn as_ptr(&self) -> *const u8 {
        return self.as_ptr()
    }
}

impl<K> SizedPtr for [K] {
    fn len(&self) -> usize {
        return self.len()
    }

    fn as_ptr(&self) -> *const u8 {
        return self.as_ptr() as *const u8;
    }
}

impl<K> SizedPtr for K {
    fn len(&self) -> usize {
        return size_of::<K>();
    }

    fn as_ptr(&self) -> *const u8 {
        return self as *const K as *const u8;
    }
}

pub struct JudyHS<K:?Sized> {
    m: Pvoid_t,
    key_type: PhantomData<*const K>,
}

impl<K:?Sized > JudyHS<K>{
    pub fn new() -> JudyHS<K> {
        JudyHS{m: null_mut(), key_type: PhantomData}
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

    pub fn clear(&mut self) {
        self.free();
    }

    pub fn is_empty(&self) -> bool {
        self.m == null_mut()
    }
}

impl<K: SizedPtr + ?Sized> JudyHS<K> {
    pub fn insert(&mut self, key: &K, value: Word_t) -> bool {
        unsafe {
            let v = JudyHSIns(&mut self.m, key.as_ptr() as Pcvoid_t, key.len() as Word_t, null_mut());
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

    pub fn get(&self, key: &K) -> Option<Word_t> {
        unsafe {
            let v = JudyHSGet(self.m, key.as_ptr() as Pcvoid_t, key.len() as Word_t);
            if v == null_mut() {
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
            1 == JudyHSDel(&mut self.m, key.as_ptr() as Pcvoid_t, key.len() as Word_t, null_mut())
        }
    }
}

impl<K:?Sized> Drop for JudyHS<K> {
    fn drop(&mut self) {
        self.free();
    }
}
