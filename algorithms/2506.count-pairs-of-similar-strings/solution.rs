use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn similar_pairs(words: Vec<String>) -> i32 {
        let mut count: HashMap<BTreeSet<u8>, i32> = HashMap::new();
        for word in words {
            let bytes: BTreeSet<u8> = word.as_bytes().iter().cloned().collect();
            let e = count.entry(bytes).or_default();
            *e += 1;
        }
        count.iter().fold(0, |sum, (_, v)| sum + (v * (v - 1) / 2))
    }
}
