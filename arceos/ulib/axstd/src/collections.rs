pub use alloc::collections::*;
use core::{
    hash::{self, Hash, Hasher},
    ops::Index,
};

use alloc::vec::Vec;

struct MyHasher(u64);

impl Hasher for MyHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 ^= *byte as u64;
            self.0 = self.0.wrapping_mul(0x100000001b3);
        }
    }
}

pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    resource_nums: usize,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap {
            buckets: Vec::new(),
            resource_nums: 0,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets
            .iter()
            .flat_map(|bucket| bucket.iter().map(|(k, v)| (k, v)))
    }
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn resize(&mut self) {
        let new_len = if self.resource_nums == 0 {
            16
        } else {
            self.buckets.len() * 2
        };
        let new_buckets: Vec<Vec<(K, V)>> = (0..new_len).map(|_| Vec::new()).collect();

        let old_buckets = core::mem::replace(&mut self.buckets, new_buckets);

        for bucket in old_buckets {
            for (k, v) in bucket {
                let index = Self::get_hashindex(&k, new_len);
                self.buckets[index].push((k, v));
            }
        }
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.resource_nums == 0 || self.resource_nums * 4 > self.buckets.len() * 3 {
            self.resize();
        }

        let index = Self::get_hashindex(&key, self.buckets.len());

        for (k, v) in self.buckets[index].iter_mut() {
            if *k == key {
                let old_v = core::mem::replace(v, value);
                return Some(old_v);
            }
        }
        self.buckets[index].push((key, value));
        self.resource_nums += 1;
        None
    }

    fn get_hashindex(key: &K, len: usize) -> usize {
        let mut hasher = MyHasher(0xcbf29ce484222325);
        key.hash(&mut hasher);
        (hasher.finish() % len as u64) as usize
    }
}
