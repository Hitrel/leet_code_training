use std::collections::{HashMap};

struct MyHashSet {
    set: HashMap<i32, ()>,
}

impl MyHashSet {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            set: HashMap::new(),
        }
    }
    
    fn add(&mut self, key: i32) {
        self.set.insert(key, ());
    }
    
    fn remove(&mut self, key: i32) {
        self.set.remove(&key);
    }
    
    /** Returns true if this set contains the specified element */
    fn contains(&self, key: i32) -> bool {
        self.set.contains_key(&key)
    }
}