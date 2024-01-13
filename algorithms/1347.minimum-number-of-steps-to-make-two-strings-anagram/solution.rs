use std::collections::HashMap;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut map: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_default() += 1;
        }
        for c in t.chars() {
            *map.entry(c).or_default() -= 1;
        }
        map.values().filter(|&v| *v > 0).sum()
    }

