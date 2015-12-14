use super::capi::*;
//use super::judyl::JudyL;
use std::ptr::null_mut;
use std::marker::PhantomData;
use std::hash::{Hash, Hasher, SipHasher};
//use std::rand::{self, Rng};
use std::boxed::into_raw;

pub struct Map<K, V> {
    hash_state: SipHasher,
    j: Pvoid_t,
    key_type: PhantomData<K>,
    value_type: PhantomData<V>,
}

pub enum Entry<'a, K: 'a, V: 'a> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

pub struct VacantEntry<'a, K:'a, V:'a> {
    v: &'a mut Vec<(K, V)>,
    key: K,
}

pub struct OccupiedEntry<'a, K:'a, V:'a> {
    v: &'a mut Vec<(K, V)>,
}


impl<K, V> Map<K, V> where K: Eq + Hash {
    pub fn new() -> Map<K, V> {
        //let mut r = rand::thread_rng();
        Map{
            //hash_state: SipHasher::new_with_keys(r.gen(), r.gen()),
            hash_state: SipHasher::new(),
            j: null_mut(),
            key_type: PhantomData,
            value_type: PhantomData,
        }
    }

    pub fn insert(&mut self, k: K,  v: V) -> Option<V> {
        let mut state = self.hash_state.clone();
        k.hash(&mut state);
        let hash = state.finish();
        unsafe {
            let v = JudyLIns(&mut self.j, hash, null_mut());
            if v == null_mut() {
                None
            } else if *v != null_mut() {
                // occupied
                None
            } else {
                let vec = Box::new(Vec::<(K, V)>::new());
                *v = into_raw(vec) as Pvoid_t;
                None
            }
        }
    }
}
