use std::collections::HashMap;

#[allow(dead_code)]
struct TimeMap {
    store: HashMap<String, Vec<(String, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl TimeMap {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.store.entry(key).or_insert(Vec::new());
        entry.push((value, timestamp));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(tv) = self.store.get(&key) {
            return match tv.binary_search_by_key(&timestamp, |&(_, t)| t) {
                Ok(i) => tv[i].0.clone(),
                Err(i) => {
                    if i > 0 {
                        tv[i - 1].0.clone()
                    } else {
                        String::from("")
                    }
                }
            };
        }
        String::from("")
    }
}
