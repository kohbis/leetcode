use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .map(|(_, n)| n)
            .collect::<HashSet<i32>>()
            .len()
            == 1
    }
}
