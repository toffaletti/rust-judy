use super::capi::*;
use std::ptr::null_mut;

pub struct JudyL {
    m: Pvoid_t,
}

impl Default for JudyL {
    fn default() -> Self {
        Self::new()
    }
}

impl JudyL {
    pub fn new() -> JudyL {
        JudyL { m: null_mut() }
    }

    pub fn insert(&mut self, index: Word_t, value: Word_t) -> bool {
        unsafe {
            let v = JudyLIns(&mut self.m, index, null_mut());
            if v.is_null() || !(*v).is_null() {
                false
            } else {
                *v = value as Pvoid_t;
                true
            }
        }
    }

    /// Warning: This function requires the keys to be sorted in relation to data already in the array.
    /// That means even if this set of keys is sorted, if it's out of sequence with the keys in
    /// the array then the function will fail.
    pub fn insert_bulk_sorted(&mut self, count: Word_t, keys: &[Word_t], vals: &[Word_t]) -> bool {
        assert!(
            keys.len() as u64 >= count && vals.len() as u64 >= count,
            "JudyL::insert_bulk_sorted: Keys or values array shorter than count argument!"
        );
        unsafe {
            let result =
                JudyLInsArray(&mut self.m, count, keys.as_ptr(), vals.as_ptr(), null_mut());
            result == 0 || result == 1
        }
    }

    pub fn get(&self, index: Word_t) -> Option<Word_t> {
        unsafe {
            let v = JudyLGet(self.m, index, null_mut());
            if v.is_null() {
                None
            } else {
                Some(*v as Word_t)
            }
        }
    }

    pub fn free(&mut self) -> Word_t {
        if !self.m.is_null() {
            unsafe {
                let ret = JudyLFreeArray(&mut self.m, null_mut());
                assert!(self.m.is_null());
                ret
            }
        } else {
            0
        }
    }

    pub fn iter(&self) -> JudyLIterator<'_> {
        JudyLIterator { j: self, i: 0 }
    }

    pub fn count(&self, index1: Word_t, index2: Word_t) -> Word_t {
        unsafe { JudyLCount(self.m, index1, index2, null_mut()) }
    }

    pub fn remove(&mut self, index: &Word_t) -> bool {
        1 == unsafe { JudyLDel(&mut self.m, *index, null_mut()) }
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

pub struct JudyLIterator<'a> {
    j: &'a JudyL,
    i: Word_t,
}

impl<'a> Iterator for JudyLIterator<'a> {
    type Item = (Word_t, Word_t);

    fn next(&mut self) -> Option<(Word_t, Word_t)> {
        unsafe {
            let v = JudyLNext(self.j.m, &mut self.i, null_mut());
            if v.is_null() {
                None
            } else {
                Some((self.i, *v as Word_t))
            }
        }
    }
}

impl Drop for JudyL {
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
