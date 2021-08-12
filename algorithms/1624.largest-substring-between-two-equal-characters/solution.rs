use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut longest = -1;
        let mut map: HashMap<char, i32> = HashMap::new();

        for (i, c) in s.chars().enumerate() {
            let entry = map.entry(c).or_insert(-1);
            if *entry == -1 {
                *entry = i as i32;
            } else {
                longest = max(longest, i as i32 - *entry - 1);
            }
        }

        longest
    }
}
