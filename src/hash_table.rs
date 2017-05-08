use std::hash::{Hash, Hasher};
use std::clone::Clone;
use std::collections::hash_map::DefaultHasher;
use std::marker::PhantomData;
use std::fmt::Debug;

pub struct HashTable<K, V>{
    elements: Vec<Option<V>>,
    key_type: PhantomData<K>,
}

impl<K, V> HashTable<K, V> where K: Hash + Debug, V: Clone {
    pub fn new() -> HashTable<K, V> {
        HashTable { elements: Vec::<Option<V>>::new(), key_type: PhantomData }
    }

    pub fn set(&mut self, key: K, value: V) {
        let hashed_key = HashTable::<K, V>::hash(&key);

        // The following is TERRIBLE, but then again, I'm using a vector
        // to store data by arbitrary keys ¯\_(ツ)_/¯
        if self.elements.len() < hashed_key {
            self.elements.resize(hashed_key + 1, None);
        }

        self.elements[hashed_key] = Some(value);
    }

    pub fn get(&self, key: K) -> &V {
        let hashed_key = HashTable::<K, V>::hash(&key);

        match self.elements[hashed_key] {
            Some(ref value) => value,
            None => panic!("Value not set for key {:?}", key),
        }
    }

    pub fn remove(&mut self, key: K) {
        let hashed_key = HashTable::<K, V>::hash(&key);
        self.elements[hashed_key] = None;
    }

    fn hash(k: &K) -> usize {
        let mut s = DefaultHasher::new();
        k.hash(&mut s);

        // Some more horrors - if I allow the hashed number to become too
        // big I get out of memory errors. Again, I'm using a vector to store
        // sparsely distributed keys over a big domain so these kind of things
        // are bound to happen. I'm not going to care about collisions like
        // the original repo, so for this implementation this should be fine.
        (s.finish() / 10000000000000) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get_element() {
        let mut hash_table = HashTable::new();
        let key = 42;
        let value = "Arthur Dent";
        hash_table.set(key, value);
        assert_eq!(value, *hash_table.get(key));
    }

    #[test]
    #[should_panic]
    fn panics_if_getting_non_set_element() {
        let hash_table = HashTable::<i32, &str>::new();
        hash_table.get(42);
    }

    #[test]
    #[should_panic]
    fn removes_an_element() {
        let mut hash_table = HashTable::new();
        hash_table.set(42, "Arthur Dent");
        hash_table.remove(42);
        hash_table.get(42);
    }
}