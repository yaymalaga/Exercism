use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let hashmap = Arc::new(Mutex::new(HashMap::new()));

    let hashmap_copy = hashmap.clone();
    *hashmap_copy.lock().unwrap()

    //Arc::try_unwrap(hashmap).unwrap().lock().unwrap()
}
