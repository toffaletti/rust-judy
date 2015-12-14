use super::capi::*;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::mem::size_of;

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

    pub fn remove(&mut self, key: K) -> bool {
        // TODO: couldn't find a good way to take a &K
        // shouldn't need to consume key
        unsafe {
            let ks = key.into();
            1 == JudyHSDel(&mut self.m, ks.as_ptr() as Pcvoid_t, ks.len() as Word_t, null_mut())
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

    pub fn clear(&mut self) {
        self.free();
    }

    pub fn is_empty(&self) -> bool {
        self.m == null_mut()
    }

    pub fn iter<'a>(&'a self) -> JudyHSIterator<'a, K> {
        JudyHSIterator{j: self, len: 0}
    }
}

pub struct JudyHSIterator<'a, K:'a> where K: Into<Vec<u8>> {
    j: &'a JudyHS<K>,
    len: Word_t,
}

impl<'a, K: Into<Vec<u8>>> Iterator for JudyHSIterator<'a, K> {
    type Item = (Word_t, Word_t);

    fn next(&mut self) -> Option<(Word_t, Word_t)> {
        unsafe {
            let v = JudyLNext(self.j.m, &mut self.len, null_mut());
            if v == null_mut() {
                None
            } else {
                if self.len as usize > size_of::<Word_t>() {
                    // TODO
                } else {
                }
                Some((self.len, *v as Word_t))
            }
        }
    }
}

impl<K :Into<Vec<u8>>> Drop for JudyHS<K> {
    fn drop(&mut self) {
        self.free();
    }
}

