use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut count: HashMap<i32, i32> = HashMap::new();
        for i in arr {
            let entry = count.entry(i).or_default();
            *entry += 1;
        }
        let set: HashSet<i32> = count.values().cloned().collect();
        count.len() == set.len()
    }
}
