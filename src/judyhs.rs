use super::capi::*;
use std::ptr::null_mut;
use std::marker::PhantomData;

pub struct JudyHS<K :Into<Vec<u8>>> {
    m: Pvoid_t,
    key_type: PhantomData<K>,
}

impl<K :Into<Vec<u8>>> JudyHS<K> {
    pub fn new() -> JudyHS<K> {
        JudyHS{m: null_mut(), key_type: PhantomData}
    }

    pub fn insert(&mut self, key: K, value: Word_t) -> bool {
        unsafe {
            let ks = key.into();
            let v = JudyHSIns(&mut self.m, ks.as_ptr() as Pcvoid_t, ks.len() as Word_t, null_mut());
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

    pub fn get(&self, key: K) -> Option<Word_t> {
        unsafe {
            let ks = key.into();
            let v = JudyHSGet(self.m, ks.as_ptr() as Pcvoid_t, ks.len() as Word_t);
            if v == null_mut() {
                None
            } else {
                Some(*v as Word_t)
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

impl<K :Into<Vec<u8>>> Drop for JudyHS<K> {
    fn drop(&mut self) {
        self.free();
    }
}


