use super::capi::*;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::mem::size_of;
use std::mem::transmute;

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
            let v = JudyHSGet(self.m, kk as Pcvoid_t, size_of::<K>() as Word_t);
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

impl<K, V> Drop for JudyHS<K, V> {
    fn drop(&mut self) {
        self.free();
    }
}


