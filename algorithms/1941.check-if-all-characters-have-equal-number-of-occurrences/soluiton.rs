use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut count: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        count
            .into_iter()
            .map(|(_, n)| n)
            .collect::<HashSet<i32>>()
            .len()
            == 1
    }
}
